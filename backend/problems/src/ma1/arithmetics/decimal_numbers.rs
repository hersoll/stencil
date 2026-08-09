use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay,
    num_gen::{self, NumberGenerator},
};
use registry::get_solution;
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

use crate::shuffle_numbers;

fn generate_multiplication_problem(
    id: i32,
    lang: Language,
    range_1: &impl NumberGenerator,
    range_2: &impl NumberGenerator,
) -> Result<Problem> {
    let mut num_1 = range_1.random();
    let mut num_2 = range_2.random();
    let swapped = shuffle_numbers(&mut num_1, &mut num_2);
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

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${num_1} dot {num_2}$"),
        answer,
        solution: format!("{solution_text} \\ {solution_math}"),
        identifiers: if swapped {
            vec![num_2, num_1]
        } else {
            vec![num_1, num_2]
        },
        combinations: range_1.len() * range_2.len(),
    }))
}

fn generate_division_problem(
    id: i32,
    lang: Language,
    answer_range: &impl NumberGenerator,
    denom_range: &impl NumberGenerator,
) -> Result<Problem> {
    let answer = answer_range.random();
    let denom = denom_range.random();
    let mut numerator = denom * answer;
    if numerator.is_integer() {
        numerator = numerator.round(0);
    }

    let int_num = numerator.as_integer();
    let int_denom = denom.as_integer();
    let int_answer = int_num / int_denom;

    let solution_text = get_solution(id, lang)?;
    let mut solution_math = Solution::block_with_text();
    solution_math
        .newline()
        .write(format!("{int_num}/{int_denom} &= {int_answer}"))
        .wide_space()
        .write(format!("{numerator}/{denom} &= {answer}"));

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {numerator}/{denom} $"),
        answer,
        solution: format!("{solution_text} \\ {solution_math}"),
        identifiers: vec![answer, denom],
        combinations: answer_range.len() * denom_range.len(),
    }))
}

/// 0,14 + 0,3
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn add_decimals_two_decimals_first(id: i32, _lang: Language) -> Result<Problem> {
    let two_decimal_range = num_gen::decimals(2).range(0.1, 0.3);
    let two_decimal_num = two_decimal_range.random();
    let one_decimal_range = num_gen::decimals(1).range(0.2, 0.6);
    let one_decimal_num = one_decimal_range.random();
    let sum = one_decimal_num + two_decimal_num;
    let question = format!("${two_decimal_num} + {one_decimal_num}$");
    let answer = format!("${sum}$");
    let solution = format!(
        "${two_decimal_num} + {one_decimal_num} = {two_decimal_num} + {one_decimal_num:.2} = {sum}$"
    );
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![one_decimal_num, two_decimal_num],
        combinations: two_decimal_range.len() * one_decimal_range.len(),
    }))
}

/// 0,3 + 0,14
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn add_decimals_one_decimal_first(id: i32, _lang: Language) -> Result<Problem> {
    let two_decimal_range = num_gen::decimals(2).range(0.1, 0.3);
    let two_decimal_num = two_decimal_range.random();
    let one_decimal_range = num_gen::decimals(1).range(0.2, 0.6);
    let one_decimal_num = one_decimal_range.random();
    let sum = one_decimal_num + two_decimal_num;
    let question = format!("${one_decimal_num} + {two_decimal_num}$");
    let answer = format!("${sum}$");
    let solution = format!(
        "${one_decimal_num} + {two_decimal_num} = {one_decimal_num:.2} + {two_decimal_num} = {sum}$"
    );
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![one_decimal_num, two_decimal_num],
        combinations: two_decimal_range.len() * one_decimal_range.len(),
    }))
}

/// 0,75 - 0,2
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn subtract_decimals_two_decimals_first(id: i32, _lang: Language) -> Result<Problem> {
    let larger_num_range = num_gen::decimals(2).range(0.3, 0.9);
    let larger_num = larger_num_range.random();
    let smaller_num_range = num_gen::decimals(1).range(0.1, larger_num);
    let smaller_num = smaller_num_range.random();
    let difference = larger_num - smaller_num;
    let question = format!("${larger_num} - {smaller_num}$");
    let answer = format!("${difference}$");
    let solution =
        format!("${larger_num} - {smaller_num} = {larger_num} - {smaller_num:.2} = {difference}$");
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![larger_num, smaller_num],
        combinations: larger_num_range.len() * smaller_num_range.len(),
    }))
}

/// 0,3 * 4
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn multiply_one_decimal_integer(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(1).range(0.1, 0.9);
    let range_2 = num_gen::integer().range(2, 9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,03 * 4
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn multiply_two_decimals_integer(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(2).range(0.01, 0.09);
    let range_2 = num_gen::integer().range(2, 9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 4
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn multiply_three_decimals_integer(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(3).range(0.001, 0.009);
    let range_2 = num_gen::integer().range(2, 9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 1,2 / 3
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn divide_one_decimal_integer(id: i32, lang: Language) -> Result<Problem> {
    let answer_range = num_gen::decimals(1).range(0.4, 0.9).exclude(0.5);
    let denom_range = num_gen::integer().range(3, 9).exclude(5);
    generate_division_problem(id, lang, &answer_range, &denom_range)
}

/// 0,03 * 40
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_two_decimals_ten(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(2).range(0.01, 0.09);
    let range_2 = num_gen::integer().numbers(&[20, 30, 40, 50, 60, 70, 80, 90]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,03 * 400
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_two_decimals_hundred(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(2).range(0.01, 0.09);
    let range_2 = num_gen::integer().numbers(&[100, 200, 300, 400, 500, 600, 700, 800, 900]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 40
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_ten(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(3).range(0.001, 0.009);
    let range_2 = num_gen::integer().numbers(&[20, 30, 40, 50, 60, 70, 80, 90]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 400
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_hundred(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(3).range(0.001, 0.009);
    let range_2 = num_gen::integer().numbers(&[100, 200, 300, 400, 500, 600, 700, 800, 900]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 4000
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_thousand(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(3).range(0.001, 0.009);
    let range_2 =
        num_gen::integer().numbers(&[1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000]);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,03 * 0,7
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_two_decimals_one_decimal(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(2).range(0.01, 0.09);
    let range_2 = num_gen::decimals(1).range(0.1, 0.9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,03 * 0,07
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_two_decimals_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(2).range(0.02, 0.09);
    let range_2 = num_gen::decimals(2).range(0.02, 0.09);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 0,7
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_one_decimal(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(3).range(0.001, 0.009);
    let range_2 = num_gen::decimals(1).range(0.1, 0.9);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,003 * 0,07
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn multiply_three_decimals_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let range_1 = num_gen::decimals(3).range(0.002, 0.009);
    let range_2 = num_gen::decimals(2).range(0.02, 0.09);
    generate_multiplication_problem(id, lang, &range_1, &range_2)
}

/// 0,1 - 0,04
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn subtract_decimals_one_decimal_first(id: i32, _lang: Language) -> Result<Problem> {
    let larger_num_range = num_gen::decimals(1).range(0.1, 0.9);
    let larger_num = larger_num_range.random();
    let smaller_num_range = num_gen::decimals(2).range(0.01, 0.09);
    let smaller_num = smaller_num_range.random();
    let difference = larger_num - smaller_num;
    let question = format!("${larger_num} - {smaller_num}$");
    let answer = format!("${difference}$");
    let solution =
        format!("${larger_num} - {smaller_num} = {larger_num:.2} - {smaller_num} = {difference}$");
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![larger_num, smaller_num],
        combinations: larger_num_range.len() * smaller_num_range.len(),
    }))
}

/// 12 / 30
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn divide_tens_tens(id: i32, lang: Language) -> Result<Problem> {
    let (denom, denom_range) = num_gen::integer().range_step(20, 90, 10).and_random();
    let (answer, answer_range) = num_gen::decimals(1)
        .range(0.2, 0.9)
        .exclude(0.5)
        .and_random();
    let numerator = denom * answer;

    // First do 12 / 3 instead
    let small_denom = denom / 10;
    let simple_answer = numerator / small_denom;

    let solution_text = get_solution(id, lang)?;
    let mut solution_math = Solution::block_with_text();
    solution_math
        .newline()
        .write(format!("{numerator}/{small_denom} &= {simple_answer}"))
        .wide_space()
        .write(format!("{numerator}/{denom} &= {answer}"));

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {numerator}/{denom} $"),
        answer,
        solution: format!("{solution_text} \\ {solution_math}"),
        identifiers: vec![answer, denom],
        combinations: denom_range.len() * answer_range.len(),
    }))
}

/// 12 / 300
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn divide_tens_hundreds(id: i32, lang: Language) -> Result<Problem> {
    let (denom, denom_range) = num_gen::integer().range_step(200, 900, 100).and_random();
    let (answer, answer_range) = num_gen::decimals(2).range(0.02, 0.09).and_random();
    let numerator = denom * answer;

    // First do 12 / 3 instead
    let small_denom = denom / 100;
    let simple_answer = numerator / small_denom;

    let solution_text = get_solution(id, lang)?;
    let mut solution_math = Solution::block_with_text();
    solution_math
        .newline()
        .write(format!("{numerator}/{small_denom} &= {simple_answer}"))
        .wide_space()
        .write(format!("{numerator}/{denom} &= {answer}"));

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {numerator}/{denom} $"),
        answer,
        solution: format!("{solution_text} \\ {solution_math}"),
        identifiers: vec![answer, denom],
        combinations: denom_range.len() * answer_range.len(),
    }))
}

/// 12 / 0,3
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn divide_tens_one_decimal(id: i32, lang: Language) -> Result<Problem> {
    let denom_range = num_gen::decimals(1).range(0.2, 0.9);
    let answer_range = num_gen::integer().range_step(20, 90, 10);
    generate_division_problem(id, lang, &answer_range, &denom_range)
}

/// 12 / 0,03
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn divide_tens_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let denom_range = num_gen::decimals(2).range(0.02, 0.09);
    let answer_range = num_gen::integer().range_step(200, 900, 100);
    generate_division_problem(id, lang, &answer_range, &denom_range)
}

/// 120 / 0,03
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn divide_hundreds_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let denom_range = num_gen::decimals(2).range(0.02, 0.09);
    let answer_range = num_gen::integer().range_step(2000, 9000, 1000);
    generate_division_problem(id, lang, &answer_range, &denom_range)
}

/// 1,2 / 0,3
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn divide_one_decimal_one_decimal(id: i32, lang: Language) -> Result<Problem> {
    let denom_range = num_gen::decimals(2).range(0.2, 0.9).exclude(0.5);
    let answer_range = num_gen::integer().range(2, 9).exclude(5);
    generate_division_problem(id, lang, &answer_range, &denom_range)
}

/// 1,2 / 0,03
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn divide_one_decimal_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let denom_range = num_gen::decimals(2).range(0.02, 0.09).exclude(0.05);
    let answer_range = num_gen::integer().range_step(20, 90, 10).exclude(50);
    let answer = answer_range.random();
    let denom = denom_range.random();
    let numerator = (denom * answer).round(1);

    let int_num = numerator.as_integer();
    let int_denom = denom.as_integer();
    let int_answer = int_num / int_denom;

    let solution_text = get_solution(id, lang)?;
    let mut solution_math = Solution::block_with_text();
    solution_math
        .newline()
        .write(format!("{int_num}/{int_denom} &= {int_answer}"))
        .wide_space()
        .write(format!(
            "{numerator}/{int_denom} &= {}",
            numerator / int_denom
        ))
        .wide_space()
        .write(format!("{numerator}/{denom} &= {answer}"));

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {numerator}/{denom} $"),
        answer,
        solution: format!("{solution_text} \\ {solution_math}"),
        identifiers: vec![answer, denom],
        combinations: answer_range.len() * denom_range.len(),
    }))
}

/// 0,12 / 0,3
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn divide_two_decimals_one_decimal(id: i32, lang: Language) -> Result<Problem> {
    let denom_range = num_gen::decimals(1).range(0.2, 0.9).exclude(0.5);
    let answer = denom_range.random();
    let denom = denom_range.random();
    let numerator = (denom * answer).round(2);

    let int_num = numerator.as_integer();
    let int_denom = denom.as_integer();
    let int_answer = int_num / int_denom;

    let solution_text = get_solution(id, lang)?;
    let mut solution_math = Solution::block_with_text();
    solution_math
        .newline()
        .write(format!("{int_num}/{int_denom} &= {int_answer}"))
        .wide_space()
        .write(format!(
            "{numerator}/{int_denom} &= {}",
            numerator / int_denom
        ))
        .wide_space()
        .write(format!("{numerator}/{denom} &= {answer}"));

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {numerator}/{denom} $"),
        answer,
        solution: format!("{solution_text} \\ {solution_math}"),
        identifiers: vec![answer, denom],
        combinations: denom_range.len().pow(2),
    }))
}

/// 0,12 / 0,03
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn divide_two_decimals_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let denom_range = num_gen::decimals(2).range(0.02, 0.09).exclude(0.05);
    let answer_range = num_gen::integer().range(2, 9).exclude(5);
    generate_division_problem(id, lang, &answer_range, &denom_range)
}

/// 0,4^2
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn one_decimal_squared(id: i32, lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::decimals(1).range(0.2, 0.9).and_random();

    let solution_text = get_solution(id, lang)?.replace_one("base", base.as_math());
    let mut solution = Solution::block_with_text();
    solution
        .write(format!("{base}^2"))
        .equals(format!("{base} dot {base}"))
        .equals(base.pow(2));

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${base}^2$"),
        answer: base.pow(2),
        solution: format!("{solution_text} \\ {solution}"),
        identifiers: base,
        combinations: base_range,
    }))
}

/// 0,04^2
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn two_decimals_squared(id: i32, lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::decimals(2).range(0.02, 0.09).and_random();

    let solution_text = get_solution(id, lang)?.replace_one("base", base.as_math());
    let mut solution = Solution::block_with_text();
    solution
        .write(format!("{base}^2"))
        .equals(format!("{base} dot {base}"))
        .equals(base * base);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${base}^2$"),
        answer: (base * base),
        solution: format!("{solution_text} \\ {solution}"),
        identifiers: base,
        combinations: base_range,
    }))
}

/// 0,2^3
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn one_decimal_cubed(id: i32, lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::decimals(1).range(0.1, 0.3).and_random();

    let solution_text = get_solution(id, lang)?.replace_one("base", base.as_math());
    let mut solution = Solution::block_with_text();
    solution
        .write(format!("{base}^3"))
        .equals(format!("{base} dot {base} dot {base}"))
        .equals(base.pow(3));

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${base}^3$"),
        answer: base.pow(3),
        solution: format!("{solution_text} \\ {solution}"),
        identifiers: base,
        combinations: base_range,
    }))
}
