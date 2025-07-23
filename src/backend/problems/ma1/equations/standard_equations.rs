use crate::Result;
use crate::backend::problems::Problem;
use crate::backend::{IntRange, solutions};
use crate::backend::{math_utils, typst_formatting};
use macros::problem;

#[problem(id = "add_sub_only", difficulty = 0)]
fn only_addition_or_subtraction(id: String, _lang: &str) -> Result<Problem> {
    let answer = IntRange::with_zero(0, 9)?.random();
    let (constant, constant_range) = IntRange::without_zero(-answer, 9)?.and_random();

    let solution = format!(
        "x {constant:+} &= {rhs}\\ {sub_constant} \\
              x &= {answer}\\",
        rhs = answer + constant,
        sub_constant = typst_formatting::subtract(constant),
    );

    let problem = Problem {
        id,
        // prefix: "Lös ekvationen",
        // group-prefix: "Lös ekvationerna"
        question: format!("$x {constant:+} = {}$", answer + constant),
        answer: format!("$x = {answer}$"),
        solution: typst_formatting::equation_solution(solution),
        identifiers: vec![constant],
        combinations: constant_range.len(),
    };
    Ok(problem)
}

#[problem(id = "mult_only", difficulty = 0)]
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
        solution: typst_formatting::equation_solution(solution),
        identifiers: vec![coefficient],
        combinations: coefficient_range.len(),
    };

    Ok(problem)
}

#[problem(id = "up_to_5", difficulty = 2)]
fn default_equation_positive_up_to_5(id: String, _lang: &str) -> Result<Problem> {
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

#[problem(id = "default_positive", difficulty = 3)]
fn default_equation_positive_answers(id: String, _lang: &str) -> Result<Problem> {
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
#[problem(id = "rational_positive", difficulty = 4)]
fn default_equation_positive_rational(id: String, _lang: &str) -> Result<Problem> {
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
