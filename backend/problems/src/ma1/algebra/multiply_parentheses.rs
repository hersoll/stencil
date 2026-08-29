use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay,
    formatting::{divide_number, subtract_number, subtract_term},
    num_gen::{self, NumberGenerator},
    symbols,
};
use registry::get_solution;
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// (x + 3)(x + 5)
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn both_positive(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 6).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var + const1;
    let expr2 = var + const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (x - 3)(x + 5)
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn first_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 7).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var - const1;
    let expr2 = var + const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (x + 3)(x - 5)
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn second_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 7).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var + const1;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (x - 3)(x - 5)
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn both_negative(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 7).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var - const1;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (3 - x)(x - 5)
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn negative_x(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(1, 7).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = const1 - var;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// (2x - 3)(x + 5)
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn one_coef(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let (coef, coef_range) = num_gen::integer().range(2, 4).and_random();
    let var = symbols::get_unknown()?;

    let expr1 = coef * var - const1;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2, coef],
        combinations: const_range.len() * (const_range.len() - 1) * coef_range.len(),
    }))
}

/// (2x - 3)(3x + 5)
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn two_coefs(id: i32, _lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let (coef1, coef_range) = num_gen::integer().range(2, 4).and_random();
    let coef2 = coef_range.random();
    let var = symbols::get_unknown()?;

    let expr1 = coef1 * var - const1;
    let expr2 = coef2 * var - const2;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2, coef1, coef2],
        combinations: const_range.len() * (const_range.len() - 1) * coef_range.len().pow(2),
    }))
}

/// 2(x - 3)(x + 5)
/// Absolute difficulty: 5
/// Relative difficulty: 8
#[problem]
fn coef_before(id: i32, lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(-3, 4).exclude(0).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let (coef, coef_range) = num_gen::integer().range(2, 4).and_random();
    let var = symbols::get_unknown()?;

    let expr1 = var - const1;
    let expr2 = var - const2;
    let mult = &expr1 * &expr2;
    let simplified_mult = mult.simplify();
    let answer = coef * simplified_mult.clone();
    let question = format!("{coef}({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .equals(format!("{coef}({simplified_mult})"))
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    let solution = get_solution(id, lang)?.replace_one("solution", solution);

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2, coef],
        combinations: const_range.len() * (const_range.len() - 1) * coef_range.len(),
    }))
}

/// (2x - y)(3x + 2y)
/// Absolute difficulty: 6
/// Relative difficulty: 9
#[problem]
fn two_variables(id: i32, _lang: Language) -> Result<Problem> {
    let (coef1, x_coef_range) = num_gen::integer().range(1, 4).and_random();
    let (coef2, y_coef_range) = num_gen::integer().range(-3, 3).exclude(0).and_random();
    let coef3 = x_coef_range.random();
    let coef4 = y_coef_range.random();
    let (x, y) = symbols::get_two_unknowns()?;

    let expr1 = coef1 * x + coef2 * y;
    let expr2 = coef3 * x + coef4 * y;
    let mult = &expr1 * &expr2;
    let answer = mult.simplify();
    let question = format!("({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .linebreak_equality()
        .equals(mult)
        .linebreak_equality()
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![coef1, coef2, coef3, coef4],
        combinations: x_coef_range.len().pow(2) * y_coef_range.len().pow(2),
    }))
}

/// (x - 3)(x + 5) = (x + 2)(x + 4) - 4x
/// Absolute difficulty: 6
/// Relative difficulty: 9
#[problem]
fn equation(id: i32, _lang: Language) -> Result<Problem> {
    // (x + a)(x + b) = (x + c)(x + d) + kx
    // To make sure the answer is an integer, we make sure ab and cd are even
    // and the expression a + b - c - d - k = 2
    let (a, even_range) = num_gen::integer()
        .range_step(-4, 4, 2)
        .exclude(0)
        .and_random();
    let c = even_range.negative();
    // To make sure we collect our x on the LHS, we need to make sure a + b > c + d,
    // which means d < a + b - c
    let (b, b_range) = num_gen::integer().range(1, 5).and_random();
    let d = num_gen::integer()
        .range(-5, a + b - c - 1)
        .exclude(0)
        .random();
    let k = a + b - c - d - 2;
    let var = symbols::get_unknown()?;

    let expr_a = var + a;
    let expr_b = var + b;
    let expr_c = var + c;
    let expr_d = var + d;
    let kx = k * var;

    let mult_lhs = (&expr_a * &expr_b).simplify();
    let mult_rhs = (&expr_c * &expr_d).simplify();
    let lhs = format!("({expr_a})({expr_b})");
    let rhs = format!("({expr_c})({expr_d}){kx:+}");
    let question = format!("{lhs} = {rhs}");
    let answer = (c * d - a * b) / 2;
    let mut solution = Solution::with_steps();
    solution
        .aligned(lhs, rhs)
        .aligned(&mult_lhs, format!("{mult_rhs}{kx:+}"))
        .aligned(mult_lhs, mult_rhs + &kx)
        .step(format!("$cancel({var}^2)$"))
        .aligned((a + b) * var + a * b, (c + d + k) * var + c * d)
        .step(subtract_term(&((c + d + k) * var)))
        .aligned(2 * var + a * b, c * d)
        .step(subtract_number(a * b))
        .aligned(2 * var, c * d - a * b)
        .step(divide_number(2))
        .aligned(var, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![a, b],
        combinations: even_range.len() * b_range.len(),
    }))
}

/// x^2 - (x - 3)(x + 5)
/// Absolute difficulty: 7
/// Relative difficulty: 10
#[problem]
fn square_first(id: i32, lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let var = symbols::get_unknown()?;

    let expr1 = var - const1;
    let expr2 = var - const2;
    let mult = (&expr1 * &expr2).simplify();
    let diff = var.powi(2) - mult.clone();
    let answer = diff.simplify();
    let question = format!("{var}^2 - ({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .linebreak_equality()
        .equals(format!("{var}^2 - ({mult})"))
        .linebreak_equality()
        .equals(format!("{var}^2 {:+}", -mult))
        .equals(&answer)
        .to_string();
    let solution = get_solution(id, lang)?.replace_one("solution", solution);

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}

/// 10x - (x - 3)(x + 5)
/// Absolute difficulty: 7
/// Relative difficulty: 10
#[problem]
fn term_first(id: i32, lang: Language) -> Result<Problem> {
    let (const1, const_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();
    let const2 = const_range.clone().exclude(const1).random();
    let coef = num_gen::integer().range(2, 10).random();
    let var = symbols::get_unknown()?;

    let expr1 = var - const1;
    let expr2 = var - const2;
    let mult = (&expr1 * &expr2).simplify();
    let term = coef * var;
    let diff = term.clone() - mult.clone();
    let answer = diff.simplify();
    let question = format!("{term} - ({expr1})({expr2})");
    let solution = Solution::inline()
        .write(&question)
        .linebreak_equality()
        .equals(format!("{term} - ({mult})"))
        .linebreak_equality()
        .equals(format!("{term} {:+}", -mult))
        .equals(&answer)
        .to_string();
    let solution = get_solution(id, lang)?.replace_one("solution", solution);

    Ok(Problem::from(ProblemParameters {
        id,
        question: question.as_math(),
        answer,
        solution,
        identifiers: vec![const1, const2],
        combinations: const_range.len() * (const_range.len() - 1),
    }))
}
