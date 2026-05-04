use crate::{Difficulty, generator::ProblemPool};
use types::errors::ApiError;

use anyhow::{Result, anyhow};
use tracing::debug;

// Ratios when choosing a difficulty level (0-10) within a Difficulty
const EASY_MEDIUM_RATIO: f32 = 0.60;
const MEDIUM_HARD_RATIO: f32 = 0.60;
const EASY_HARD_RATIO: f32 = 0.70;
const EASY_RATIO: f32 = 0.40;
const MEDIUM_RATIO: f32 = 0.30;

/// Designates which difficulties contain problems
/// and thus can be used when generating problems
#[derive(Debug)]
struct AvailableDifficulties {
    intro: bool,
    easy: bool,
    medium: bool,
    hard: bool,
}

/// Struct representing how many of each [`Difficulty`] number should be generated.
///
/// Has a length of 11 since the difficulties go from 0-10.
#[derive(Debug, Copy, Clone)]
pub struct CountPerDifficultyNumber(pub [u8; 11]);

/// How many problems there should be within each Difficulty
#[derive(Debug)]
struct CountPerDifficulty {
    intro: u8,
    easy: u8,
    medium: u8,
    hard: u8,
}

impl CountPerDifficulty {
    fn get_count(&self, difficulty: Difficulty) -> u8 {
        match difficulty {
            Difficulty::Intro => self.intro,
            Difficulty::Easy => self.easy,
            Difficulty::Medium => self.medium,
            Difficulty::Hard => self.hard,
        }
    }
}

/// Filters the [`ProblemPool`] to only include problems in the difficulty range
///
/// Returns an [`Err`] if the filtered pool is empty - we need at least one problem!
pub fn filter_pool_by_difficulty(problem_pool: &mut ProblemPool) -> Result<(), ApiError> {
    // A range of numbers, for example 3..7, as a Vec
    let difficulty_range = Difficulty::enums_to_nums(
        problem_pool.starting_difficulty,
        problem_pool.ending_difficulty,
    );

    problem_pool
        .problem_candidates
        .retain(|candidate| difficulty_range.contains(&candidate.difficulty));

    debug!(
        "Found {} problems within difficulty range",
        problem_pool.problem_candidates.len()
    );

    if problem_pool.problem_candidates.is_empty() {
        Err(ApiError::BadRequest(
            "No valid problems in difficulty range".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Finds out how many of each [`Difficulty`] number should be generated.
pub fn distribute_problems(problem_pool: &ProblemPool) -> Result<CountPerDifficultyNumber> {
    // How many each Easy, Medium, etc.?
    let counts = distribute_problems_by_difficulty(problem_pool)?;
    debug!("Problem distribution among difficulties: {:?}", counts);

    // Within Easy, how many from 2, 3, 4?
    let distribution_per_difficulty_number =
        distribute_problems_by_difficulty_number(problem_pool, &counts)?;
    debug!(
        "Problem distribution among difficulty numbers: {:?}",
        distribution_per_difficulty_number
    );

    Ok(distribution_per_difficulty_number)
}

/// Checks which difficulty ranges have available problems (`true`/`false`)
fn check_available_difficulties(problem_pool: &ProblemPool) -> AvailableDifficulties {
    let candidates = &problem_pool.problem_candidates;

    AvailableDifficulties {
        intro: candidates.iter().any(|c| c.difficulty <= 1),
        easy: candidates.iter().any(|c| (2..=4).contains(&c.difficulty)),
        medium: candidates.iter().any(|c| (5..=7).contains(&c.difficulty)),
        hard: candidates.iter().any(|c| (8..=10).contains(&c.difficulty)),
    }
}

/// Calculates how many problems there should be of each [`Difficulty`].
///
/// This function and it's nested functions are where most of the "business logic" of problem distribution lives.
fn distribute_problems_by_difficulty(problem_pool: &ProblemPool) -> Result<CountPerDifficulty> {
    if problem_pool.problem_candidates.is_empty() {
        return Err(anyhow!(
            "determine_difficulty_distribution() called with empty Vec"
        ));
    }

    let difficulties = check_available_difficulties(problem_pool);
    debug!("Difficulties with available problems: {:?}", difficulties);

    // Calculate intro count
    let intro = if !difficulties.easy && !difficulties.medium && !difficulties.hard {
        // If no other difficulties found, make all problems Intro
        problem_pool.n
    } else if difficulties.intro {
        // Use 20% of total, capped at 5
        5.min((0.2 * problem_pool.n as f32).ceil() as u8)
    } else {
        0
    };

    let remaining_count = problem_pool.n - intro;
    let [easy, medium, hard] = get_problem_ratios(&difficulties, remaining_count);

    Ok(CountPerDifficulty {
        intro,
        easy,
        medium,
        hard,
    })
}

/// Distributes problem counts across all difficulty numbers (0-10)
fn distribute_problems_by_difficulty_number(
    problem_pool: &ProblemPool,
    count_per_difficulty: &CountPerDifficulty,
) -> Result<CountPerDifficultyNumber> {
    if problem_pool.problem_candidates.is_empty() {
        return Err(anyhow!(
            "Cannot distribute problems with empty candidate pool"
        ));
    }

    let mut result = CountPerDifficultyNumber([0u8; 11]);

    for difficulty in Difficulty::all() {
        // How many problems should we distribute within this difficulty?
        let count = count_per_difficulty.get_count(difficulty);
        if count == 0 {
            continue;
        }

        let nums = Difficulty::enum_to_nums(difficulty);
        let distribution = distribute_within_difficulty(problem_pool, &nums, count)?;

        // Copy distribution into the appropriate slots
        for (i, &num) in nums.iter().enumerate() {
            result.0[num as usize] = distribution[i];
        }
    }

    Ok(result)
}

/// Distributes a count across a specific difficulty level (e.g., Easy = [2,3,4])
///
/// `difficulty_numbers`: The numbers to distribute between (e.g. 0, 1 or 8, 9, 10)
fn distribute_within_difficulty(
    problem_pool: &ProblemPool,
    difficulty_numbers: &[u8],
    problem_count: u8,
) -> Result<[u8; 3]> {
    if problem_count == 0 {
        return Ok([0, 0, 0]);
    }
    if difficulty_numbers.len() > 3 {
        return Err(anyhow!(
            "More than three numbers assigned to difficulty without changing distribute_within_difficulty(). Difficulties received: {:#?}",
            difficulty_numbers
        ));
    }

    // Check which specific difficulty numbers are available
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

    Ok(get_problem_ratios(&available, problem_count))
}

/// Distributes problems across difficulty levels based on which difficulties are available
///
/// Returns [easy, medium, hard] counts that sum to n
///
/// NOTE: Currently this is "hacky" since we use the same distribution for Difficulty levels and
/// Difficulty numbers. In the future, I think I want to role a specific function for numbers that
/// encourages a bit more mixing.
fn get_problem_ratios(avail: &AvailableDifficulties, n: u8) -> [u8; 3] {
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
