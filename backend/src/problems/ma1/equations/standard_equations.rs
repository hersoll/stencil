use crate::problems::Problem;
use crate::{IntRange, solutions};
use crate::{math_utils, typst_utils};
use anyhow::Result;
use macros::problem;

/// x + 3 = 12
/// Difficulty: 0
#[problem]
fn only_addition_or_subtraction(id: String, _lang: &str) -> Result<Problem> {
    let answer = IntRange::with_zero(0, 9)?.random();
    let (constant, constant_range) = IntRange::without_zero(-answer, 9)?.and_random();

    let solution = format!(
        "x {constant:+} &= {rhs}\\ {sub_constant} \\
              x &= {answer}\\",
        rhs = answer + constant,
        sub_constant = typst_utils::formatting::subtract_number(constant),
    );

    let problem = Problem {
        id,
        // prefix: "Lös ekvationen",
        // group-prefix: "Lös ekvationerna"
        question: format!("$x {constant:+} = {}$", answer + constant),
        answer: format!("$x = {answer}$"),
        solution: typst_utils::formatting::equation_solution(solution),
        identifiers: vec![constant],
        combinations: constant_range.len(),
    };
    Ok(problem)
}

/// 3x = 12
/// Difficulty: 0
#[problem]
fn only_multiplication(id: String, _lang: &str) -> Result<Problem> {
    let answer = IntRange::without_zero(2, 5)?.random();
    let (coefficient, coefficient_range) = IntRange::without_zero(3, 9)?.and_random();

    let solution = format!(
        "{c}x &= {rhs} \\ div {c}\\
              x &= {a} \\",
        c = coefficient,
        a = answer,
        rhs = answer * coefficient,
    );

    let problem = Problem {
        id,
        question: format!("${}x = {}$", coefficient, answer * coefficient),
        answer: format!("$x = {}$", answer),
        solution: typst_utils::formatting::equation_solution(solution),
        identifiers: vec![coefficient],
        combinations: coefficient_range.len(),
    };

    Ok(problem)
}

/// 4x + 1 = 13
/// Difficulty: 1
#[problem]
fn positive_up_to_5(id: String, _lang: &str) -> Result<Problem> {
    let answer = IntRange::without_zero(0, 5)?.random();
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 5)?.and_random();
    let (constant, constant_range) =
        IntRange::without_zero((-coefficient * answer).max(-5), 5)?.and_random();

    let solution = solutions::linear_equations::integer_answer(coefficient, 'x', constant, answer);

    let problem = Problem {
        id,
        question: format!(
            "${cf}x {co:+} = {rhs}$",
            cf = coefficient,
            co = constant,
            rhs = coefficient * answer + constant
        ),
        answer: format!("$x = {}$", answer),
        solution,
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// 6x + 8 = 20
/// Difficulty: 2
#[problem]
fn positive_answers(id: String, _lang: &str) -> Result<Problem> {
    let answer = IntRange::without_zero(0, 10)?.random();
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 9)?.and_random();
    let (constant, constant_range) =
        IntRange::without_zero((-coefficient * answer).max(-10), 10)?.and_random();

    let solution = solutions::linear_equations::integer_answer(coefficient, 'x', constant, answer);

    let problem = Problem {
        id,
        question: format!(
            "${cf}x {co:+} = {rhs}$",
            cf = coefficient,
            co = constant,
            rhs = coefficient * answer + constant
        ),
        answer: format!("$x = {}$", answer),
        solution,
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// 6x + 8 = 19
/// Difficulty: 3
#[problem]
fn positive_rational(id: String, _lang: &str) -> Result<Problem> {
    let (denominator, denominator_range) = IntRange::without_zero(2, 9)?.and_random();
    let numerator = IntRange::without_zero(1, denominator * 2 - 1)?
        .exclude(denominator)
        .random();
    let coefficient = denominator;
    let (constant, constant_range) =
        IntRange::without_zero((-numerator).max(-10), 10)?.and_random();

    let solution = solutions::linear_equations::positive_rational_answer(
        coefficient,
        'x',
        constant,
        numerator,
        denominator,
    );
    let (simplified_numerator, simplified_denominator) =
        math_utils::simplified_fraction(numerator, denominator);

    let problem = Problem {
        id,
        question: format!(
            "${cf}x {co:+} = {rhs}$",
            cf = coefficient,
            co = constant,
            rhs = numerator + constant
        ),
        answer: format!("$x = {simplified_numerator}/{simplified_denominator}$"),
        solution,
        identifiers: vec![coefficient, constant],
        combinations: denominator_range.len() * constant_range.len(),
    };
    Ok(problem)
}
