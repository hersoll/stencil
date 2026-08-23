use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay,
    num_gen::{self, NumberGenerator},
    symbols,
};
use registry::get_solution;
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// What's missing? (x + 3)^2 = x^2 + 6x + A
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn simple_missing_last_positive(id: i32, lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let missing = constant.pow(2);
    let double = constant * 2;
    let missing_icon = symbols::get_drawable()?;
    let var = symbols::X;

    let solution =
        get_solution(id, lang)?.replace_multiple(&[("num", constant), ("square", missing)]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$({var}{constant:+})^2 = {var}^2 {double:+}{var} + {missing_icon}$"),
        answer: format!("${missing_icon} = {missing}$"),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// What's missing? (x - 3)^2 = x^2 - 6x + A
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn simple_missing_last_negative(id: i32, lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, -1).and_random();
    let missing = constant.pow(2);
    let double = constant * 2;
    let missing_icon = symbols::get_drawable()?;
    let var = symbols::X;

    let solution =
        get_solution(id, lang)?.replace_multiple(&[("num", constant.abs()), ("square", missing)]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$({var}{constant:+})^2 = {var}^2 {double:+}{var} + {missing_icon}$"),
        answer: format!("${missing_icon} = {missing}$"),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// What's missing? (x + 3)^2 = x^2 + A + 9
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn simple_missing_middle_positive(id: i32, lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let last = constant.pow(2);
    let double = constant * 2;
    let missing_icon = symbols::get_drawable()?;
    let var = symbols::X;
    let middle_term = double * var;

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("num", constant.abs().to_string()),
        ("var", var.to_string()),
        ("middle", middle_term.to_string()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$({var}{constant:+})^2 = {var}^2 + {missing_icon} + {last}$"),
        answer: format!("${missing_icon} = {middle_term}$"),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// What's missing? (x - 3)^2 = x^2 - A + 9
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn simple_missing_middle_negative(id: i32, lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, -1).and_random();
    let last = constant.pow(2);
    let double = constant.abs() * 2;
    let missing_icon = symbols::get_drawable()?;
    let var = symbols::X;
    let middle_term = double * var;

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("num", constant.abs().to_string()),
        ("var", var.to_string()),
        ("middle", middle_term.to_string()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$({var}{constant:+})^2 = {var}^2 - {missing_icon} + {last}$"),
        answer: format!("${missing_icon} = {middle_term}$"),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (x + 3)^2
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn simple_first_rule(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let answer = (expr.clone() * expr.clone()).simplify();

    let question = format!("({expr})^2");
    let solution = Solution::inline()
        .write(&question)
        .align(format!(
            "{var}^2 + 2 dot {var} dot {constant} + {constant}^2"
        ))
        .linebreak_equality()
        .align(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (x - 3)^2
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn simple_second_rule(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let var = symbols::get_unknown()?;
    let expr = var - constant;
    let answer = (expr.clone() * expr.clone()).simplify();

    let question = format!("({expr})^2");
    let solution = Solution::inline()
        .write(&question)
        .align(format!(
            "{var}^2 - 2 dot {var} dot {constant} + {constant}^2"
        ))
        .linebreak_equality()
        .align(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (x + 3)^2 - 6x
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn first_rule_with_cancelling_middle(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let cancelling_term = 2 * constant * var;
    let answer = square.clone() - &cancelling_term;

    let question = format!("({expr})^2 - {cancelling_term}");
    let solution = Solution::inline()
        .write(&question)
        .equals(format!("colored({square}) - {cancelling_term}"))
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (x - 3)^2 + 6x
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn second_rule_with_cancelling_middle(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let var = symbols::get_unknown()?;
    let expr = var - constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let cancelling_term = 2 * constant * var;
    let answer = square.clone() + &cancelling_term;

    let question = format!("({expr})^2 + {cancelling_term}");
    let solution = Solution::inline()
        .write(&question)
        .equals(format!("colored({square}) + {cancelling_term}"))
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (x + 4)^2 + 7x
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn random_rule_with_random_middle(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let middle_coef = const_range.random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let middle_term = middle_coef * var;
    let answer = square.clone() + &middle_term;

    let question = format!("({expr})^2 {middle_term:+}");
    let solution = Solution::inline()
        .write(&question)
        .equals(format!("colored({square}) {middle_term:+}"))
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (x + 4)^2 + 7
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn random_rule_with_random_const(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let added_const = const_range.random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let answer = square.clone() + added_const;

    let question = format!("({expr})^2 {added_const:+}");
    let solution = Solution::inline()
        .write(&question)
        .equals(format!("colored({square}) {added_const:+}"))
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (x + 4)^2 + x^2
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn random_rule_with_square(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let added_square = var.powi(2);
    let answer = square.clone() + added_square.clone();

    let question = format!("({expr})^2 {added_square:+}");
    let solution = Solution::inline()
        .write(&question)
        .equals(format!("colored({square}) {added_square:+}"))
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand x^2 + (x + 4)^2
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn square_with_random_rule(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let added_square = var.powi(2);
    let answer = square.clone() + added_square.clone();

    let question = format!("{added_square} + ({expr})^2");
    let solution = Solution::inline()
        .write(&question)
        .equals(format!("{added_square} + colored({square})"))
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// What's missing? (x + A)^2 = x^2 + B + 9
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn missing_two(id: i32, lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 9).and_random();
    let last = constant.pow(2);
    let double = constant * 2;
    let missing_icon_a = symbols::HEART;
    let missing_icon_b = symbols::DIAMOND;
    let var = symbols::get_unknown()?;
    let middle_term = double * var;

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("missing_a", missing_icon_a.to_string()),
        ("missing_b", missing_icon_b.to_string()),
        ("num", constant.to_string()),
        ("square", last.to_string()),
        ("var", var.to_string()),
        ("middle", middle_term.to_string()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$({var} + {missing_icon_a})^2 = {var}^2 + {missing_icon_b} + {last}$"),
        answer: format!("${missing_icon_a} = {constant}, quad {missing_icon_b} = {middle_term}$"),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand (2x + 3)^2
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn random_rule_coef_x(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let (coef, coef_range) = num_gen::integer().range(2, 4).and_random();
    let var = symbols::get_unknown()?;
    let term = coef * var;
    let expr = term.clone() + constant;
    let answer = (expr.clone() * expr.clone()).simplify();
    let sign = if constant > 0 { "+" } else { "-" };
    let abs = constant.abs();

    let question = format!("({expr})^2");
    let solution = Solution::inline()
        .write(&question)
        .align(format!(
            "({term})^2 {sign} 2 dot {term} dot {abs} + {abs}^2"
        ))
        .linebreak_equality()
        .align(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: vec![constant, coef],
        combinations: const_range.len() * coef_range.len(),
    }))
}

/// Expand 2(x + 3)^2
/// Absolute difficulty: 5
/// Relative difficulty: 8
#[problem]
fn random_rule_with_coef(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-3, 3).exclude(0).and_random();
    let (coef, coef_range) = num_gen::integer().range(2, 4).and_random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let answer = coef * square.clone();

    let question = format!("{coef}({expr})^2");
    let solution = Solution::inline()
        .write(format!("{coef}colored(({expr})^2)"))
        .align(format!("{coef}colored(({square}))"))
        .linebreak_equality()
        .align(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: vec![constant, coef],
        combinations: const_range.len() * coef_range.len(),
    }))
}

/// Expand (2x + 3y)^2
/// Absolute difficulty: 6
/// Relative difficulty: 9
#[problem]
fn random_rule_with_two_vars(id: i32, _lang: Language) -> Result<Problem> {
    let (coef1, coef_range) = num_gen::integer().range(2, 5).and_random();
    let coef2 = coef_range.clone().exclude(coef1).random();
    let (var1, var2) = symbols::get_two_unknowns()?;
    let term1 = coef1 * var1;
    let term2 = coef2 * var2;
    let expr = &term1 + &term2;
    let answer = (expr.clone() * expr.clone()).simplify();

    let question = format!("({expr})^2");
    let solution = Solution::inline()
        .write(&question)
        .align(format!(
            "({term1})^2 + 2 dot {term1} dot {term2} + ({term2})^2"
        ))
        .linebreak_equality()
        .align(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: vec![coef1, coef2],
        combinations: coef_range.len() * (coef_range.len() - 1),
    }))
}

/// Expand x^2 - (x + 3)^2
/// Absolute difficulty: 6
/// Relative difficulty: 10
#[problem]
fn square_minus_square(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let var = symbols::get_unknown()?;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let answer = var.powi(2) - square.clone();

    let question = format!("{var}^2 - ({expr})^2");
    let solution = Solution::inline()
        .write(format!("{var}^2 - colored(({expr})^2)"))
        .equals(format!("{var}^2 - colored(({square}))"))
        .linebreak_equality()
        .equals(format!("{var}^2 {:+}", -square))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand 2x - (x + 3)^2
/// Absolute difficulty: 6
/// Relative difficulty: 10
#[problem]
fn var_term_minus_square(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let (coef, coef_range) = num_gen::integer().range(1, 9).and_random();
    let var = symbols::get_unknown()?;
    let term = coef * var;
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let answer = term.clone() - square.clone();

    let question = format!("{term} - ({expr})^2");
    let solution = Solution::inline()
        .write(format!("{term} - colored(({expr})^2)"))
        .equals(format!("{term} - colored(({square}))"))
        .linebreak_equality()
        .equals(format!("{term} {:+}", -square))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: vec![coef, constant],
        combinations: const_range.len() * coef_range.len(),
    }))
}

/// Expand (x^2 + 3)^2
/// Absolute difficulty: 7
/// Relative difficulty: 11
#[problem]
fn square_in_square(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();
    let var = symbols::get_unknown()?;
    let term = var.powi(2);
    let expr = term.clone() + constant;
    let answer = (expr.clone() * expr.clone()).simplify();
    let sign = if constant > 0 { "+" } else { "-" };
    let abs = constant.abs();

    let question = format!("({expr})^2");
    let solution = Solution::inline()
        .write(&question)
        .align(format!(
            "({term})^2 {sign} 2 dot {term} dot {abs} + {abs}^2"
        ))
        .linebreak_equality()
        .align(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}

/// Expand ((x + 1)^2 - 1)^2
/// Absolute difficulty: 8
/// Relative difficulty: 12
#[problem]
fn square_and_term_in_square(id: i32, _lang: Language) -> Result<Problem> {
    let (constant, const_range) = num_gen::integer().range(1, 5).and_random();
    let var = symbols::get_unknown()?;
    let extra_term = constant.pow(2);
    let expr = var + constant;
    let square = (expr.clone() * expr.clone()).simplify();
    let second_expr = var.powi(2) + 2 * constant * var;
    let answer = (second_expr.clone() * second_expr.clone()).simplify();

    let question = format!("(({expr})^2 - {extra_term})^2");
    let solution = Solution::inline()
        .write(format!("(colored(({expr})^2) - {extra_term})^2"))
        .linebreak_equality()
        .equals(format!("(colored({square}) - {extra_term})^2"))
        .linebreak_equality()
        .equals(format!("({second_expr})^2"))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: answer.as_math(),
        solution,
        identifiers: constant,
        combinations: const_range,
    }))
}
