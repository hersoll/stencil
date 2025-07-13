use std::collections::HashMap;

use crate::{
    backend::{
        translations::{QUESTION_TRANSLATIONS}, typst_formatting::equation_solution, IntRange, Problem
    },Result
};
use macros::problem;

// In this module, problems in the form of f(3) is known as "calculating y"
// and problems like f(x) = 3 are known as "calculating x"

#[problem(id = "without_notation_y", difficulty = 0)]
fn without_notation_y(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let x = IntRange::without_zero(1, 5)?.random();
    let y = coefficient * x + constant;

    let expression = format!("y = {}x {:+}", coefficient, constant);
    let map = HashMap::from([("expression", expression), ("x", x.to_string())]);

    let question = QUESTION_TRANSLATIONS.get_placeholder_phrase(&id, map, lang)?;

    let solution = format!(
        "y &= {coefficient}x {constant:+} \\x={x} \\
       y &= {coefficient} dot.op gray({x}) {constant:+} \\ \\
       y &= {prod} {constant:+} \\ \\
       y &= {y} \\",
        prod = x * coefficient
    );

    let problem = Problem {
        id,
        question,
        answer: format!("$y = {}$", y),
        solution: equation_solution(solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

#[problem(id = "without_notation_x", difficulty = 1)]
fn without_notation_x(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let x = IntRange::without_zero(1, 5)?.random();
    let y = coefficient * x + constant;

    let expression = format!("y = {}x {:+}", coefficient, constant);
    let map = HashMap::from([("expression", expression), ("y", y.to_string())]);

    let question = QUESTION_TRANSLATIONS.get_placeholder_phrase(&id, map, lang)?;

    let solution = format!(
        "y &= {coefficient}x {constant:+} \\y={y} \\
        {y} &= {coefficient}x {constant:+} \\ {inverse:+} \\
        {lhs} &= {coefficient}x \\ div {coefficient} \\
        {x} &= x \\",
        inverse = -constant,
        lhs = x * coefficient
    );

    let problem = Problem {
        id,
        question,
        answer: format!("$x = {}$", x),
        solution: equation_solution(solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}
