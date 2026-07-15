use anyhow::Result;
use macros::problem;
use math::{Term, num_gen, symbols};
use types::{lang::Language, problems::Problem};

/// 5^4 * 5^2
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn simple_multiplication(id: i32, _lang: Language) -> Result<Problem> {
    let base = num_gen::integer().range(4, 9).random();
    let (exp1, exp1_range) = num_gen::integer().range(2, 6).and_random();
    let (exp2, exp2_range) = num_gen::integer().range(2, 6).and_random();
    let total_exp = exp1 + exp2;
    let question = format!("${base}^{exp1} dot {base}^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "${base}^{exp1} dot {base}^{exp2} = {base}^({exp1} + {exp2}) = {base}^{total_exp}$"
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    })
}

/// x^4 * x^2
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn simple_multiplication_variables(id: i32, _lang: Language) -> Result<Problem> {
    let base = symbols::get_unknown()?;
    let (exp1, exp1_range) = num_gen::integer().range(2, 6).and_random();
    let (exp2, exp2_range) = num_gen::integer().range(2, 6).and_random();
    let total_exp = exp1 + exp2;
    let question = format!("${base}^{exp1} dot {base}^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "${base}^{exp1} dot {base}^{exp2} = {base}^({exp1} + {exp2}) = {base}^{total_exp}$"
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    })
}

/// 5^4 / 5^2
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn simple_division(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(4, 9).and_random();
    let (exp1, exp1_range) = num_gen::integer().range(4, 10).and_random();
    let exp2 = num_gen::integer().range(2, exp1 - 2).random();
    let total_exp = exp1 - exp2;
    let question = format!("$display({base}^{exp1} / {base}^{exp2})$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "$display({base}^{exp1} / {base}^{exp2} = {base}^({exp1} - {exp2}) = {base}^{total_exp})$"
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    })
}

/// a^4 / a^2
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn simple_division_variables(id: i32, _lang: Language) -> Result<Problem> {
    let base = symbols::get_unknown()?;
    let (exp1, exp1_range) = num_gen::integer().range(4, 10).and_random();
    let exp2 = num_gen::integer().range(2, exp1 - 2).random();
    let total_exp = exp1 - exp2;
    let question = format!("$display({base}^{exp1} / {base}^{exp2})$");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "$display({base}^{exp1} / {base}^{exp2} = {base}^({exp1} - {exp2}) = {base}^{total_exp})$"
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1],
        combinations: exp1_range.len(),
    })
}
/// (5^4)^2
/// Absolute difficulty: 2
/// Relative difficulty: 3
#[problem]
fn double_exponentiation(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(2, 9).and_random();
    let (exp1, exp1_range) = num_gen::integer().range(2, 6).and_random();
    let exp2 = num_gen::integer().range(3, 6).random();
    let total_exp = exp1 * exp2;
    let question = format!("$({base}^{exp1})^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution =
        format!("$ ({base}^{exp1})^{exp2} = {base}^({exp1} dot {exp2}) = {base}^{total_exp} $");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    })
}

/// (x^4)^2
/// Absolute difficulty: 2
/// Relative difficulty: 4
#[problem]
fn double_exponentiation_variables(id: i32, _lang: Language) -> Result<Problem> {
    let base = symbols::get_unknown()?;
    let (exp1, exp1_range) = num_gen::integer().range(2, 6).and_random();
    let exp2 = num_gen::integer().range(3, 6).random();
    let total_exp = exp1 * exp2;
    let question = format!("$({base}^{exp1})^{exp2}$");
    let answer = format!("${base}^{total_exp}$");
    let solution =
        format!("$ ({base}^{exp1})^{exp2} = {base}^({exp1} dot {exp2}) = {base}^{total_exp} $");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1],
        combinations: exp1_range.len(),
    })
}

/// (5^3 * 5^6) / 5^2
/// Absolute difficulty: 3
/// Relative difficulty: 5
#[problem]
fn multiplication_and_division(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(3, 9).and_random();
    let (exp1, exp1_range) = num_gen::integer().range(5, 10).and_random();
    let (exp2, exp2_range) = num_gen::integer().range(2, 10).and_random();
    let exp3 = num_gen::integer()
        .range(2, exp1 + exp2 - 2)
        .exclude(exp1)
        .exclude(exp2)
        .random();
    let total_exp = exp1 + exp2 - exp3;
    let question = format!("$ ({base}^{exp1} dot {base}^{exp2})/{base}^{exp3} $");
    let answer = format!("${base}^{total_exp}$");
    let solution = format!(
        "$ ({base}^{exp1} dot {base}^{exp2})/{base}^{exp3} = {base}^{}/{base}^{exp3} = {base}^{total_exp} $",
        exp1 + exp2
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base, exp1, exp2],
        combinations: exp1_range.len() * base_range.len() * exp2_range.len(),
    })
}

/// (4x)^2
/// Absolute difficulty: 3
/// Relative difficulty: 6
#[problem]
fn variable_term_power_2(id: i32, _lang: Language) -> Result<Problem> {
    let variable = symbols::get_unknown()?;
    let (coef, coef_range) = num_gen::integer().range(2, 10).and_random();
    let (exp, exp_range) = num_gen::integer().number(2).and_random();
    let final_coef = coef.pow(exp);
    let question = format!("$({coef}{variable})^{exp}$");
    let answer = format!("${final_coef}{variable}^{exp}$");
    let solution = format!(
        "$ ({coef}{variable})^{exp} = {coef}^{exp}{variable}^{exp} = {final_coef}{variable}^{exp} $"
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    })
}

/// (2x)^3
/// Absolute difficulty: 3
/// Relative difficulty: 7
#[problem]
fn variable_term_power_3(id: i32, _lang: Language) -> Result<Problem> {
    let variable = symbols::get_unknown()?;
    let (coef, coef_range) = num_gen::integer().numbers(&[2, 3, 10]).and_random();
    let (exp, exp_range) = num_gen::integer().number(3).and_random();
    let final_coef = coef.pow(exp);
    let question = format!("$({coef}{variable})^{exp}$");
    let answer = format!("${final_coef}{variable}^{exp}$");
    let solution = format!(
        "$ ({coef}{variable})^{exp} = {coef}^{exp}{variable}^{exp} = {final_coef}{variable}^{exp} $"
    );

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    })
}

/// (2x)^3 / 4x
/// Absolute difficulty: 4
/// Relative difficulty: 8
#[problem]
fn variable_term_power_and_divide_x(id: i32, _lang: Language) -> Result<Problem> {
    let variable = symbols::get_unknown()?;
    let (coef, coef_range) = num_gen::integer().numbers(&[2, 3, 10]).and_random();
    let (exp, exp_range) = num_gen::integer().range(2, 3).and_random();
    let denom_coef = coef.pow(num_gen::integer().range(1, 2).random());
    let numerator_term = coef * Term::from_var(variable);
    let denominator_term = denom_coef * Term::from_var(variable);
    let exponentiated_coef = coef.pow(exp);
    let final_coef = exponentiated_coef / denom_coef;
    // Used in the final line of the solution to help pretty print coefs of 1
    let helper_term = final_coef * Term::from_var(variable);
    let answer = final_coef * Term::from_var((variable, exp - 1));
    let question = format!("$ ({numerator_term})^{exp} / ({denominator_term}) $");
    let solution = format!(
        "$ ({numerator_term})^{exp} / ({denominator_term}) = 
            ({coef}^{exp}{variable}^{exp}) / ({denominator_term}) = 
            ({exponentiated_coef}{variable}^{exp}) / ({denominator_term}) = $
        $
            = {helper_term}^({exp} - 1) = {answer}
        $"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    })
}

/// (3x)^3 / 9x^2
/// Absolute difficulty: 5
/// Relative difficulty: 9
#[problem]
fn variable_term_power_and_divide_x_squared(id: i32, _lang: Language) -> Result<Problem> {
    let variable = symbols::get_unknown()?;
    let (coef, coef_range) = num_gen::integer().numbers(&[2, 3, 10]).and_random();
    let (exp, exp_range) = num_gen::integer().range(2, 3).and_random();
    let denom_coef = coef.pow(num_gen::integer().range(1, 2).random());
    let numerator_term = coef * Term::from_var(variable);
    let denominator_term = denom_coef * Term::from_var((variable, 2));
    let exponentiated_coef = coef.pow(exp);
    let final_coef = exponentiated_coef / denom_coef;
    // Used in the final line of the solution to help pretty print coefs of 1
    let helper_term = final_coef * variable;
    let answer = final_coef * Term::from_var((variable, exp - 2));
    let question = format!("$ ({numerator_term})^{exp} / ({denominator_term}) $");
    let solution = format!(
        "$ ({numerator_term})^{exp} / ({denominator_term}) = 
            ({coef}^{exp}{variable}^{exp}) / ({denominator_term}) = 
            ({exponentiated_coef}{variable}^{exp}) / ({denominator_term}) = $
        $
            = {helper_term}^({exp} - 2) = {answer}
        $"
    );

    Ok(Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    })
}
