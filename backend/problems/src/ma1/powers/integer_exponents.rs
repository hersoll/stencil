use crate::shuffle;
use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay, Number, Term, formatting,
    num_gen::{self, NumberGenerator},
    symbols,
};
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    }))
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, exp2],
        combinations: exp1_range.len() * exp2_range.len(),
    }))
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    }))
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: exp1,
        combinations: exp1_range,
    }))
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![exp1, base],
        combinations: exp1_range.len() * base_range.len(),
    }))
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: exp1,
        combinations: exp1_range,
    }))
}

/// x * x
/// Absolute difficulty: 3
/// Relative difficulty: 5
#[problem]
fn x_times_x(id: i32, _lang: Language) -> Result<Problem> {
    let var = symbols::get_unknown()?;
    let solution = Solution::inline()
        .write(format!("{var} dot {var}"))
        .equals(format!("{var}^1 dot {var}^1"))
        .equals(format!("{var}^(1+1)"))
        .equals(var * var)
        .to_string();
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${var} dot {var}$"),
        answer: var * var,
        solution,
        identifiers: 1,
        combinations: 1,
    }))
}

/// x * 2x
/// Absolute difficulty: 3
/// Relative difficulty: 6
#[problem]
fn x_times_coef_x(id: i32, _lang: Language) -> Result<Problem> {
    let (mut coef_1, coef_range) = num_gen::integer().range(2, 9).and_random();
    let mut coef_2 = Number::Integer(1);
    shuffle(&mut coef_1, &mut coef_2);

    let var = symbols::get_unknown()?;
    let t1 = coef_1 * var;
    let t2 = coef_2 * var;
    let total_coef = coef_1 * coef_2;
    let answer = &t1 * &t2;

    let solution = Solution::inline()
        .write(format!("{t1} dot {t2}"))
        .equals(format!("{total_coef} dot {var} dot {var}"))
        .equals(&answer)
        .to_string();
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${t1} dot {t2}$"),
        answer,
        solution,
        identifiers: vec![coef_1, coef_2],
        combinations: coef_range.len() * 2,
    }))
}

/// 3x * 2x
/// Absolute difficulty: 3
/// Relative difficulty: 7
#[problem]
fn coef_x_times_coef_x(id: i32, _lang: Language) -> Result<Problem> {
    let (coef_1, coef_range) = num_gen::integer().range(2, 9).and_random();
    let coef_2 = coef_range.random();

    let var = symbols::get_unknown()?;
    let t1 = coef_1 * var;
    let t2 = coef_2 * var;
    let answer = &t1 * &t2;

    let solution = Solution::inline()
        .write(format!("{t1} dot {t2}"))
        .equals(format!("{coef_1} dot {coef_2} dot {var} dot {var}"))
        .equals(&answer)
        .to_string();
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${t1} dot {t2}$"),
        answer,
        solution,
        identifiers: vec![coef_1, coef_2],
        combinations: coef_range.len().pow(2),
    }))
}

/// x * x^2
/// Absolute difficulty: 3
/// Relative difficulty: 7
#[problem]
fn x_squared_times_x(id: i32, _lang: Language) -> Result<Problem> {
    let var = symbols::get_unknown()?;
    let mut exp_1 = 1;
    let mut exp_2 = 2;
    shuffle(&mut exp_1, &mut exp_2);
    let t1 = var.powi(exp_1);
    let t2 = var.powi(exp_2);
    let answer = &t1 * &t2;

    let solution = Solution::inline()
        .write(format!("{t1} dot {t2}"))
        .equals(format!("{var}^({exp_1}+{exp_2})"))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${t1} dot {t2}$"),
        answer,
        solution,
        identifiers: 1,
        combinations: 1,
    }))
}

/// x^2 * 2x
/// Absolute difficulty: 3
/// Relative difficulty: 8
#[problem]
fn x_squared_times_coef_x(id: i32, _lang: Language) -> Result<Problem> {
    let (mut coef_1, coef_range) = num_gen::integer().range(2, 9).and_random();
    let mut coef_2 = Number::Integer(1);
    shuffle(&mut coef_1, &mut coef_2);
    let mut exp_1 = Number::Integer(1);
    let mut exp_2 = Number::Integer(2);
    shuffle(&mut exp_1, &mut exp_2);

    let var = symbols::get_unknown()?;
    let var_1 = var.powi(exp_1);
    let var_2 = var.powi(exp_2);
    let t1 = coef_1 * var_1.clone();
    let t2 = coef_2 * var_2.clone();
    let total_coef = coef_1 * coef_2;
    let answer = &t1 * &t2;

    let solution = Solution::inline()
        .write(format!("{t1} dot {t2}"))
        .equals(format!("{total_coef} dot {var_1} dot {var_2}"))
        .equals(&answer)
        .to_string();
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${t1} dot {t2}$"),
        answer,
        solution,
        identifiers: vec![coef_1, coef_2],
        combinations: coef_range.len() * 2,
    }))
}

/// 3x^2 * 2x
/// Absolute difficulty: 3
/// Relative difficulty: 8
#[problem]
fn coef_x_squared_times_coef_x(id: i32, _lang: Language) -> Result<Problem> {
    let (coef_1, coef_range) = num_gen::integer().range(2, 9).and_random();
    let coef_2 = coef_range.random();
    let mut exp_1 = Number::Integer(1);
    let mut exp_2 = Number::Integer(2);
    shuffle(&mut exp_1, &mut exp_2);

    let var = symbols::get_unknown()?;
    let var_1 = var.powi(exp_1);
    let var_2 = var.powi(exp_2);
    let t1 = coef_1 * var_1.clone();
    let t2 = coef_2 * var_2.clone();
    let answer = &t1 * &t2;

    let solution = Solution::inline()
        .write(format!("{t1} dot {t2}"))
        .equals(format!("{coef_1} dot {coef_2} dot {var_1} dot {var_2}"))
        .equals(&answer)
        .to_string();
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${t1} dot {t2}$"),
        answer,
        solution,
        identifiers: vec![coef_1, coef_2],
        combinations: coef_range.len() * 2,
    }))
}

/// 3x^4 * 2x^3
/// Absolute difficulty: 3
/// Relative difficulty: 8
#[problem]
fn variable_term_times_variable_term(id: i32, _lang: Language) -> Result<Problem> {
    let (coef_1, coef_range) = num_gen::integer().range(2, 9).and_random();
    let coef_2 = coef_range.random();
    let (exp_1, exp_range) = num_gen::integer().range(2, 6).and_random();
    let exp_2 = exp_range.random();

    let var = symbols::get_unknown()?;
    let var_1 = var.powi(exp_1);
    let var_2 = var.powi(exp_2);
    let t1 = coef_1 * var_1.clone();
    let t2 = coef_2 * var_2.clone();
    let total_coef = coef_1 * coef_2;
    let answer = &t1 * &t2;

    let solution = Solution::inline()
        .write(format!("{t1} dot {t2}"))
        .equals(format!("{coef_1} dot {coef_2} dot {var_1} dot {var_2}"))
        .linebreak_equality()
        .equals(format!("{total_coef} dot {var}^({exp_1} + {exp_2})"))
        .equals(&answer)
        .to_string();
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${t1} dot {t2}$"),
        answer,
        solution,
        identifiers: vec![exp_1, exp_2],
        combinations: exp_range.len() * 2,
    }))
}

/// (5^3 * 5^6) / 5^2
/// Absolute difficulty: 4
/// Relative difficulty: 10
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base, exp1, exp2],
        combinations: exp1_range.len() * base_range.len() * exp2_range.len(),
    }))
}

/// (4x)^2
/// Absolute difficulty: 4
/// Relative difficulty: 10
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    }))
}

/// (2x)^3
/// Absolute difficulty: 4
/// Relative difficulty: 11
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    }))
}

/// (2x)^3 / 4x
/// Absolute difficulty: 5
/// Relative difficulty: 15
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    }))
}

/// (3x)^3 / 9x^2
/// Absolute difficulty: 6
/// Relative difficulty: 18
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

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![coef, exp],
        combinations: coef_range.len() * exp_range.len(),
    }))
}

/// Solve the equation 5^x * 5^3 = 5^12
/// Absolute difficulty: 4
/// Relative difficulty: 10
#[problem]
fn find_x_multiplication(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(2, 12).random();
    let (constant, const_range) = num_gen::integer().range(2, 7).and_random();
    let base = const_range.random();
    let rhs_num = answer + constant;
    let var = symbols::get_unknown()?;

    let lhs = format!("{base}^({var}) dot {base}^{constant}");
    let rhs = format!("{base}^({rhs_num})");
    let equation = format!("{lhs} = {rhs}");
    let solution = Solution::with_steps()
        .aligned(lhs, &rhs)
        .aligned(format!("{base}^({var} + {constant})"), rhs)
        .step(format!("cancel({base})"))
        .aligned(format!("{var} + {constant}"), rhs_num)
        .step(formatting::subtract_number(constant))
        .aligned(var, answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: equation.as_math(),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: constant,
        combinations: const_range.len(),
    }))
}

/// Solve the equation 5^x / 5^3 = 5^12
/// Absolute difficulty: 4
/// Relative difficulty: 10
#[problem]
fn find_x_division(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(5, 14).random();
    let (constant, const_range) = num_gen::integer().range(2, answer - 2).and_random();
    let base = const_range.random();
    let rhs_num = answer - constant;
    let var = symbols::get_unknown()?;

    let lhs = format!("{base}^({var}) / {base}^{constant}");
    let rhs = format!("{base}^({rhs_num})");
    let equation = format!("{lhs} = {rhs}");
    let solution = Solution::with_steps()
        .aligned(lhs, &rhs)
        .aligned(format!("{base}^({var} - {constant})"), rhs)
        .step(format!("cancel({base})"))
        .aligned(format!("{var} - {constant}"), rhs_num)
        .step(formatting::add_number(constant))
        .aligned(var, answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: equation.as_block_math(),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: constant,
        combinations: const_range.len(),
    }))
}

/// Solve the equation 5^2x * 5^3 = 5^12
/// Absolute difficulty: 5
/// Relative difficulty: 15
#[problem]
fn find_coef_x_multiplication(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(2, 8).random();
    let (constant, const_range) = num_gen::integer().range(2, 7).and_random();
    let (coef, coef_range) = num_gen::integer().range(2, 5).and_random();
    let base = const_range.random();
    let rhs_num = coef * answer + constant;
    let var = symbols::get_unknown()?;
    let var_term = coef * var;

    let lhs = format!("{base}^({var_term}) dot {base}^{constant}");
    let rhs = format!("{base}^({rhs_num})");
    let equation = format!("{lhs} = {rhs}");
    let solution = Solution::with_steps()
        .aligned(lhs, &rhs)
        .aligned(format!("{base}^({var_term} + {constant})"), rhs)
        .step(format!("cancel({base})"))
        .aligned(format!("{var_term} + {constant}"), rhs_num)
        .step(formatting::subtract_number(constant))
        .aligned(var_term, answer * coef)
        .step(formatting::divide_number(coef))
        .aligned(var, answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: equation.as_math(),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![coef, constant],
        combinations: const_range.len() * coef_range.len(),
    }))
}

/// Solve the equation 5^2x / 5^3 = 5^12
/// Absolute difficulty: 5
/// Relative difficulty: 15
#[problem]
fn find_coef_x_division(id: i32, _lang: Language) -> Result<Problem> {
    let (answer, answer_range) = num_gen::integer().range(2, 8).and_random();
    let (coef, coef_range) = num_gen::integer().range(2, 5).and_random();
    let constant = num_gen::integer().range(2, coef * answer - 2).random();
    let base = answer_range.random();
    let rhs_num = coef * answer - constant;
    let var = symbols::get_unknown()?;
    let var_term = coef * var;

    let lhs = format!("{base}^({var_term}) / {base}^{constant}");
    let rhs = format!("{base}^({rhs_num})");
    let equation = format!("{lhs} = {rhs}");
    let solution = Solution::with_steps()
        .aligned(lhs, &rhs)
        .aligned(format!("{base}^({var_term} - {constant})"), rhs)
        .step(format!("cancel({base})"))
        .aligned(format!("{var_term} - {constant}"), rhs_num)
        .step(formatting::add_number(constant))
        .aligned(var_term, answer * coef)
        .step(formatting::divide_number(coef))
        .aligned(var, answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: equation.as_block_math(),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: coef,
        combinations: coef_range,
    }))
}

/// ab^2 * a^3b^5
/// Absolute difficulty: 5
/// Relative difficulty: 15
#[problem]
fn multiply_two_variables(id: i32, _lang: Language) -> Result<Problem> {
    let (var1, var2) = symbols::get_two_unknowns()?;
    let exp_range = num_gen::integer().range(1, 5);
    let (exp_1, exp_2) = (exp_range.random(), exp_range.random());
    let (exp_3, exp_4) = (exp_range.random(), exp_range.random());
    let v1 = var1.powi(exp_1);
    let v2 = var2.powi(exp_2);
    let v3 = var1.powi(exp_3);
    let v4 = var2.powi(exp_4);
    let t1 = &v1 * &v2;
    let t2 = &v3 * &v4;
    let equation = format!("{t1} dot {t2}");
    let answer = t1 * t2;

    let solution = Solution::inline()
        .write(&equation)
        .equals(format!("{v1} dot {v3} dot {v2} dot {v4}"))
        .linebreak_equality()
        .equals(format!("{} dot {}", v1 * v3, v2 * v4))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: equation.as_math(),
        answer,
        solution,
        identifiers: vec![exp_1, exp_2],
        combinations: exp_range.len().pow(2),
    }))
}

/// Solve the equation 5^2x / 5^3 = 5^x * 5^4
/// Absolute difficulty: 6
/// Relative difficulty: 18
#[problem]
fn find_x_on_both_sides(id: i32, _lang: Language) -> Result<Problem> {
    let (answer, answer_range) = num_gen::integer().range(4, 8).and_random();
    let (left_coef, coef_range) = num_gen::integer().range(2, 5).and_random();
    let right_coef = 1;
    let left_constant = num_gen::integer()
        .range(2, (left_coef - right_coef) * answer - 2)
        .random();
    let right_constant = answer * (left_coef - right_coef) - left_constant;
    let base = answer_range.random();
    let var = symbols::get_unknown()?;
    let var_term = left_coef * var;
    let coef_diff = left_coef - right_coef;
    let simplified_term = coef_diff * var;

    let lhs = format!("{base}^({var_term}) / {base}^{left_constant}");
    let rhs = format!("{base}^{var} dot {base}^({right_constant})");
    let equation = format!("{lhs} = {rhs}");
    let mut solution = Solution::with_steps();
    solution
        .aligned(lhs, &rhs)
        .aligned(
            format!("{base}^({var_term} - {left_constant})"),
            format!("{base}^({var} + {right_constant})"),
        )
        .step(format!("cancel({base})"))
        .aligned(var_term.and(&-left_constant), var + right_constant)
        .step(formatting::subtract_term(&var))
        .aligned((simplified_term).and(&-left_constant), right_constant)
        .step(formatting::add_number(left_constant))
        .aligned(simplified_term, answer * coef_diff);
    if coef_diff > 1 {
        solution
            .step(formatting::divide_number(coef_diff))
            .aligned(var, answer);
    }

    Ok(Problem::from(ProblemParameters {
        id,
        question: equation.as_block_math(),
        answer: format!("${var} = {answer}$"),
        solution: solution.to_string(),
        identifiers: left_coef,
        combinations: coef_range,
    }))
}

/// 4ab^2 * 2a^3b^5
/// Absolute difficulty: 6
/// Relative difficulty: 18
#[problem]
fn multiply_two_variables_with_coefs(id: i32, _lang: Language) -> Result<Problem> {
    let (var1, var2) = symbols::get_two_unknowns()?;
    let coef_range = num_gen::integer().range(2, 6);
    let coef_1 = coef_range.random();
    let coef_2 = coef_range.random();
    let exp_range = num_gen::integer().range(1, 5);
    let (exp_1, exp_2) = (exp_range.random(), exp_range.random());
    let (exp_3, exp_4) = (exp_range.random(), exp_range.random());
    let v1 = var1.powi(exp_1);
    let v2 = var2.powi(exp_2);
    let v3 = var1.powi(exp_3);
    let v4 = var2.powi(exp_4);
    let t1 = coef_1 * (&v1 * &v2);
    let t2 = coef_2 * (&v3 * &v4);
    let equation = format!("{t1} dot {t2}");
    let answer = t1 * t2;

    let solution = Solution::inline()
        .write(&equation)
        .equals(format!(
            "{coef_1} dot {coef_2} dot {v1} dot {v3} dot {v2} dot {v4}"
        ))
        .linebreak_equality()
        .equals(format!(
            "{} dot {} dot {}",
            coef_1 * coef_2,
            v1 * v3,
            v2 * v4
        ))
        .equals(&answer)
        .to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question: equation.as_math(),
        answer,
        solution,
        identifiers: vec![exp_1, exp_2],
        combinations: exp_range.len().pow(2),
    }))
}
