use anyhow::Result;
use macros::problem;
use math::{MathDisplay, num_gen};
use types::{lang::Language, problems::Problem};

/// Calculate 9^(1/2)
/// Absolute difficulty: 5
/// Relative difficulty: 1
#[problem]
fn square_root(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer()
        .numbers(&[1, 4, 9, 16, 25, 36, 49, 64, 81, 100])
        .and_random();

    let question = format!("${base}^(1/2)$");
    let answer = base.sqrt();
    let solution = format!("${base}^(1/2) = sqrt({base}) = {answer}$");

    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}

/// Calculate 8^(1/3)
/// Absolute difficulty: 5
/// Relative difficulty: 2
#[problem]
fn cubic_root(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().numbers(&[1, 8, 27]).and_random();

    let question = format!("${base}^(1/3)$");
    let answer = base.root(3);
    let solution = format!("${base}^(1/3) = root(3, {base}) = {answer}$");

    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}
