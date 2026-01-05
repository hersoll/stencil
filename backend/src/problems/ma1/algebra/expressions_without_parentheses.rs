use crate::Language;
use crate::math::IntRange;
use crate::math::symbols;
use crate::math::{Polynomial, Term, Variables};
use crate::problems::Problem;
use crate::registry::{get_problem_data, replace_placeholders};
use anyhow::Result;
use macros::problem;
use rand::seq::IndexedRandom;

/// 3x + 4 + 2x + 1
/// Difficulty: 0
#[problem]
fn one_variable_and_constants_no_negatives(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (first_coef, first_coef_range) = IntRange::without_zero(1, 6)?.and_random();
    let second_coef = IntRange::without_zero(-(first_coef - 1), 6)?.random();
    let (first_const, first_const_range) = IntRange::without_zero(1, 6)?.and_random();
    let second_const = IntRange::without_zero(-(first_const - 1), 6)?.random();
    let first_term: Term = (first_coef, unknown).into();
    let second_term: Term = (second_coef, unknown).into();
    let first_const_term: Term = first_const.into();
    let second_const_term: Term = second_const.into();
    let original_expression: Polynomial = vec![
        &first_term,
        &first_const_term,
        &second_term,
        &second_const_term,
    ]
    .into();
    let simplified_expression = original_expression.simplify();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$&{original_expression} = \\
        = &colored({first_term}{second_term:+}) {first_const_term:+}{second_const_term:+} = \\
        = &{simplified_expression}$",
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_const],
        combinations: first_coef_range.len() * first_const_range.len(),
    })
}

/// 3x - 5 - 5x + 2
/// Difficulty: 1
#[problem]
fn one_variable_and_constants(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (first_coef, first_coef_range) = IntRange::without_zero(-6, 6)?.and_random();
    let second_coef = IntRange::without_zero(-6, 6)?.random();
    let (first_const, first_const_range) = IntRange::without_zero(6, 6)?.and_random();
    let second_const = IntRange::without_zero(-6, 6)?.random();
    let first_term: Term = (first_coef, unknown).into();
    let second_term: Term = (second_coef, unknown).into();
    let first_const_term: Term = first_const.into();
    let second_const_term: Term = second_const.into();
    let original_expression: Polynomial = vec![
        &first_term,
        &first_const_term,
        &second_term,
        &second_const_term,
    ]
    .into();
    let simplified_expression = original_expression.simplify();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$&{original_expression} = \\
        = &colored({first_term}{second_term:+}) {first_const_term:+}{second_const_term:+} = \\
        = &{simplified_expression}$",
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_const],
        combinations: first_coef_range.len() * first_const_range.len(),
    })
}

/// 2x - 3y + 3x - 8y + 1
/// Difficulty: 2
#[problem]
fn two_variables_and_constants(name: String, _lang: &Language) -> Result<Problem> {
    let (first_unknown, second_unknown) = symbols::get_two_unknowns()?;
    let (first_coef_a, first_coef_a_range) = IntRange::with_zero(-9, 9)?.and_random();
    let first_coef_b = IntRange::with_zero(-9, 9)?.random();
    let (second_coef_a, second_coef_a_range) = IntRange::with_zero(-9, 9)?.and_random();
    let second_coef_b = IntRange::with_zero(-9, 9)?.random();
    let first_const = IntRange::with_zero(-9, 9)?.random();
    let second_const = IntRange::with_zero(-9, 9)?.random();

    let mut first_term_a: Term = (first_coef_a, first_unknown).into();
    let mut first_term_b: Term = (first_coef_b, first_unknown).into();
    let second_term_a: Term = (second_coef_a, second_unknown).into();
    let second_term_b: Term = (second_coef_b, second_unknown).into();
    let mut first_const_term: Term = first_const.into();
    let mut second_const_term: Term = second_const.into();

    let original_expression: Polynomial = Polynomial::random_order(vec![
        &first_term_a,
        &first_term_b,
        &second_term_a,
        &second_term_b,
        &first_const_term,
        &second_const_term,
    ]);
    let simplified_expression = original_expression.simplify();
    first_term_a.colored = true;
    first_term_b.colored = true;
    first_const_term.colored = true;
    second_const_term.colored = true;
    let first_terms: Polynomial = vec![&first_term_a, &first_term_b].into();
    let second_terms: Polynomial = vec![&second_term_a, &second_term_b].into();
    let const_terms: Polynomial = vec![&first_const_term, &second_const_term].into();
    let sorted_expression: Polynomial =
        first_terms.sorted() + second_terms.sorted() + const_terms.sorted();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$&{original_expression} = \\
        = &{sorted_expression} = \\
        = &{simplified_expression}$",
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![first_coef_a, second_coef_a],
        combinations: first_coef_a_range.len() * second_coef_a_range.len(),
    })
}

/// Evaluate 3x - 1 when x = -3
/// Difficulty: 1
#[problem]
fn evaluate_simple(name: String, lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (coef, coef_range) = IntRange::without_ones_and_zero(-9, 9)?.and_random();
    let constant = IntRange::without_zero(-9, 9)?.random();
    let (value, value_range) = IntRange::without_zero(-5, 5)?.and_random();

    let first_term: Term = (coef, unknown).into();
    let const_term: Term = constant.into();

    let expression: Polynomial = vec![&first_term, &const_term].into();
    let map = vec![
        ("expression", expression.to_string()),
        ("unknown", unknown.to_string()),
        ("value", value.to_string()),
    ];
    let problem_data = get_problem_data(&name)?;
    let question = replace_placeholders(problem_data.get_question(lang), &map);
    let replacements = vec![(unknown, value)];
    let answer = expression.evaluate(&replacements);

    let solution = format!(
        "$&{expression} = \\
        = &{} = \\
        = &{}{constant:+} = {answer}$",
        expression.show_replacements(&replacements),
        coef * value,
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef, value],
        combinations: coef_range.len() * value_range.len(),
    })
}

/// Evaluate 3x - 2y + 1 when x = -3 and y = 2
/// Difficulty: 2
#[problem]
fn evaluate_intermediate(name: String, lang: &Language) -> Result<Problem> {
    let (first_unknown, second_unknown) = symbols::get_two_unknowns()?;
    let (coef, coef_range) = IntRange::without_ones_and_zero(-9, 9)?.and_random();
    let (coef_2, coef_2_range) = IntRange::without_ones_and_zero(-9, 9)?.and_random();
    let constant = IntRange::without_zero(-9, 9)?.random();
    let value_x = IntRange::without_zero(-5, 5)?.random();
    let value_y = IntRange::without_zero(-5, -1)?.random();

    let first_term: Term = (coef, first_unknown).into();
    let second_term: Term = (coef_2, second_unknown).into();
    let const_term: Term = constant.into();

    let expression: Polynomial = vec![&first_term, &second_term, &const_term].into();
    let problem_data = get_problem_data(&name)?;
    let question = replace_placeholders(
        problem_data.get_question(lang),
        &[
            ("expression", expression.to_string()),
            ("unknown_a", first_unknown.to_string()),
            ("unknown_b", second_unknown.to_string()),
            ("value_x", value_x.to_string()),
            ("value_y", value_y.to_string()),
        ],
    );
    let replacements = vec![(first_unknown, value_x), (second_unknown, value_y)];
    let answer = expression.evaluate(&replacements);

    let solution = format!(
        "$&{expression} = \\
        = &{} = \\
        = &{}{:+}{constant:+} = {answer}$",
        expression.show_replacements(&replacements),
        coef * value_x,
        coef_2 * value_y,
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef, coef_2],
        combinations: coef_range.len() * coef_2_range.len(),
    })
}
/// x^2 + 2x + 3x^2 - 4x
/// Difficulty: 3
#[problem]
fn one_variable_different_exponents(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (first_coef, first_coef_range) = IntRange::with_zero(-9, 9)?.and_random();
    let second_coef = IntRange::with_zero(-9, 9)?.random();
    let third_coef = IntRange::with_zero(-9, 9)?.random();
    let fourth_coef = IntRange::with_zero(-9, 9)?.random();
    let (first_exp, first_exp_range) = IntRange::with_zero(1, 2)?.and_random();
    let second_exp = IntRange::with_zero(1, 2)?.random();
    let third_exp = 2;
    let fourth_exp = 1;

    let first_term: Term = (first_coef, (unknown, first_exp)).into();
    let second_term: Term = (second_coef, (unknown, second_exp)).into();
    let third_term: Term = (third_coef, (unknown, third_exp)).into();
    let fourth_term: Term = (fourth_coef, (unknown, fourth_exp)).into();

    let original_expression: Polynomial =
        Polynomial::random_order(vec![&first_term, &second_term, &third_term, &fourth_term]);
    let sorted_expression = original_expression.sorted();
    let simplified_expression = original_expression.simplify();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$&{original_expression} = \\
        = &{sorted_expression} = \\
        = &{simplified_expression}$",
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_exp],
        combinations: first_coef_range.len() * first_exp_range.len(),
    })
}

/// 3x - xy + 4x + 4xy - y^2
/// Difficulty: 5
#[problem]
fn simplify_variable_combinations(name: String, _lang: &Language) -> Result<Problem> {
    let mut rng = rand::rng();
    let total_terms = 5;
    let variable_combinations: Vec<(i32, i32)> = vec![
        (1, 0),
        (2, 0),
        (2, 0),
        (1, 1),
        (1, 1),
        (0, 1),
        (0, 2),
        (0, 2),
    ];
    let (first_unknown, second_unknown) = symbols::get_two_unknowns()?;
    let mut expression = Polynomial::new();
    let mut i = 0;
    while i < total_terms {
        let coef = IntRange::without_zero(-6, 6)?.random();
        let (first_exponent, second_exponent) = variable_combinations.choose(&mut rng).unwrap();
        let vars: Variables = vec![
            (first_unknown, *first_exponent),
            (second_unknown, *second_exponent),
        ]
        .into();
        let term: Term = (coef, vars).into();
        expression.push(term);
        i += 1;
    }
    let sorted = expression.sorted();
    let simplified = expression.simplify();

    let solution = format!(
        "$&{expression} = \\
        = &{sorted} = \\
        = &{simplified}$",
    );

    Ok(Problem {
        name,
        question: format!("${expression}$"),
        answer: format!("${simplified}$"),
        solution,
        identifiers: vec![1],
        combinations: 1,
    })
}

/// Evaluate 3x2 + 2xy^2 + 3x if x = -3 and y = 2
/// Difficulty: 5
#[problem]
fn evaluate_advanced(name: String, lang: &Language) -> Result<Problem> {
    let mut total_terms = 2;
    let mut exp_combinations: Vec<(i32, i32)> = Vec::new();
    let (first_unknown, second_unknown) = symbols::get_two_unknowns()?;
    let mut expression = Polynomial::new();
    while total_terms > 0 {
        let coef = IntRange::without_ones_and_zero(-4, 4)?.random();
        let first_exponent = IntRange::with_zero(1, 2)?.random();
        let second_exponent = IntRange::with_zero(1, 2)?.random();
        if exp_combinations.contains(&(first_exponent, second_exponent)) {
            continue;
        }
        exp_combinations.push((first_exponent, second_exponent));
        let vars: Variables = vec![
            (first_unknown, first_exponent),
            (second_unknown, second_exponent),
        ]
        .into();
        expression += Term::from((coef, vars));
        total_terms -= 1;
    }
    let value_x = IntRange::without_zero(-2, -1)?.random();
    let value_y = IntRange::without_zero(-2, -1)?.random();

    let problem_data = get_problem_data(&name)?;
    let question = replace_placeholders(
        problem_data.get_question(lang),
        &[
            ("expression", expression.to_string()),
            ("unknown_a", first_unknown.to_string()),
            ("unknown_b", second_unknown.to_string()),
            ("value_x", value_x.to_string()),
            ("value_y", value_y.to_string()),
        ],
    );
    let replacements = vec![(first_unknown, value_x), (second_unknown, value_y)];
    let answer = expression.evaluate(&replacements);

    let solution = format!(
        "$&{expression} = \\
        = &{} = \\
        = &{} = {answer}$",
        expression.show_replacements(&replacements),
        expression.show_evaluation(&replacements)
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![1],
        combinations: 1,
    })
}
