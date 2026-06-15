use db::ProblemIdAndDifficulties;
use rand::prelude::*;
use std::{collections::HashMap, iter::zip};
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
    problem_id: i32,
    topic_id: i32,
    absolute_difficulty: AbsoluteDifficulty,
    relative_difficulty: RelativeDifficulty,
    /// How many times the problem will occur in the set (determined by [`get_count_per_problem()`])
    occurrences: u8,
}

impl ProblemForSelection {
    fn from_problem(problem: &ProblemIdAndDifficulties) -> Self {
        Self {
            problem_id: problem.id,
            topic_id: 0,
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
    zip(difficulties_with_problems, problem_count_per_difficulty).for_each(
        |(difficulty, count)| {
            set_count_per_problem_for_difficulty(&mut problems, difficulty, count)
        },
    );

    Ok(order_problems(&mut problems))
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
fn set_count_per_problem_for_difficulty(
    problems: &mut [ProblemForSelection],
    difficulty: AbsoluteDifficulty,
    number_of_problems: u8,
) {
    let mut problems_in_difficulty: Vec<&mut ProblemForSelection> = problems
        .iter_mut()
        .filter(|p| p.absolute_difficulty == difficulty)
        .collect();
    let problem_count = problems_in_difficulty.len();
    // Used for determining which problems should be increased; the ones from a "lower count" difficulty
    let mut count_per_relative_difficulty: HashMap<RelativeDifficulty, u8> = HashMap::new();

    let minimum_occurrence_per_problem = (number_of_problems as usize / problem_count) as u8;
    let number_left_to_distribute = (number_of_problems as usize % problem_count) as u8;
    problems_in_difficulty.iter_mut().for_each(|problem| {
        problem.occurrences = minimum_occurrence_per_problem;
        *count_per_relative_difficulty
            .entry(problem.relative_difficulty)
            .or_insert(0) += problem.occurrences;
    });

    let mut rng = rand::rng();
    for _ in 0..number_left_to_distribute {
        let lowest_relative_difficulty = *count_per_relative_difficulty
            .iter()
            // First sort by count, then choose the lowest difficulty if multiple with lowest count
            .min_by_key(|(diff, count)| (*count, diff.number))
            .map(|(diff, _)| diff)
            .unwrap(); // Will have at least one difficulty :)
        let min_occurrences = problems_in_difficulty
            .iter()
            .filter(|p| p.relative_difficulty == lowest_relative_difficulty)
            .map(|p| p.occurrences)
            .min()
            .unwrap();

        problems_in_difficulty
            .iter_mut()
            .filter(|p| {
                p.relative_difficulty == lowest_relative_difficulty
                    && p.occurrences == min_occurrences
            })
            .choose(&mut rng)
            .unwrap()
            .occurrences += 1;
        *count_per_relative_difficulty
            .get_mut(&lowest_relative_difficulty)
            .unwrap() += 1;
    }
}

/// Decides the order the problems will appear in, expressed as a `Vec` of `id`s.
fn order_problems(problems: &mut [ProblemForSelection]) -> Vec<i32> {
    let mut order_per_difficulty: HashMap<u8, Vec<i32>> = HashMap::new();

    // Start by filling the Map with one of each problem
    for problem in &mut *problems {
        if problem.occurrences > 0 {
            order_per_difficulty
                .entry(problem.relative_difficulty.number)
                .or_default()
                .push(problem.problem_id);
            problem.occurrences -= 1;
        }
    }
    // Then, alternate between placing problems at the end of their own Vec, or inject them in the
    // middle of the next Vec
    let mut place_at_end = true;
    while problems.iter().any(|p| p.occurrences > 0) {
        for problem in problems.iter_mut().filter(|p| p.occurrences > 0) {
            if place_at_end {
                order_per_difficulty
                    .entry(problem.relative_difficulty.number)
                    .or_default()
                    .push(problem.problem_id);
            } else {
                let next_difficulty = order_per_difficulty
                    .keys()
                    .filter(|&&k| k > problem.relative_difficulty.number)
                    .min()
                    .copied();
                if let Some(difficulty) = next_difficulty {
                    let next_vec = order_per_difficulty.get_mut(&difficulty).unwrap();
                    let insertion_index = next_vec.len().div_ceil(2);
                    next_vec.insert(insertion_index, problem.problem_id);
                } else {
                    order_per_difficulty
                        .entry(problem.relative_difficulty.number)
                        .or_default()
                        .push(problem.problem_id);
                }
            }
            problem.occurrences -= 1;
        }

        place_at_end = !place_at_end;
    }

    let mut entries: Vec<_> = order_per_difficulty.into_iter().collect();
    entries.sort_by_key(|(k, _)| *k);
    entries.into_iter().flat_map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PROBLEMS: [(i32, u8, u8); 13] = [
        (1, 1, 1),
        (2, 1, 1),
        (3, 1, 2),
        (4, 2, 3),
        (5, 2, 4),
        (6, 2, 5),
        (7, 2, 5),
        (8, 3, 6),
        (9, 3, 6),
        (10, 3, 7),
        (11, 4, 8),
        (12, 5, 9),
        (13, 5, 10),
    ];

    /// Helper function
    fn problem_from_nums(
        problem_id: i32,
        absolute_difficulty: u8,
        relative_difficulty: u8,
    ) -> ProblemForSelection {
        ProblemForSelection {
            problem_id,
            topic_id: 0,
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
        );
    }

    #[test]
    fn sets_problem_count() {
        let mut problems: Vec<ProblemForSelection> = TEST_PROBLEMS
            .into_iter()
            .map(|(a, b, c)| problem_from_nums(a, b, c))
            .collect();
        let difficulties = [1, 2, 3, 4, 5];
        let counts = [4, 5, 5, 4, 2];
        for (difficulty, count) in zip(difficulties, counts) {
            set_count_per_problem_for_difficulty(
                &mut problems,
                AbsoluteDifficulty::from_num(difficulty),
                count,
            );
        }
        let problem_occurrences: Vec<u8> =
            problems.iter().map(|problem| problem.occurrences).collect();
        assert!(
            problem_occurrences == vec![1, 1, 2, 2, 1, 1, 1, 1, 2, 2, 4, 1, 1]
                || problem_occurrences == vec![1, 1, 2, 2, 1, 1, 1, 2, 1, 2, 4, 1, 1]
        );
    }

    #[test]
    fn orders_problems() {
        let mut problems: Vec<ProblemForSelection> = TEST_PROBLEMS
            .into_iter()
            .map(|(a, b, c)| problem_from_nums(a, b, c))
            .collect();
        let difficulties = [1, 2, 3, 4, 5];
        let counts = [4, 5, 5, 4, 2];
        for (difficulty, count) in zip(difficulties, counts) {
            set_count_per_problem_for_difficulty(
                &mut problems,
                AbsoluteDifficulty::from_num(difficulty),
                count,
            );
        }
        let order = order_problems(&mut problems);
        dbg!(&order);
        assert!(
            order
                == vec![
                    1, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9, 8, 10, 10, 11, 11, 11, 12, 11, 13
                ]
                || order
                    == vec![
                        1, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9, 9, 10, 10, 11, 11, 11, 12, 11, 13
                    ]
        )
    }
}
