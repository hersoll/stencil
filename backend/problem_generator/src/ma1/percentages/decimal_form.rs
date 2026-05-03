use anyhow::Result;
use macros::problem;
use math::num_gen;
use types::{lang::Language, problems::Problem};

/// Write 56% in decimal form
/// Difficulty: 0
#[problem]
fn two_digit_to_decimal_form(name: String, _lang: &Language) -> Result<Problem> {
    let percentage_range = num_gen::integer()
        .range(11, 99)
        .exclude_multiple(&[20, 30, 40, 50, 60, 70, 80, 90]);
    let percentage = percentage_range.random();
    let decimal_form = (percentage / 100).to_decimal();

    let question = format!("${percentage}%$");
    let answer = format!("${decimal_form}$");
    let solution = format!("$1% = num(0.01) \\ 10% = num(0.10) \\ {percentage}% = {decimal_form}$");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![percentage],
        combinations: percentage_range.len(),
    })
}

// /// Write 145% in decimal form
// /// Difficulty: 0
// #[problem]
// fn three_digit_to_decimal_form(name: String, _lang: &Language) -> Result<Problem> {
//     todo!()
// }
