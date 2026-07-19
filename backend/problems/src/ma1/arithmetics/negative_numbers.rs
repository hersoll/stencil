use anyhow::Result;
use macros::problem;
use math::num_gen;
use registry::get_problem_data;
use types::{format_strings::HasReplacements, lang::Language, problems::Problem};
use typst_writer::{
    drawing::NumberLine,
    formatting::{add_number, parentheses, subtract_number},
};

/// 5 - 9
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn subtract_larger(id: i32, lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(1, 5).and_random();
    let second = num_gen::integer().range(first + 1, first + 8).random();
    let number_line = NumberLine::from_ends(first, first - second)
        .with_arc(first, first - second, subtract_number(second))
        .build_string()?;

    let answer = first - second;
    let solution = registry::get_problem_data(id)?
        .get_solution(lang)
        .replace_placeholders(&[
            ("number_line", number_line),
            (
                "reverse",
                format!("${second} - {first} = {result}$", result = second - first),
            ),
            ("normal", format!("${first} - {second} = {answer}$")),
        ]);

    Ok(Problem {
        id,
        question: format!("${first} - {second}$"),
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// 5 - 78
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn subtract_larger_with_large_number(id: i32, lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(1, 7).and_random();

    // To bring the point home, avoid things like 73 - 7: not too nice! Better to have 73 - 2
    let first_digit = num_gen::integer().range(5, 19).random();
    let second_digit = num_gen::integer().range(first + 1, 9).random();
    let second = first_digit * 10 + second_digit;

    let answer = first - second;
    let reverse = second - first; //used as hint in solution
    let problem_data = registry::get_problem_data(id)?;
    let solution_str = problem_data.get_solution(lang);
    let solution = format!(
        "{solution_str} ${second} - {first} = {reverse} arrow.r.double {first} - {second} = {answer}$"
    );

    Ok(Problem {
        id,
        question: format!("${first} - {second}$"),
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// 15 - 78
/// Absolute difficulty: 1
/// Relative difficulty: 3
#[problem]
fn subtract_larger_with_large_numbers(id: i32, lang: Language) -> Result<Problem> {
    let first_digit = num_gen::integer().range(1, 5).random();
    let (second_digit, first_range) = num_gen::integer().range(1, 7).and_random();
    let first = first_digit * 10 + second_digit;

    // To bring the point home, avoid things like 73 - 7: not too nice! Better to have 73 - 2
    let first_digit_2 = num_gen::integer().range(6, 9).random();
    let second_digit_2 = num_gen::integer().range(second_digit + 1, 9).random();
    let second = first_digit_2 * 10 + second_digit_2;

    let answer = first - second;
    let reverse = second - first; //used as hint in solution
    let problem_data = registry::get_problem_data(id)?;
    let solution_str = problem_data.get_solution(lang);
    let solution = format!(
        "{solution_str} ${second} - {first} = {reverse} arrow.r.double {first} - {second} = {answer}$"
    );

    Ok(Problem {
        id,
        question: format!("${first} - {second}$"),
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// -4 + 2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn start_negative_addition_below_zero(id: i32, lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-9, -2).and_random();
    let second = num_gen::integer().range(1, -first - 1).random();
    let answer = first + second;

    let number_line = NumberLine::from_ends(-10, 0)
        .with_arc(first, answer, add_number(second))
        .build_string()?;
    let solution = registry::get_problem_data(id)?
        .get_solution(lang)
        .replace_placeholders(&[
            ("number_line", number_line),
            ("first", first.to_string()),
            ("second", second.to_string()),
            ("abs_first", first.abs().to_string()),
            ("answer", answer.to_string()),
        ]);
    Ok(Problem {
        id,
        question: format!("${first}+{second}$"),
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// -4 + 6
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn start_negative_addition_above_zero(id: i32, lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-9, -2).and_random();
    let second = num_gen::integer().range(-first + 1, 10).random();
    let answer = first + second;

    let number_line = NumberLine::from_ends(first, answer)
        .with_arc(first, answer, add_number(second))
        .build_string()?;
    let solution = registry::get_problem_data(id)?
        .get_solution(lang)
        .replace_placeholders(&[
            ("number_line", number_line),
            ("first", first.to_string()),
            ("second", second.to_string()),
            ("abs_first", first.abs().to_string()),
            ("answer", answer.to_string()),
        ]);
    Ok(Problem {
        id,
        question: format!("${first}+{second}$"),
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// -4 - 6
/// Absolute difficulty: 1
/// Relative difficulty: 5
#[problem]
fn start_negative_subtraction(id: i32, lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-5, -2).and_random();
    let second = num_gen::integer().range(1, 5).random();
    let answer = first - second;

    let number_line = NumberLine::from_ends(answer, 0)
        .with_arc(first, answer, subtract_number(second))
        .build_string()?;
    let problem_data = registry::get_problem_data(id)?;
    let solution_str = problem_data.get_solution(lang);
    let solution = format!("{solution_str} {number_line}");
    Ok(Problem {
        id,
        question: format!("${first}-{second}$"),
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![first],
        combinations: first_range.len(),
    })
}

/// 4 + (-2)
/// Absolute difficulty: 1
/// Relative difficulty: 6
#[problem]
fn add_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(1, 10).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first + second;
    Ok(Problem {
        id,
        question: format!("${first} + {second_p}$", second_p = parentheses(&second)),
        answer: format!("${ans}$"),
        solution: format!(
            "${first} + {second_p} = {first} - {second_a} = {ans}$",
            second_p = parentheses(&second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// 4 - (-2)
/// Absolute difficulty: 1
/// Relative difficulty: 7
#[problem]
fn subtract_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(1, 10).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first - second;
    Ok(Problem {
        id,
        question: format!("${first} - {second_p}$", second_p = parentheses(&second)),
        answer: format!("${ans}$"),
        solution: format!(
            "${first} - {second_p} = {first} + {second_a} = {ans}$",
            second_p = parentheses(&second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// 4 * (-2)
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn positive_times_negative(id: i32, lang: Language) -> Result<Problem> {
    let (first_val, first_range) = num_gen::integer().range(1, 10).and_random();
    let (second_val, second_range) = num_gen::integer().range(-10, -1).and_random();
    let question = format!("${first_val} dot ({second_val})$");
    let ans = first_val * second_val;
    let problem_data = registry::get_problem_data(id)?;
    let solution = problem_data.get_solution(lang).to_string();

    Ok(Problem {
        id,
        question,
        answer: format!("${ans}$"),
        solution,
        identifiers: vec![first_val, second_val],
        combinations: first_range.len() * second_range.len(),
    })
}

/// (-4) * 2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn negative_times_positive(id: i32, lang: Language) -> Result<Problem> {
    let (first_val, first_range) = num_gen::integer().range(-10, -1).and_random();
    let (second_val, second_range) = num_gen::integer().range(1, 10).and_random();
    let question = format!("$({first_val}) dot {second_val}$");
    let ans = first_val * second_val;
    let problem_data = registry::get_problem_data(id)?;
    let solution = problem_data.get_solution(lang).to_string();

    Ok(Problem {
        id,
        question,
        answer: format!("${ans}$"),
        solution,
        identifiers: vec![first_val, second_val],
        combinations: first_range.len() * second_range.len(),
    })
}

/// (-4) * (-2)
/// Absolute difficulty: 1
/// Relative difficulty: 5
#[problem]
fn negative_times_negative(id: i32, lang: Language) -> Result<Problem> {
    let (first_val, first_range) = num_gen::integer().range(-10, -1).and_random();
    let (second_val, second_range) = num_gen::integer().range(-10, -1).and_random();
    let question = format!("$({first_val}) dot ({second_val})$");
    let ans = first_val * second_val;
    let problem_data = registry::get_problem_data(id)?;
    let solution = problem_data.get_solution(lang).to_string();

    Ok(Problem {
        id,
        question,
        answer: format!("${ans}$"),
        solution,
        identifiers: vec![first_val, second_val],
        combinations: first_range.len() * second_range.len(),
    })
}

/// (-4) + (-2)
/// Absolute difficulty: 1
/// Relative difficulty: 8
#[problem]
fn negative_plus_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-10, -1).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first + second;
    Ok(Problem {
        id,
        question: format!(
            "${first_p} + {second_p}$",
            first_p = parentheses(&first),
            second_p = parentheses(&second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first_p} + {second_p} = {first} - {second_a} = {ans}$",
            first_p = parentheses(&first),
            second_p = parentheses(&second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// (-4) - (-2)
/// Absolute difficulty: 1
/// Relative difficulty: 9
#[problem]
fn negative_minus_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (first, first_range) = num_gen::integer().range(-10, -1).and_random();
    let (second, second_range) = num_gen::integer().range(-10, -1).and_random();
    let ans = first - second;
    Ok(Problem {
        id,
        question: format!(
            "${first_p} - {second_p}$",
            first_p = parentheses(&first),
            second_p = parentheses(&second)
        ),
        answer: format!("${ans}$"),
        solution: format!(
            "${first_p} - {second_p} = {first} + {second_a} = {ans}$",
            first_p = parentheses(&first),
            second_p = parentheses(&second),
            second_a = second.abs()
        ),
        identifiers: vec![first, second],
        combinations: first_range.len() * second_range.len(),
    })
}

/// (-3)^2
/// Absolute difficulty: 2
/// Relative difficulty: 8
#[problem]
fn negative_number_squared(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(-10, -1).and_random();
    let question = format!("$({base})^2$");
    let answer = base.pow(2);
    let solution = format!("$({base})^2 = ({base}) dot ({base}) = {answer}$");

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}

/// (-2)^3
/// Absolute difficulty: 2
/// Relative difficulty: 9
#[problem]
fn negative_number_cubed(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(-3, -1).and_random();
    let question = format!("$({base})^3$");
    let square = base.pow(2);
    let answer = base.pow(3);
    let solution = format!(
        "$({base})^3 &= ({base}) dot ({base}) dot ({base}) = \\
        &= colored({square}) dot ({base}) = {answer}$"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}

/// -2^2
/// Absolute difficulty: 2
/// Relative difficulty: 10
#[problem]
fn number_squared_then_negative(id: i32, lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(1, 10).and_random();
    let question = format!("$-{base}^2$");
    let square = base.pow(2);
    let answer = -square;
    let solution_text = get_problem_data(id)?.get_solution(lang).to_string();
    let solution = format!(
        "{solution_text} \\
        \\
        $-{base}^2 = -({base}^2) = -({square}) = {answer}$"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}
