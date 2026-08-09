use crate::shuffle;
use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay,
    num_gen::{self, NumberGenerator},
    symbols,
    utils::parenthesize,
};
use registry::get_solution;
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// 3^2 * 3^-3
/// Absolute difficulty: 3
/// Relative difficulty: 1
#[problem]
fn multiply_powers(id: i32, _lang: Language) -> Result<Problem> {
    let base = num_gen::integer().range(2, 8).random();
    let (mut exp1, exp1_range) = num_gen::integer().range(2, 9).and_random();
    let (mut exp2, exp2_range) = num_gen::integer()
        .range(-9, -2)
        .exclude(-exp1)
        .exclude(-exp1 + 1)
        .and_random();
    shuffle(&mut exp1, &mut exp2);
    let total_exp = exp1 + exp2;

    let eq = format!("{base}^({exp1}) dot {base}^({exp2})");
    let answer = format!("{base}^({total_exp})");
    let solution = Solution::inline()
        .write(&eq)
        .equals(format!("{base}^({exp1} + {})", parenthesize(&exp2)))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: eq.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    }))
}

/// 3^2 / 3^-3
/// Absolute difficulty: 3
/// Relative difficulty: 1
#[problem]
fn divide_powers(id: i32, _lang: Language) -> Result<Problem> {
    let base = num_gen::integer().range(2, 8).random();
    let (mut exp1, exp1_range) = num_gen::integer().range(2, 9).and_random();
    let (mut exp2, exp2_range) = num_gen::integer()
        .range(-9, -2)
        .exclude(-exp1)
        .exclude(-exp1 - 1)
        .and_random();
    shuffle(&mut exp1, &mut exp2);
    let total_exp = exp1 - exp2;

    let eq = format!("{base}^({exp1}) / {base}^({exp2})");
    let answer = format!("{base}^({total_exp})");
    let solution = Solution::block()
        .write(&eq)
        .equals(format!("{base}^({exp1} - {})", parenthesize(&exp2)))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: eq.as_block_math(),
        answer: answer.as_math(),
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    }))
}

/// (3^-2)^2
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn exponentiate_powers(id: i32, _lang: Language) -> Result<Problem> {
    let base = num_gen::integer().range(2, 8).random();
    let (mut exp1, exp1_range) = num_gen::integer().range(2, 9).and_random();
    let (mut exp2, exp2_range) = num_gen::integer().range(-9, -2).and_random();
    shuffle(&mut exp1, &mut exp2);
    let total_exp = exp1 * exp2;

    let eq = format!("({base}^({exp1}))^({exp2})");
    let answer = format!("{base}^({total_exp})");
    let solution = Solution::block()
        .write(&eq)
        .equals(format!(
            "{base}^({} dot {})",
            parenthesize(&exp1),
            parenthesize(&exp2)
        ))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: eq.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    }))
}

/// Write x^-3 as a fraction
/// Absolute difficulty: 3
/// Relative difficulty: 3
#[problem]
fn as_fraction(id: i32, lang: Language) -> Result<Problem> {
    let (exp, exp_range) = num_gen::integer().range(2, 6).and_random();
    let var = symbols::get_unknown()?;
    let solution = get_solution(id, lang)?.replace_one("law", "$ a^(-x) = 1/a^x $");
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${var}^(-{exp})$"),
        answer: format!("1 / ({var}^{exp})").as_block_math(),
        solution,
        identifiers: exp,
        combinations: exp_range,
    }))
}

/// Calculate 7^-2
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn calculate_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(2, 9).and_random();
    let question = format!("{base}^(-2)");
    let square = base.pow(2);
    let answer = format!("1 / {square}");
    let solution = Solution::block()
        .write(&question)
        .equals(format!("1 / {base}^2"))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_block_math(),
        solution,
        identifiers: base,
        combinations: base_range,
    }))
}
