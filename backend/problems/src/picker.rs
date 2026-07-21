//! The `picker` is responsible for choosing which problems will be included in a problem set, and
//! in what order.
//!
//! The algorithm is different depending on whether a single topic is included in the set or not:
//!
//! - Single topic: It is assumed this is a "lesson stencil", where it is important to get a good progression.
//!   We distribute the problems evenly across the available ones to make sure every variant gets
//!   its chance to shine, and can be certain that earlier problems have appeared so we can build
//!   on them.
//!
//! - Multiple topics: Thought of as a "revising stencil". Now it's more important that
//!   difficulty-appropriate problems appear. Even if a topic has more intro-level problems, it's
//!   no use to include a bunch of those if the intent is to revise properly. The problems are in
//!   this case distributed according to a distribution function (normal distribution). More
//!   problems in difficulties 4-5, less the more extreme we go.
//!
//! The ordering of the problems is the same no matter which selection algorithm has been done.
//! The gist is that the problems are sorted by absolute difficulty and then relative difficulty,
//! with randomization within a relative difficulty of the same topic.

use db::ProblemIdsAndDifficulties;
use rand::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    iter::zip,
};
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
    // NOTE: Surely we parse this earlier?
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

    // The selection process is different depending on if we have multiple topics or not
    let multiple_topics = problems
        .iter()
        .any(|p| p.topic_id != problems.first().unwrap().topic_id);

    if multiple_topics {
        distribute_problems_across_multiple_topics(
            number_of_problems,
            &mut problems,
            min_difficulty,
            max_difficulty,
        );
    } else {
        distribute_problems_for_single_topic(
            number_of_problems,
            &mut problems,
            min_difficulty,
            max_difficulty,
        );
    }

    // We now know precisely how many of each problem should be included in the set.
    // The only thing left to do is order them in a thoughtful way!
    Ok(order_problems(problems))
}

/// Distribute problems according to a difficulty curve determined by the
/// `difficulty_distribution_function`.
fn distribute_problems_across_multiple_topics(
    number_of_problems: u8,
    problems: &mut [ProblemForSelection],
    min_difficulty: AbsoluteDifficulty,
    max_difficulty: AbsoluteDifficulty,
) {
    // To know how to distribute our problems (and most importantly, how many of each),
    // we need to know which difficulties have at least one corresponding problem.
    //
    // This is important to make sure the distribution function is applied properly. Otherwise
    // we might calculate that Difficulty 4 should have three problems, but then there are no problems
    // to apply to that difficulty
    let absolute_difficulties_with_problems =
        check_which_difficulties_have_problems(problems, min_difficulty, max_difficulty);
    tracing::debug!(
        "Absolute difficulties with problems: \n{absolute_difficulties_with_problems:?}"
    );

    let problem_count_per_absolute_difficulty = get_problem_count_per_difficulty(
        &absolute_difficulties_with_problems,
        get_difficulty_distribution_function(),
        number_of_problems,
    );
    tracing::debug!(
        "Problem count per absolute_difficulty:\n{problem_count_per_absolute_difficulty:?}"
    );

    // Determine the count of each problem (one difficulty at a time)
    //
    // We can zip the two vecs since they are aligned. If only five difficulties have problems,
    // the `problem_count_per_absolute_difficulty` vec will have five elements.
    zip(
        absolute_difficulties_with_problems,
        problem_count_per_absolute_difficulty,
    )
    .for_each(|(difficulty, count)| {
        set_count_per_problem_for_difficulty(
            &mut problems
                .iter_mut()
                .filter(|p| p.absolute_difficulty == difficulty)
                .collect::<Vec<&mut ProblemForSelection>>(),
            count,
        )
    });
}

/// Distribute problems evenly across to make sure every problem variant gets a chance to shine
fn distribute_problems_for_single_topic(
    number_of_problems: u8,
    problems: &mut [ProblemForSelection],
    min_difficulty: AbsoluteDifficulty,
    max_difficulty: AbsoluteDifficulty,
) {
    let mut relevant_problems: Vec<&mut ProblemForSelection> = problems
        .iter_mut()
        .filter(|p| {
            p.absolute_difficulty >= min_difficulty && p.absolute_difficulty <= max_difficulty
        })
        .collect();

    occurrences_within_topic(&mut relevant_problems, number_of_problems.into());
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
fn get_difficulty_distribution_function() -> impl Fn(u8) -> f64 {
    move |x| {
        let diff = x as f64 - NORMAL_DISTRIBUTION_MEAN;
        f64::exp(-0.5 * (diff / NORMAL_DISTRIBUTION_STDEV).powi(2))
    }
}

/// Calculates how many problems should be generated for each [`AbsoluteDifficulty`], using the
/// given `distribution_function`.
///
/// Note that the returned `Vec` is aligned with the submitted `difficulties`: `difficulties[0]`
/// will have the count of the first value in `difficulties`.
fn get_problem_count_per_difficulty(
    difficulties: &[AbsoluteDifficulty],
    distribution_function: impl Fn(u8) -> f64,
    number_of_problems: u8,
) -> Vec<u8> {
    // Apply the normal distribution: How large of a share should each difficulty represent?
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
    problems: &mut [&mut ProblemForSelection],
    number_of_problems: u8,
) {
    // The problem count per topic is used to determine how many occurrences to assign to each topic
    let mut problems_per_topic: HashMap<i32, u8> = HashMap::new();

    // Count problems them by topic
    problems
        .iter()
        .for_each(|p| *problems_per_topic.entry(p.topic_id).or_default() += 1);

    let problem_count = problems.len();

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

    // We now know how many problems should be included for each topic. To determine how those problems
    // are distributed within the topic, we pass it on to the next function,
    // `occurrences_within_topic()`
    for (topic_id, occurrence_count) in occurrences_per_topic {
        let mut problems_in_topic: Vec<&mut ProblemForSelection> = problems
            .iter_mut()
            .map(|p| &mut **p)
            .filter(|problem| problem.topic_id == topic_id)
            .collect();
        occurrences_within_topic(&mut problems_in_topic, occurrence_count as usize);
    }
}

/// Sets the `occurrences` field of every [`ProblemForSelection`] in the list.
///
/// Given a set of problems which all come from the same topic and the same absolute difficulty,
/// and the total `number_of_occurrences` for the list, determines how many of each problem
/// should be included
///
/// NOTE: Mutates the array in-place, no return
fn occurrences_within_topic(
    problems: &mut [&mut ProblemForSelection],
    number_of_occurrences: usize,
) {
    // All of the problems have the same absolute difficulty (if we've gone the multiple topics route)
    // and come from the same topic.
    // In the spirit of Marxism we start by dividing as many occurences evenly between them as we can.
    let problem_count = problems.len();
    let minimum_occurrence_per_problem = (number_of_occurrences / problem_count) as u8;
    let number_left_to_distribute = (number_of_occurrences % problem_count) as u8;

    // If there are occurrences left over to distribute, we start looking at relative difficulties:
    // which difficulties have gotten the least amount of occurrences? Start with those.
    //
    // Here we start by simply tallying the difficulties
    let mut count_per_relative_difficulty: HashMap<RelativeDifficulty, u8> = HashMap::new();
    problems.iter_mut().for_each(|problem| {
        problem.occurrences = minimum_occurrence_per_problem;
        *count_per_relative_difficulty
            .entry(problem.relative_difficulty)
            .or_insert(0) += problem.occurrences;
    });

    // Give to the needy
    for _ in 0..number_left_to_distribute {
        let neediest_relative_difficulty = *count_per_relative_difficulty
            .iter()
            // First sort by count, then choose the lowest difficulty if multiple with lowest count
            .min_by_key(|(diff, count)| (*count, diff.number))
            .map(|(diff, _)| diff)
            .unwrap(); // safety: will have at least one difficulty :)

        // It's not enough to find the neediest difficulty, we can only increment one problem at a time.
        // We need to find the neediest problem within the neediest difficulty
        problems
            .iter_mut()
            .filter(|p| p.relative_difficulty == neediest_relative_difficulty)
            .min_by_key(|p| p.occurrences)
            .unwrap()
            .occurrences += 1;

        // Don't forget to let the bookkeeper know that we've incremented one problem
        *count_per_relative_difficulty
            .get_mut(&neediest_relative_difficulty)
            .unwrap() += 1;
    }
}

/// Decides the order the problems will appear in, expressed as a `Vec` of `id`s.
fn order_problems(problems: Vec<ProblemForSelection>) -> Vec<i32> {
    // Sort the problems by topic
    let mut problems_by_topic: HashMap<i32, Vec<ProblemForSelection>> = HashMap::new();
    for problem in problems {
        problems_by_topic
            .entry(problem.topic_id)
            .or_default()
            .push(problem);
    }

    // To merge different topics properly, it's easier if we merge them by absolute difficulty.
    // Since the relative difficulty is topic-dependent, we can't use that as a metric when merging.
    //
    // Each topic first gets passed to a function to order the problems within that topic correctly
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
    problems: Vec<ProblemForSelection>,
) -> HashMap<AbsoluteDifficulty, Vec<i32>> {
    // During the ordering, we want the problems to increase according to their relative difficulty
    // (and we also want to shuffle them within each relative difficulty).
    //
    // Thus we start by collecting them into categories depending on their relative difficulty, and
    // later split them depending on absolute difficulty
    //
    // BTreeMap since we want the difficulties in order
    let mut problems_per_relative_difficulty: BTreeMap<
        RelativeDifficulty,
        Vec<ProblemForSelection>,
    > = BTreeMap::new();

    for problem in problems {
        if problem.occurrences > 0 {
            problems_per_relative_difficulty
                .entry(problem.relative_difficulty)
                .or_default()
                .push(problem);
        }
    }

    // Within each relative_difficulty, we append a random problem to the order until all the occurrences
    // are depleted. This keeps the ordering kinda fresh between PDF generations.
    //
    // This will also make sense since problems with the same relative difficulty are, well...
    // equally difficult. The order shouldn't matter, and if it does, then the difficulties should
    // be adjusted to reflect that.

    // We could just add the problem's id to the order function (that is what we'll return in the end),
    // but since we will group them by absolute difficulty in the end we also need to include that
    // info in the vec.
    struct OrderData {
        id: i32,
        absolute_difficulty: AbsoluteDifficulty,
    }
    let mut order: Vec<OrderData> = Vec::new();
    let mut rng = rand::rng();
    for problems in problems_per_relative_difficulty.values_mut() {
        while problems.iter().any(|p| p.occurrences > 0) {
            let candidates: Vec<usize> = problems
                .iter()
                .enumerate()
                .filter_map(|(i, p)| (p.occurrences > 0).then_some(i))
                .collect();

            let idx = *candidates.choose(&mut rng).unwrap();

            let problem = &mut problems[idx];
            order.push(OrderData {
                id: problem.problem_id,
                absolute_difficulty: problem.absolute_difficulty,
            });
            problem.occurrences -= 1;
        }
    }

    // Even though the `order` vec is sorted by relative difficulty, the absolute difficulties
    // won't necessarily be ordered, if there is something funky with the assigned difficulties in
    // the DB. But there are still clearly defined "chunks". For example, the absolute difficulties
    // might look like this:
    //
    // Vec:     1, 1, 1, 2, 2, 1, 2, 3, 3, 2, 2, 3, 4, 4, 3     (absolute_difficulties)
    // Chunks:  ---1---  ----2-----  ------3------  ---4---
    //
    // A new "chunk" is defined when the first problem of that absolute difficulty is encountered.
    //
    // We then separate them by chunk so the order is preserved, instead of simply putting each
    // problem in its absolute difficulty bucket
    let mut order_per_absolute_difficulty: HashMap<AbsoluteDifficulty, Vec<i32>> = HashMap::new();
    let mut current_difficulty = AbsoluteDifficulty::from_num(1);

    for problem in order {
        // we might encounter the next chunk
        while problem.absolute_difficulty > current_difficulty {
            current_difficulty.number += 1;
        }
        order_per_absolute_difficulty
            .entry(current_difficulty)
            .or_default()
            .push(problem.id);
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
}
