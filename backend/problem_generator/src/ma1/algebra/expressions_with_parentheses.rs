use anyhow::Result;
use macros::problem;
use math::{Number, Polynomial, Term, num_gen, symbols};
use types::{lang::Language, problems::Problem};
use typst_writer::{self, formatting::parentheses};

/// 3(x+1)
/// Difficulty: 0
#[problem]
fn positive_integer_mult(name: String, _lang: &Language) -> Result<Problem> {
    let (factor, f_range) = num_gen::integer().range(2, 5).and_random();
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();

    let t1 = Term::from(unknown);
    let t2 = Term::from_num(constant);
    let exp = Polynomial::from_terms(&[&t1, &t2]);

    let question = format!("${factor}({exp})$");
    let answer = (factor * exp.clone()).simplify();
    let solution = format!(
        "${factor}({exp}) = colored({factor} dot) {unknown} {sign} colored({factor} dot) {abs_const} = {answer}$",
        sign = if constant > 0 { "+" } else { "-" },
        abs_const = constant.abs()
    );

    Ok(Problem {
        name,
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
fn negative_integer_mult(name: String, _lang: &Language) -> Result<Problem> {
    let (factor, f_range) = num_gen::integer().range(-5, -2).and_random();
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = num_gen::integer().range(-7, -1).and_random();

    let t1 = Term::from(unknown);
    let t2 = Term::from_num(constant);
    let exp = Polynomial::from_terms(&[&t1, &t2]);

    let question = format!("${factor}({exp})$");
    let answer = factor * exp.clone();
    let simplified = answer.simplify();
    let solution = format!(
        "$&{factor}({exp}) = colored({factor_p} dot) {unknown} + colored({factor_p} dot) {const_p} =\\
            =&{answer} = {simplified}$",
        factor_p = parentheses(&factor),
        const_p = parentheses(&constant),
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len(),
    })
}

/// 3(2a-4)
/// Difficulty: 2
#[problem]
fn with_coefficient_on_variable(name: String, _lang: &Language) -> Result<Problem> {
    let (factor, f_range) = num_gen::integer().range(2, 5).and_random();
    let unknown = symbols::get_unknown()?;
    let coef = num_gen::integer().range(2, 5).random();
    let (constant, c_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();

    let t1 = Term::from_num_and_vars(coef, unknown);
    let t2 = Term::from_num(constant);
    let exp = Polynomial::from_terms(&[&t1, &t2]);

    let question = format!("${factor}({exp})$");
    let answer = (factor * exp.clone()).simplify();
    let solution = format!(
        "${factor}({exp}) = colored({factor} dot) {t1} {sign} colored({factor} dot) {abs_const} = {answer}$",
        sign = if constant > 0 { "+" } else { "-" },
        abs_const = constant.abs()
    );

    Ok(Problem {
        name,
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
fn multiply_by_variable(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let (constant, c_range) = num_gen::integer().range(-7, 7).exclude(0).and_random();

    let factor = Term::from(unknown);
    let t1 = Term::from(unknown);
    let t2 = Term::from_num(constant);
    let exp = Polynomial::from_terms(&[&t1, &t2]);

    let question = format!("${factor}({exp})$");
    let answer = (factor.clone() * exp.clone()).simplify();
    let solution = format!(
        "${factor}({exp}) = colored({factor} dot) {t1} {sign} colored({factor} dot) {abs_const} = {answer}$",
        sign = if constant > 0 { "+" } else { "-" },
        abs_const = constant.abs()
    );

    Ok(Problem {
        name,
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
fn add_parentheses(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_range = num_gen::integer().range(-9, 9).exclude(0);
    let const_1 = const_range.random();
    let const_2 = const_range.random();

    let mut term_var_1 = Term::from_num_and_vars(coef_1, unknown);
    let mut term_const_1 = Term::from_num(const_1);
    let mut term_var_2 = Term::from_num_and_vars(coef_2, unknown);
    let mut term_const_2 = Term::from_num(const_2);
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);
    let exp_1 = Polynomial::from_terms(&[&term_var_1, &term_const_1]).sorted();
    let exp_2 = Polynomial::from_terms(&[&term_var_2, &term_const_2]).sorted();

    let question = format!("$({exp_1}) + ({exp_2})$");
    let answer = (exp_1.clone() + exp_2.clone()).simplify();
    let solution = format!(
        "$&({exp_1}) + ({exp_2}) = {exp_1} + {exp_2} = \\ 
            = &{term_var_1} {term_var_2:+} {term_const_1:+}{term_const_2:+} = {answer}$",
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, const_1],
        combinations: coef_range.len() * const_range.len(),
    })
}

/// (2x + 1) - (3x - 4)
/// Difficulty: 3
#[problem]
fn subtract_parentheses(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_range = num_gen::integer().range(-9, 9).exclude(0);
    let const_1 = const_range.random();
    let const_2 = const_range.random();

    let mut term_var_1 = Term::from_num_and_vars(coef_1, unknown);
    let mut term_const_1 = Term::from_num(const_1);
    let mut term_var_2 = Term::from_num_and_vars(coef_2, unknown);
    let mut term_const_2 = Term::from_num(const_2);
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);

    let exp_1 = Polynomial::from_terms(&[&term_var_1, &term_const_1]).sorted();
    let exp_2 = Polynomial::from_terms(&[&term_var_2, &term_const_2]).sorted();

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
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, const_1],
        combinations: coef_range.len() * const_range.len(),
    })
}

/// -4(3 - 4x)
/// Difficulty: 2
#[problem]
fn negative_factor_and_coef(name: String, _lang: &Language) -> Result<Problem> {
    let (factor, f_range) = num_gen::integer().range(-8, -2).and_random();
    let unknown = symbols::get_unknown()?;
    let coef = num_gen::integer().range(-5, -2).random();
    let (constant, c_range) = num_gen::integer().range(2, 9).and_random();

    let t1 = Term::from_num(constant);
    let t2 = Term::from_num_and_vars(coef, unknown);
    let exp = Polynomial::from_terms(&[&t1, &t2]);

    let question = format!("${factor}({exp})$");
    let answer = factor * exp.clone();
    let simplified_answer = answer.simplify();
    let solution = format!(
        "$&{factor}({exp}) = colored({factor_p} dot) {t1_p} + colored({factor_p} dot) {t2_p} =\\
            =&{answer} = {simplified_answer}$",
        factor_p = parentheses(&factor),
        t1_p = parentheses(&t1),
        t2_p = parentheses(&t2),
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${simplified_answer}$"),
        solution,
        identifiers: vec![factor, constant],
        combinations: f_range.len() * c_range.len(),
    })
}

/// 3 - (2x - 1)
/// Difficulty: 3
#[problem]
fn const_minus_parenthesis(name: String, _lang: &Language) -> Result<Problem> {
    let (initial_const, i_range) = num_gen::integer().range(1, 8).and_random();
    let unknown = symbols::get_unknown()?;
    let coef = num_gen::integer().range(2, 6).random();
    let (constant, c_range) = num_gen::integer().range(-7, -1).and_random();

    let t1 = Term::from_num(initial_const);
    let t2 = Term::from_num_and_vars(coef, unknown);
    let t3 = Term::from_num(constant);

    let exp_1 = Polynomial::from(t1);
    let exp_2 = Polynomial::from_terms(&[&t2, &t3]);

    let question = format!("${exp_1}-({exp_2})$");
    let answer = (exp_1.clone() - exp_2.clone()).simplify();
    let solution = format!(
        "$&{exp_1}-({exp_2}) = {exp_1} {t2_m:+} {t3_m:+} = \\
            = &{answer}$",
        t2_m = -&t2,
        t3_m = -&t3,
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![initial_const, constant],
        combinations: i_range.len() * c_range.len(),
    })
}

/// 2x - (7x - 1)
/// Difficulty: 3
#[problem]
fn var_term_minus_parenthesis(name: String, _lang: &Language) -> Result<Problem> {
    let (initial, i_range) = num_gen::integer().range(1, 4).and_random();
    let unknown = symbols::get_unknown()?;
    let coef = num_gen::integer().range(5, 10).random();
    let (constant, c_range) = num_gen::integer().range(-7, -1).and_random();

    let t1 = Term::from_num_and_vars(initial, unknown);
    let t2 = Term::from_num_and_vars(coef, unknown);
    let t3 = Term::from_num(constant);

    let exp_1 = Polynomial::from(t1);
    let exp_2 = Polynomial::from_terms(&[&t2, &t3]);

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
        name,
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
fn multiply_and_add(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let const_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_2 = coef_range.random();
    let factor_range = num_gen::integer().range(2, 5);
    let factor_1 = factor_range.random();
    let factor_2 = factor_range.random();

    let mut term_var_1 = Term::from_num_and_vars(coef_1, unknown);
    let mut term_const_1 = Term::from_num(const_1);
    let mut term_var_2 = Term::from_num_and_vars(coef_2, unknown);
    let mut term_const_2 = Term::from_num(const_2);
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);
    let exp_1 = Polynomial::from_terms(&[&term_var_1, &term_const_1]);
    let exp_2 = Polynomial::from_terms(&[&term_var_2, &term_const_2]);
    let mult_1 = factor_1 * exp_1.clone();
    let mult_2 = factor_2 * exp_2.clone();

    let question = format!("${factor_1}({exp_1}) + {factor_2}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) + {factor_2}({exp_2}) = \\ = &{mult_1} {mult_2:+} = {answer}$",
    );

    Ok(Problem {
        name,
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
fn multiply_first_and_subtract(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_range = num_gen::integer().range(-9, 9).exclude(0);
    let const_1 = const_range.random();
    let const_2 = const_range.random();
    let factor_range = num_gen::integer().range(2, 5);
    let factor_1 = factor_range.random();

    let mut term_var_1 = Term::from_num_and_vars(coef_1, unknown);
    let mut term_const_1 = Term::from_num(const_1);
    let mut term_var_2 = Term::from_num_and_vars(coef_2, unknown);
    let mut term_const_2 = Term::from_num(const_2);
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);

    let exp_1 = Polynomial::from_terms(&[&term_var_1, &term_const_1]).sorted();
    let exp_2 = Polynomial::from_terms(&[&term_var_2, &term_const_2]).sorted();
    let mult_1 = factor_1 * exp_1.clone();

    let question = format!("${factor_1}({exp_1}) - ({exp_2})$");
    let answer = (mult_1.clone() - exp_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) - ({exp_2}) = \\ = &{mult_1} {exp_2_m:+} = {answer}$",
        exp_2_m = -&exp_2,
    );

    Ok(Problem {
        name,
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
fn multiply_and_subtract(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-9, 9)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let const_range = num_gen::integer().range(-9, 9).exclude(0);
    let const_1 = const_range.random();
    let const_2 = const_range.random();
    let factor_range = num_gen::integer().range(2, 5);
    let factor_1 = factor_range.random();
    let factor_2 = factor_range.random();

    let mut term_var_1 = Term::from_num_and_vars(coef_1, unknown);
    let mut term_const_1 = Term::from_num(const_1);
    let mut term_var_2 = Term::from_num_and_vars(coef_2, unknown);
    let mut term_const_2 = Term::from_num(const_2);
    Term::assert_one_positive(&mut term_var_1, &mut term_const_1);
    Term::assert_one_positive(&mut term_var_2, &mut term_const_2);

    let exp_1 = Polynomial::from_terms(&[&term_var_1, &term_const_1]).sorted();
    let exp_2 = Polynomial::from_terms(&[&term_var_2, &term_const_2]).sorted();
    let mult_1 = factor_1 * exp_1.clone();
    let mult_2 = -factor_2 * exp_2.clone();

    let question = format!("${factor_1}({exp_1}) - {factor_2}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) - {factor_2}({exp_2}) = \\ = &{mult_1} {mult_2:+} = {answer}$",
    );

    Ok(Problem {
        name,
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
fn multiply_by_variable_term(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let constant = num_gen::integer().range(1, 7).random();
    let coef_range = num_gen::integer()
        .range(-5, 9)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.positive();
    let coef_2 = coef_range.random();

    let factor = Term::from_num_and_vars(coef_1, unknown);
    let t1 = Term::from_num_and_vars(coef_2, unknown);
    let t2 = Term::from(constant);
    let exp = Polynomial::from_terms(&[&t1, &t2]).sorted();

    let question = format!("${factor}({exp})$");
    let answer = (factor.clone() * exp.clone()).simplify();
    let solution = format!(
        "$&{factor}({exp}) = colored({factor} dot) {e1} {sign} colored({factor} dot) {e2_abs} = \\ =&{answer}$",
        e1 = exp.terms[0],
        e2_abs = exp.terms[1].abs(),
        sign = if exp.terms[1].coefficient > Number::from(0) {
            "+"
        } else {
            "-"
        }
    );

    Ok(Problem {
        name,
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
fn one_variable_one_constant(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-5, 5)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let mut t1 = Term::from_num_and_vars(coef_1, unknown);
    let mut t3 = Term::from_num_and_vars(coef_2, unknown);
    let mut t2 = Term::from_num(coef_range.random());
    let mut t4 = Term::from_num(coef_range.random());
    Term::assert_one_positive(&mut t1, &mut t2);
    Term::assert_one_positive(&mut t3, &mut t4);

    let factor = coef_range.random();
    let factor_term = Term::from_num(factor);
    let variable_factor = Term::from(unknown);

    let exp_1 = Polynomial::from_terms(&[&t1, &t2]).sorted();
    let exp_2 = Polynomial::from_terms(&[&t3, &t4]).sorted();
    let mult_1 = variable_factor.clone() * exp_1.clone();
    let mult_2 = factor_term.clone() * exp_2.clone();

    let question = format!("${variable_factor}({exp_1}) {factor_term:+}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{variable_factor}({exp_1}) {factor_term:+}({exp_2}) = \\
         =&{mult_1}{mult_2:+} = {answer}$"
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, factor],
        combinations: coef_range.len().pow(2),
    })
}

/// 3(3x + 1) - x(2 + x)
/// Difficulty: 6
#[problem]
fn one_constant_one_variable(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-5, 5)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let mut t1 = Term::from_num_and_vars(coef_1, unknown);
    let mut t3 = Term::from_num_and_vars(coef_2, unknown);
    let mut t2 = Term::from_num(coef_range.random());
    let mut t4 = Term::from_num(coef_range.random());
    Term::assert_one_positive(&mut t1, &mut t2);
    Term::assert_one_positive(&mut t3, &mut t4);

    let factor = coef_range.random();
    let factor_term = Term::from_num(factor);
    let variable_factor = Term::from(unknown);

    let exp_1 = Polynomial::from_terms(&[&t1, &t2]).sorted();
    let exp_2 = Polynomial::from_terms(&[&t3, &t4]).sorted();
    let mult_1 = variable_factor.clone() * exp_1.clone();
    let mult_2 = factor_term.clone() * exp_2.clone();

    let question = format!("${factor_term}({exp_1}) {variable_factor:+}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor_term}({exp_1}) {variable_factor:+}({exp_2}) = \\
         =&{mult_1}{mult_2:+} = {answer}$"
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, factor],
        combinations: coef_range.len().pow(2),
    })
}

/// 3x(3x + 1) - 2x(2 + x)
/// Difficulty: 7
#[problem]
fn multiply_both_by_variable_terms(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let coef_range = num_gen::integer()
        .range(-5, 5)
        .exclude_multiple(&[-1, 0, 1]);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let mut t1 = Term::from_num_and_vars(coef_1, unknown);
    let mut t3 = Term::from_num_and_vars(coef_2, unknown);
    let mut t2 = Term::from_num(coef_range.random());
    let mut t4 = Term::from_num(coef_range.random());
    Term::assert_one_positive(&mut t1, &mut t2);
    Term::assert_one_positive(&mut t3, &mut t4);
    let factor_1 = Term::from_num_and_vars(coef_range.random(), unknown);
    let factor_2 = Term::from_num_and_vars(coef_range.random(), unknown);

    let exp_1 = Polynomial::from_terms(&[&t1, &t2]).sorted();
    let exp_2 = Polynomial::from_terms(&[&t3, &t4]).sorted();
    let mult_1 = factor_1.clone() * exp_1.clone();
    let mult_2 = factor_2.clone() * exp_2.clone();

    let question = format!("${factor_1}({exp_1}) {factor_2:+}({exp_2})$");
    let answer = (mult_1.clone() + mult_2.clone()).simplify();
    let solution = format!(
        "$&{factor_1}({exp_1}) {factor_2:+}({exp_2}) = \\
         =&{mult_1}{mult_2:+} = {answer}$"
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef_1, coef_2],
        combinations: coef_range.len().pow(2),
    })
}

/// 2x(1 + y) - 3(x + y)
/// Difficulty: 8
#[problem]
fn mixing_variables(name: String, _lang: &Language) -> Result<Problem> {
    let (unknown1, unknown2) = symbols::get_two_unknowns()?;
    let num_range = num_gen::integer().range(-5, 5).exclude(0);
    let factor1 = num_range.positive();
    let factor2 = num_range.negative();
    let constant = num_range.random();
    let coef1 = num_range.random();
    let coef2 = num_range.random();
    let coef3 = num_range.random();

    let factor1_term = Term::from_num_and_vars(factor1, unknown1);
    let factor2_term = Term::from_num_and_vars(factor2, unknown2);
    let mut t1 = Term::from_num(constant);
    let mut t2 = Term::from_num_and_vars(coef1, unknown2);
    let mut t3 = Term::from_num_and_vars(coef2, unknown1);
    let mut t4 = Term::from_num_and_vars(coef3, unknown2);
    Term::assert_one_positive(&mut t1, &mut t2);
    Term::assert_one_positive(&mut t3, &mut t4);

    let exp_1 = Polynomial::from_terms(&[&t1, &t2]).sorted();
    let exp_2 = Polynomial::from_terms(&[&t3, &t4]).sorted();

    let question = format!("${factor1_term}({exp_1}) {factor2_term:+}({exp_2})$");
    let mult1 = factor1_term.clone() * exp_1.clone();
    let mult2 = factor2_term.clone() * exp_2.clone();
    let answer = (mult1.clone() + mult2.clone()).simplify();
    let solution = format!(
        "$&{factor1_term}({exp_1}) {factor2_term:+}({exp_2}) = \\
         =&{mult1} {mult2:+} = \\
         =& {answer}$"
    );

    Ok(Problem {
        name,
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
fn mixing_variables_and_exponents(name: String, _lang: &Language) -> Result<Problem> {
    let (unknown1, unknown2) = symbols::get_two_unknowns()?;
    let num_range = num_gen::integer().range(-5, 5).exclude(0);
    let factor1 = num_range.positive();
    let factor2 = num_range.random();
    let factor3 = num_range.random();
    let constant = num_range.random();
    let coef1 = num_range.random();
    let coef2 = num_range.random();
    let coef3 = num_range.random();
    let coef4 = num_range.random();
    let coef5 = num_range.random();

    let factor1_term = Term::from_num_and_vars(factor1, (unknown1, 2));
    let factor2_term = Term::from_num_and_vars(factor2, unknown1);
    let factor3_term = Term::from_num_and_vars(factor3, unknown2);
    let mut term1 = Term::from_num(constant);
    let mut term2 = Term::from_num_and_vars(coef1, unknown2);
    let mut term3 = Term::from_num_and_vars(coef2, unknown2);
    let mut term4 = Term::from_num_and_vars(coef3, unknown1);
    let mut term5 = Term::from_num_and_vars(coef4, unknown1);
    let mut term6 = Term::from_num(coef5);
    Term::assert_one_positive(&mut term1, &mut term2);
    Term::assert_one_positive(&mut term3, &mut term4);
    Term::assert_one_positive(&mut term5, &mut term6);

    let exp1 = Polynomial::from_terms(&[&term1, &term2]).sorted();
    let exp2 = Polynomial::from_terms(&[&term3, &term4]).sorted();
    let exp3 = Polynomial::from_terms(&[&term5, &term6]).sorted();

    let question =
        format!("${factor1_term}({exp1}) {factor2_term:+}({exp2}) {factor3_term:+}({exp3})$");
    let mult1 = factor1_term.clone() * exp1.clone();
    let mult2 = factor2_term.clone() * exp2.clone();
    let mult3 = factor3_term.clone() * exp3.clone();
    let answer = (mult1.clone() + mult2.clone() + mult3.clone()).simplify();
    let solution = format!(
        "  #set text(size: 0.9em)
        $&{factor1_term}({exp1}) {factor2_term:+}({exp2}) {factor3_term:+}({exp3}) = \\
         =&{mult1} {mult2:+} {mult3:+}=\\
        =&{answer}$"
    );

    Ok(Problem {
        name,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![factor1, factor2, factor3],
        combinations: num_range.len() * 2,
    })
}
