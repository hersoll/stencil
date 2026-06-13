use db::ProblemIdAndDifficulties;
use std::iter::zip;
use types::{
    difficulty::{AbsoluteDifficulty, RelativeDifficulty},
    errors::ApiError::{self, BadRequest},
};

/// Standard deviation for the difficulty distribution.
///
/// Currently eyeballed to make a function that "looks good".
const NORMAL_DISTRIBUTION_STDEV: f64 = 2.2;

/// Helper struct for easier data grouping
struct ProblemForSelection {
    id: i32,
    absolute_difficulty: AbsoluteDifficulty,
    relative_difficulty: RelativeDifficulty,
    /// How many times the problem will occur in the set (determined by [`get_count_per_problem()`])
    occurrences: u8,
}

impl ProblemForSelection {
    fn from_problem(problem: &ProblemIdAndDifficulties) -> Self {
        Self {
            id: problem.id,
            absolute_difficulty: problem.absolute_difficulty,
            relative_difficulty: problem.relative_difficulty,
            occurrences: 0,
        }
    }
}

/// Finds out which problems should be used in the set
///
/// Generates a `Vec` of length `n` with the order and id of every problem in the set.
pub fn select_problems(
    number_of_problems: u8,
    problems: &[ProblemIdAndDifficulties],
    min_difficulty: AbsoluteDifficulty,
    max_difficulty: AbsoluteDifficulty,
) -> Result<Vec<i32>, ApiError> {
    // Surely we parse this earlier?
    if problems.is_empty() {
        return Err(BadRequest(String::from(
            "Problem set contains no valid problems",
        )));
    }

    // Re-structure the data to include `occurrences`
    let mut problems: Vec<ProblemForSelection> = problems
        .iter()
        .map(ProblemForSelection::from_problem)
        .collect();

    let difficulties_with_problems =
        check_which_difficulties_have_problems(&problems, min_difficulty, max_difficulty);
    let distribution_function =
        get_difficulty_distribution_function(min_difficulty, max_difficulty);

    let problem_count_per_difficulty = get_problem_count_per_difficulty(
        &difficulties_with_problems,
        distribution_function,
        number_of_problems,
    );

    // Determine the count of each problem (one difficulty at a time)
    zip(difficulties_with_problems, problem_count_per_difficulty)
        .for_each(|(difficulty, count)| set_count_per_problem(&mut problems, difficulty, count));

    Ok(order_problems(&problems))
}

/// Returns a `Vec` of every `AbsoluteDifficulty` in the range that has at least one problem.
///
/// Since subsequent steps in the selection process requires us to calculate the amount of problems
/// per difficulty, the calculations are less accurate if we include difficulties which don't even
/// have problems (the ratios for the actual difficulties might change).
fn check_which_difficulties_have_problems(
    problems: &[ProblemForSelection],
    min_difficulty: AbsoluteDifficulty,
    max_difficulty: AbsoluteDifficulty,
) -> Vec<AbsoluteDifficulty> {
    (min_difficulty.number..=max_difficulty.number)
        .filter(|num| {
            problems
                .iter()
                .find(|problem| problem.absolute_difficulty.number == *num)
                .is_some()
        })
        .map(AbsoluteDifficulty::from_num)
        .collect()
}

/// Calculates the difficulty distribution function to be used for the set generation.
///
/// The difficulties are currently spread using a normal distribution, where the mean is decided by
/// the minimum and maximum difficulties (ensuring the peak is in a reasonable place) and the
/// standard deviation is an eyeballed magic number.
///
/// Note that the denominator of the normal distribution is skipped since that's simply a
/// normalizer; we normalize later anyway depending on the desired number of problems.
fn get_difficulty_distribution_function(
    min_difficulty: AbsoluteDifficulty,
    max_difficulty: AbsoluteDifficulty,
) -> impl Fn(u8) -> f64 {
    let mean = (min_difficulty.number + max_difficulty.number - 1) as f64 / 2.0;

    move |x| {
        let diff = x as f64 - mean;
        f64::exp(-0.5 * (diff / NORMAL_DISTRIBUTION_STDEV).powi(2))
    }
}

/// Calculates how many problems should be generated for each [`AbsoluteDifficulty`], using the
/// given `distribution_function`.
///
/// Note that the returned `Vec` is aligned with the submitted `difficulties`: `difficulties[0]`
/// will have the count of the first value in the `Vec`.
fn get_problem_count_per_difficulty(
    difficulties: &[AbsoluteDifficulty],
    distribution_function: impl Fn(u8) -> f64,
    number_of_problems: u8,
) -> Vec<u8> {
    let ratio_per_difficulty: Vec<f64> = difficulties
        .iter()
        .map(|difficulty| distribution_function(difficulty.number))
        .collect();
    // To convert the ratios into actual counts that add up to `number_of_problems`,
    // we multiply them with a normalization constant
    let normalization_constant =
        number_of_problems as f64 / ratio_per_difficulty.iter().sum::<f64>();
    let mut count_per_difficulty: Vec<u8> = ratio_per_difficulty
        .iter()
        .map(|ratio| (ratio * normalization_constant).round() as u8)
        .collect();

    // The rounding might make the total be off by 1 (or, god forbid, even more)
    let mut total: u8 = count_per_difficulty.iter().sum();
    while total > number_of_problems {
        *count_per_difficulty.iter_mut().max_by_key(|x| **x).unwrap() -= 1;
        total -= 1;
    }
    while total < number_of_problems {
        *count_per_difficulty.iter_mut().min_by_key(|x| **x).unwrap() += 1;
        total += 1;
    }

    count_per_difficulty
}

/// Takes a set of problems and decides how many of each
/// individual problem should be generated to fill the `number_of_problems` quota.
///
/// Done for **a single `AbsoluteDifficulty`** at a time!
fn set_count_per_problem(
    problems: &mut [ProblemForSelection],
    difficulty: AbsoluteDifficulty,
    number_of_problems: u8,
) {
    todo!()
}

/// Decides the order the problems will appear in, expressed as a `Vec` of `id`s.
fn order_problems(problems: &[ProblemForSelection]) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function
    fn problem_from_nums(
        id: i32,
        absolute_difficulty: u8,
        relative_difficulty: u8,
    ) -> ProblemForSelection {
        ProblemForSelection {
            id,
            absolute_difficulty: AbsoluteDifficulty::from_num(absolute_difficulty),
            relative_difficulty: RelativeDifficulty::from_num(relative_difficulty),
            occurrences: 0,
        }
    }

    #[test]
    fn filters_empty_difficulties() {
        let problems = [(1, 2, 3), (2, 3, 4), (3, 5, 5)]
            .map(|(id, absolute, relative)| problem_from_nums(id, absolute, relative));
        // Are there problems between 1 and 4 in absolute difficulty?
        let difficulties = check_which_difficulties_have_problems(
            &problems,
            AbsoluteDifficulty::from_num(1),
            AbsoluteDifficulty::from_num(4),
        );
        // Only 2 and 3 should have problems
        assert_eq!(
            difficulties,
            vec![
                AbsoluteDifficulty::from_num(2),
                AbsoluteDifficulty::from_num(3)
            ]
        )
    }
}
