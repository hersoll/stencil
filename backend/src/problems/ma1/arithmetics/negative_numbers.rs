use crate::{IntRange, Problem, typst_utils};
use anyhow::Result;
use macros::problem;

/// 5 - 9
/// Difficulty: 0
#[problem]
fn subtract_larger(id: String, _lang: &str) -> Result<Problem> {
    let (first, first_range) = IntRange::without_zero(1, 10)?.and_random();
    let second = IntRange::without_zero(first + 1, first + 10)?.random();
    Ok(Problem {
        id,
        question: format!("${first} - {second}$"),
        answer: format!("${}$", first - second),
        solution: format!("Tallinje"),
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// -4 + 2
/// Difficulty: 2
#[problem]
fn start_negative(id: String, _lang: &str) -> Result<Problem> {
    let (first, first_range) = IntRange::without_zero(-10, -1)?.and_random();
    let (second, second_range) = IntRange::without_zero(-10, 10)?.and_random();
    Ok(Problem {
        id,
        question: format!("${first} {second:+}$"),
        answer: format!("${}$", first + second),
        solution: format!("Tallinje"),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// 4 + (-2)
/// Difficulty: 2
#[problem]
fn add_negative(id: String, _lang: &str) -> Result<Problem> {
    let (first, first_range) = IntRange::without_zero(1, 10)?.and_random();
    let (second, second_range) = IntRange::without_zero(-10, -1)?.and_random();
    let ans = first + second;
    Ok(Problem {
        id,
        question: format!(
            "${first} + {second_p}$",
            second_p = typst_utils::formatting::parentheses(second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first} + {second_p} = {first} - {second_a} = {ans}$",
            second_p = typst_utils::formatting::parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// 4 - (-2)
/// Difficulty: 2
#[problem]
fn subtract_negative(id: String, _lang: &str) -> Result<Problem> {
    let (first, first_range) = IntRange::without_zero(1, 10)?.and_random();
    let (second, second_range) = IntRange::without_zero(-10, -1)?.and_random();
    let ans = first - second;
    Ok(Problem {
        id,
        question: format!(
            "${first} - {second_p}$",
            second_p = typst_utils::formatting::parentheses(second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first} - {second_p} = {first} + {second_a} = {ans}$",
            second_p = typst_utils::formatting::parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

fn make_multiplication_problem(first: IntRange, second: IntRange, id: String) -> Result<Problem> {
    let (first_val, first_range) = first.and_random();
    let (second_val, second_range) = second.and_random();
    let ans = first_val * second_val;
    Ok(Problem {
        id,
        question: format!(
            "${first_p} dot {second_p}$",
            first_p = typst_utils::formatting::parentheses(first_val),
            second_p = typst_utils::formatting::parentheses(second_val)
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
fn positive_times_negative(id: String, _lang: &str) -> Result<Problem> {
    make_multiplication_problem(
        IntRange::without_zero(1, 10)?,
        IntRange::without_zero(-10, -1)?,
        id,
    )
}

/// (-4) * 2
/// Difficulty: 2
#[problem]
fn negative_times_positive(id: String, _lang: &str) -> Result<Problem> {
    make_multiplication_problem(
        IntRange::without_zero(-10, -1)?,
        IntRange::without_zero(1, 10)?,
        id,
    )
}

/// (-4) * (-2)
/// Difficulty: 2
#[problem]
fn negative_times_negative(id: String, _lang: &str) -> Result<Problem> {
    make_multiplication_problem(
        IntRange::without_zero(-10, -1)?,
        IntRange::without_zero(-10, -1)?,
        id,
    )
}

/// (-4) + (-2)
/// Difficulty: 3
#[problem]
fn negative_plus_negative(id: String, _lang: &str) -> Result<Problem> {
    let (first, first_range) = IntRange::without_zero(-10, -1)?.and_random();
    let (second, second_range) = IntRange::without_zero(-10, -1)?.and_random();
    let ans = first + second;
    Ok(Problem {
        id,
        question: format!(
            "${first_p} + {second_p}$",
            first_p = typst_utils::formatting::parentheses(first),
            second_p = typst_utils::formatting::parentheses(second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first_p} + {second_p} = {first} - {second_a} = {ans}$",
            first_p = typst_utils::formatting::parentheses(first),
            second_p = typst_utils::formatting::parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// (-4) - (-2)
/// Difficulty: 3
#[problem]
fn negative_minus_negative(id: String, _lang: &str) -> Result<Problem> {
    let (first, first_range) = IntRange::without_zero(-10, -1)?.and_random();
    let (second, second_range) = IntRange::without_zero(-10, -1)?.and_random();
    let ans = first - second;
    Ok(Problem {
        id,
        question: format!(
            "${first_p} - {second_p}$",
            first_p = typst_utils::formatting::parentheses(first),
            second_p = typst_utils::formatting::parentheses(second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first_p} - {second_p} = {first} + {second_a} = {ans}$",
            first_p = typst_utils::formatting::parentheses(first),
            second_p = typst_utils::formatting::parentheses(second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}
