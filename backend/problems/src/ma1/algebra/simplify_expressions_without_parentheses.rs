use anyhow::Result;
use macros::problem;
use math::{
    Number, Polynomial, Term, VariableList,
    num_gen::{self, NumberGenerator},
    symbols,
};
use rand::seq::IndexedRandom;
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters},
};

/// 3x + 4 + 2x + 1
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn one_variable_and_constants_no_negatives(id: i32, _lang: Language) -> Result<Problem> {
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_const],
        combinations: first_coef_range.len() * first_const_range.len(),
    }))
}

/// 3x - 5 - 5x + 2
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn one_variable_and_constants(id: i32, _lang: Language) -> Result<Problem> {
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_const],
        combinations: coef_range.len() * const_range.len(),
    }))
}

/// 2x - 3y + 3x - 8y + 1
/// Absolute difficulty: 1
/// Relative difficulty: 3
#[problem]
fn two_variables_and_constants(id: i32, _lang: Language) -> Result<Problem> {
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

    let original_expression = Polynomial::random_order(&[
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef_a, second_coef_a],
        combinations: coef_range.len().pow(2),
    }))
}

/// x^2 + 2x + 3x^2 - 4x
/// Absolute difficulty: 3
/// Relative difficulty: 5
#[problem]
fn one_variable_different_exponents(id: i32, _lang: Language) -> Result<Problem> {
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
        Polynomial::random_order(&[&first_term, &second_term, &third_term, &fourth_term]);
    let sorted_expression = original_expression.sorted();
    let simplified_expression = original_expression.simplify();

    let question = format!("${original_expression}$");
    let answer = format!("${simplified_expression}$");

    let solution = format!(
        "$&{original_expression} = \\
        = &{sorted_expression} = \\
        = &{simplified_expression}$",
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![first_coef, first_exp],
        combinations: coef_range.len() * exp_range.len(),
    }))
}

/// 3x - xy + 4x + 4xy - y^2
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn simplify_variable_combinations(id: i32, _lang: Language) -> Result<Problem> {
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
        let vars: VariableList = vec![
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

    Ok(Problem::from(ProblemParameters {
        id,
        question: expression,
        answer: simplified,
        solution,
        identifiers: Number::Integer(1),
        combinations: 1,
    }))
}
