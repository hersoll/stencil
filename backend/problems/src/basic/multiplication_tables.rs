use anyhow::Result;
use macros::problem;
use math::num_gen::{self, NumberGenerator};
use registry::get_solution;
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters},
};

use crate::shuffle;

/// 2 - 5
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn up_to_five(id: i32, lang: Language) -> Result<Problem> {
    let num1_range = num_gen::integer().range(2, 5);
    let num2_range = num_gen::integer().range(2, 10);
    let mut num1 = num1_range.random();
    let mut num2 = num2_range.random();
    shuffle(&mut num1, &mut num2);
    let solution = get_solution(id, lang)?;

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${num1} dot {num2}$"),
        answer: num1 * num2,
        solution,
        identifiers: vec![num1, num2],
        combinations: num1_range.len() * num2_range.len(),
    }))
}

/// 6 - 9
/// Absolute difficulty: 2
/// Relative difficulty: 2
#[problem]
fn six_to_nine(id: i32, lang: Language) -> Result<Problem> {
    let num1_range = num_gen::integer().range(6, 9);
    let num2_range = num_gen::integer().range(2, 10);
    let mut num1 = num1_range.random();
    let mut num2 = num2_range.random();
    shuffle(&mut num1, &mut num2);
    let solution = get_solution(id, lang)?;

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${num1} dot {num2}$"),
        answer: num1 * num2,
        solution,
        identifiers: vec![num1, num2],
        combinations: num1_range.len() * num2_range.len(),
    }))
}

/// 2 - 10
/// Absolute difficulty: 3
/// Relative difficulty: 3
#[problem]
fn up_to_ten(id: i32, lang: Language) -> Result<Problem> {
    let num_range = num_gen::integer().range(2, 10);
    let num1 = num_range.random();
    let num2 = num_range.random();
    let solution = get_solution(id, lang)?;

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${num1} dot {num2}$"),
        answer: num1 * num2,
        solution,
        identifiers: vec![num1, num2],
        combinations: num_range.len().pow(2),
    }))
}
