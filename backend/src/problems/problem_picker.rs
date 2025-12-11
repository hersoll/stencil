use crate::{
    errors::ApiError,
    problem_generator::ProblemPool,
    problems::Difficulty,
};
use anyhow::{Result, anyhow};

// Ratios when choosing a difficulty level (0-10) within a Difficulty
const EASY_MEDIUM_RATIO: f32 = 0.60;
const MEDIUM_HARD_RATIO: f32 = 0.60;
const EASY_HARD_RATIO: f32 = 0.70;
const EASY_RATIO: f32 = 0.40;
const MEDIUM_RATIO: f32 = 0.30;

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

/// Filters the ProblemPool depending on the difficulty range
pub fn choose_problems(problem_pool: &mut ProblemPool) -> Result<(), ApiError> {
    // A range of numbers, for example 3..7, as a Vec
    let difficulty_range = Difficulty::enums_to_nums(
        problem_pool.starting_difficulty,
        problem_pool.ending_difficulty,
    );

    // Retain all problems which match the desired difficulties
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

/// Finds out how many of each difficulty number should be generated.
pub fn distribute_problems(problem_pool: &ProblemPool) -> Result<[u8; 11]> {
    let counts = get_count_per_difficulty(problem_pool)?;
    let distribution_per_difficulty_number =
        distribute_problems_by_difficulty_number(problem_pool, &counts)?;

    Ok(distribution_per_difficulty_number)
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



