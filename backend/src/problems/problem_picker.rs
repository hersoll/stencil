use crate::{
    RegistryError, db,
    errors::ApiError,
    pdf_generation::ProblemSetSpec,
    problems::{Difficulty, Problem},
};
use anyhow::{Context, Result, anyhow};
use std::{cmp::Ordering, collections::HashSet};

use rand::{rngs::ThreadRng, seq::IndexedRandom};

const DEFAULT_SCORE: u8 = 100;

// Ratios when choosing a difficulty level (0-10) within a Difficulty
const EASY_MEDIUM_RATIO: f32 = 0.60;
const MEDIUM_HARD_RATIO: f32 = 0.60;
const EASY_HARD_RATIO: f32 = 0.70;
const EASY_RATIO: f32 = 0.40;
const MEDIUM_RATIO: f32 = 0.30;

/// Problem that is a candidate for selection.
/// Data grouping for easier ergonomics when choosing problems
///
/// The problem_picker will filter down the candidate list throughout the module,
/// until problems are generated
#[derive(Debug, Clone, Eq)]
pub struct ProblemCandidate {
    pub name: String,
    pub difficulty: u8,
    /// The "score" is what the module uses to determine which problem is chosen.
    /// When a problem is generated, that problem's score is lowered.
    ///
    /// This assures a variety of problems (if there is variety, that is)
    pub score: u8,
    pub identifiers: HashSet<Vec<i32>>,
}

impl PartialEq for ProblemCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub type ProblemGenerator = fn(String, &str) -> Result<Problem>;

/// Defines which problems are available to choose from and which
/// criteria the problem_picker functions can use to
/// choose which problems to generate
#[derive(Debug)]
pub struct ProblemPool {
    problem_candidates: Vec<ProblemCandidate>,
    lang: String,
    starting_difficulty: Difficulty,
    ending_difficulty: Difficulty,
    n: u8,
}
/// Designates which difficulties contain problems
/// and thus can be used when generating problems
struct AvailableDifficulties {
    intro: bool,
    easy: bool,
    medium: bool,
    hard: bool,
}

/// How many problems there should be within each Difficulty
struct DifficultyDistribution {
    intro: u8,
    easy: u8,
    medium: u8,
    hard: u8,
}

impl DifficultyDistribution {
    fn get(&self, difficulty: Difficulty) -> u8 {
        match difficulty {
            Difficulty::Intro => self.intro,
            Difficulty::Easy => self.easy,
            Difficulty::Medium => self.medium,
            Difficulty::Hard => self.hard,
        }
    }
}

/// The entry point of the module.
///
/// Given the SetInformation, finds the appropriate problems
/// and distributes them appropriately across the difficulties given
pub async fn generate_problems_for_set(
    set: ProblemSetSpec,
    lang: String,
) -> Result<Vec<Problem>, ApiError> {
    let problems = db::problems::get_problem_names_from_topics(set.topics, set.exclusions).await?;
    // Construct an initial list of candidates from the problem names,
    // with some default values for now
    let problem_candidates: Vec<ProblemCandidate> = problems
        .into_iter()
        .map(|problem| {
            Ok(ProblemCandidate {
                difficulty: get_problem_difficulty(&problem)?,
                name: problem,
                score: DEFAULT_SCORE.max(set.n),
                identifiers: HashSet::new(),
            })
        })
        .collect::<Result<Vec<ProblemCandidate>>>()?;

    let mut problem_pool = ProblemPool {
        problem_candidates,
        lang,
        starting_difficulty: set.starting_difficulty,
        ending_difficulty: set.ending_difficulty,
        n: set.n,
    };
    choose_problems(&mut problem_pool)?;
    let distribution_by_difficulty_num = distribute_problems(&problem_pool)?;

    let mut rng = rand::rng();
    let problem_set =
        generate_problems(&mut problem_pool, distribution_by_difficulty_num, &mut rng)?;
    Ok(problem_set)
}

/// Filters the ProblemPool depending on the difficulty range
fn choose_problems(problem_pool: &mut ProblemPool) -> Result<(), ApiError> {
    // A range of numbers, for example 3..7, as a Vec
    let difficulty_range = Difficulty::enums_to_nums(
        problem_pool.starting_difficulty,
        problem_pool.ending_difficulty,
    );

    // Retain all problems which match the desired difficulties
    // TODO: Test the error
    problem_pool
        .problem_candidates
        .retain(|candidate| difficulty_range.contains(&candidate.difficulty));
    if problem_pool.problem_candidates.is_empty() {
        return Err(ApiError::BadRequest(
            "No valid problems in difficulty range".to_string(),
        ));
    } else {
        Ok(())
    }
}

/// This function finds out how many of each difficulty number should be generated.
///
/// After that's done, it also assigns
fn distribute_problems(problem_pool: &ProblemPool) -> Result<[u8; 11]> {
    let counts = get_count_per_difficulty(problem_pool)?;
    let distribution_per_difficulty_number =
        distribute_problems_by_difficulty_number(problem_pool, &counts)?;

    Ok(distribution_per_difficulty_number)
}

fn generate_problems(
    problem_pool: &mut ProblemPool,
    distribution_by_difficulty_num: [u8; 11],
    rng: &mut ThreadRng,
) -> Result<Vec<Problem>> {
    // The actual generated problems
    let mut problems = Vec::new();

    // Take all candidates and sort them into difficulty categories.
    // This will speed up the problem generation significantly
    let mut problem_indices_by_difficulty: [Vec<usize>; 11] = Default::default();
    for (i, candidate) in problem_pool.problem_candidates.iter().enumerate() {
        let difficulty = candidate.difficulty as usize;
        //Check bounds
        if difficulty < problem_indices_by_difficulty.len() {
            problem_indices_by_difficulty[difficulty].push(i);
        }
    }

    // This loop goes through each difficulty number, and finds the max score of that
    // difficulty number. If several problems has the max score, one is chosen at random.
    // It generates a problem of that type, and lowers that problem's score.
    // Indices are tracked to be able to change the relevant score in `candidates_with_scores`
    for (difficulty, &count) in distribution_by_difficulty_num.iter().enumerate() {
        let indices = &problem_indices_by_difficulty[difficulty];
        // Skip difficulties that don't have problems
        if indices.is_empty() || count == 0 {
            continue;
        }

        for _ in 0..count {
            // Find the max score among the indices
            let mut max_score = 0u8;
            let mut best_indices = Vec::new();

            for &idx in indices {
                let candidate = &problem_pool.problem_candidates[idx];
                match candidate.score.cmp(&max_score) {
                    Ordering::Greater => {
                        max_score = candidate.score;
                        best_indices.clear();
                        best_indices.push(idx);
                    }
                    Ordering::Equal => {
                        best_indices.push(idx);
                    }
                    _ => {}
                }
            }

            let chosen_index = *best_indices
                .choose(rng)
                .context("No valid problems within the max_indices")?;

            let chosen_candidate = &mut problem_pool.problem_candidates[chosen_index];
            let problem = get_unique_problem(chosen_candidate, &problem_pool.lang)?;
            // Lower the score
            chosen_candidate.score = chosen_candidate.score.saturating_sub(1);
            problems.push(problem);
        }
    }
    Ok(problems)
}

/// Generates a problem with a unique ID (the actual numbers that makes the problem different).
///
/// If there are no more possible IDs to generate, reset (which might repeat problems)
fn get_unique_problem(candidate: &mut ProblemCandidate, lang: &str) -> Result<Problem> {
    let generator = get_generator_function(&candidate.name)?;
    let mut problem = (generator)(candidate.name.clone(), lang)?;

    // Reset if all combinations are exhausted
    if candidate.identifiers.len() >= problem.combinations {
        candidate.identifiers.clear();
    }

    let mut tries = 0u16;
    while candidate.identifiers.contains(&problem.identifiers) {
        problem = (generator)(candidate.name.clone(), lang)?;
        tries += 1;
        if tries >= u16::MAX {
            return Err(anyhow!(
                "Stuck while generating problem {}!",
                candidate.name
            ));
        }
    }
    candidate.identifiers.insert(problem.identifiers.clone());
    Ok(problem)
}

/// Checks which difficulty ranges have available problems
fn check_available_difficulties(problem_pool: &ProblemPool) -> AvailableDifficulties {
    let mut intro = false;
    let mut easy = false;
    let mut medium = false;
    let mut hard = false;

    for c in &problem_pool.problem_candidates {
        if c.difficulty <= 1 {
            intro = true;
        }
        if (2..=4).contains(&c.difficulty) {
            easy = true;
        }
        if (5..=7).contains(&c.difficulty) {
            medium = true;
        }
        if (8..=10).contains(&c.difficulty) {
            hard = true;
        }

        // Early exit if all found
        if intro && easy && medium && hard {
            break;
        }
    }

    AvailableDifficulties {
        intro,
        easy,
        medium,
        hard,
    }
}

/// Calculates how many problems there are of each Difficulty
///
/// The return might look like Ok([3, 6, 2, 1])
fn get_count_per_difficulty(problem_pool: &ProblemPool) -> Result<DifficultyDistribution> {
    if problem_pool.problem_candidates.is_empty() {
        return Err(anyhow!("get_count_per_difficulty() called with empty Vec"));
    }

    let avail = check_available_difficulties(problem_pool);

    // Calculate intro count
    let intro = if !avail.easy && !avail.medium && !avail.hard {
        // If no other difficulties found, use all problems as intro
        problem_pool.n
    } else if avail.intro {
        // Use 20% of total, capped at 5
        5.min((0.2 * problem_pool.n as f32).ceil() as u8)
    } else {
        0
    };

    let remaining = problem_pool.n - intro;
    let [easy, medium, hard] = get_problem_ratios(avail, remaining);

    Ok(DifficultyDistribution {
        intro,
        easy,
        medium,
        hard,
    })
}

/// Distributes problem counts across all difficulty numbers (0-10)
fn distribute_problems_by_difficulty_number(
    problem_pool: &ProblemPool,
    counts: &DifficultyDistribution,
) -> Result<[u8; 11]> {
    if problem_pool.problem_candidates.is_empty() {
        return Err(anyhow!(
            "Cannot distribute problems with empty candidate pool"
        ));
    }

    let mut result = [0u8; 11];

    for difficulty in Difficulty::all() {
        // How many problems should we distribute within this difficulty?
        let count = counts.get(difficulty);
        if count == 0 {
            continue;
        }

        let nums = Difficulty::enum_to_nums(difficulty);
        let distribution = distribute_within_difficulty(problem_pool, &nums, count)?;

        // Copy distribution into the appropriate slots
        for (i, &num) in nums.iter().enumerate() {
            result[num as usize] = distribution[i];
        }
    }

    Ok(result)
}

/// Distributes a count across a specific difficulty level (e.g., Easy = [2,3,4])
fn distribute_within_difficulty(
    problem_pool: &ProblemPool,
    difficulty_numbers: &[u8],
    count: u8,
) -> Result<[u8; 3]> {
    if count == 0 {
        return Ok([0, 0, 0]);
    }
    if difficulty_numbers.len() > 3 {
        return Err(anyhow!(
            "More than three numbers assigned to difficulty without changing distribute_within_difficulty(). Difficulties received: {:#?}",
            difficulty_numbers
        ));
    }

    // Check which specific difficulty numbers are available
    // NOTE: (assumes 2-3 numbers per difficulty)
    let available = AvailableDifficulties {
        intro: false,
        easy: problem_pool
            .problem_candidates
            .iter()
            .any(|c| c.difficulty == difficulty_numbers[0]),
        medium: problem_pool
            .problem_candidates
            .iter()
            .any(|c| c.difficulty == difficulty_numbers[1]),
        hard: if difficulty_numbers.len() <= 2 {
            false
        } else {
            problem_pool
                .problem_candidates
                .iter()
                .any(|c| c.difficulty == difficulty_numbers[2])
        },
    };

    Ok(get_problem_ratios(available, count))
}

/// Distributes problems across difficulty levels based on which difficulties are available
///
/// Returns [easy, medium, hard] counts that sum to n
fn get_problem_ratios(avail: AvailableDifficulties, n: u8) -> [u8; 3] {
    match (avail.easy, avail.medium, avail.hard) {
        // Only one difficulty available - use all problems for it
        (true, false, false) => [n, 0, 0],
        (false, true, false) => [0, n, 0],
        (false, false, true) => [0, 0, n],

        // Two difficulties available - split between them
        (true, true, false) => {
            let easy = (n as f32 * EASY_MEDIUM_RATIO).round() as u8;
            [easy, n - easy, 0]
        }
        (false, true, true) => {
            let medium = (n as f32 * MEDIUM_HARD_RATIO).round() as u8;
            [0, medium, n - medium]
        }
        (true, false, true) => {
            let easy = (n as f32 * EASY_HARD_RATIO).round() as u8;
            [easy, 0, n - easy]
        }

        // All three difficulties available - split three ways
        (true, true, true) => {
            // It's intentional to have one ceil() and one round().
            // This ensures that n = 1 works correctly and nets an easy problem
            let easy = (n as f32 * EASY_RATIO).ceil() as u8;
            let medium = (n as f32 * MEDIUM_RATIO).round() as u8;
            let hard = n - easy - medium;
            [easy, medium, hard]
        }

        // No difficulties available - shouldn't happen, but return zeros
        (false, false, false) => [0, 0, 0],
    }
}

/// Given a complete problem name (module_problem),
/// returns the difficulty of that problem.
///
/// Used after retrieving the problem names from
/// PROBLEM_MAP matching a HTTP request.
fn get_problem_difficulty(name: &String) -> Result<u8> {
    let difficulty = {
        let lock = crate::PROBLEM_DATA.read().expect("Mutex is poisoned");
        lock.get(name)
            .ok_or(RegistryError::ProblemNotFound {
                id: name.to_string(),
            })?
            .difficulty
    }; // Lock is dropped here

    // Validate the difficulty
    match difficulty {
        0..=10 => Ok(difficulty as u8),
        _ => Err(anyhow!(
            "Difficulty {difficulty} from problem {name} outside range."
        )),
    }
}

/// Given a complete problem name (module_problem),
/// returns a pointer to the function that generates that problem.
fn get_generator_function(name: &String) -> Result<ProblemGenerator> {
    let generator = {
        let lock = crate::PROBLEM_MAP.read().expect("Mutex is poisoned");
        lock.get(name)
            .copied()
            .ok_or(RegistryError::ProblemNotFound {
                id: name.to_string(),
            })?
    }; // Lock is dropped here

    Ok(generator)
}
