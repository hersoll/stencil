use anyhow::Result;
use macros::problem;
use math::num_gen;
use types::{lang::Language, problems::Problem};

/// Which change factor is equivalent to an increase of 10%?
/// Difficulty: 0
#[problem]
fn integer_increase_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let increase_range = num_gen::integer().range(2, 99);
    let increase = increase_range.random();
    let change_factor = math::utils::to_change_factor(increase);

    let problem_translations = registry::get_problem_data(&name)?;
    let question = registry::replace_placeholders(
        problem_translations.get_question(lang),
        &[("percent", increase.to_string())],
    );

    let solution = format!(
        "$100% + {increase}% = {total}% = {change_factor}$",
        total = 100 + increase
    );
    Ok(Problem {
        name,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![increase],
        combinations: increase_range.len(),
    })
}

/// Which change factor is equivalent to an decrease of 10%?
/// Difficulty: 0
#[problem]
fn integer_decrease_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let decrease_range = num_gen::integer().range(2, 99);
    let decrease = decrease_range.random();
    let change_factor = math::utils::to_change_factor(-decrease);

    let problem_translations = registry::get_problem_data(&name)?;
    let question = registry::replace_placeholders(
        problem_translations.get_question(lang),
        &[("percent", decrease.to_string())],
    );

    let solution = format!(
        "$100% - {decrease}% = {total}% = {change_factor}$",
        total = 100 - decrease
    );
    Ok(Problem {
        name,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![decrease],
        combinations: decrease_range.len(),
    })
}
