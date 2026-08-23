use anyhow::Result;
use macros::problem;
use math::{
    num_gen::{self, NumberGenerator},
    symbols,
};
use registry::get_solution;
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters},
};

/// What's missing? (x + 3)^2 = x^2 + 6x + A
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn simple_missing_last_positive(id: i32, lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let missing = constant.pow(2);
    let double = constant * 2;
    let missing_icon = "suit.heart.filled";
    let var = symbols::X;

    let solution =
        get_solution(id, lang)?.replace_multiple(&[("num", constant), ("square", missing)]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$({var}{constant:+})^2 = {var}^2 {double:+}{var} + {missing_icon}$"),
        answer: format!("${missing_icon} = {missing}$"),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// What's missing? (x - 3)^2 = x^2 - 6x + A
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn simple_missing_last_negative(id: i32, lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, -1).and_random();
    let missing = constant.pow(2);
    let double = constant * 2;
    let missing_icon = "triangle.filled.t";
    let var = symbols::X;

    let solution =
        get_solution(id, lang)?.replace_multiple(&[("num", constant.abs()), ("square", missing)]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$({var}{constant:+})^2 = {var}^2 {double:+}{var} + {missing_icon}$"),
        answer: format!("${missing_icon} = {missing}$"),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}
