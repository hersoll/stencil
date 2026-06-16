use db::ProblemIdsAndDifficulties;
use rand::prelude::*;
use std::{collections::HashMap, iter::zip};
use types::{
    difficulty::{AbsoluteDifficulty, RelativeDifficulty},
    errors::ApiError::{self, BadRequest},
};

/// Standard deviation for the difficulty distribution.
///
/// Currently eyeballed to make a function that "looks good".
const NORMAL_DISTRIBUTION_STDEV: f64 = 3.1;
/// Mean for the difficulty distribution.
///
/// Currently eyeballed to make a function that "looks good".
const NORMAL_DISTRIBUTION_MEAN: f64 = 4.3;

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
    fn from_problem(problem: &ProblemIdsAndDifficulties) -> Self {
        Self {
            problem_id: problem.problem_id,
            topic_id: problem.topic_id,
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
    problems: &[ProblemIdsAndDifficulties],
    min_difficulty: AbsoluteDifficulty,
    max_difficulty: AbsoluteDifficulty,
) -> Result<Vec<i32>, ApiError> {
    // Surely we parse this earlier?
    if problems.is_empty() {
        tracing::error!("Problem set is empty");
        return Err(BadRequest(String::from(
            "Problem set contains no valid problems",
        )));
    }

    // Re-structure the data into [`ProblemForSelection`] to include `occurrences`
    let mut problems: Vec<ProblemForSelection> = problems
        .iter()
        .map(ProblemForSelection::from_problem)
        .collect();

    let absolute_difficulties_with_problems =
        check_which_difficulties_have_problems(&problems, min_difficulty, max_difficulty);
    tracing::debug!(
        "Absolute difficulties with problems: \n{absolute_difficulties_with_problems:?}"
    );

    let distribution_function =
        get_difficulty_distribution_function(min_difficulty, max_difficulty);

    let problem_count_per_absolute_difficulty = get_problem_count_per_difficulty(
        &absolute_difficulties_with_problems,
        distribution_function,
        number_of_problems,
    );
    tracing::debug!(
        "Problem count per absolute_difficulty:\n{problem_count_per_absolute_difficulty:?}"
    );

    // Determine the count of each problem (one difficulty at a time)
    zip(
        absolute_difficulties_with_problems,
        problem_count_per_absolute_difficulty,
    )
    .for_each(|(difficulty, count)| {
        set_count_per_problem_for_difficulty(&mut problems, difficulty, count)
    });

    Ok(order_problems(problems))
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
    // Currently not used, static mean instead
    let _mean = (min_difficulty.number + max_difficulty.number - 1) as f64 / 2.0;

    move |x| {
        let diff = x as f64 - NORMAL_DISTRIBUTION_MEAN;
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
    // The problem count per topic is used to determine how many occurrences to assign to each topic
    let mut problems_per_topic: HashMap<i32, u8> = HashMap::new();

    let mut problems_in_difficulty: Vec<&mut ProblemForSelection> = problems
        .iter_mut()
        .filter(|p| p.absolute_difficulty == difficulty)
        .collect();
    problems_in_difficulty
        .iter()
        .for_each(|p| *problems_per_topic.entry(p.topic_id).or_default() += 1);

    let problem_count = problems_in_difficulty.len();

    // The number of occurences should be proportional to the amount of problems per topic.
    // Since some rounding might happen, we want to do the topics with the lowest counts first since
    // they are more sensitive to rounding errors
    let mut occurrences_per_topic: Vec<(i32, u8)> = problems_per_topic.into_iter().collect();
    occurrences_per_topic.sort_by_key(|(_, count)| *count);
    occurrences_per_topic = occurrences_per_topic
        .iter()
        .map(|(topic_id, count)| {
            let ratio = *count as f64 / problem_count as f64;
            let float_count = ratio * number_of_problems as f64;
            let int_count = float_count.round() as u8;
            (*topic_id, int_count)
        })
        .collect();

    // Handle rounding errors
    let mut total_occurrence: u8 = occurrences_per_topic.iter().map(|(_, count)| *count).sum();
    let last_index = occurrences_per_topic.len() - 1;
    while total_occurrence > number_of_problems {
        occurrences_per_topic[last_index].1 -= 1;
        total_occurrence -= 1;
    }
    while total_occurrence < number_of_problems {
        occurrences_per_topic[last_index].1 += 1;
        total_occurrence += 1;
    }
    for (topic_id, occurrence_count) in occurrences_per_topic {
        let mut problems_in_topic: Vec<&mut ProblemForSelection> = problems_in_difficulty
            .iter_mut()
            .map(|p| &mut **p)
            .filter(|problem| problem.topic_id == topic_id)
            .collect();
        set_count_for_one_topic(&mut problems_in_topic, occurrence_count as usize);
    }
}

fn set_count_for_one_topic(
    problems: &mut [&mut ProblemForSelection],
    number_of_occurrences: usize,
) {
    let problem_count = problems.len();
    // Used for determining which difficulty should have its problems increased in occurrence
    let mut count_per_relative_difficulty: HashMap<RelativeDifficulty, u8> = HashMap::new();

    let minimum_occurrence_per_problem = (number_of_occurrences / problem_count) as u8;
    let number_left_to_distribute = (number_of_occurrences % problem_count) as u8;
    problems.iter_mut().for_each(|problem| {
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
        let min_occurrences = problems
            .iter()
            .filter(|p| p.relative_difficulty == lowest_relative_difficulty)
            .map(|p| p.occurrences)
            .min()
            .unwrap();

        problems
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
fn order_problems(problems: Vec<ProblemForSelection>) -> Vec<i32> {
    let mut problems_by_topic: HashMap<i32, Vec<ProblemForSelection>> = HashMap::new();
    for problem in problems {
        problems_by_topic
            .entry(problem.topic_id)
            .or_default()
            .push(problem);
    }

    let mut ordered_problems_by_topic_and_absolute_difficulty: HashMap<
        AbsoluteDifficulty,
        Vec<Vec<i32>>,
    > = HashMap::new();
    for (_, problem_vec) in problems_by_topic {
        let topic_map = order_problems_in_topic(problem_vec);
        for (difficulty, order) in topic_map {
            ordered_problems_by_topic_and_absolute_difficulty
                .entry(difficulty)
                .or_default()
                .push(order);
        }
    }

    // Weave the vecs from different topics together
    let mut ordered_problems_by_absolute_difficulty: HashMap<AbsoluteDifficulty, Vec<i32>> =
        HashMap::new();
    for (difficulty, orders) in ordered_problems_by_topic_and_absolute_difficulty {
        ordered_problems_by_absolute_difficulty.insert(difficulty, interweave(&orders));
    }
    tracing::debug!(
        "Problem order per absolute_difficulty:\n {ordered_problems_by_absolute_difficulty:?}"
    );

    let mut orders_as_vec: Vec<_> = ordered_problems_by_absolute_difficulty
        .into_iter()
        .collect();
    orders_as_vec.sort_by_key(|(difficulty, _)| *difficulty);
    orders_as_vec.into_iter().flat_map(|(_, vec)| vec).collect()
}

/// Takes the problems one topic at a time and decides the order they should appear in
///
/// The reason it does one topic at a time is that the RelativeDifficulty is topic-specific. We
/// can't mix different topics since the relative difficulties will have wildly different meanings.
///
/// Returns a HashMap which groups the problems by absolute difficulty. This makes it easy to mix problems
/// from different topics
fn order_problems_in_topic(
    mut problems: Vec<ProblemForSelection>,
) -> HashMap<AbsoluteDifficulty, Vec<i32>> {
    // During the ordering, we want the problems to increase according to their relative difficulty
    // (and we also want to "interweave" them depending on their relative difficulty).
    //
    // Thus we start by collecting them into categories depending on their relative difficulty, and
    // later split them depending on absolute difficulty
    let mut order_per_relative_difficulty: HashMap<
        RelativeDifficulty,
        Vec<(i32, AbsoluteDifficulty)>,
    > = HashMap::new();

    // To keep orders fresh, shuffle within each relative difficulty group
    let mut rng = rand::rng();
    problems.shuffle(&mut rng);
    problems.sort_by_key(|problem| problem.relative_difficulty.number);

    // Start by filling the Map with one of each problem
    for problem in &mut *problems {
        if problem.occurrences > 0 {
            order_per_relative_difficulty
                .entry(problem.relative_difficulty)
                .or_default()
                .push((problem.problem_id, problem.absolute_difficulty));
            problem.occurrences -= 1;
        }
    }

    // Then, alternate between placing problems at the end of their own Vec, or inject them in the
    // middle of the next Vec
    let mut place_at_end = true;
    while problems.iter().any(|p| p.occurrences > 0) {
        for problem in problems.iter_mut().filter(|p| p.occurrences > 0) {
            if place_at_end {
                order_per_relative_difficulty
                    .entry(problem.relative_difficulty)
                    .or_default()
                    .push((problem.problem_id, problem.absolute_difficulty));
            } else {
                let next_difficulty = order_per_relative_difficulty
                    .keys()
                    .filter(|&&k| k > problem.relative_difficulty)
                    .min()
                    .copied();
                if let Some(difficulty) = next_difficulty {
                    let next_vec = order_per_relative_difficulty.get_mut(&difficulty).unwrap();
                    let insertion_index = next_vec.len().div_ceil(2);
                    next_vec.insert(
                        insertion_index,
                        (problem.problem_id, problem.absolute_difficulty),
                    );
                } else {
                    order_per_relative_difficulty
                        .entry(problem.relative_difficulty)
                        .or_default()
                        .push((problem.problem_id, problem.absolute_difficulty));
                }
            }
            problem.occurrences -= 1;
        }

        place_at_end = !place_at_end;
    }

    // For easier iteration, turn the previous hashmap into a flat vec
    let mut entries: Vec<_> = order_per_relative_difficulty.into_iter().collect();
    entries.sort_by_key(|(k, _)| *k);
    let flat_order = entries.into_iter().flat_map(|(_, v)| v);

    let mut order_per_absolute_difficulty: HashMap<AbsoluteDifficulty, Vec<i32>> = HashMap::new();
    // Since we have mixed the problems a bit, the absolute difficulties won't be ordered. But there
    // are still clearly defined "chunks". For example, it might look like this:
    //
    // 1, 1, 1, 2, 2, 1, 2, 3, 3, 2, 2, 3, 4, 4, 3
    // ---1---  ----2-----  ------3------  ---4---
    // So every problem in the same chunk goes into their respective vec.
    let mut current_difficulty = AbsoluteDifficulty::from_num(1);

    for problem in flat_order {
        while problem.1 > current_difficulty {
            // next chunk
            current_difficulty.number += 1;
        }
        order_per_absolute_difficulty
            .entry(current_difficulty)
            .or_default()
            .push(problem.0);
    }
    order_per_absolute_difficulty
}

fn interweave(vecs: &[Vec<i32>]) -> Vec<i32> {
    let total: usize = vecs.iter().map(|v| v.len()).sum();
    let lengths: Vec<usize> = vecs.iter().map(|v| v.len()).collect();
    let mut indices = vec![0usize; vecs.len()]; // how many we've taken from each vec
    let mut result = Vec::with_capacity(total);

    for i in 0..total {
        // Pick the vec that is most "due" based on how much of its quota we've used
        let best = (0..vecs.len())
            .filter(|&j| indices[j] < lengths[j])
            .max_by_key(|&j| {
                // Bresenham-style: how far ahead of its fair share is this vec?
                (lengths[j] * (i + 1)) as isize - (indices[j] * total) as isize
            })
            .unwrap();

        result.push(vecs[best][indices[best]]);
        indices[best] += 1;
    }
    result
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
}
