use crate::Result;
use crate::backend::problems::expressions::{Expression, Term};
use crate::backend::problems::symbols;
use crate::backend::{IntRange, Problem};
use macros::problem;

#[problem(id = "one_variable_and_constants_no_negatives", difficulty = 0)]
fn one_variable_and_constants_no_negatives(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (first_coef, first_coef_range) = IntRange::without_zero(1, 6)?.and_random();
    let second_coef = IntRange::without_zero(-(first_coef - 1), 6)?.random();
    let (first_const, first_const_range) = IntRange::without_zero(1, 6)?.and_random();
    let second_const = IntRange::without_zero(-(first_const - 1), 6)?.random();
    let first_term: Term = (first_coef, unknown).into();
    let second_term: Term = (second_coef, unknown).into();
    let first_const_term: Term = first_const.into();
    let second_const_term: Term = second_const.into();
    let original_expression: Expression = vec![
        &first_term,
        &first_const_term,
        &second_term,
        &second_const_term,
    ]
    .into();
    let simplified_expression = original_expression.simplify();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$ &{original_expression} = \\
        = &colored({first_term}{second_term:+}) {first_const_term:+}{second_const_term:+} = \\
        = &{simplified_expression} $",
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_const],
        combinations: first_coef_range.len() * first_const_range.len(),
    })
}

#[problem(id = "one_variable_and_constants", difficulty = 1)]
fn one_variable_and_constants(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (first_coef, first_coef_range) = IntRange::without_zero(-6, 6)?.and_random();
    let second_coef = IntRange::without_zero(-6, 6)?.random();
    let (first_const, first_const_range) = IntRange::without_zero(6, 6)?.and_random();
    let second_const = IntRange::without_zero(-6, 6)?.random();
    let first_term: Term = (first_coef, unknown).into();
    let second_term: Term = (second_coef, unknown).into();
    let first_const_term: Term = first_const.into();
    let second_const_term: Term = second_const.into();
    let original_expression: Expression = vec![
        &first_term,
        &first_const_term,
        &second_term,
        &second_const_term,
    ]
    .into();
    let simplified_expression = original_expression.simplify();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$ &{original_expression} = \\
        = &colored({first_term}{second_term:+}) {first_const_term:+}{second_const_term:+} = \\
        = &{simplified_expression} $",
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_const],
        combinations: first_coef_range.len() * first_const_range.len(),
    })
}
