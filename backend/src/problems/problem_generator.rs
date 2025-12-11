use crate::{
    RegistryError, db,
    errors::ApiError,
    pdf_generation::ProblemSetSpec,
    problem_picker,
    problems::{Difficulty, Problem},
};
use anyhow::{Context, Result, anyhow};
use std::{cmp::Ordering, collections::HashSet};

use rand::{rngs::ThreadRng, seq::IndexedRandom};

const DEFAULT_SCORE: u8 = 100;

pub type ProblemGenerator = fn(String, &str) -> Result<Problem>;

/// Defines which problems are available to choose from and which
/// criteria the problem_picker functions can use to
/// choose which problems to generate
#[derive(Debug)]
pub struct ProblemPool {
    pub problem_candidates: Vec<ProblemCandidate>,
    pub lang: String,
    pub starting_difficulty: Difficulty,
    pub ending_difficulty: Difficulty,
    pub n: u8,
}

/// Problem that is a candidate for selection.
/// Data grouping for easier ergonomics when choosing problems
///
/// The problem_picker module will filter down the candidate list throughout the module,
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

/// The entry point of the module.
///
/// Given the SetInformation, finds the appropriate problems
/// and distributes them appropriately across the difficulties given
pub async fn generate_problem_set(
    set: ProblemSetSpec,
    lang: String,
) -> Result<Vec<Problem>, ApiError> {
    // (name, dificulty)
    let problems: Vec<(String, u8)> =
        db::problems::get_problem_names_and_difficulties_from_topics(set.topics, set.exclusions)
            .await?;
    // Construct an initial list of candidates from the problem names
    let problem_candidates: Vec<ProblemCandidate> = problems
        .into_iter()
        .map(|problem| {
            Ok(ProblemCandidate {
                name: problem.0,
                difficulty: problem.1,
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
    problem_picker::choose_problems(&mut problem_pool)?;
    let distribution_by_difficulty_num = problem_picker::distribute_problems(&problem_pool)?;

    let mut rng = rand::rng();
    let problem_set =
        generate_problems(&mut problem_pool, distribution_by_difficulty_num, &mut rng)?;
    Ok(problem_set)
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
