use crate::backend::typst_formatting::{self, equation_solution};
use crate::backend::{IntRange, Problem};
use crate::Result;
use macros::problem;

/// x/3 = 4
/// Difficulty: 0
#[problem]
fn one_denominator_one_variable(id: String, _lang: &str) -> Result<Problem> {
    let (denominator, denominator_range) = IntRange::without_zero(2, 10)?.and_random();
    let rhs = IntRange::without_zero(1, 10)?.random();
    let final_answer = denominator * rhs;
    let unknown = 'x';

    let question = format!("${unknown}/{denominator} &= {rhs}$");
    let answer = format!("${unknown} = {final_answer}$");
    let solution = equation_solution(format!(
        "{unknown}/{denominator} &= {rhs} \\ dot.op {denominator} \\
        {unknown} &= {final_answer} \\"
    ));

    Ok(Problem {
        question,
        answer,
        solution,
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}

/// x/5 + x = 12
/// Difficulty: 2
#[problem]
fn one_denom_and_unit_variable_integers_positive(id: String, _lang: &str) -> Result<Problem> {
    let (denominator, denominator_range) = IntRange::without_zero(3, 5)?.and_random();
    // n is the multiple of the denominator + 1, will show up in both the question and answer
    let n = IntRange::without_zero(1, 3)?.random();
    let rhs = n * (denominator + 1);
    let final_answer = n * denominator;
    let unknown = 'x';

    let question = format!("${unknown}/{denominator} + {unknown} &= {rhs}$");
    let answer = format!("${unknown} = {final_answer}$");
    let solution = equation_solution(format!(
        "{unknown}/{denominator} + {unknown} &= {rhs} \\ dot.op {denominator} \\
        {unknown} + {denominator}{unknown} &= {rhs_denom} \\ \\
        {denom_plus_one}{unknown} &= {rhs_denom} \\ div {denom_plus_one}\\
        {unknown} &= {final_answer} \\",
        denom_plus_one = denominator + 1,
        rhs_denom = rhs * denominator,
    ));

    Ok(Problem {
        question,
        answer,
        solution,
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}
/// x - x/3 = 8
/// Difficulty: 2
#[problem]
fn unit_variable_and_one_denom_integers_positive(id: String, _lang: &str) -> Result<Problem> {
    let (denominator, denominator_range) = IntRange::without_zero(3, 5)?.and_random();
    // n is the multiple of the denominator - 1, will show up in both the question and answer
    let n = IntRange::without_zero(1, 3)?.random();
    let rhs = n * (denominator - 1);
    let final_answer = n * denominator;
    let unknown = 'x';

    let question = format!("${unknown} - {unknown}/{denominator} &= {rhs}$");
    let answer = format!("${unknown} = {final_answer}$");
    let solution = equation_solution(format!(
        "{unknown} - {unknown}/{denominator} &= {rhs} \\ dot.op {denominator} \\
        {denominator}{unknown} - {unknown} &= {rhs_denom} \\ \\
        {denom_minus_one}{unknown} &= {rhs_denom} \\ div {denom_minus_one}\\
        {unknown} &= {final_answer} \\",
        denom_minus_one = denominator - 1,
        rhs_denom = rhs * denominator,
    ));

    Ok(Problem {
        question,
        answer,
        solution,
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}

/// x/4 - x = 9
/// Difficulty: 3
#[problem]
fn unit_variable_and_one_denom_integers_with_negatives(id: String, _lang: &str) -> Result<Problem> {
    let (denominator, denominator_range) = IntRange::without_zero(3, 5)?.and_random();
    // n is a multiple, will show up in both the question and answer
    let n = IntRange::without_zero(-3, 3)?.random();
    let rhs = (1 - denominator) * n;
    let final_answer = denominator * n;
    let unknown = 'x';

    let question = format!("${unknown}/{denominator} - {unknown} &= {rhs}$");
    let answer = format!("${unknown} = {final_answer}$");
    let solution = equation_solution(format!(
        "{unknown}/{denominator} - {unknown} &= {rhs} \\ dot.op {denominator} \\
        {unknown} - {denominator}{unknown} &= {rhs_denom} \\ \\
        {one_minus_denom}{unknown} &= {rhs_denom} \\ div {one_minus_denom_par}\\
        {unknown} &= {final_answer} \\",
        one_minus_denom = 1 - denominator,
        one_minus_denom_par = typst_formatting::parentheses(1 - denominator),
        rhs_denom = rhs * denominator,
    ));

    Ok(Problem {
        question,
        answer,
        solution,
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}
