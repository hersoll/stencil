use anyhow::Result;
use macros::problem;
use math::{IntRange, Term, symbols};
use types::{lang::Language, problems::Problem};
use typst_writer::formatting::equation_solution;

/// 4x + 1 = 2x - 3
/// Difficulty: 1
#[problem]
fn two_positive_coefs_lhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = IntRange::without_zero(1, 6)?.random();
    let (lhs_coef, lhs_range) = IntRange::without_zero(3, 9)?.and_random();
    let lhs_var: Term = (lhs_coef, unknown).into();
    let rhs_coef = IntRange::without_zero(1, lhs_coef - 2)?.random();
    let rhs_var: Term = (rhs_coef, unknown).into();
    let lhs_const = IntRange::without_zero(1, 9)?.random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;

    let question = format!("${lhs_var}{lhs_const:+} = {rhs_var}{rhs_const:+}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_var}{lhs_const:+} &= {rhs_var}{rhs_const:+} \\ {sub_rhs} \\
        {total_var}{lhs_const:+} &= {rhs_const} \\ {sub_lhs} \\
        {total_var} &= {total_const} \\ {div_coef} \\
        {unknown} &= {answer} \\ \\",
        sub_rhs = typst_writer::formatting::subtract_term(&rhs_var),
        sub_lhs = typst_writer::formatting::subtract_number(lhs_const),
        total_var = lhs_var.clone() - rhs_var.clone(),
        div_coef = typst_writer::formatting::divide_number(lhs_coef - rhs_coef),
        total_const = rhs_const - lhs_const
    ));

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range.len(),
    })
}
