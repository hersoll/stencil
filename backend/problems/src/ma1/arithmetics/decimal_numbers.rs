use anyhow::Result;
use macros::problem;
use math::num_gen;
use types::{lang::Language, problems::Problem};

/// 0,14 + 0,3
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn add_decimals_two_decimals_first(id: i32, _lang: Language) -> Result<Problem> {
    let two_decimal_range = num_gen::decimal().with_decimals(2).range(0.1, 0.3);
    let two_decimal_num = two_decimal_range.random();
    let one_decimal_range = num_gen::decimal().with_decimals(1).range(0.2, 0.6);
    let one_decimal_num = one_decimal_range.random();
    let sum = one_decimal_num + two_decimal_num;
    let question = format!("${two_decimal_num} + {one_decimal_num}$");
    let answer = format!("${sum}$");
    let solution = format!(
        "${two_decimal_num} + {one_decimal_num} = {two_decimal_num} + {one_decimal_num:.2} = {sum}$"
    );
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![one_decimal_num, two_decimal_num],
        combinations: two_decimal_range.len() * one_decimal_range.len(),
    })
}

/// 0,3 + 0,14
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn add_decimals_one_decimal_first(id: i32, _lang: Language) -> Result<Problem> {
    let two_decimal_range = num_gen::decimal().with_decimals(2).range(0.1, 0.3);
    let two_decimal_num = two_decimal_range.random();
    let one_decimal_range = num_gen::decimal().with_decimals(1).range(0.2, 0.6);
    let one_decimal_num = one_decimal_range.random();
    let sum = one_decimal_num + two_decimal_num;
    let question = format!("${one_decimal_num} + {two_decimal_num}$");
    let answer = format!("${sum}$");
    let solution = format!(
        "${one_decimal_num} + {two_decimal_num} = {one_decimal_num:.2} + {two_decimal_num} = {sum}$"
    );
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![one_decimal_num, two_decimal_num],
        combinations: two_decimal_range.len() * one_decimal_range.len(),
    })
}

/// 0,75 - 0,2
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn subtract_decimals_two_decimals_first(id: i32, _lang: Language) -> Result<Problem> {
    let larger_num_range = num_gen::decimal().with_decimals(2).range(0.3, 0.9);
    let larger_num = larger_num_range.random();
    let smaller_num_range = num_gen::decimal().with_decimals(1).range(0.1, larger_num);
    let smaller_num = smaller_num_range.random();
    let difference = larger_num - smaller_num;
    let question = format!("${larger_num} - {smaller_num}$");
    let answer = format!("${difference}$");
    let solution =
        format!("${larger_num} - {smaller_num} = {larger_num} - {smaller_num:.2} = {difference}$");
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![larger_num, smaller_num],
        combinations: larger_num_range.len() * smaller_num_range.len(),
    })
}

/// 0,1 - 0,04
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn subtract_decimals_one_decimal_first(id: i32, _lang: Language) -> Result<Problem> {
    let larger_num_range = num_gen::decimal().with_decimals(1).range(0.1, 0.9);
    let larger_num = larger_num_range.random();
    let smaller_num_range = num_gen::decimal().with_decimals(2).range(0.01, 0.09);
    let smaller_num = smaller_num_range.random();
    let difference = larger_num - smaller_num;
    let question = format!("${larger_num} - {smaller_num}$");
    let answer = format!("${difference}$");
    let solution =
        format!("${larger_num} - {smaller_num} = {larger_num:.2} - {smaller_num} = {difference}$");
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![larger_num, smaller_num],
        combinations: larger_num_range.len() * smaller_num_range.len(),
    })
}
