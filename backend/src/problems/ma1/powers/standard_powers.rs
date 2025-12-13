use crate::problems::{IntRange, Problem, symbols};
use anyhow::Result;
use macros::problem;

/// 5^4 * 5^2
/// Difficulty: 0
#[problem]
fn simple_multiplication(name: String, _lang: &str) -> Result<Problem> {
    let base = IntRange::without_zero(4, 9)?.random();
    let (exp1, exp1_range) = IntRange::without_zero(2, 6)?.and_random();
    let (exp2, exp2_range) = IntRange::without_zero(2, 6)?.and_random();
    let total_exp = exp1 + exp2;
    let question = format!("${base}^{exp1} dot {base}^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "${base}^{exp1} dot {base}^{exp2} = {base}^({exp1} + {exp2}) = {base}^{total_exp}$"
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    })
}

/// x^4 * x^2
/// Difficulty: 1
#[problem]
fn simple_multiplication_variables(name: String, _lang: &str) -> Result<Problem> {
    let base = symbols::get_unknown()?;
    let (exp1, exp1_range) = IntRange::without_zero(2, 6)?.and_random();
    let (exp2, exp2_range) = IntRange::without_zero(2, 6)?.and_random();
    let total_exp = exp1 + exp2;
    let question = format!("${base}^{exp1} dot {base}^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "${base}^{exp1} dot {base}^{exp2} = {base}^({exp1} + {exp2}) = {base}^{total_exp}$"
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    })
}

/// 5^4 / 5^2
/// Difficulty: 0
#[problem]
fn simple_division(name: String, _lang: &str) -> Result<Problem> {
    let (base, base_range) = IntRange::without_zero(4, 9)?.and_random();
    let (exp1, exp1_range) = IntRange::without_zero(4, 10)?.and_random();
    let exp2 = IntRange::without_zero(2, exp1 - 2)?.random();
    let total_exp = exp1 - exp2;
    let question = format!("$display({base}^{exp1} / {base}^{exp2})$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "$display({base}^{exp1} / {base}^{exp2} = {base}^({exp1} - {exp2}) = {base}^{total_exp})$"
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    })
}

/// a^4 / a^2
/// Difficulty: 1
#[problem]
fn simple_division_variables(name: String, _lang: &str) -> Result<Problem> {
    let base = symbols::get_unknown()?;
    let (exp1, exp1_range) = IntRange::without_zero(4, 10)?.and_random();
    let exp2 = IntRange::without_zero(2, exp1 - 2)?.random();
    let total_exp = exp1 - exp2;
    let question = format!("$display({base}^{exp1} / {base}^{exp2})$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "$display({base}^{exp1} / {base}^{exp2} = {base}^({exp1} - {exp2}) = {base}^{total_exp})$"
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![exp1],
        combinations: exp1_range.len(),
    })
}
/// (5^4)^2
/// Difficulty: 1
#[problem]
fn double_exponentiation(name: String, _lang: &str) -> Result<Problem> {
    let (base, base_range) = IntRange::without_zero(2, 9)?.and_random();
    let (exp1, exp1_range) = IntRange::without_zero(2, 6)?.and_random();
    let exp2 = IntRange::without_zero(3, 6)?.random();
    let total_exp = exp1 * exp2;
    let question = format!("$({base}^{exp1})^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution =
        format!("$ ({base}^{exp1})^{exp2} = {base}^({exp1} dot {exp2}) = {base}^{total_exp} $");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    })
}

/// (x^4)^2
/// Difficulty: 1
#[problem]
fn double_exponentiation_variables(name: String, _lang: &str) -> Result<Problem> {
    let base = symbols::get_unknown()?;
    let (exp1, exp1_range) = IntRange::without_zero(2, 6)?.and_random();
    let exp2 = IntRange::without_zero(3, 6)?.random();
    let total_exp = exp1 * exp2;
    let question = format!("$({base}^{exp1})^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution =
        format!("$ ({base}^{exp1})^{exp2} = {base}^({exp1} dot {exp2}) = {base}^{total_exp} $");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![exp1],
        combinations: exp1_range.len(),
    })
}

// (5^3 * 5^6) / 5^2
// Difficulty: 2
#[problem]
fn multiplication_and_division(name: String, _lang: &str) -> Result<Problem> {
    let (base, base_range) = IntRange::without_zero(3, 9)?.and_random();
    let (exp1, exp1_range) = IntRange::without_zero(2, 10)?.and_random();
    let (exp2, exp2_range) = IntRange::without_zero(2, 10)?.and_random();
    let exp3 = IntRange::without_zero(2, exp1 + exp2 - 2)?
        .exclude(exp1)
        .exclude(exp2)
        .random();
    let total_exp = exp1 + exp2 - exp3;
    let question = format!("$ ({base}^{exp1} dot {base}^{exp2})/{base}^{exp3} $");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "$ ({base}^{exp1} dot {base}^{exp2})/{base}^{exp3} = {base}^{}/{base}^{exp3} = {base}^{total_exp} $",
        exp1 + exp2
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![base, exp1, exp2],
        combinations: exp1_range.len() * base_range.len() * exp2_range.len(),
    })
}
