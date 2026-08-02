use anyhow::Result;
use macros::problem;
use math::num_gen::{self, NumberGenerator};
use types::{format_strings::HasReplacements, lang::Language, problems::Problem};

/// Which change factor is equivalent to an increase of 10%?
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn integer_increase_to_factor(id: i32, lang: Language) -> Result<Problem> {
    let increase_range = num_gen::integer().range(2, 99);
    let increase = increase_range.random();
    let change_factor = math::utils::to_change_factor(increase);

    let question = registry::get_question(id, lang)?.replace_placeholders(&[("percent", increase)]);

    let solution = format!(
        "$100% + {increase}% = {total}% = {change_factor}$",
        total = 100 + increase
    );
    Ok(Problem {
        id,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![increase],
        combinations: increase_range.len(),
    })
}

/// Which change factor is equivalent to a decrease of 10%?
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn integer_decrease_to_factor(id: i32, lang: Language) -> Result<Problem> {
    let decrease_range = num_gen::integer().range(2, 29).exclude(10).exclude(20);
    let decrease = decrease_range.random();
    let change_factor = math::utils::to_change_factor(-decrease);

    let question = registry::get_question(id, lang)?.replace_placeholders(&[("percent", decrease)]);

    let solution = format!(
        "$100% - {decrease}% = {total}% = {change_factor}$",
        total = 100 - decrease
    );
    Ok(Problem {
        id,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![decrease],
        combinations: decrease_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 1.23
/// Absolute difficulty: 2
/// Relative difficulty: 2
#[problem]
fn factor_to_increase_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_decimals(2).range(1.04, 1.25);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let answer = registry::get_answer(id, lang)?.replace_placeholders(&[("percent", increase)]);
    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        id,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 0.87
/// Absolute difficulty: 2
/// Relative difficulty: 2
#[problem]
fn factor_to_decrease_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_decimals(2).range(0.73, 0.98); // yes, very arbitrary :)
    let change_factor = factor_range.random();
    let decrease = math::utils::change_factor_to_percentage(change_factor).abs();

    let answer = registry::get_answer(id, lang)?.replace_placeholders(&[("percent", decrease)]);

    let solution = format!(
        "${change_factor} = {total}% = 100% - {decrease}%$",
        total = 100 - decrease
    );
    Ok(Problem {
        id,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 0.23
/// Absolute difficulty: 3
/// Relative difficulty: 3
#[problem]
fn factor_to_large_decrease_two_decimals(id: i32, lang: Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_decimals(2).range(0.11, 0.39);
    let change_factor = factor_range.random();
    let decrease = math::utils::change_factor_to_percentage(change_factor).abs();

    let answer = registry::get_answer(id, lang)?.replace_placeholders(&[("percent", decrease)]);

    let solution = format!(
        "${change_factor} = {total}% = 100% - {decrease}%$",
        total = 100 - decrease
    );
    Ok(Problem {
        id,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 1.2
/// Absolute difficulty: 2
/// Relative difficulty: 2
#[problem]
fn factor_to_increase_one_decimal(id: i32, lang: Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_decimals(1).range(1.1, 1.9);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let answer = registry::get_answer(id, lang)?.replace_placeholders(&[("percent", increase)]);

    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        id,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 0.8
/// Absolute difficulty: 2
/// Relative difficulty: 2
#[problem]
fn factor_to_decrease_one_decimal(id: i32, lang: Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_decimals(1).range(0.1, 0.9);
    let change_factor = factor_range.random();
    let decrease = math::utils::change_factor_to_percentage(change_factor).abs();

    let answer = registry::get_answer(id, lang)?.replace_placeholders(&[("percent", decrease)]);

    let solution = format!(
        "${change_factor} = {total}% = 100% - {decrease}%$",
        total = 100 - decrease
    );
    Ok(Problem {
        id,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 2.12
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn factor_to_increase_above_2(id: i32, lang: Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_decimals(2).range(2.01, 2.99);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let answer = registry::get_answer(id, lang)?.replace_placeholders(&[("percent", increase)]);

    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        id,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Decide if there is an increase or a decrease, and by how many percentages, when the change factor is 8.12
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn factor_to_increase_large_number(id: i32, lang: Language) -> Result<Problem> {
    let factor_range = num_gen::decimal().with_decimals(2).range(5.0, 9.99);
    let change_factor = factor_range.random();
    let increase = math::utils::change_factor_to_percentage(change_factor);

    let answer = registry::get_answer(id, lang)?.replace_placeholders(&[("percent", increase)]);

    let solution = format!(
        "${change_factor} = {total}% = 100% + {increase}%$",
        total = 100 + increase
    );
    Ok(Problem {
        id,
        question: format!("${change_factor}$"),
        answer,
        solution,
        identifiers: vec![change_factor],
        combinations: factor_range.len(),
    })
}

/// Which change factor is equivalent to an increase of 4.3%?
/// Absolute difficulty: 3
/// Relative difficulty: 5
#[problem]
fn decimal_increase_to_factor(id: i32, lang: Language) -> Result<Problem> {
    let increase_range = num_gen::decimal().with_decimals(1).range(2, 10);
    let increase = increase_range.random();
    let change_factor = math::utils::to_change_factor(increase);

    let question = registry::get_question(id, lang)?.replace_placeholders(&[("percent", increase)]);

    let solution = format!(
        "$100% + {increase}% = {total}% = {change_factor}$",
        total = 100 + increase
    );
    Ok(Problem {
        id,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![increase],
        combinations: increase_range.len(),
    })
}

/// Which change factor is equivalent to a decrease of 4.3%?
/// Absolute difficulty: 3
/// Relative difficulty: 5
#[problem]
fn decimal_decrease_to_factor(id: i32, lang: Language) -> Result<Problem> {
    let decrease_range = num_gen::decimal().with_decimals(1).range(1, 10);
    let decrease = decrease_range.random();
    let change_factor = math::utils::to_change_factor(-decrease);

    let question = registry::get_question(id, lang)?.replace_placeholders(&[("percent", decrease)]);

    let solution = format!(
        "$100% - {decrease}% = {total}% = {change_factor}$",
        total = 100 - decrease
    );
    Ok(Problem {
        id,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![decrease],
        combinations: decrease_range.len(),
    })
}

/// Which change factor is equivalent to an increase of 10.3%?
/// Absolute difficulty: 3
/// Relative difficulty: 4
#[problem]
fn large_decimal_increase_to_factor(id: i32, lang: Language) -> Result<Problem> {
    let increase_range = num_gen::decimal().with_decimals(1).range(10, 40);
    let increase = increase_range.random();
    let change_factor = math::utils::to_change_factor(increase);

    let question = registry::get_question(id, lang)?.replace_placeholders(&[("percent", increase)]);

    let solution = format!(
        "$100% + {increase}% = {total}% = {change_factor}$",
        total = 100 + increase
    );
    Ok(Problem {
        id,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![increase],
        combinations: increase_range.len(),
    })
}

/// Which change factor is equivalent to a decrease of 14.3%?
/// Absolute difficulty: 3
/// Relative difficulty: 4
#[problem]
fn large_decimal_decrease_to_factor(id: i32, lang: Language) -> Result<Problem> {
    let decrease_range = num_gen::decimal().with_decimals(1).range(10, 40);
    let decrease = decrease_range.random();
    let change_factor = math::utils::to_change_factor(-decrease);

    let question = registry::get_question(id, lang)?.replace_placeholders(&[("percent", decrease)]);

    let solution = format!(
        "$100% - {decrease}% = {total}% = {change_factor}$",
        total = 100 - decrease
    );
    Ok(Problem {
        id,
        question,
        answer: format!("${change_factor}$"),
        solution,
        identifiers: vec![decrease],
        combinations: decrease_range.len(),
    })
}
