use anyhow::Result;
use macros::problem;
use math::{
    self, Number, Term, num_gen,
    symbols::{self, Symbol},
};
use types::{lang::Language, problems::Problem};
use typst_writer::{
    custom_math::solutions,
    formatting::{SolutionWithSteps, divide_number, subtract_number},
};

/// x + 3 = 12
/// Difficulty: 0
#[problem]
fn only_addition_or_subtraction(name: String, _lang: &Language) -> Result<Problem> {
    let answer = num_gen::integer().range(0, 9).random();
    let (constant, constant_range) = num_gen::integer().range(-answer, 9).and_random();
    let unknown = symbols::get_unknown()?;

    let mut solution = SolutionWithSteps::new();
    solution
        .add_aligned(format!("{unknown}{constant:+}"), answer + constant) // x + 3 = 12
        .with_step(subtract_number(constant))
        .add_aligned(unknown, answer); // x = 9

    Ok(Problem {
        name,
        question: format!("${unknown}{constant:+} = {}$", answer + constant),
        answer: format!("${unknown} = {answer}$"),
        solution: solution.to_string(),
        identifiers: vec![constant],
        combinations: constant_range.len(),
    })
}

/// 3x = 12
/// Difficulty: 0
#[problem]
fn only_multiplication(name: String, _lang: &Language) -> Result<Problem> {
    let answer = num_gen::integer().range(2, 5).random();
    let (coefficient, coefficient_range) = num_gen::integer().range(3, 9).and_random();
    let unknown = symbols::get_unknown()?;

    let mut solution = SolutionWithSteps::new();
    solution
        .add_aligned(format!("{coefficient}{unknown}"), coefficient * answer) // 3x = 12
        .with_step(divide_number(coefficient))
        .add_aligned(unknown, answer); // x = 4

    Ok(Problem {
        name,
        question: format!("${coefficient}{unknown} = {}$", answer * coefficient),
        answer: format!("${unknown} = {answer}$"),
        solution: solution.to_string(),
        identifiers: vec![coefficient],
        combinations: coefficient_range.len(),
    })
}

/// 4x + 1 = 13
/// Difficulty: 1
#[problem]
fn positive_up_to_5(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(0, 5).random();
    let (coefficient, coefficient_range) = num_gen::integer().range(2, 5).and_random();
    let (constant, constant_range) = num_gen::integer()
        .range((-coefficient * answer).max(math::Number::Integer(-5)), 5)
        .and_random();
    let term = Term::from_num_and_vars(coefficient, unknown);

    let mut solution = SolutionWithSteps::new();
    solution
        .add_aligned(
            format!("{term}{constant:+}"),
            coefficient * answer + constant,
        )
        .with_step(subtract_number(constant))
        .add_aligned(&term, coefficient * answer)
        .with_step(divide_number(coefficient))
        .add_aligned(unknown, answer);

    Ok(Problem {
        name,
        question: format!(
            "${term}{constant:+} = {rhs}$",
            rhs = coefficient * answer + constant
        ),
        answer: format!("${unknown} = {answer}$"),
        solution: solution.to_string(),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    })
}

/// 6x + 8 = 20
/// Difficulty: 2
#[problem]
fn positive_answers(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(0, 10).random();
    let (coefficient, coefficient_range) = num_gen::integer().range(2, 9).and_random();
    let (constant, constant_range) = num_gen::integer()
        .range((-coefficient * answer).max(Number::Integer(-10)), 10)
        .and_random();

    let term = Term::from_num_and_vars(coefficient, unknown);

    let mut solution = SolutionWithSteps::new();
    solution
        .add_aligned(
            format!("{term}{constant:+}"),
            coefficient * answer + constant,
        )
        .with_step(subtract_number(constant))
        .add_aligned(&term, coefficient * answer)
        .with_step(divide_number(coefficient))
        .add_aligned(unknown, answer);

    Ok(Problem {
        name,
        question: format!(
            "${term}{constant:+} = {rhs}$",
            rhs = coefficient * answer + constant
        ),
        answer: format!("${unknown} = {answer}$"),
        solution: solution.to_string(),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    })
}

/// 6x + 8 = 19
/// Difficulty: 3
#[problem]
fn positive_rational(name: String, _lang: &Language) -> Result<Problem> {
    let (denominator, denominator_range) = num_gen::integer().range(2, 9).and_random();
    let numerator = num_gen::integer()
        .range(1, denominator * 2 - 1)
        .exclude(denominator)
        .random();
    let coefficient = denominator;
    let (constant, constant_range) = num_gen::integer()
        .range((-numerator).max(Number::Integer(-10)), 10)
        .and_random();

    let solution = solutions::linear_equations::positive_rational_answer(
        coefficient,
        Symbol("x"),
        constant,
        numerator,
        denominator,
    );
    let (simplified_numerator, simplified_denominator) =
        if let (Number::Integer(num), Number::Integer(denom)) = (numerator, denominator) {
            math::utils::simplified_fraction(num, denom)
        } else {
            (1, 1)
        };

    Ok(Problem {
        name,
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
    })
}
