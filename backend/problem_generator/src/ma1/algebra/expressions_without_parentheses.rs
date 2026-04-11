use anyhow::Result;
use macros::problem;
use math::{Polynomial, Term, Variables, num_gen, symbols};
use rand::seq::IndexedRandom;
use registry::{get_problem_data, replace_placeholders};
use types::{lang::Language, problems::Problem};
use typst_writer::custom_math::polynomials::{
    show_polynomial_evaluation, show_polynomial_replacements,
};

/// 3x + 4 + 2x + 1
/// Difficulty: 0
#[problem]
fn one_variable_and_constants_no_negatives(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (first_coef, first_coef_range) = num_gen::integer().range(1, 6).and_random();
    let second_coef = num_gen::integer()
        .range(-(first_coef - 1), 6)
        .exclude(0)
        .random();
    let (first_const, first_const_range) = num_gen::integer().range(1, 6).and_random();
    let second_const = num_gen::integer()
        .range(-(first_const - 1), 6)
        .exclude(0)
        .random();
    let first_term = Term::from_num_and_vars(first_coef, unknown);
    let second_term = Term::from_num_and_vars(second_coef, unknown);
    let first_const_term = Term::from_num(first_const);
    let second_const_term = Term::from_num(second_const);
    let original_expression = Polynomial::from_terms(&[
        &first_term,
        &first_const_term,
        &second_term,
        &second_const_term,
    ]);
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
    let coef_range = num_gen::integer().range(-6, 6).exclude(0);
    let first_coef = coef_range.random();
    let second_coef = coef_range.random();
    let const_range = num_gen::integer().range(-6, 6).exclude(0);
    let first_const = const_range.random();
    let second_const = const_range.random();
    let first_term = Term::from_num_and_vars(first_coef, unknown);
    let second_term = Term::from_num_and_vars(second_coef, unknown);
    let first_const_term = Term::from_num(first_const);
    let second_const_term = Term::from_num(second_const);
    let original_expression = Polynomial::from_terms(&[
        &first_term,
        &first_const_term,
        &second_term,
        &second_const_term,
    ]);
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
        combinations: coef_range.len() * const_range.len(),
    })
}

/// 2x - 3y + 3x - 8y + 1
/// Difficulty: 2
#[problem]
fn two_variables_and_constants(name: String, _lang: &Language) -> Result<Problem> {
    let (first_unknown, second_unknown) = symbols::get_two_unknowns()?;
    let coef_range = num_gen::integer().range(-9, 9);
    let first_coef_a = coef_range.random();
    let first_coef_b = coef_range.random();
    let second_coef_a = coef_range.random();
    let second_coef_b = coef_range.random();
    let const_range = num_gen::integer().range(-9, 9);
    let first_const = const_range.random();
    let second_const = const_range.random();

    let mut first_term_a = Term::from_num_and_vars(first_coef_a, first_unknown);
    let mut first_term_b = Term::from_num_and_vars(first_coef_b, first_unknown);
    let second_term_a = Term::from_num_and_vars(second_coef_a, second_unknown);
    let second_term_b = Term::from_num_and_vars(second_coef_b, second_unknown);
    let mut first_const_term = Term::from_num(first_const);
    let mut second_const_term = Term::from_num(second_const);

    let original_expression = Polynomial::random_order(vec![
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
    let first_terms = Polynomial::from_terms(&[&first_term_a, &first_term_b]).sorted();
    let second_terms = Polynomial::from_terms(&[&second_term_a, &second_term_b]).sorted();
    let const_terms = Polynomial::from_terms(&[&first_const_term, &second_const_term]).sorted();
    let sorted_expression = first_terms + second_terms + const_terms;

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
        combinations: coef_range.len().pow(2),
    })
}

/// Evaluate 3x - 1 when x = -3
/// Difficulty: 1
#[problem]
fn evaluate_simple(name: String, lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (coef, coef_range) = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1])
        .and_random();
    let constant = num_gen::integer().range(-9, 9).exclude(0).random();
    let (value, value_range) = num_gen::integer().range(-5, 5).exclude(0).and_random();

    let first_term = Term::from_num_and_vars(coef, unknown);
    let const_term = Term::from(constant);

    let expression = Polynomial::from_terms(&[&first_term, &const_term]);
    let replacement_map = vec![
        ("expression", expression.to_string()),
        ("unknown", unknown.to_string()),
        ("value", value.to_string()),
    ];
    let problem_data = get_problem_data(&name)?;
    let question = replace_placeholders(problem_data.get_question(lang), &replacement_map);
    let replacements = vec![(unknown, value)];
    let answer = expression.evaluate(&replacements);

    let solution = format!(
        "$&{expression} = \\
        = &{} = \\
        = &{}{constant:+} = {answer}$",
        show_polynomial_replacements(&expression, &replacements),
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
    let coef_range = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1]);
    let coef = coef_range.random();
    let coef_2 = coef_range.random();
    let constant = num_gen::integer().range(-9, 9).exclude(0).random();
    let value_x = num_gen::integer().range(-5, 5).exclude(0).random();
    let value_y = num_gen::integer().range(-5, -1).random();

    let first_term = Term::from_num_and_vars(coef, first_unknown);
    let second_term = Term::from_num_and_vars(coef_2, second_unknown);
    let const_term = Term::from_num(constant);

    let expression = Polynomial::from_terms(&[&first_term, &second_term, &const_term]);
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
        show_polynomial_replacements(&expression, &replacements),
        coef * value_x,
        coef_2 * value_y,
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef, coef_2],
        combinations: coef_range.len().pow(2),
    })
}

/// x^2 + 2x + 3x^2 - 4x
/// Difficulty: 3
#[problem]
fn one_variable_different_exponents(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer().range(-9, 9).exclude(0);
    let first_coef = coef_range.random();
    let second_coef = coef_range.random();
    let third_coef = coef_range.random();
    let fourth_coef = coef_range.random();
    let exp_range = num_gen::integer().range(1, 2);
    let first_exp = exp_range.random();
    let second_exp = exp_range.random();
    let third_exp = 2;
    let fourth_exp = 1;

    let first_term = Term::from_num_and_vars(first_coef, (unknown, first_exp));
    let second_term = Term::from_num_and_vars(second_coef, (unknown, second_exp));
    let third_term = Term::from_num_and_vars(third_coef, (unknown, third_exp));
    let fourth_term = Term::from_num_and_vars(fourth_coef, (unknown, fourth_exp));

    let original_expression =
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
        combinations: coef_range.len() * exp_range.len(),
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
        let coef = num_gen::integer().range(-6, 6).exclude(0).random();
        let (first_exponent, second_exponent) = variable_combinations.choose(&mut rng).unwrap();
        let vars: Variables = vec![
            (first_unknown, *first_exponent),
            (second_unknown, *second_exponent),
        ]
        .into();
        let term = Term::from_num_and_vars(coef, vars);
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
    let coef_range = num_gen::integer()
        .range(-4, 4)
        .exclude_multiple(&[-1, 0, 1]);
    let exponent_range = num_gen::integer().range(1, 2);
    while total_terms > 0 {
        let coef = coef_range.random();
        let first_exponent = exponent_range.random();
        let second_exponent = exponent_range.random();
        if exp_combinations.contains(&(first_exponent, second_exponent)) {
            continue;
        }
        exp_combinations.push((first_exponent, second_exponent));
        let vars: Variables = vec![
            (first_unknown, first_exponent),
            (second_unknown, second_exponent),
        ]
        .into();
        expression += Term::from_num_and_vars(coef, vars);
        total_terms -= 1;
    }
    let value_x = num_gen::integer().range(-2, -1).random();
    let value_y = num_gen::integer().range(-2, -1).random();

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
        show_polynomial_replacements(&expression, &replacements),
        show_polynomial_evaluation(&expression, &replacements),
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
