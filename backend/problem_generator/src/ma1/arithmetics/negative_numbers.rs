use std::cmp::{max, min};

use anyhow::Result;
use macros::problem;
use math::num_gen::{self, IntegerGenerator};
use types::{lang::Language, problems::Problem};
use typst_writer::{
    drawing::NumberLine,
    formatting::{add_number, parentheses, subtract_number},
};

/// 5 - 9
/// Difficulty: 0
#[problem]
fn subtract_larger(name: String, _lang: &Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(1, 5).and_random();
    let second = num_gen::integer().range(first + 1, first + 8).random();
    let number_line = NumberLine::from_ends(first, first - second)
        .with_arc(first, first - second, subtract_number(second))
        .build_string()?;
    Ok(Problem {
        name,
        question: format!("${first} - {second}$"),
        answer: format!("${}$", first - second),
        solution: number_line,
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// -4 + 2
/// Difficulty: 2
#[problem]
fn start_negative(name: String, _lang: &Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-5, -1).and_random();
    let (second, second_range) = num_gen::integer().range(-8, 8).exclude(0).and_random();
    let number_line = NumberLine::from_ends(
        min(first, first + second),
        max(max(0, first), first + second),
    )
    .with_arc(first, first + second, add_number(second))
    .build_string()?;
    Ok(Problem {
        name,
        question: format!("${first} {second:+}$"),
        answer: format!("${}$", first + second),
        solution: number_line,
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// 4 + (-2)
/// Difficulty: 2
#[problem]
fn add_negative(name: String, _lang: &Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(1, 10).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first + second;
    Ok(Problem {
        name,
        question: format!("${first} + {second_p}$", second_p = parentheses(second)),
        answer: format!("${ans}$"),
        solution: format!(
            "${first} + {second_p} = {first} - {second_a} = {ans}$",
            second_p = parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// 4 - (-2)
/// Difficulty: 2
#[problem]
fn subtract_negative(name: String, _lang: &Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(1, 10).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first - second;
    Ok(Problem {
        name,
        question: format!("${first} - {second_p}$", second_p = parentheses(second)),
        answer: format!("${ans}$"),
        solution: format!(
            "${first} - {second_p} = {first} + {second_a} = {ans}$",
            second_p = parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

fn make_multiplication_problem(
    first: IntegerGenerator,
    second: IntegerGenerator,
    name: String,
) -> Result<Problem> {
    let (first_val, first_range) = first.and_random();
    let (second_val, second_range) = second.and_random();
    let ans = first_val * second_val;
    Ok(Problem {
        name: name,
        question: format!(
            "${first_p} dot {second_p}$",
            first_p = parentheses(first_val),
            second_p = parentheses(second_val)
        ),
        answer: format!("${ans}$"),
        solution: if ans > 0 {
            format!("Två negativa faktorer ger ett positivt svar")
        } else {
            format!("En positiv och en negativ faktor ger ett negativt svar")
        },
        identifiers: vec![first_val, second_val],
        combinations: first_range.len() * second_range.len(),
    })
}

/// 4 * (-2)
/// Difficulty: 2
#[problem]
fn positive_times_negative(name: String, _lang: &Language) -> Result<Problem> {
    make_multiplication_problem(
        num_gen::integer().range(1, 10),
        num_gen::integer().range(-10, -1),
        name,
    )
}

/// (-4) * 2
/// Difficulty: 2
#[problem]
fn negative_times_positive(name: String, _lang: &Language) -> Result<Problem> {
    make_multiplication_problem(
        num_gen::integer().range(-10, -1),
        num_gen::integer().range(1, 10),
        name,
    )
}

/// (-4) * (-2)
/// Difficulty: 2
#[problem]
fn negative_times_negative(name: String, _lang: &Language) -> Result<Problem> {
    make_multiplication_problem(
        num_gen::integer().range(-10, -1),
        num_gen::integer().range(-10, -1),
        name,
    )
}

/// (-4) + (-2)
/// Difficulty: 3
#[problem]
fn negative_plus_negative(name: String, _lang: &Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-10, -1).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first + second;
    Ok(Problem {
        name,
        question: format!(
            "${first_p} + {second_p}$",
            first_p = parentheses(first),
            second_p = parentheses(second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first_p} + {second_p} = {first} - {second_a} = {ans}$",
            first_p = parentheses(first),
            second_p = parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// (-4) - (-2)
/// Difficulty: 3
#[problem]
fn negative_minus_negative(name: String, _lang: &Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-10, -1).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first - second;
    Ok(Problem {
        name,
        question: format!(
            "${first_p} - {second_p}$",
            first_p = parentheses(first),
            second_p = parentheses(second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first_p} - {second_p} = {first} + {second_a} = {ans}$",
            first_p = parentheses(first),
            second_p = parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}
