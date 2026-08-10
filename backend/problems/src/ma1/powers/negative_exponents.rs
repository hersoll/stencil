use crate::shuffle;
use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay, Number,
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

/// Calculate 7^-2 * 2^3
/// Absolute difficulty: 6
/// Relative difficulty: 8
#[problem]
fn calculate_two_powers(id: i32, _lang: Language) -> Result<Problem> {
    let (base1, base_range) = num_gen::integer().range(2, 9).and_random();
    let base2 = base_range.clone().exclude(base1).random();
    // 4^3 and above are too large to calculate
    let max_exp1 = if base1 < 4 { 3 } else { 2 };
    let max_exp2 = if base2 < 4 { 3 } else { 2 };
    let exp1 = num_gen::integer().range(2, max_exp1).random();
    let exp2 = num_gen::integer().range(2, max_exp2).random();

    let question = format!("{base1}^(-{exp1}) dot {base2}^({exp2})");
    let numerator = base2.pow(exp2);
    let denominator = base1.pow(exp1);
    let answer = Number::fraction(numerator, denominator);
    let mut solution = Solution::block();
    solution
        .write(&question)
        .equals(format!("1 / {base1}^{exp1} dot {base2}^({exp2})"))
        .equals(format!("1 / {denominator} dot {numerator}"))
        .equals(answer);
    if answer.can_be_simplified() {
        solution.equals(answer.simplify());
    }

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.simplify().as_block_math(),
        solution: solution.to_string(),
        identifiers: vec![base1, base2],
        combinations: base_range.len() * (base_range.len() - 1),
    }))
}

/// Write 2x^-3 as a fraction
/// Absolute difficulty: 6
/// Relative difficulty: 8
#[problem]
fn coef_x_as_fraction(id: i32, lang: Language) -> Result<Problem> {
    let (exp, exp_range) = num_gen::integer().range(2, 6).and_random();
    let (coef, coef_range) = num_gen::integer().range(2, 7).and_random();
    let var = symbols::get_unknown()?;

    let question = format!("{coef}{var}^(-{exp})");
    let answer = format!("{coef} / ({var}^{exp})");

    let mut solution = Solution::block_with_text();
    solution
        .write(&question)
        .equals(format!("{coef} dot {var}^(-{exp})"))
        .equals(format!("{coef} dot 1 / {var}^({exp})"))
        .equals(&answer);
    let solution = get_solution(id, lang)?
        .replace_multiple(&[("solution", solution.to_string()), ("var", var.as_math())]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_block_math(),
        solution,
        identifiers: vec![exp, coef],
        combinations: exp_range.len() * coef_range.len(),
    }))
}

/// Write (4x)^-2 as a fraction
/// Absolute difficulty: 6
/// Relative difficulty: 8
#[problem]
fn parenthesis_x_as_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(2, 7).and_random();
    let max_exp = if coef < 4 { 3 } else { 2 };
    let exp = num_gen::integer().range(2, max_exp).random();
    let var = symbols::get_unknown()?;
    let final_coef = coef.pow(exp);

    let question = format!("({coef}{var})^(-{exp})");
    let answer = format!("1 / ({final_coef}{var}^{exp})");

    let solution = Solution::block()
        .write(&question)
        .equals(format!("1 / ({coef}{var})^({exp})"))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_block_math(),
        solution,
        identifiers: coef,
        combinations: coef_range,
    }))
}
