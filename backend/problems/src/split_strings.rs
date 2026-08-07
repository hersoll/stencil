//! Some questions, answers and solutions in generated problems have several text variations.
//! These are stored within the same row (as one long string) in the DB, formatted like:
//!
//! "Text... [s1 || s2 || s3] ...more text... [s4 || s5 || s6] ...more text".
//!
//! The string can have any number of points to split at, and each point can have any number of
//! variants (as long as the number of variants is consistent across the string).
//!
//! This module splits the string into the different variants, and modifies the problem so only
//! variant is chosen. This is done post-generation in the [`generator`](crate::generator) module.
//! It uses the index passed from the `generator` to make sure the variants are generated cyclically.

use anyhow::{Result, ensure};
use std::fmt::Write;
use types::problems::{Answer, Problem, Question, Solution};

/// Selects the text variant with number `index` if the problem has splittable strings in it, and
/// increments the index.
///
/// Returns Ok(true) if there are splits in the problem, Ok(false) if the problem is unsplittable
/// and Err() if something goes wrong.
pub fn select_variant(problem: &mut Problem, index: &mut usize) -> Result<bool> {
    // Variant count will keep track of how many variants there are, and will make sure the split_string
    // function errors if the count differs between strings.
    let mut variant_count = 0;
    let questions = split_string(problem.question.as_str(), &mut variant_count)?;
    let answers = split_string(problem.answer.as_str(), &mut variant_count)?;
    let solutions = split_string(problem.solution.as_str(), &mut variant_count)?;

    if variant_count == 0 {
        // No splits in this problem, we are finished
        return Ok(false);
    }

    // Just because one field might be split doesn't mean they all are.
    if let Some(question) = questions.get(*index) {
        dbg!(&question);
        problem.question = Question::from(question.clone());
    }
    if let Some(answer) = answers.get(*index) {
        dbg!(&answer);
        problem.answer = Answer::from(answer.clone());
    }
    if let Some(solution) = solutions.get(*index) {
        problem.solution = Solution::from(solution.clone());
    }

    // Increment the counter so the next problem has another variant
    *index = (*index + 1) % variant_count;

    Ok(true)
}

/// Split the string on its [[s1 || s2]] parts.
///
/// variant_count specifies how many variants there MUST be, if the argument != 0
fn split_string(s: &str, variant_count: &mut usize) -> Result<Vec<String>> {
    const SPLIT_START: &str = "[[";
    const SPLIT_END: &str = "]]";
    let marker_length = SPLIT_START.len();
    let mut index_to_slice_at = 0;
    // The trunk is the non-changing part of the String
    let mut trunk = String::with_capacity(s.len());
    // The branches are the possible variants
    let mut branches: Vec<Vec<&str>> = Vec::new();
    while let Some(start_index) = s[index_to_slice_at..].find(SPLIT_START)
        && let Some(end_index) = s[index_to_slice_at..].find(SPLIT_END)
        && start_index < end_index
    {
        // The indices first_bracket and second_bracket are relative to the sliced string.
        // We can't look at s inside this loop since the indices will be out of sync
        let slice = &s[index_to_slice_at..];
        let variants: Vec<&str> = slice[(start_index + marker_length)..end_index]
            .split(" || ")
            .collect();
        if variants.len() == 1 {
            // We have encountered the markers in some other context, for example
            // a Typst #block[]. Ignore!
            // Everything, including the contents inside [], is written to the trunk.
            write!(trunk, "{}", &slice[..end_index + marker_length])?;
        } else {
            // Everything before the [] is written to the trunk
            write!(trunk, "{}", &slice[..start_index])?;
            // Add the spot for the branch to be inserted
            write!(trunk, "{{_branch_}}")?;
            branches.push(variants);
        }

        // Find more brackets after the point we ended on
        index_to_slice_at += end_index + marker_length;
    }
    if index_to_slice_at < s.len() {
        // Write any dangling suffix to the trunk
        write!(trunk, "{}", &s[index_to_slice_at..])?;
    }

    // Every split MUST have the same number of variants
    ensure!(
        branches
            .iter()
            .all(|parts| parts.len() == branches.first().unwrap().iter().len()),
        "Encountered a split string where splits had different number of variants: {s}, first: {:#?}",
        branches.first()
    );

    let current_branch_count = branches.first().unwrap_or(&Vec::<&str>::new()).iter().len();

    // Zeroes are okay, otherwise the current variant count MUST match variant_count
    ensure!(
        *variant_count == 0 || current_branch_count == 0 || current_branch_count == *variant_count,
        "The split {s} has a different amount of splits than an earlier string."
    );

    let mut out = Vec::new();
    for branch in 0..current_branch_count {
        let mut current_trunk = trunk.clone();
        // Do once for each [] block (i).
        // `branch` is the index of the branch:
        // [[0 || 1 || 2]]
        for (i, _) in branches.iter().enumerate() {
            current_trunk = current_trunk.replacen("{_branch_}", branches[i][branch], 1);
        }
        out.push(current_trunk);
    }

    if current_branch_count != 0 {
        *variant_count = current_branch_count;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_when_no_split() -> Result<()> {
        let input = "This string has no split!";
        assert_eq!(split_string(input, &mut 0)?, Vec::<String>::new());
        Ok(())
    }

    #[test]
    fn returns_empty_when_no_split_in_bracket() -> Result<()> {
        let input = "This string has no || in its [[brackets]]!";
        assert_eq!(split_string(input, &mut 0)?, Vec::<String>::new());
        Ok(())
    }

    #[test]
    fn returns_empty_when_only_start_bracket() -> Result<()> {
        let input = "This string only has [[start brackets!";
        assert_eq!(split_string(input, &mut 0)?, Vec::<String>::new());
        Ok(())
    }

    #[test]
    fn returns_empty_when_only_end_bracket() -> Result<()> {
        let input = "This string only has end]] brackets!";
        assert_eq!(split_string(input, &mut 0)?, Vec::<String>::new());
        Ok(())
    }

    #[test]
    fn splits_with_one_correct_split_into_two() -> Result<()> {
        let input = "I want to [[split || share]] this.";
        let expected = vec![
            String::from("I want to split this."),
            String::from("I want to share this."),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }

    #[test]
    fn splits_with_one_correct_split_into_three() -> Result<()> {
        let input = "I want to [[split || share || keep]] this.";
        let expected = vec![
            String::from("I want to split this."),
            String::from("I want to share this."),
            String::from("I want to keep this."),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }

    #[test]
    fn splits_with_two_correct_splits() -> Result<()> {
        let input = "The [[dog || cat]] [[barks || meows]].";
        let expected = vec![
            String::from("The dog barks."),
            String::from("The cat meows."),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }

    #[test]
    fn ignores_brackets_without_split() -> Result<()> {
        let input = "The word [[split]] should not be split but [[I || you]] should.";
        let expected = vec![
            String::from("The word [[split]] should not be split but I should."),
            String::from("The word [[split]] should not be split but you should."),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }

    #[test]
    fn splits_typst_brackets_correctly() -> Result<()> {
        let input = "Here is a split question inside brackets: [Alternative [[one || two]]]";
        let expected = vec![
            String::from("Here is a split question inside brackets: [Alternative one]"),
            String::from("Here is a split question inside brackets: [Alternative two]"),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }
}
