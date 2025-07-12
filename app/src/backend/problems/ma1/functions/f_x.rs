use crate::{
    backend::{translations, IntRange, Problem}, Result
};
/// In this module, problems in the form of f(3) is known as "calculating y"
/// and problems like f(x) = 3 are known as "calculating x"
use macros::problem;

#[problem(id = "without_notation_y", difficulty = 0)]
fn without_notation_y(id: String) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let x = IntRange::without_zero(1, 5)?.random();
    let y = coefficient * x + constant;

    let problem = Problem {
        id,
        question: "without_notation_y".to_string(),
        answer: "without_notation_y".to_string(),
        solution: "  sol".to_string(),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

#[problem(id = "without_notation_x", difficulty = 0)]
fn without_notation_x(id: String) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let x = IntRange::without_zero(1, 5)?.random();
    let y = coefficient * x + constant;

    let problem = Problem::new("without_notation_x", "without_notation_x");
    Ok(problem)
}
