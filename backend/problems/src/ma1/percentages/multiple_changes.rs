use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay,
    num_gen::{self, NumberGenerator},
    symbols::X,
    utils::{change_factor_to_percentage, to_change_factor},
};
use registry::{get_answer, get_question};
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

use crate::ma1::percentages::labels;

/// Value after +10% +20%
/// Absolute difficulty: 3
/// Relative difficulty: 1
#[problem]
fn value_after_two_increases(id: i32, lang: Language) -> Result<Problem> {
    let starting_value = num_gen::integer()
        .range(21, 59)
        .exclude_multiple(&[30, 40, 50])
        .random();
    let (first_increase, inc_range) = num_gen::integer()
        .range(6, 29)
        .exclude_multiple(&[10, 20])
        .and_random();
    let second_increase = inc_range.clone().exclude(first_increase).random();

    let first_factor = to_change_factor(first_increase);
    let second_factor = to_change_factor(second_increase);
    let total_factor = first_factor * second_factor;
    let new_value = starting_value * total_factor;
    let answer = new_value.round(0);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("value", starting_value),
        ("first", first_increase),
        ("second", second_increase),
    ]);

    let new_label = super::new_label(lang);
    let old_label = super::old_label(lang);
    let ff_label = super::ff_label(lang);
    let mut solution = Solution::with_steps();
    solution
        .aligned(new_label, format!("{old_label} dot {ff_label}"))
        .aligned(
            X,
            format!("{starting_value} dot {first_factor} dot {second_factor}"),
        )
        .aligned(X, format!("{new_value} approx {answer}"));

    let answer = get_answer(id, lang)?.replace_one("answer", answer.as_math());

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_increase, second_increase],
        combinations: inc_range.len() * (inc_range.len() - 1),
    }))
}

/// Value after -13% -17%
/// Absolute difficulty: 4
/// Relative difficulty: 1
#[problem]
fn value_after_two_decreases(id: i32, lang: Language) -> Result<Problem> {
    let starting_value = num_gen::integer()
        .range(21, 59)
        .exclude_multiple(&[30, 40, 50])
        .random();
    let (first_decrease, dec_range) = num_gen::integer()
        .range(6, 29)
        .exclude_multiple(&[10, 20])
        .and_random();
    let second_decrease = dec_range.clone().exclude(first_decrease).random();

    let first_factor = to_change_factor(-first_decrease);
    let second_factor = to_change_factor(-second_decrease);
    let total_factor = first_factor * second_factor;
    let new_value = starting_value * total_factor;
    let answer = new_value.round(0);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("value", starting_value),
        ("first", first_decrease),
        ("second", second_decrease),
    ]);

    let new_label = super::new_label(lang);
    let old_label = super::old_label(lang);
    let ff_label = super::ff_label(lang);
    let mut solution = Solution::with_steps();
    solution
        .aligned(new_label, format!("{old_label} dot {ff_label}"))
        .aligned(
            X,
            format!("{starting_value} dot {first_factor} dot {second_factor}"),
        )
        .aligned(X, format!("{new_value} approx {answer}"));

    let answer = get_answer(id, lang)?.replace_one("answer", answer.as_math());

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_decrease, second_decrease],
        combinations: dec_range.len() * (dec_range.len() - 1),
    }))
}

/// Total increase after +10% +20%
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn two_simple_increases(id: i32, lang: Language) -> Result<Problem> {
    let (first_increase, inc_range) = num_gen::integer().range_step(10, 40, 10).and_random();
    let second_increase = inc_range.clone().exclude(first_increase).random();

    let first_factor = to_change_factor(first_increase);
    let second_factor = to_change_factor(second_increase);
    let total_factor = first_factor * second_factor;
    let total_change = change_factor_to_percentage(total_factor);

    let question = get_question(id, lang)?
        .replace_multiple(&[("first", first_increase), ("second", second_increase)]);

    let mut solution = Solution::inline();
    solution.write(format!(
        "{first_factor} dot {second_factor} = {total_factor} = +{total_change}%"
    ));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${total_change}%$"),
        solution,
        identifiers: vec![first_increase, second_increase],
        combinations: inc_range.len() * (inc_range.len() - 1),
    }))
}

/// Total decrease after -10% -20%
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn two_simple_decreases(id: i32, lang: Language) -> Result<Problem> {
    let (first_decrease, dec_range) = num_gen::integer().range_step(10, 40, 10).and_random();
    let second_decrease = dec_range.clone().exclude(first_decrease).random();

    let first_factor = to_change_factor(-first_decrease);
    let second_factor = to_change_factor(-second_decrease);
    let total_factor = first_factor * second_factor;
    let total_change = change_factor_to_percentage(total_factor);

    let question = get_question(id, lang)?
        .replace_multiple(&[("first", first_decrease), ("second", second_decrease)]);

    let mut solution = Solution::inline();
    solution.write(format!(
        "{first_factor} dot {second_factor} = {total_factor} = {total_change}%"
    ));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${}%$", total_change.abs()),
        solution,
        identifiers: vec![first_decrease, second_decrease],
        combinations: dec_range.len() * (dec_range.len() - 1),
    }))
}

/// Total increase after +8% +23%
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn two_increases(id: i32, lang: Language) -> Result<Problem> {
    let (first_increase, inc_range) = num_gen::integer()
        .range(6, 29)
        .exclude_multiple(&[10, 20])
        .and_random();
    let second_increase = inc_range.clone().exclude(first_increase).random();

    let first_factor = to_change_factor(first_increase);
    let second_factor = to_change_factor(second_increase);
    let total_factor = first_factor * second_factor;
    let rounded_factor = total_factor.round(2);
    let total_change = change_factor_to_percentage(rounded_factor);

    let question = get_question(id, lang)?
        .replace_multiple(&[("first", first_increase), ("second", second_increase)]);

    let mut solution = Solution::inline();
    solution.write(format!(
        "{first_factor} dot {second_factor} = {total_factor}"
    ));
    if rounded_factor != total_factor {
        solution.approx(rounded_factor);
    }
    solution.equals(format!("+{total_change}%"));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${total_change}%$"),
        solution,
        identifiers: vec![first_increase, second_increase],
        combinations: inc_range.len() * (inc_range.len() - 1),
    }))
}

/// Total decrease after -8% -23%
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn two_decreases(id: i32, lang: Language) -> Result<Problem> {
    let (first_decrease, dec_range) = num_gen::integer()
        .range(6, 29)
        .exclude_multiple(&[10, 20])
        .and_random();
    let second_decrease = dec_range.clone().exclude(first_decrease).random();

    let first_factor = to_change_factor(-first_decrease);
    let second_factor = to_change_factor(-second_decrease);
    let total_factor = first_factor * second_factor;
    let rounded_factor = total_factor.round(2);
    let total_change = change_factor_to_percentage(rounded_factor);

    let question = get_question(id, lang)?
        .replace_multiple(&[("first", first_decrease), ("second", second_decrease)]);

    let mut solution = Solution::inline();
    solution.write(format!(
        "{first_factor} dot {second_factor} = {total_factor}"
    ));
    if rounded_factor != total_factor {
        solution.approx(rounded_factor);
    }
    solution.equals(format!("{total_change}%"));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${}%$", total_change.abs()),
        solution,
        identifiers: vec![first_decrease, second_decrease],
        combinations: dec_range.len() * (dec_range.len() - 1),
    }))
}

/// Total change after +8% -23%
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn one_of_each_change(id: i32, lang: Language) -> Result<Problem> {
    let (increase, inc_range) = num_gen::integer()
        .range(6, 29)
        .exclude_multiple(&[10, 20])
        .and_random();
    let decrease = inc_range.random();

    let first_factor = to_change_factor(increase);
    let second_factor = to_change_factor(-decrease);
    let total_factor = first_factor * second_factor;
    let rounded_factor = total_factor.round(3);
    let total_change = change_factor_to_percentage(rounded_factor);

    let question =
        get_question(id, lang)?.replace_multiple(&[("first", increase), ("second", decrease)]);

    let mut solution = Solution::inline();
    solution.write(format!(
        "{first_factor} dot {second_factor} = {total_factor}"
    ));
    if rounded_factor != total_factor {
        solution.approx(rounded_factor);
    }
    solution.equals(format!("{total_change:+}%"));

    use Language::*;
    let answer = match (lang, total_change > 1) {
        (Sv, true) => format!("Ökning med ${total_change}%$"),
        (Sv, false) => format!("Minskning med ${}%$", total_change.abs()),
        (En, true) => format!("Increase by ${total_change}%$"),
        (En, false) => format!("Decrease by ${}%$", total_change.abs()),
    };

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![increase, decrease],
        combinations: inc_range.len() * (inc_range.len() - 1),
    }))
}

/// New value after increase by 6% 4 times
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn repeated_increases(id: i32, lang: Language) -> Result<Problem> {
    let (increase, inc_range) = num_gen::integer().range(2, 9).and_random();
    let duration = num_gen::integer().range(4, 9).random();
    let old_value = num_gen::integer().range(79, 149).exclude(100).random();
    let factor = to_change_factor(increase);
    let new_value = (old_value * factor.pow(duration)).round(1);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("old", old_value),
        ("increase", increase),
        ("duration", duration),
    ]);

    let answer = get_answer(id, lang)?.replace_one("answer", new_value.as_math());

    let mut solution = Solution::with_steps();
    let (new, old, ff, time) = labels(lang);
    solution
        .aligned(new, format!("{old} dot {ff}^{time}"))
        .aligned(
            X,
            format!("{old_value} dot {factor}^{duration} approx {new_value}"),
        );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: increase,
        combinations: inc_range,
    }))
}

/// New value after decrease by 6% 4 times
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn repeated_decreases(id: i32, lang: Language) -> Result<Problem> {
    let (decrease, dec_range) = num_gen::integer().range(2, 9).and_random();
    let duration = num_gen::integer().range(4, 9).random();
    let old_value = num_gen::integer().range(201, 299).exclude(250).random();
    let factor = to_change_factor(-decrease);
    let new_value = (old_value * factor.pow(duration)).round(1);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("old", old_value),
        ("increase", decrease),
        ("duration", duration),
    ]);

    let answer = get_answer(id, lang)?.replace_one("answer", new_value.as_math());

    let mut solution = Solution::with_steps();
    let (new, old, ff, time) = labels(lang);
    solution
        .aligned(new, format!("{old} dot {ff}^{time}"))
        .aligned(
            X,
            format!("{old_value} dot {factor}^{duration} approx {new_value}"),
        );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: decrease,
        combinations: dec_range,
    }))
}
