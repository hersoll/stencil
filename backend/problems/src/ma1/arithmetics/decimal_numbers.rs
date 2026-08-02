use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay, Number,
    num_gen::{self, NumberGenerator},
};
use rand::Rng;
use registry::get_solution;
use types::{lang::Language, problems::Problem};
use typst_writer::formatting::Solution;

/// Shuffles the order of two numbers. Returns true if switched.
fn shuffle_order(num_1: &mut Number, num_2: &mut Number) -> bool {
    let mut rng = rand::rng();
    if rng.random::<f32>() > 0.5 {
        std::mem::swap(&mut *num_1, &mut *num_2);
        true
    } else {
        false
    }
}

fn generate_multiplication_problem(
    id: i32,
    lang: Language,
    range_1: &impl NumberGenerator,
    range_2: &impl NumberGenerator,
) -> Result<Problem> {
    let mut num_1 = range_1.random();
    let mut num_2 = range_2.random();
    let swapped = shuffle_order(&mut num_1, &mut num_2);
    let answer = num_1 * num_2;
    // Needed for solution
    let int_1 = num_1.as_integer();
    let int_2 = num_2.as_integer();
    let int_total = int_1 * int_2;
    let total_decimals = num_1.decimals() + num_2.decimals();
    let solution_text = get_solution(id, lang)?;
    let mut solution_math = Solution::block_with_text();
    solution_math
        .newline()
        .write(format!("{int_1} dot {int_2} &= {int_total}"))
        .newline()
        .write(format!(
            "{num_1} dot {num_2} &= {answer:.*}",
            total_decimals as usize
        ));

    Ok(Problem {
        id,
        question: format!("${num_1} dot {num_2}$"),
        answer: answer.as_math(),
        solution: format!("{solution_text} \\ {solution_math}"),
        identifiers: if swapped {
            vec![num_2, num_1]
        } else {
            vec![num_1, num_2]
        },
        combinations: range_1.len() * range_2.len(),
    })
}

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

/// 0,3 * 4
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn multiply_one_decimal_integer(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(1).range(0.1, 0.9);
    let range_2 = num_gen::integer().range(2, 9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,03 * 4
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn multiply_two_decimals_integer(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(2).range(0.01, 0.09);
    let range_2 = num_gen::integer().range(2, 9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 4
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn multiply_three_decimals_integer(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(3).range(0.001, 0.009);
    let range_2 = num_gen::integer().range(2, 9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,03 * 40
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_two_decimals_ten(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(2).range(0.01, 0.09);
    let range_2 = num_gen::integer().numbers(&[10, 20, 30, 40, 50, 60, 70, 80, 90]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,03 * 400
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_two_decimals_hundred(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(2).range(0.01, 0.09);
    let range_2 = num_gen::integer().numbers(&[100, 200, 300, 400, 500, 600, 700, 800, 900]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 40
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_ten(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(3).range(0.001, 0.009);
    let range_2 = num_gen::integer().numbers(&[10, 20, 30, 40, 50, 60, 70, 80, 90]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 400
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_hundred(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(3).range(0.001, 0.009);
    let range_2 = num_gen::integer().numbers(&[100, 200, 300, 400, 500, 600, 700, 800, 900]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 4000
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_thousand(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimal().with_decimals(3).range(0.001, 0.009);
    let range_2 =
        num_gen::integer().numbers(&[1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
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
