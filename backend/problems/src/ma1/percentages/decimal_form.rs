use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay,
    num_gen::{self, NumberGenerator},
};
use types::{lang::Language, problems::Problem};

/// Write 56% in decimal form
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn two_digit_to_decimal_form(id: i32, _lang: Language) -> Result<Problem> {
    let percentage_range = num_gen::integer()
        .range(11, 99)
        .exclude_multiple(&[20, 30, 40, 50, 60, 70, 80, 90]);
    let percentage = percentage_range.random();
    let decimal_form = (percentage / 100).to_decimal();

    let question = format!("${percentage}%$").as_math();
    let answer = decimal_form.as_math();
    let solution = format!("$1% = num(0.01) \\ 10% = num(0.10) \\ {percentage}% = {decimal_form}$");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![percentage],
        combinations: percentage_range.len(),
    })
}

/// Write 156% in decimal form
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn three_digit_to_decimal_form(id: i32, _lang: Language) -> Result<Problem> {
    let percentage_range = num_gen::integer()
        .range(101, 499)
        .exclude_multiple(&[200, 300, 400]);
    let percentage = percentage_range.random();
    let decimal_form = (percentage / 100).to_decimal();

    let question = format!("${percentage}%$");
    let answer = decimal_form.as_math();
    let solution = format!("$1% = num(0.01) \\ 10% = num(0.10) \\ {percentage}% = {decimal_form}$");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![percentage],
        combinations: percentage_range.len(),
    })
}

/// Convert 0,3 to percent form
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn decimal_to_percent(id: i32, _lang: Language) -> Result<Problem> {
    let percentage_range = num_gen::integer().numbers(&[10, 20, 30, 40, 50, 60, 70, 80, 90]);
    let percentage = percentage_range.random();
    let decimal_form = (percentage / 100).to_decimal();

    let question = decimal_form.as_math();
    let answer = format!("${percentage}%$");
    let solution = format!("${decimal_form} = {decimal_form:.2} = {percentage}%$");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![percentage],
        combinations: percentage_range.len(),
    })
}
