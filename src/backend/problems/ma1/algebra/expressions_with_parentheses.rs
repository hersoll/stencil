use crate::Result;
use crate::backend::problems::symbols;
use crate::backend::{typst_formatting, Polynomial, IntRange, Number, Problem, Term};
use macros::problem;

/// 3(x+1)
/// Difficulty: 0
#[problem]
fn positive_integer_mult(id: String, _lang: &str) -> Result<Problem> {
    let (factor, f_range) = IntRange::without_zero(2, 5)?.and_random();
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = IntRange::without_zero(-7, 7)?.and_random();

    let t1: Term = unknown.into();
    let t2: Term = constant.into();
    let exp: Polynomial = vec![t1, t2].into();

    let question = format!("${factor}({exp})$");
    let answer = (factor * exp.clone()).simplify();
    let solution = format!(
        "${factor}({exp}) = colored({factor} dot) {unknown} {sign} colored({factor} dot) {abs_const} = {answer}$",
        sign = if constant > 0 { "+" } else { "-" },
        abs_const = constant.abs()
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len(),
    })
}

/// -2(x+4)
/// Difficulty: 1
#[problem]
fn negative_integer_mult(id: String, _lang: &str) -> Result<Problem> {
    let (factor, f_range) = IntRange::without_zero(-5, -2)?.and_random();
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = IntRange::without_zero(-7, -1)?.and_random();

    let t1: Term = unknown.into();
    let t2: Term = constant.into();
    let exp: Polynomial = vec![t1, t2].into();

    let question = format!("${factor}({exp})$");
    let answer = factor * exp.clone();
    let simplified = answer.simplify();
    let solution = format!(
        "$&{factor}({exp}) = colored({factor_p} dot) {unknown} + colored({factor_p} dot) {const_p} =\\
            =&{answer} = {simplified}$",
        factor_p = typst_formatting::parentheses(factor),
        const_p = typst_formatting::parentheses(constant),
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${simplified}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len(),
    })
}

/// 3(2a-4)
/// Difficulty: 2
#[problem]
fn with_coefficient_on_variable(id: String, _lang: &str) -> Result<Problem> {
    let (factor, f_range) = IntRange::without_zero(2, 5)?.and_random();
    let unknown = symbols::get_unknown()?;
    let coef = IntRange::without_zero(2, 5)?.random();
    let (constant, c_range) = IntRange::without_zero(-7, 7)?.and_random();

    let t1: Term = (coef, unknown).into();
    let t2: Term = constant.into();
    let exp: Polynomial = vec![&t1, &t2].into();

    let question = format!("${factor}({exp})$");
    let answer = (factor * exp.clone()).simplify();
    let solution = format!(
        "${factor}({exp}) = colored({factor} dot) {t1} {sign} colored({factor} dot) {abs_const} = {answer}$",
        sign = if constant > 0 { "+" } else { "-" },
        abs_const = constant.abs()
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len(),
    })
}

/// x(x+1)
/// Difficulty: 2
#[problem]
fn multiply_by_variable(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = IntRange::without_zero(-7, 7)?.and_random();

    let factor: Term = unknown.into();
    let t1: Term = unknown.into();
    let t2: Term = constant.into();
    let exp: Polynomial = vec![&t1, &t2].into();

    let question = format!("${factor}({exp})$");
    let answer = (factor.clone() * exp.clone()).simplify();
    let solution = format!(
        "${factor}({exp}) = colored({factor} dot) {t1} {sign} colored({factor} dot) {abs_const} = {answer}$",
        sign = if constant > 0 { "+" } else { "-" },
        abs_const = constant.abs()
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![constant],
        combinations: c_range.len(),
    })
}

/// (2x + 1) + (3x - 4)
/// Difficulty: 2
#[problem]
fn add_parentheses(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-9, 9)?;
    let coef_1 = coef_range.random();
    let const_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_2 = coef_range.random();

    let mut term_var_1: Term = (coef_1, unknown).into();
    let mut term_const_1: Term = const_1.into();
    let mut term_var_2: Term = (coef_2, unknown).into();
    let mut term_const_2: Term = const_2.into();
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);
    let mut exp_1: Polynomial = vec![&term_var_1, &term_const_1].into();
    let mut exp_2: Polynomial = vec![&term_var_2, &term_const_2].into();
    exp_1.sort();
    exp_2.sort();

    let question = format!("$({exp_1}) + ({exp_2})$");
    let answer = (exp_1.clone() + exp_2.clone()).simplify();
    let solution = format!(
        "$&({exp_1}) + ({exp_2}) = {exp_1} + {exp_2} = \\ 
            = &{term_var_1} {term_var_2:+} {term_const_1:+}{term_const_2:+} = {answer}$",
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, const_1],
        combinations: coef_range.len() * coef_range.len(),
    })
}

/// (2x + 1) - (3x - 4)
/// Difficulty: 3
#[problem]
fn subtract_parentheses(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-9, 9)?;
    let coef_1 = coef_range.random();
    let const_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_2 = coef_range.random();

    let mut term_var_1: Term = (coef_1, unknown).into();
    let mut term_const_1: Term = const_1.into();
    let mut term_var_2: Term = (coef_2, unknown).into();
    let mut term_const_2: Term = const_2.into();
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);

    let mut exp_1: Polynomial = vec![&term_var_1, &term_const_1].into();
    let mut exp_2: Polynomial = vec![&term_var_2, &term_const_2].into();
    exp_1.sort();
    exp_2.sort();

    let question = format!("$({exp_1}) - ({exp_2})$");
    let answer = (exp_1.clone() - exp_2.clone()).simplify();
    let solution = format!(
        "$&({exp_1}) - ({exp_2}) = {exp_1} {exp_2_m:+} = \\ 
            = &{term_var_1} {term_var_2_m:+} {term_const_1:+}{term_const_2_m:+} = {answer}$",
        exp_2_m = -&exp_2,
        term_var_2_m = -term_var_2,
        term_const_2_m = -term_const_2,
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, const_1],
        combinations: coef_range.len() * coef_range.len(),
    })
}

/// -4(3 - 4x)
/// Difficulty: 2
#[problem]
fn negative_factor_and_coef(id: String, _lang: &str) -> Result<Problem> {
    let (factor, f_range) = IntRange::without_zero(-8, -2)?.and_random();
    let unknown = symbols::get_unknown()?;
    let coef = IntRange::without_zero(-5, -2)?.random();
    let (constant, c_range) = IntRange::without_zero(2, 9)?.and_random();

    let t1: Term = constant.into();
    let t2: Term = (coef, unknown).into();
    let exp: Polynomial = vec![&t1, &t2].into();

    let question = format!("${factor}({exp})$");
    let answer = factor * exp.clone();
    let simplified = answer.simplify();
    let solution = format!(
        "$&{factor}({exp}) = colored({factor_p} dot) {t1_p} + colored({factor_p} dot) {t2_p} =\\
            =&{answer} = {simplified}$",
        factor_p = typst_formatting::parentheses(factor),
        t1_p = typst_formatting::parentheses(t1),
        t2_p = typst_formatting::parentheses(t2),
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${simplified}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len(),
    })
}

/// 3 - (2x - 1)
/// Difficulty: 3
#[problem]
fn const_minus_parenthesis(id: String, _lang: &str) -> Result<Problem> {
    let (initial, i_range) = IntRange::without_zero(2, 5)?.and_random();
    let unknown = symbols::get_unknown()?;
    let coef = IntRange::without_zero(2, 6)?.random();
    let (constant, c_range) = IntRange::without_zero(-7, -1)?.and_random();

    let t1: Term = initial.into();
    let t2: Term = (coef, unknown).into();
    let t3: Term = constant.into();

    let exp_1: Polynomial = t1.into();
    let exp_2: Polynomial = vec![&t2, &t3].into();

    let question = format!("${exp_1}-({exp_2})$");
    let answer = (exp_1.clone() - exp_2.clone()).simplify();
    let solution = format!(
        "$&{exp_1}-({exp_2}) = {exp_1} {t2_m:+} {t3_m:+} = \\
            = &{answer}$",
        t2_m = -&t2,
        t3_m = -&t3,
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![initial, constant],
        combinations: i_range.len() * c_range.len(),
    })
}

/// 2x - (7x - 1)
/// Difficulty: 3
#[problem]
fn var_term_minus_parenthesis(id: String, _lang: &str) -> Result<Problem> {
    let (initial, i_range) = IntRange::without_zero(1, 4)?.and_random();
    let unknown = symbols::get_unknown()?;
    let coef = IntRange::without_zero(5, 10)?.random();
    let (constant, c_range) = IntRange::without_zero(-7, -1)?.and_random();

    let t1: Term = (initial, unknown).into();
    let t2: Term = (coef, unknown).into();
    let t3: Term = constant.into();

    let exp_1: Polynomial = t1.into();
    let exp_2: Polynomial = vec![&t2, &t3].into();

    let question = format!("${exp_1}-({exp_2})$");
    let answer = exp_1.clone() - exp_2.clone();
    let simplified = answer.simplify();
    let solution = format!(
        "$&{exp_1}-({exp_2}) = {exp_1} {t2_m:+} {t3_m:+} = \\
            = &{answer} = {simplified}$",
        t2_m = -&t2,
        t3_m = -&t3,
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${simplified}$"),
        solution,
        identifiers: vec![initial, constant],
        combinations: i_range.len() * c_range.len(),
    })
}

/// 4(2x + 1) + 2(3x - 4)
/// Difficulty: 3
#[problem]
fn multiply_and_add(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-9, 9)?;
    let factor_range = IntRange::without_zero(2, 5)?;
    let factor_1 = factor_range.random();
    let factor_2 = factor_range.random();
    let coef_1 = coef_range.random();
    let const_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_2 = coef_range.random();

    let mut term_var_1: Term = (coef_1, unknown).into();
    let mut term_const_1: Term = const_1.into();
    let mut term_var_2: Term = (coef_2, unknown).into();
    let mut term_const_2: Term = const_2.into();
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);
    let mut exp_1: Polynomial = vec![&term_var_1, &term_const_1].into();
    let mut exp_2: Polynomial = vec![&term_var_2, &term_const_2].into();
    exp_1.sort();
    exp_2.sort();
    let mult_1 = factor_1 * exp_1.clone();
    let mult_2 = factor_2 * exp_2.clone();

    let question = format!("${factor_1}({exp_1}) + {factor_2}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) + {factor_2}({exp_2}) = \\ = &{mult_1} {mult_2:+} = {answer}$",
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, const_1],
        combinations: coef_range.len() * coef_range.len(),
    })
}

/// 4(2x + 1) - (3x - 4)
/// Difficulty: 3
#[problem]
fn multiply_first_and_subtract(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-9, 9)?;
    let factor_range = IntRange::without_zero(2, 5)?;
    let factor_1 = factor_range.random();
    let coef_1 = coef_range.random();
    let const_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_2 = coef_range.random();

    let mut term_var_1: Term = (coef_1, unknown).into();
    let mut term_const_1: Term = const_1.into();
    let mut term_var_2: Term = (coef_2, unknown).into();
    let mut term_const_2: Term = const_2.into();
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);

    let mut exp_1: Polynomial = vec![&term_var_1, &term_const_1].into();
    let mut exp_2: Polynomial = vec![&term_var_2, &term_const_2].into();
    exp_1.sort();
    exp_2.sort();
    let mult_1 = factor_1 * exp_1.clone();

    let question = format!("${factor_1}({exp_1}) - ({exp_2})$");
    let answer = (mult_1.clone() - exp_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) - ({exp_2}) = \\ = &{mult_1} {exp_2_m:+} = {answer}$",
        exp_2_m = -&exp_2,
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, const_1],
        combinations: coef_range.len() * coef_range.len(),
    })
}

/// 4(2x + 1) - 2(3x - 4)
/// Difficulty: 4
#[problem]
fn multiply_and_subtract(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-9, 9)?;
    let factor_range = IntRange::without_zero(2, 5)?;
    let factor_1 = factor_range.random();
    let factor_2 = factor_range.random();
    let coef_1 = coef_range.random();
    let const_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_2 = coef_range.random();

    let mut term_var_1: Term = (coef_1, unknown).into();
    let mut term_const_1: Term = const_1.into();
    let mut term_var_2: Term = (coef_2, unknown).into();
    let mut term_const_2: Term = const_2.into();
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);

    let mut exp_1: Polynomial = vec![&term_var_1, &term_const_1].into();
    let mut exp_2: Polynomial = vec![&term_var_2, &term_const_2].into();
    exp_1.sort();
    exp_2.sort();
    let mult_1 = factor_1 * exp_1.clone();
    let mult_2 = -factor_2 * exp_2.clone();

    let question = format!("${factor_1}({exp_1}) - {factor_2}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) - {factor_2}({exp_2}) = \\ = &{mult_1} {mult_2:+} = {answer}$",
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, const_1],
        combinations: coef_range.len() * coef_range.len(),
    })
}

/// 3x(1 - 2x)
/// Difficulty: 5
#[problem]
fn multiply_by_variable_term(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let constant = IntRange::without_zero(2, 7)?.random();
    let coef_range = IntRange::without_ones_and_zero(-5, 5)?;
    let coef_1 = coef_range.random().abs();
    let coef_2 = coef_range.random();

    let factor: Term = (coef_1, unknown).into();
    let t1: Term = (coef_2, unknown).into();
    let t2: Term = constant.into();
    let mut exp: Polynomial = vec![&t1, &t2].into();
    exp.sort();

    let question = format!("${factor}({exp})$");
    let answer = (factor.clone() * exp.clone()).simplify();
    let solution = format!(
        "$&{factor}({exp}) = colored({factor} dot) {e1} {sign} colored({factor} dot) {e2_abs} = \\ =&{answer}$",
        e1 = exp.terms[0],
        e2_abs = exp.terms[1].abs(),
        sign = if exp.terms[1].coefficient > Number::from(0) {"+"} else {"-"}
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, coef_2],
        combinations: coef_range.len() * coef_range.len(),
    })
}

/// x(3x + 1) - 3(2 + x)
/// Difficulty: 6
#[problem]
fn one_variable_one_constant(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-5, 5)?;
    let mut t1: Term = (coef_range.random(), unknown).into();
    let mut t3: Term = (coef_range.random(), unknown).into();
    let mut t2: Term = coef_range.random().into();
    let mut t4: Term = coef_range.random().into();
    Term::assert_one_positive(&mut t1, &mut t2);
    Term::assert_one_positive(&mut t3, &mut t4);

    let factor: Term = coef_range.random().into();
    let v_factor: Term = unknown.into();

    let mut exp_1: Polynomial = vec![&t1, &t2].into();
    let mut exp_2: Polynomial = vec![&t3, &t4].into();
    exp_1.sort();
    exp_2.sort();
    let mult_1 = v_factor.clone() * exp_1.clone();
    let mult_2 = factor.clone() * exp_2.clone();

    let question = format!("${v_factor}({exp_1}) {factor:+}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{v_factor}({exp_1}) {factor:+}({exp_2}) = \\
         =&{mult_1}{mult_2:+} = {answer}$"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![2],
        combinations: 1,
    })
}

/// 3(3x + 1) - x(2 + x)
/// Difficulty: 6
#[problem]
fn one_constant_one_variable(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-5, 5)?;
    let mut t1: Term = (coef_range.random(), unknown).into();
    let mut t3: Term = (coef_range.random(), unknown).into();
    let mut t2: Term = coef_range.random().into();
    let mut t4: Term = coef_range.random().into();
    Term::assert_one_positive(&mut t1, &mut t2);
    Term::assert_one_positive(&mut t3, &mut t4);

    let factor: Term = coef_range.random().into();
    let v_factor: Term = unknown.into();

    let mut exp_1: Polynomial = vec![&t1, &t2].into();
    let mut exp_2: Polynomial = vec![&t3, &t4].into();
    exp_1.sort();
    exp_2.sort();
    let mult_1 = factor.clone() * exp_1.clone();
    let mult_2 = v_factor.clone() * exp_2.clone();

    let question = format!("${factor}({exp_1}) {v_factor:+}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor}({exp_1}) {v_factor:+}({exp_2}) = \\
         =&{mult_1}{mult_2:+} = {answer}$"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![2],
        combinations: 1,
    })
}

/// 3x(3x + 1) - 2x(2 + x)
/// Difficulty: 7
#[problem]
fn multiply_both_by_variable_terms(id: String, _lang: &str) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = IntRange::without_ones_and_zero(-5, 5)?;
    let mut t1: Term = (coef_range.random(), unknown).into();
    let mut t3: Term = (coef_range.random(), unknown).into();
    let mut t2: Term = coef_range.random().into();
    let mut t4: Term = coef_range.random().into();
    Term::assert_one_positive(&mut t1, &mut t2);
    Term::assert_one_positive(&mut t3, &mut t4);
    let factor_1: Term = (coef_range.random(), unknown).into();
    let factor_2: Term = (coef_range.random(), unknown).into();

    let mut exp_1: Polynomial = vec![&t1, &t2].into();
    let mut exp_2: Polynomial = vec![&t3, &t4].into();
    exp_1.sort();
    exp_2.sort();
    let mult_1 = factor_1.clone() * exp_1.clone();
    let mult_2 = factor_2.clone() * exp_2.clone();

    let question = format!("${factor_1}({exp_1}) {factor_2:+}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) {factor_2:+}({exp_2}) = \\
         =&{mult_1}{mult_2:+} = {answer}$"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![2],
        combinations: 1,
    })
}

/// 2x(1 + y) - 3(x + y)
/// Difficulty: 8
#[problem]
fn mixing_variables(id: String, _lang: &str) -> Result<Problem> {
    let (unknown1, unknown2) = symbols::get_two_unknowns()?;
    let num_range = IntRange::without_zero(-5, 5)?;
    let factor1 = num_range.positive();
    let factor2 = num_range.negative();
    let constant = num_range.random();
    let coef1 = num_range.random();
    let coef2 = num_range.random();
    let coef3 = num_range.random();

    let factor1_term: Term = (factor1, unknown1).into();
    let factor2_term: Term = (factor2, unknown2).into();
    let mut term1: Term = constant.into();
    let mut term2: Term = (coef1, unknown2).into();
    let mut term3: Term = (coef2, unknown1).into();
    let mut term4: Term = (coef3, unknown2).into();
    Term::assert_one_positive(&mut term1, &mut term2);
    Term::assert_one_positive(&mut term3, &mut term4);

    let mut exp1: Polynomial = vec![&term1, &term2].into();
    let mut exp2: Polynomial = vec![&term3, &term4].into();
    exp1.sort();
    exp2.sort();

    let question = format!("${factor1_term}({exp1}) {factor2_term:+}({exp2})$");
    let mult1 = factor1_term.clone() * exp1.clone();
    let mult2 = factor2_term.clone() * exp2.clone();
    let answer = (mult1.clone() + mult2.clone()).simplify();
    let solution = format!(
        "$&{factor1_term}({exp1}) {factor2_term:+}({exp2}) = \\
         =&{mult1} {mult2:+} = \\
         =& {answer}$"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![factor1, factor2],
        combinations: num_range.len(),
    })
}

/// x^2(1 - y) + 3x(y - 1) - y(3x + 1)
/// Difficulty: 9
#[problem]
fn mixing_variables_and_exponents(id: String, _lang: &str) -> Result<Problem> {
    let (unknown1, unknown2) = symbols::get_two_unknowns()?;
    let num_range = IntRange::without_zero(-5, 5)?;
    let factor1 = num_range.positive();
    let factor2 = num_range.random();
    let factor3 = num_range.random();
    let constant = num_range.random();
    let coef1 = num_range.random();
    let coef2 = num_range.random();
    let coef3 = num_range.random();
    let coef4 = num_range.random();
    let coef5 = num_range.random();

    let factor1_term: Term = (factor1, (unknown1, 2)).into();
    let factor2_term: Term = (factor2, unknown1).into();
    let factor3_term: Term = (factor3, unknown2).into();
    let mut term1: Term = constant.into();
    let mut term2: Term = (coef1, unknown2).into();
    let mut term3: Term = (coef2, unknown2).into();
    let mut term4: Term = (coef3, unknown1).into();
    let mut term5: Term = (coef4, unknown1).into();
    let mut term6: Term = coef5.into();
    Term::assert_one_positive(&mut term1, &mut term2);
    Term::assert_one_positive(&mut term3, &mut term4);
    Term::assert_one_positive(&mut term5, &mut term6);

    let mut exp1: Polynomial = vec![&term1, &term2].into();
    let mut exp2: Polynomial = vec![&term3, &term4].into();
    let mut exp3: Polynomial = vec![&term5, &term6].into();
    exp1.sort();
    exp2.sort();
    exp3.sort();

    let question =
        format!("${factor1_term}({exp1}) {factor2_term:+}({exp2}) {factor3_term:+}({exp3})$");
    let mult1 = factor1_term.clone() * exp1.clone();
    let mult2 = factor2_term.clone() * exp2.clone();
    let mult3 = factor3_term.clone() * exp3.clone();
    let answer = (mult1.clone() + mult2.clone() + mult3.clone()).simplify();
    let solution = format!(
        "$&{factor1_term}({exp1}) {factor2_term:+}({exp2}) {factor3_term:+}({exp3}) = \\
         =&{mult1} {mult2:+} {mult3:+}=\\
        =&{answer}$"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![factor1, factor2, factor3],
        combinations: num_range.len() * 2,
    })
}
