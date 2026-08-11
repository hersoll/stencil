use anyhow::Result;
use macros::problem;
use math::formatting::{self, divide_number, multiply_number};
use math::{
    Number,
    num_gen::{self, NumberGenerator},
    symbols,
};
use registry::get_solution;
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// x/3 = 4
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn one_denom_one_variable(id: i32, _lang: Language) -> Result<Problem> {
    let denominator_range = num_gen::integer().range(2, 10);
    let denominator = denominator_range.random();
    let rhs = num_gen::integer().range(1, 10).random();
    let final_answer = denominator * rhs;
    let unknown = 'x';

    let question = format!("$ {unknown}/{denominator} &= {rhs} $");
    let answer = format!("${unknown} = {final_answer}$");
    let mut solution = Solution::with_steps();
    solution
        .aligned(format!("{unknown}/{denominator}"), rhs)
        .step(multiply_number(denominator))
        .aligned(unknown, final_answer);

    Ok(Problem::from(ProblemParameters {
        question,
        answer,
        solution,
        id,
        identifiers: denominator,
        combinations: denominator_range,
    }))
}

/// (2x + 1)/3 = 4
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn expression_divided_by_number(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(-8, 8).random();
    let (coef, coef_range) = num_gen::integer().range(2, 9).and_random();
    let (denom, denom_range) = num_gen::integer()
        .range(2, 7)
        .exclude(coef)
        .exclude(coef / 2)
        .exclude(coef / 3)
        .exclude(coef / 4)
        .and_random();
    let rhs = num_gen::integer().range(-10, 10).exclude(0).random();
    let constant = rhs * denom - coef * answer;
    let var = symbols::get_unknown()?;
    let var_term = coef * var;

    let lhs = format!("({var_term} {constant:+})/{denom}");
    let solution = Solution::with_steps()
        .aligned(&lhs, rhs)
        .step(formatting::multiply_number(denom))
        .aligned(var_term.and(&constant), rhs * denom)
        .step(formatting::subtract_number(constant))
        .aligned(var_term, rhs * denom - constant)
        .step(formatting::divide_number(coef))
        .aligned(var, answer)
        .to_string();
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {lhs} = {rhs} $"),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![coef, denom],
        combinations: coef_range.len() * denom_range.len(),
    }))
}

/// 2/x = 5
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn divided_by_x(id: i32, _lang: Language) -> Result<Problem> {
    let (numerator, num_range) = num_gen::integer().range(2, 9).and_random();
    let rhs = num_range
        .clone()
        .exclude(numerator)
        .exclude(numerator / 2)
        .exclude(numerator / 3)
        .exclude(numerator / 4)
        .random();
    let var = symbols::get_unknown()?;
    let answer = Number::fraction(numerator, rhs);
    let lhs = format!("{numerator} / {var}");
    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, rhs)
        .step(format!("dot {var}"))
        .aligned(numerator, rhs * var)
        .step(formatting::divide_number(rhs));
    if answer.can_be_simplified() {
        solution.aligned(
            var,
            format!(
                "{} = {}",
                formatting::show_simplification(answer),
                answer.simplify()
            ),
        );
    } else {
        solution.aligned(var, answer);
    }
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {lhs} = {rhs} $"),
        answer: format!("${var} = {}$", answer.simplify()),
        solution,
        identifiers: numerator,
        combinations: num_range,
    }))
}

/// 2/x = 5/7
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn two_fractions(id: i32, lang: Language) -> Result<Problem> {
    // Choose from odd and even to prevent awkward fractions, like 2/4
    let (numerator1, even_range) = num_gen::integer().range_step(2, 8, 2).and_random();
    let (numerator2, odd_range) = num_gen::integer().range_step(3, 9, 2).and_random();
    let denom2 = even_range.random();

    let var = symbols::get_unknown()?;
    let answer = Number::fraction(numerator1 * denom2, numerator2);
    let lhs = format!("{numerator1} / {var}");
    let rhs = format!("{numerator2} / {denom2}");

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, &rhs)
        .step(format!("dot {var} dot {denom2}"))
        .aligned(numerator1 * denom2, numerator2 * var)
        .step(formatting::divide_number(numerator2));
    if answer.can_be_simplified() {
        solution.aligned(
            var,
            format!(
                "{} = {}",
                formatting::show_simplification(answer),
                answer.simplify()
            ),
        );
    } else {
        solution.aligned(var, answer);
    }

    let solution = get_solution(id, lang)?.replace_one("solution", solution);
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$ {lhs} = {rhs} $"),
        answer: format!("${var} = {}$", answer.simplify()),
        solution,
        identifiers: vec![numerator1, numerator2],
        combinations: even_range.len() * odd_range.len(),
    }))
}

/// x/5 + x = 12
/// Absolute difficulty: 6
/// Relative difficulty: 7
#[problem]
fn one_denom_and_unit_variable_integers_positive(id: i32, _lang: Language) -> Result<Problem> {
    let denominator_range = num_gen::integer().range(3, 5);
    let denominator = denominator_range.random();
    // n is the multiple of the denominator + 1, will show up in both the question and answer
    let n = num_gen::integer().range(1, 3).random();
    let rhs = n * (denominator + 1);
    let final_answer = n * denominator;
    let unknown = 'x';

    let question = format!("$ {unknown}/{denominator} + {unknown} &= {rhs} $");
    let answer = format!("${unknown} = {final_answer}$");
    let mut solution = Solution::with_steps();
    solution
        .aligned(format!("{unknown}/{denominator} + {unknown}"), rhs)
        .step(multiply_number(denominator))
        .aligned(
            format!("{unknown} + {denominator}{unknown}"),
            rhs * denominator,
        )
        .aligned(format!("{}{unknown}", denominator + 1), rhs * denominator)
        .step(divide_number(denominator + 1))
        .aligned(unknown, final_answer);

    Ok(Problem::from(ProblemParameters {
        question,
        answer,
        solution,
        id,
        identifiers: denominator,
        combinations: denominator_range,
    }))
}
/// x - x/3 = 8
/// Absolute difficulty: 6
/// Relative difficulty: 7
#[problem]
fn unit_variable_and_one_denom_integers_positive(id: i32, _lang: Language) -> Result<Problem> {
    let denominator_range = num_gen::integer().range(3, 5);
    let denominator = denominator_range.random();
    // n is the multiple of the denominator - 1, will show up in both the question and answer
    let n = num_gen::integer().range(1, 3).random();
    let rhs = n * (denominator - 1);
    let final_answer = n * denominator;
    let unknown = 'x';

    let question = format!("$ {unknown} - {unknown}/{denominator} &= {rhs} $");
    let answer = format!("${unknown} = {final_answer}$");
    let mut solution = Solution::with_steps();
    solution
        .aligned(format!("{unknown} - {unknown}/{denominator}"), rhs)
        .step(multiply_number(denominator))
        .aligned(
            format!("{denominator}{unknown} - {unknown}"),
            rhs * denominator,
        )
        .aligned(format!("{}{unknown}", denominator - 1), rhs * denominator)
        .step(divide_number(denominator - 1))
        .aligned(unknown, final_answer);

    Ok(Problem::from(ProblemParameters {
        question,
        answer,
        solution,
        id,
        identifiers: denominator,
        combinations: denominator_range,
    }))
}

/// x/4 - x = 9
/// Absolute difficulty: 6
/// Relative difficulty: 7
#[problem]
fn unit_variable_and_one_denom_integers_with_negatives(
    id: i32,
    _lang: Language,
) -> Result<Problem> {
    let denominator_range = num_gen::integer().range(3, 5);
    let denominator = denominator_range.random();
    // n is a multiple, will show up in both the question and answer
    let n = num_gen::integer().range(-3, 3).exclude(0).random();
    let rhs = (1 - denominator) * n;
    let final_answer = denominator * n;
    let unknown = 'x';

    let question = format!("$ {unknown}/{denominator} - {unknown} &= {rhs} $");
    let answer = format!("${unknown} = {final_answer}$");

    let mut solution = Solution::with_steps();
    solution
        .aligned(format!("{unknown}/{denominator} - {unknown}"), rhs)
        .step(formatting::multiply_number(denominator))
        .aligned(
            format!("{unknown} - {denominator}{unknown}"),
            rhs * denominator,
        )
        .aligned(format!("{}{unknown}", 1 - denominator), rhs * denominator)
        .step(formatting::divide_number(1 - denominator))
        .aligned(unknown, final_answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: denominator,
        combinations: denominator_range,
    }))
}
