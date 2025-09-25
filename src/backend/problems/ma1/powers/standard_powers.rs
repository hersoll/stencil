use crate::Result;
use crate::backend::{IntRange, Problem};
use macros::problem;

/// 5^4 * 5^2
/// Difficulty: 1
#[problem]
fn simple_multiplication(id: String, _lang: &str) -> Result<Problem> {
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
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    })
}

/// 5^4 / 5^2
/// Difficulty: 1
#[problem]
fn simple_division(id: String, _lang: &str) -> Result<Problem> {
    let (base, base_range) = IntRange::without_zero(4, 9)?.and_random();
    let (exp1, exp1_range) = IntRange::without_zero(4, 10)?.and_random();
    let exp2 = IntRange::without_zero(2, exp1 - 2)?.random();
    let total_exp = exp1 - exp2;
    let question = format!("$ {base}^{exp1} / {base}^{exp2} $");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "$ {base}^{exp1} / {base}^{exp2} = {base}^({exp1} - {exp2}) = {base}^{total_exp} $"
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    })
}

/// (5^4)^2
/// Difficulty: 1
#[problem]
fn double_exponentiation(id: String, _lang: &str) -> Result<Problem> {
    let (base, base_range) = IntRange::without_zero(2, 9)?.and_random();
    let (exp1, exp1_range) = IntRange::without_zero(2, 6)?.and_random();
    let exp2 = IntRange::without_zero(3, 6)?.random();
    let total_exp = exp1 * exp2;
    let question = format!("$({base}^{exp1})^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution =
        format!("$ ({base}^{exp1})^{exp2} = {base}^({exp1} dot {exp2}) = {base}^{total_exp} $");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    })
}
