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

/// Which change factor is equivalent to a decrease of 10%?
/// Difficulty: 0
#[problem]
fn integer_decrease_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let decrease_range = num_gen::integer().range(2, 29).exclude(10).exclude(20);
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

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 1.23
/// Difficulty: 0
#[problem]
fn factor_to_increase_two_decimals(name: String, lang: &Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_places(2).range(1.04, 1.25);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let problem_data = registry::get_problem_data(&name)?;
    let answer = registry::replace_placeholders(
        problem_data.get_answer(lang),
        &[("percent", increase.to_string())],
    );
    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        name,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 0.87
/// Difficulty: 0
#[problem]
fn factor_to_decrease_two_decimals(name: String, lang: &Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_places(2).range(0.73, 0.98); // yes, very arbitrary :)
    let change_factor = factor_range.random();
    let decrease = math::utils::change_factor_to_percentage(change_factor).abs();

    let problem_data = registry::get_problem_data(&name)?;
    let answer = registry::replace_placeholders(
        problem_data.get_answer(lang),
        &[("percent", decrease.to_string())],
    );
    let solution = format!(
        "${change_factor} = {total}% = 100% - {decrease}%$",
        total = 100 - decrease
    );
    Ok(Problem {
        name,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 0.23
/// Difficulty: 1
#[problem]
fn factor_to_large_decrease_two_decimals(name: String, lang: &Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_places(2).range(0.11, 0.39);
    let change_factor = factor_range.random();
    let decrease = math::utils::change_factor_to_percentage(change_factor).abs();

    let problem_data = registry::get_problem_data(&name)?;
    let answer = registry::replace_placeholders(
        problem_data.get_answer(lang),
        &[("percent", decrease.to_string())],
    );
    let solution = format!(
        "${change_factor} = {total}% = 100% - {decrease}%$",
        total = 100 - decrease
    );
    Ok(Problem {
        name,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 1.2
/// Difficulty: 1
#[problem]
fn factor_to_increase_one_decimal(name: String, lang: &Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_places(1).range(1.1, 1.9);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let problem_data = registry::get_problem_data(&name)?;
    let answer = registry::replace_placeholders(
        problem_data.get_answer(lang),
        &[("percent", increase.to_string())],
    );
    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        name,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 0.8
/// Difficulty: 1
#[problem]
fn factor_to_decrease_one_decimal(name: String, lang: &Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_places(1).range(0.1, 0.9);
    let change_factor = factor_range.random();
    let decrease = math::utils::change_factor_to_percentage(change_factor).abs();

    let problem_data = registry::get_problem_data(&name)?;
    let answer = registry::replace_placeholders(
        problem_data.get_answer(lang),
        &[("percent", decrease.to_string())],
    );
    let solution = format!(
        "${change_factor} = {total}% = 100% - {decrease}%$",
        total = 100 - decrease
    );
    Ok(Problem {
        name,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 2.12
/// Difficulty: 1
#[problem]
fn factor_to_increase_above_2(name: String, lang: &Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_places(2).range(2.01, 2.99);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let problem_data = registry::get_problem_data(&name)?;
    let answer = registry::replace_placeholders(
        problem_data.get_answer(lang),
        &[("percent", increase.to_string())],
    );
    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        name,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 8.12
/// Difficulty: 2
#[problem]
fn factor_to_increase_large_number(name: String, lang: &Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_places(2).range(5.0, 9.99);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let problem_data = registry::get_problem_data(&name)?;
    let answer = registry::replace_placeholders(
        problem_data.get_answer(lang),
        &[("percent", increase.to_string())],
    );
    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        name,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Which change factor is equivalent to an increase of 4.3%?
/// Difficulty: 2
#[problem]
fn decimal_increase_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let increase_range = num_gen::decimal().with_places(1).range(2, 10);
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

/// Which change factor is equivalent to a decrease of 4.3%?
/// Difficulty: 2
#[problem]
fn decimal_decrease_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let decrease_range = num_gen::decimal().with_places(1).range(1, 10);
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

/// Which change factor is equivalent to an increase of 10.3%?
/// Difficulty: 2
#[problem]
fn large_decimal_increase_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let increase_range = num_gen::decimal().with_places(1).range(10, 40);
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

/// Which change factor is equivalent to a decrease of 14.3%?
/// Difficulty: 2
#[problem]
fn large_decimal_decrease_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let decrease_range = num_gen::decimal().with_places(1).range(10, 40);
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
