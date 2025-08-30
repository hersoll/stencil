use macros::problem;
use crate::backend::problems::symbols;
use crate::backend::{typst_formatting, Expression, IntRange, Problem, Term};
use crate::Result;

/// 3(x+1)
/// Difficulty: 0
#[problem]
fn positive_integer_mult(id: String, _lang: &str) -> Result<Problem> {
    let (factor, f_range) = IntRange::without_zero(2, 5)?.and_random();
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = IntRange::without_zero(-7, 7)?.and_random();

    let t1: Term = unknown.into();
    let t2: Term = constant.into();
    let exp: Expression = vec![t1, t2].into();

    let question = format!("${factor}({exp})$");
    let answer = (factor * exp.clone()).simplify();
    let solution = format!(
        "${factor}({exp}) = colored({factor} dot) {unknown} {sign} colored({factor} dot) {abs_const} = {answer}$",
        sign = if constant > 0 {"+"} else {"-"},
        abs_const = constant.abs()
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len()
    })
}

/// -2(x+4)
/// Difficulty: 1
#[problem]
fn negative_integer_mult(id: String, _lang: &str) -> Result<Problem> {
    let (factor, f_range) = IntRange::without_zero(-5, -2)?.and_random();
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = IntRange::without_zero(-7, -1)?.and_random();

    let t1: Term = unknown.into();
    let t2: Term = constant.into();
    let exp: Expression = vec![t1, t2].into();

    let question = format!("${factor}({exp})$");
    let answer = factor * exp.clone();
    let simplified = answer.simplify();
    let solution = format!(
        "$&{factor}({exp}) = colored({factor_p} dot) {unknown} + colored({factor_p} dot) {const_p} =\\
            =&{answer} = {simplified}$",
        factor_p = typst_formatting::parentheses(factor),
        const_p = typst_formatting::parentheses(constant),
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${simplified}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len()
    })
}
