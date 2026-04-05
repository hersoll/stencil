use anyhow::Result;
use macros::problem;
use math::{Term, num_gen, symbols};
use types::{lang::Language, problems::Problem};
use typst_writer::formatting::{self, equation_solution};

/// 4x + 1 = 2x + 3
/// Difficulty: 1
#[problem]
fn two_positive_coefs_lhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(1, 6).random();
    let lhs_range = num_gen::integer().range(3, 9);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(1, lhs_coef - 2).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_const = num_gen::integer().range(1, 9).random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;

    let question = format!("${lhs_term}{lhs_const:+} = {rhs_term}{rhs_const:+}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_term}{lhs_const:+} &= {rhs_term}{rhs_const:+} \\ {sub_rhs} \\
        {total_var}{lhs_const:+} &= {rhs_const} \\ {sub_lhs} \\
        {total_var} &= {total_const} \\ {div_coef} \\
        {unknown} &= {answer} \\ \\",
        sub_rhs = formatting::subtract_term(&rhs_term),
        sub_lhs = formatting::subtract_number(lhs_const),
        total_var = lhs_term.clone() - rhs_term.clone(),
        div_coef = formatting::divide_number(lhs_coef - rhs_coef),
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

/// 2x + 1 = 4x + 3
/// Difficulty: 1
#[problem]
fn two_positive_coefs_rhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(1, 6).random();
    let rhs_range = num_gen::integer().range(3, 9);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(1, rhs_coef - 2).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_const = num_gen::integer().range(1, 9).random();
    let lhs_const = (rhs_coef - lhs_coef) * answer + rhs_const;

    let question = format!("${lhs_term}{lhs_const:+} = {rhs_term}{rhs_const:+}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_term}{lhs_const:+} &= {rhs_term}{rhs_const:+} \\ {sub_lhs} \\
        {lhs_const} &= {total_var}{rhs_const:+} \\ {sub_rhs} \\
        {total_const} &= {total_var} \\ {div_coef} \\
        {answer} &= {unknown} \\ \\",
        sub_lhs = formatting::subtract_term(&lhs_term),
        sub_rhs = formatting::subtract_number(rhs_const),
        total_var = rhs_term.clone() - lhs_term.clone(),
        div_coef = formatting::divide_number(rhs_coef - lhs_coef),
        total_const = lhs_const - rhs_const
    ));

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range.len(),
    })
}
