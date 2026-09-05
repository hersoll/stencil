use anyhow::Result;
use macros::problem;
use math::{
    num_gen::{self, NumberGenerator},
    utils::to_change_factor,
};
use registry::{get_question, get_solution};
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters},
};

/// 40 is increased by 12%
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn simple_increase(id: i32, lang: Language) -> Result<Problem> {
    let value = num_gen::integer().range(11, 99).random();
    let (percent, perc_range) = num_gen::integer()
        .range(7, 35)
        .exclude_multiple(&[10, 20, 30])
        .and_random();
    let change = to_change_factor(percent);
    let answer = value * change;
    let rounded = answer.round(1);

    let question =
        get_question(id, lang)?.replace_multiple(&[("value", value), ("percent", percent)]);

    let calculation = format!("${value} dot {change} = {answer} approx {rounded}$");
    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("percent", percent.to_string()),
        ("change", change.to_string()),
        ("calc", calculation),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: rounded,
        solution,
        identifiers: percent,
        combinations: perc_range,
    }))
}

/// 40 is decreased by 12%
/// Absolute difficulty: 3
/// Relative difficulty: 3
#[problem]
fn simple_decrease(id: i32, lang: Language) -> Result<Problem> {
    let value = num_gen::integer().range(11, 99).random();
    let (percent, perc_range) = num_gen::integer()
        .range(7, 35)
        .exclude_multiple(&[10, 20, 30])
        .and_random();
    let change = to_change_factor(-percent);
    let answer = value * change;

    let question =
        get_question(id, lang)?.replace_multiple(&[("value", value), ("percent", percent)]);

    let calculation = format!("${value} dot {change} = {answer}$");
    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("percent", percent.to_string()),
        ("change", change.to_string()),
        ("calc", calculation),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: percent,
        combinations: perc_range,
    }))
}
