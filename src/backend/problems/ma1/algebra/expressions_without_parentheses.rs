use crate::backend::problems::symbols;
use crate::backend::problems::types::{Expression, Term};
use crate::backend::{IntRange, Problem};
use crate::Result;
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

#[problem(id = "two_variables_and_constants", difficulty = 2)]
fn two_variables_and_constants(id: String, _lang: &str) -> Result<Problem> {
    let (first_unknown, second_unknown) = symbols::get_two_unknowns()?;
    let (first_coef_a, first_coef_a_range) = IntRange::with_zero(-9, 9)?.and_random();
    let first_coef_b = IntRange::with_zero(-9, 9)?.random();
    let (second_coef_a, second_coef_a_range) = IntRange::with_zero(-9, 9)?.and_random();
    let second_coef_b = IntRange::with_zero(-9, 9)?.random();
    let first_const = IntRange::with_zero(-9, 9)?.random();
    let second_const = IntRange::with_zero(-9, 9)?.random();

    let mut first_term_a: Term = (first_coef_a, first_unknown).into();
    let mut first_term_b: Term = (first_coef_b, first_unknown).into();
    let second_term_a: Term = (second_coef_a, second_unknown).into();
    let second_term_b: Term = (second_coef_b, second_unknown).into();
    let mut first_const_term: Term = first_const.into();
    let mut second_const_term: Term = second_const.into();

    let original_expression: Expression = Expression::random_order(vec![
        &first_term_a,
        &first_term_b,
        &second_term_a,
        &second_term_b,
        &first_const_term,
        &second_const_term,
    ]);
    let simplified_expression = original_expression.simplify();
    first_term_a.colored = true;
    first_term_b.colored = true;
    first_const_term.colored = true;
    second_const_term.colored = true;
    let first_terms: Expression = vec![&first_term_a, &first_term_b].into();
    let second_terms: Expression = vec![&second_term_a, &second_term_b].into();
    let const_terms: Expression = vec![&first_const_term, &second_const_term].into();
    let sorted_expression: Expression =
        first_terms.sorted() + second_terms.sorted() + const_terms.sorted();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$ &{original_expression} = \\
        = &{sorted_expression} = \\
        = &{simplified_expression} $",
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef_a, second_coef_a],
        combinations: first_coef_a_range.len() * second_coef_a_range.len(),
    })
}

#[problem(id = "one_variable_different_exponents", difficulty = 3)]
fn one_variable_different_exponents(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (first_coef, first_coef_range) = IntRange::with_zero(-9, 9)?.and_random();
    let second_coef = IntRange::with_zero(-9, 9)?.random();
    let third_coef = IntRange::with_zero(-9, 9)?.random();
    let fourth_coef = IntRange::with_zero(-9, 9)?.random();
    let (first_exp, first_exp_range) = IntRange::with_zero(0, 2)?.and_random();
    let second_exp = IntRange::with_zero(0, 2)?.random();
    let third_exp = 2;
    let fourth_exp = 1;

    let  first_term: Term = (first_coef, unknown, first_exp).into();
    let  second_term: Term = (second_coef, unknown, second_exp).into();
    let  third_term: Term = (third_coef, unknown, third_exp).into();
    let  fourth_term: Term = (fourth_coef, unknown, fourth_exp).into();

    let original_expression: Expression = Expression::random_order(vec![
        &first_term,
        &second_term,
        &third_term,
        &fourth_term,
    ]);
    let sorted_expression = original_expression.sorted();
    let simplified_expression = original_expression.simplify();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$ &{original_expression} = \\
        = &{sorted_expression} = \\
        = &{simplified_expression} $",
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_exp],
        combinations: first_coef_range.len() * first_exp_range.len(),
    })
}
