use anyhow::Result;
use macros::problem;
use math::{
    formatting::divide_number,
    num_gen::{self, NumberGenerator},
    symbols::X,
    utils::{change_factor_to_percentage, to_change_factor},
};
use registry::{get_answer, get_question, get_solution};
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// 40 is increased by 12%
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn simple_increase(id: i32, lang: Language) -> Result<Problem> {
    let value = num_gen::integer()
        .range(11, 49)
        .exclude_multiple(&[20, 30, 40])
        .random();
    let (percent, perc_range) = num_gen::integer()
        .range(7, 35)
        .exclude_multiple(&[10, 20, 30])
        .and_random();
    let change = to_change_factor(percent);
    let answer = value * change;
    let rounded = answer.round(1);

    let question =
        get_question(id, lang)?.replace_multiple(&[("value", value), ("percent", percent)]);

    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::inline();
    solution
        .write(equation_text)
        .equals(format!("{value} dot {change}"))
        .linebreak_equality()
        .align(answer);
    if answer != rounded {
        solution.write(format!("approx {rounded}"));
    }

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
    let value = num_gen::integer()
        .range(11, 49)
        .exclude_multiple(&[20, 30, 40])
        .random();
    let (percent, perc_range) = num_gen::integer()
        .range(7, 35)
        .exclude_multiple(&[10, 20, 30])
        .and_random();
    let change = to_change_factor(-percent);
    let answer = value * change;
    let rounded = answer.round(1);

    let question =
        get_question(id, lang)?.replace_multiple(&[("value", value), ("percent", percent)]);

    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::inline();
    solution
        .write(equation_text)
        .equals(format!("{value} dot {change}"))
        .linebreak_equality()
        .align(answer);
    if answer != rounded {
        solution.write(format!("approx {rounded}"));
    }

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: rounded,
        solution,
        identifiers: percent,
        combinations: perc_range,
    }))
}

/// Text version of simple_increase
/// Absolute difficulty: 3
/// Relative difficulty: 4
#[problem]
fn text_simple_increase(id: i32, lang: Language) -> Result<Problem> {
    let mut problem = simple_increase(id, lang)?;
    problem.answer = get_answer(id, lang)?
        .replace_one("answer", problem.answer)
        .into();
    Ok(problem)
}

/// Text version of simple_decrease
/// Absolute difficulty: 3
/// Relative difficulty: 4
#[problem]
fn text_simple_decrease(id: i32, lang: Language) -> Result<Problem> {
    let value = num_gen::integer()
        .numbers(&[149, 199, 249, 299, 349, 399, 449, 499])
        .random();
    let (percent, perc_range) = num_gen::integer()
        .range(16, 34)
        .exclude_multiple(&[20, 25, 30])
        .and_random();
    let change = to_change_factor(-percent);
    let answer = value * change;
    let rounded = answer.round(0);

    let question =
        get_question(id, lang)?.replace_multiple(&[("value", value), ("percent", percent)]);

    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::inline();
    solution
        .write(equation_text)
        .equals(format!("{value} dot {change}"))
        .linebreak_equality()
        .align(answer);
    if answer != rounded {
        solution.write(format!("approx {rounded}"));
    }

    let answer = get_answer(id, lang)?.replace_one("answer", rounded);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: percent,
        combinations: perc_range,
    }))
}

/// Increase from 34 to 56, how many percent?
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn calculate_small_increase(id: i32, lang: Language) -> Result<Problem> {
    let old_range = num_gen::integer().range(23, 44).exclude_multiple(&[30, 40]);
    let old = old_range.random();
    let increase = num_gen::decimals(2).range(1.11, 1.89).random();
    let new = (old * increase).round(0).as_integer();
    let division = new / old;
    let change_factor = division.to_decimal().round(2);

    let question = get_question(id, lang)?.replace_multiple(&[("old", old), ("new", new)]);

    let answer = change_factor_to_percentage(change_factor);

    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::with_steps();
    solution
        .line(equation_text)
        .aligned(new, format!("{old} dot {X}"))
        .step(divide_number(old))
        .aligned(
            X,
            format!("{new} / {old} approx {change_factor} = +{answer}%"),
        );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${answer}%$"),
        solution,
        identifiers: old,
        combinations: old_range,
    }))
}

/// Increase from 34 to 82, how many percent?
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn calculate_large_increase(id: i32, lang: Language) -> Result<Problem> {
    let old_range = num_gen::integer().range(23, 44).exclude_multiple(&[30, 40]);
    let old = old_range.random();
    let increase = num_gen::decimals(2).range(2.11, 2.99).random();
    let new = (old * increase).round(0).as_integer();
    let division = new / old;
    let change_factor = division.to_decimal().round(2);

    let question = get_question(id, lang)?.replace_multiple(&[("old", old), ("new", new)]);

    let answer = change_factor_to_percentage(change_factor);
    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::with_steps();
    solution
        .line(equation_text)
        .aligned(new, format!("{old} dot {X}"))
        .step(divide_number(old))
        .aligned(
            X,
            format!("{new} / {old} approx {change_factor} = +{answer}%"),
        );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${answer}%$"),
        solution,
        identifiers: old,
        combinations: old_range,
    }))
}

/// Decrease from 56 to 34, how many percent?
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn calculate_decrease(id: i32, lang: Language) -> Result<Problem> {
    let old_range = num_gen::integer()
        .range(23, 84)
        .exclude_multiple(&[30, 40, 50, 60, 70, 80]);
    let old = old_range.random();
    let decrease_factor = num_gen::decimals(2).range(0.61, 0.89).random();
    let new = (old * decrease_factor).round(0).as_integer();
    let division = new / old;
    let change_factor = division.to_decimal().round(2);

    let question = get_question(id, lang)?.replace_multiple(&[("old", old), ("new", new)]);
    let answer = change_factor_to_percentage(change_factor);
    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::with_steps();
    solution
        .line(equation_text)
        .aligned(new, format!("{old} dot {X}"))
        .step(divide_number(old))
        .aligned(
            X,
            format!("{new} / {old} approx {change_factor} = {answer}%"),
        );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${}%$", answer.abs()),
        solution,
        identifiers: old,
        combinations: old_range,
    }))
}

/// Text version of calculate_small_increase
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn text_calculate_increase(id: i32, lang: Language) -> Result<Problem> {
    calculate_small_increase(id, lang)
}

/// Text version of calculate_decrease
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn text_calculate_decrease(id: i32, lang: Language) -> Result<Problem> {
    let old_range = num_gen::integer().range_step(199, 499, 20);
    let old = old_range.random();
    let new = num_gen::integer().range_step(149, old - 40, 10).random();
    let division = new / old;
    let change_factor = division.to_decimal().round(2);

    let question = get_question(id, lang)?.replace_multiple(&[("old", old), ("new", new)]);
    let answer = change_factor_to_percentage(change_factor);
    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::with_steps();
    solution
        .line(equation_text)
        .aligned(new, format!("{old} dot {X}"))
        .step(divide_number(old))
        .aligned(
            X,
            format!("{new} / {old} approx {change_factor} = {answer}%"),
        );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${}%$", answer.abs()),
        solution,
        identifiers: old,
        combinations: old_range,
    }))
}

/// A number has increased by 23% and is now 56. What was the original number?
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn find_old_number_increase(id: i32, lang: Language) -> Result<Problem> {
    let (new, new_range) = num_gen::integer().range(101, 299).exclude(200).and_random();
    let increase = num_gen::integer()
        .range(11, 89)
        .exclude_multiple(&[20, 30, 40, 50, 60, 70, 80])
        .random();
    let change_factor = to_change_factor(increase);
    let old = (new / change_factor).to_decimal().round(0).as_integer();

    let question =
        get_question(id, lang)?.replace_multiple(&[("new", new), ("increase", increase)]);

    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::with_steps();
    solution
        .line(equation_text)
        .aligned(new, format!("{X} dot {change_factor}"))
        .step(divide_number(change_factor))
        .aligned(X, format!("{new} / {change_factor} approx {old}"));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: old,
        solution,
        identifiers: new,
        combinations: new_range,
    }))
}

/// A number has decreased by 23% and is now 56. What was the original number?
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn find_old_number_decrease(id: i32, lang: Language) -> Result<Problem> {
    let (new, new_range) = num_gen::integer().range(101, 299).exclude(200).and_random();
    let decrease = num_gen::integer()
        .range(11, 89)
        .exclude_multiple(&[20, 30, 40, 50, 60, 70, 80])
        .random();
    let change_factor = to_change_factor(-decrease);
    let old = (new / change_factor).to_decimal().round(0).as_integer();

    let question =
        get_question(id, lang)?.replace_multiple(&[("new", new), ("decrease", decrease)]);

    let equation_text = get_solution(id, lang)?;
    let mut solution = Solution::with_steps();
    solution
        .line(equation_text)
        .aligned(new, format!("{X} dot {change_factor}"))
        .step(divide_number(change_factor))
        .aligned(X, format!("{new} / {change_factor} approx {old}"));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: old,
        solution,
        identifiers: new,
        combinations: new_range,
    }))
}

/// Text version of find_old_number_increase
/// Absolute difficulty: 5
/// Relative difficulty: 8
#[problem]
fn text_find_old_increase(id: i32, lang: Language) -> Result<Problem> {
    find_old_number_increase(id, lang)
}

/// Text version of find_old_number_decrease
/// Absolute difficulty: 5
/// Relative difficulty: 8
#[problem]
fn text_find_old_decrease(id: i32, lang: Language) -> Result<Problem> {
    find_old_number_decrease(id, lang)
}
