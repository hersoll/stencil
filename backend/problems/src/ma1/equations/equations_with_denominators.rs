use anyhow::Result;
use macros::problem;
use math::num_gen;
use types::{lang::Language, problems::Problem};
use typst_writer::formatting::{self, SolutionWithSteps, divide_number, multiply_number};

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
    let mut solution = SolutionWithSteps::new();
    solution
        .add_aligned(format!("{unknown}/{denominator}"), rhs)
        .with_step(multiply_number(denominator))
        .add_aligned(unknown, final_answer);

    Ok(Problem {
        question,
        answer,
        solution: solution.to_string(),
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}

/// x/5 + x = 12
/// Absolute difficulty: 5
/// Relative difficulty: 2
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
    let mut solution = SolutionWithSteps::new();
    solution
        .add_aligned(format!("{unknown}/{denominator} + {unknown}"), rhs)
        .with_step(multiply_number(denominator))
        .add_aligned(
            format!("{unknown} + {denominator}{unknown}"),
            rhs * denominator,
        )
        .add_aligned(format!("{}{unknown}", denominator + 1), rhs * denominator)
        .with_step(divide_number(denominator + 1))
        .add_aligned(unknown, final_answer);

    Ok(Problem {
        question,
        answer,
        solution: solution.to_string(),
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}
/// x - x/3 = 8
/// Absolute difficulty: 5
/// Relative difficulty: 3
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
    let mut solution = SolutionWithSteps::new();
    solution
        .add_aligned(format!("{unknown} - {unknown}/{denominator}"), rhs)
        .with_step(multiply_number(denominator))
        .add_aligned(
            format!("{denominator}{unknown} - {unknown}"),
            rhs * denominator,
        )
        .add_aligned(format!("{}{unknown}", denominator - 1), rhs * denominator)
        .with_step(divide_number(denominator - 1))
        .add_aligned(unknown, final_answer);

    Ok(Problem {
        question,
        answer,
        solution: solution.to_string(),
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}

/// x/4 - x = 9
/// Absolute difficulty: 5
/// Relative difficulty: 3
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

    let mut sol = formatting::SolutionWithSteps::new();
    sol.add_aligned(format!("{unknown}/{denominator} - {unknown}"), rhs)
        .with_step(formatting::multiply_number(denominator));
    sol.add_aligned(
        format!("{unknown} - {denominator}{unknown}"),
        rhs * denominator,
    );
    sol.add_aligned(format!("{}{unknown}", 1 - denominator), rhs * denominator)
        .with_step(formatting::divide_number(1 - denominator));
    sol.add_aligned(unknown, final_answer);

    Ok(Problem {
        question,
        answer,
        solution: sol.to_string(),
        id,
        identifiers: vec![denominator],
        combinations: denominator_range.len(),
    })
}
