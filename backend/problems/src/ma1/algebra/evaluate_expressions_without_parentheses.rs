use anyhow::Result;
use macros::problem;
use math::{Evaluable, MathDisplay, Number, Polynomial, Term, VariableList, num_gen, symbols};
use types::{format_strings::HasReplacements, lang::Language, problems::Problem};

/// Evaluate 3x - 1 when x = -3
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn evaluate_simple(id: i32, lang: Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (coef, coef_range) = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1])
        .and_random();
    let constant = num_gen::integer().range(-9, 9).exclude(0).random();
    let (value, value_range) = num_gen::integer().range(-5, 5).exclude(0).and_random();

    let first_term = Term::from_num_and_vars(coef, unknown);
    let const_term = Term::from(constant);

    let expression = first_term.and(&const_term);
    let replacement_map = vec![
        ("expression", expression.to_string()),
        ("unknown", unknown.to_string()),
        ("value", value.to_string()),
    ];
    let question = registry::get_question(id, lang)?.replace_placeholders(&replacement_map);
    let replacements = [(unknown, &value)];
    let answer = expression.evaluate(&replacements);

    let solution = format!(
        "$&{expression} = \\
        = &{} = \\
        = &{}{constant:+} = {answer}$",
        expression.print_replacements(&replacements),
        coef * value,
    );

    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![coef, value],
        combinations: coef_range.len() * value_range.len(),
    })
}

/// Evaluate 3x - 2y + 1 when x = -3 and y = 2
/// Absolute difficulty: 2
/// Relative difficulty: 2
#[problem]
fn evaluate_intermediate(id: i32, lang: Language) -> Result<Problem> {
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
    let question = registry::get_question(id, lang)?.replace_placeholders(&[
        ("expression", expression.to_string()),
        ("unknown_a", first_unknown.to_string()),
        ("unknown_b", second_unknown.to_string()),
        ("value_x", value_x.to_string()),
        ("value_y", value_y.to_string()),
    ]);
    let replacements = [(first_unknown, &value_x), (second_unknown, &value_y)];
    let answer = expression.evaluate(&replacements);

    let solution = format!(
        "$&{expression} = \\
        = &{} = \\
        = &{}{:+}{constant:+} = {answer}$",
        expression.print_replacements(&replacements),
        coef * value_x,
        coef_2 * value_y,
    );

    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![coef, coef_2],
        combinations: coef_range.len().pow(2),
    })
}

/// Evaluate 3x^2 + 2xy^2 + 3x if x = -3 and y = 2
/// Absolute difficulty: 5
/// Relative difficulty: 3
#[problem]
fn evaluate_advanced(id: i32, lang: Language) -> Result<Problem> {
    let mut total_terms = 2;
    let mut exp_combinations: Vec<(Number, Number)> = Vec::new();
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
        let vars: VariableList = vec![
            (first_unknown, first_exponent),
            (second_unknown, second_exponent),
        ]
        .into();
        expression += Term::from_num_and_vars(coef, vars);
        total_terms -= 1;
    }
    let value_x = num_gen::integer().range(-2, -1).random();
    let value_y = num_gen::integer().range(-2, -1).random();

    let question = registry::get_question(id, lang)?.replace_placeholders(&[
        ("expression", expression.to_string()),
        ("unknown_a", first_unknown.to_string()),
        ("unknown_b", second_unknown.to_string()),
        ("value_x", value_x.to_string()),
        ("value_y", value_y.to_string()),
    ]);
    let replacements = [(first_unknown, &value_x), (second_unknown, &value_y)];
    let answer = expression.evaluate(&replacements);

    let solution = format!(
        "$&{expression} = \\
        = &{} = \\
        = &{} = {answer}$",
        expression.print_replacements(&replacements),
        expression.print_evaluation_by_parts(&replacements),
    );

    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![Number::Integer(0)],
        combinations: 1,
    })
}
