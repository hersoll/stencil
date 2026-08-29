use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay,
    num_gen::{self, NumberGenerator},
    symbols,
};
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// (x + 3)(x + 5)
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn both_positive(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 6).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var + const1;
    let expr2 = var + const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (x - 3)(x + 5)
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn first_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 7).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var - const1;
    let expr2 = var + const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (x + 3)(x - 5)
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn second_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 7).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var + const1;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (x - 3)(x - 5)
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn both_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 7).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var - const1;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (2x - 3)(x + 5)
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn one_coef(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let (coef, coef_range) = num_gen::integer().range(2, 4).and_random();
    let var = symbols::get_unknown()?;

    let expr1 = coef * var - const1;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2, coef],
        combinations: const_range.len() * (const_range.len() - 1) * coef_range.len(),
    }))
}

/// (2x - 3)(3x + 5)
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn two_coefs(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let (coef1, coef_range) = num_gen::integer().range(2, 4).and_random();
    let coef2 = coef_range.random();
    let var = symbols::get_unknown()?;

    let expr1 = coef1 * var - const1;
    let expr2 = coef2 * var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2, coef1, coef2],
        combinations: const_range.len() * (const_range.len() - 1) * coef_range.len().pow(2),
    }))
}
