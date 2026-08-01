use anyhow::Result;
use macros::problem;
use math::{MathDisplay, PolynomialVariable, Term, num_gen, symbols};
use registry::get_solution;
use types::{lang::Language, problems::Problem};

/// Calculate 9^(1/2)
/// Absolute difficulty: 5
/// Relative difficulty: 1
#[problem]
fn square_root(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer()
        .numbers(&[1, 4, 9, 16, 25, 36, 49, 64, 81, 100])
        .and_random();

    let question = format!("${base}^(1/2)$");
    let answer = base.sqrt();
    let solution = format!("${base}^(1/2) = sqrt({base}) = {answer}$");

    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}

/// Calculate 8^(1/3)
/// Absolute difficulty: 5
/// Relative difficulty: 2
#[problem]
fn cubic_root(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().numbers(&[1, 8, 27, 1000]).and_random();

    let question = format!("${base}^(1/3)$");
    let answer = base.root(3);
    let solution = format!("${base}^(1/3) = root(3, {base}) = {answer}$");

    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}

/// Simplify 7^(1/2) * 7^(1/2)
/// Absolute difficulty: 5
/// Relative difficulty: 3
#[problem]
fn fraction_times_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(2, 30).and_random();

    let question = format!("${base}^(1/2) dot {base}^(1/2)$");
    let answer = base.as_math();
    let solution =
        format!("${base}^(1/2) dot {base}^(1/2) = {base}^(1/2 + 1/2) = {base}^1 = {base}$");
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}

/// Simplify 7^(1/3) * 7^(1/3) * 7^(1/3)
/// Absolute difficulty: 5
/// Relative difficulty: 4
#[problem]
fn fraction_times_fraction_times_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(2, 30).and_random();

    let question = format!("${base}^(1/3) dot {base}^(1/3) dot {base}^(1/3)$");
    let answer = base.as_math();
    let solution = format!(
        "${base}^(1/3) dot {base}^(1/3) dot {base}^(1/3) = {base}^(1/3 + 1/3 + 1/3) = {base}^1 = {base}$"
    );
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base],
        combinations: base_range.len(),
    })
}

/// Simplify x^(1/3) * x^(2/3)
/// Absolute difficulty: 5
/// Relative difficulty: 4
#[problem]
fn multiply_variables_to_exponent_one(id: i32, _lang: Language) -> Result<Problem> {
    let (denominator, denom_range) = num_gen::integer().range(2, 9).and_random();
    // Randomize whether the first fraction has numerator 1 or the rest
    let first_numerator = num_gen::integer()
        .numbers(&[1, denominator.as_i32() - 1])
        .random();
    let second_numerator = denominator - first_numerator;
    let var = symbols::get_unknown()?;

    Ok(Problem {
        id,
        question: format!(
            "${var}^({first_numerator}/{denominator}) dot {var}^({second_numerator}/{denominator})$"
        ),
        answer: var.as_math(),
        solution: format!(
            "${var}^({first_numerator}/{denominator}) dot {var}^({second_numerator}/{denominator}) =
            {var}^({first_numerator}/{denominator} + {second_numerator}/{denominator}) = {var}^1 = {var}$"
        ),
        identifiers: vec![denominator],
        combinations: denom_range.len(),
    })
}

/// Simplify (7^(1/2))^2
/// Absolute difficulty: 5
/// Relative difficulty: 4
#[problem]
fn fraction_power_integer(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(2, 30).and_random();
    let (exponent, exp_range) = num_gen::integer().range(2, 9).and_random();

    let question = format!("$({base}^(1/{exponent}))^{exponent}$");
    let answer = base.as_math();
    let solution = format!(
        "$({base}^(1/{exponent}))^{exponent} = {base}^(1/{exponent} dot {exponent}) = 
        {base}^({exponent}/{exponent}) = {base}^1 = {base}$"
    );
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base, exponent],
        combinations: base_range.len() * exp_range.len(),
    })
}

/// Simplify (7^2)^(1/2)
/// Absolute difficulty: 5
/// Relative difficulty: 4
#[problem]
fn integer_power_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(2, 30).and_random();
    let (exponent, exp_range) = num_gen::integer().range(2, 9).and_random();

    let question = format!("$({base}^{exponent})^(1/{exponent})$");
    let answer = base.as_math();
    let solution = format!(
        "$({base}^{exponent})^(1/{exponent}) = {base}^({exponent} dot 1/{exponent}) = 
        {base}^({exponent}/{exponent}) = {base}^1 = {base}$"
    );
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base, exponent],
        combinations: base_range.len() * exp_range.len(),
    })
}

/// Simplify (7^8)^(1/2)
/// Absolute difficulty: 5
/// Relative difficulty: 4
#[problem]
fn large_integer_power_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(4, 9).and_random();
    // To ensure an integer answer, we work backwards
    let (final_exponent, exp_range) = num_gen::integer().range(4, 9).and_random();
    let (frac_exponent, frac_range) = num_gen::integer().range(2, 5).and_random();
    let initial_exponent = final_exponent * frac_exponent;

    let question = format!("$({base}^{initial_exponent})^(1/{frac_exponent})$");
    let answer = format!("${base}^{final_exponent}$");
    let solution = format!(
        "$({base}^{initial_exponent})^(1/{frac_exponent}) = {base}^({initial_exponent} dot 1/{frac_exponent}) = 
        {base}^({initial_exponent}/{frac_exponent}) = {base}^{final_exponent}$"
    );
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base, final_exponent, frac_exponent],
        combinations: base_range.len() * exp_range.len() * frac_range.len(),
    })
}

/// Simplify (7^(1/2))^12
/// Absolute difficulty: 5
/// Relative difficulty: 4
#[problem]
fn fraction_power_large_integer(id: i32, _lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(4, 9).and_random();
    // To ensure an integer answer, we work backwards
    let (final_exponent, exp_range) = num_gen::integer().range(4, 9).and_random();
    let (frac_exponent, frac_range) = num_gen::integer().range(2, 5).and_random();
    let initial_exponent = final_exponent * frac_exponent;

    let question = format!("$({base}^(1/{frac_exponent}))^{initial_exponent}$");
    let answer = format!("${base}^{final_exponent}$");
    let solution = format!(
        "$({base}^(1/{frac_exponent}))^{initial_exponent} = {base}^(1/{frac_exponent} dot {initial_exponent}) = 
        {base}^({initial_exponent}/{frac_exponent}) = {base}^{final_exponent}$"
    );
    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![base, final_exponent, frac_exponent],
        combinations: base_range.len() * exp_range.len() * frac_range.len(),
    })
}

/// Simplify x^(3/2) * sqrt(x)
/// Absolute difficulty: 6
/// Relative difficulty: 5
#[problem]
fn multiply_half_and_sqrt(id: i32, _lang: Language) -> Result<Problem> {
    // The numerator in the fraction can be any odd number to make the answer an integer exponent
    let (numerator, num_range) = num_gen::integer().numbers(&[1, 3, 5, 7, 9]).and_random();
    let var = symbols::get_unknown()?;
    let final_exponent = (numerator + 1) / 2;
    let answer = PolynomialVariable::from((var, final_exponent));
    Ok(Problem {
        id,
        question: format!("${var}^({numerator}/2) dot sqrt({var})$"),
        answer: answer.as_math(),
        solution: format!(
            "${var}^({numerator}/2) dot sqrt({var}) = {var}^({numerator}/2) dot {var}^(1/2) = 
            {var}^({numerator}/2 + 1/2) = {var}^({}/2) = {answer}$",
            numerator + 1,
        ),
        identifiers: vec![numerator],
        combinations: num_range.len(),
    })
}

/// Simplify sqrt(x) * x^(3/2)
/// Absolute difficulty: 6
/// Relative difficulty: 5
#[problem]
fn multiply_sqrt_and_half(id: i32, _lang: Language) -> Result<Problem> {
    // The numerator in the fraction can be any odd number to make the answer an integer exponent
    let (numerator, num_range) = num_gen::integer().numbers(&[1, 3, 5, 7, 9]).and_random();
    let var = symbols::get_unknown()?;
    let final_exponent = (numerator + 1) / 2;
    let answer = PolynomialVariable::from((var, final_exponent));
    Ok(Problem {
        id,
        question: format!("$sqrt({var}) dot {var}^({numerator}/2)$"),
        answer: answer.as_math(),
        solution: format!(
            "$sqrt({var}) dot {var}^({numerator}/2) = {var}^(1/2) dot {var}^({numerator}/2) = 
            {var}^(1/2 + {numerator}/2) = {var}^({}/2) = {answer}$",
            numerator + 1,
        ),
        identifiers: vec![numerator],
        combinations: num_range.len(),
    })
}

/// Simplify 3x^(1/2) * x^(5/2)
/// Absolute difficulty: 6
/// Relative difficulty: 5
#[problem]
fn multiply_variables_first_coefficient(id: i32, _lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(2, 9).and_random();
    let (first_exp, exp_range) = num_gen::integer().numbers(&[1, 3, 5, 7, 9]).and_random();
    let second_exp = exp_range.random();
    let var = symbols::get_unknown()?;

    let total_numerator = first_exp + second_exp;
    let final_exp = total_numerator / 2;
    let answer = coef * Term::from_var((var, final_exp));
    Ok(Problem {
        id,
        question: format!("${coef}{var}^({first_exp}/2) dot {var}^({second_exp}/2)$"),
        answer: answer.as_math(),
        solution: format!("${coef}{var}^({first_exp}/2) dot {var}^({second_exp}/2) = 
            {coef}{var}^({first_exp}/2 + {second_exp}/2) = {coef}{var}^({total_numerator}/2) = {answer}$"),
        identifiers: vec![coef, first_exp, second_exp],
        combinations: coef_range.len() * exp_range.len().pow(2),
    })
}

/// Simplify x^(1/2) * 3x^(5/2)
/// Absolute difficulty: 6
/// Relative difficulty: 5
#[problem]
fn multiply_variables_second_coefficient(id: i32, _lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(2, 9).and_random();
    let (first_exp, exp_range) = num_gen::integer().numbers(&[1, 3, 5, 7, 9]).and_random();
    let second_exp = exp_range.random();
    let var = symbols::get_unknown()?;

    let total_numerator = first_exp + second_exp;
    let final_exp = total_numerator / 2;
    let answer = coef * Term::from_var((var, final_exp));
    Ok(Problem {
        id,
        question: format!("${var}^({first_exp}/2) dot {coef}{var}^({second_exp}/2)$"),
        answer: answer.as_math(),
        solution: format!("${var}^({first_exp}/2) dot {coef}{var}^({second_exp}/2) = 
            {coef}{var}^({first_exp}/2 + {second_exp}/2) = {coef}{var}^({total_numerator}/2) = {answer}$"),
        identifiers: vec![coef, first_exp, second_exp],
        combinations: coef_range.len() * exp_range.len().pow(2),
    })
}

/// Simplify 2x^(1/2) * 3x^(5/2)
/// Absolute difficulty: 6
/// Relative difficulty: 5
#[problem]
fn multiply_variables_both_coefficients(id: i32, lang: Language) -> Result<Problem> {
    let (first_coef, coef_range) = num_gen::integer().range(2, 9).and_random();
    let second_coef = coef_range.random();
    let (first_exp, exp_range) = num_gen::integer().numbers(&[1, 3, 5, 7, 9]).and_random();
    let second_exp = exp_range.random();
    let var = symbols::get_unknown()?;

    let total_coef = first_coef * second_coef;
    let total_numerator = first_exp + second_exp;
    let final_exp = total_numerator / 2;
    let answer = total_coef * Term::from_var((var, final_exp));
    let solution_text = get_solution(id, lang)?;
    Ok(Problem {
        id,
        question: format!("${first_coef}{var}^({first_exp}/2) dot {second_coef}{var}^({second_exp}/2)$"),
        answer: answer.as_math(),
        solution: format!("{solution_text} \\ $ {first_coef}{var}^({first_exp}/2) dot {second_coef}{var}^({second_exp}/2) = 
            {total_coef}{var}^({first_exp}/2 + {second_exp}/2) = {total_coef}{var}^({total_numerator}/2) = {answer} $"),
        identifiers: vec![first_coef, second_coef, first_exp, second_exp],
        combinations: coef_range.len().pow(2) * exp_range.len().pow(2),
    })
}

/// Simplify: Nx^n / sqrt(n)
/// Absolute difficulty: 7
/// Relative difficulty: 6
#[problem]
fn power_divided_by_sqrt(id: i32, _lang: Language) -> Result<Problem> {
    let coef = num_gen::integer().range(2, 9).random();
    let (exp, exp_range) = num_gen::integer().range(2, 5).and_random();
    let var = symbols::get_unknown()?;

    let double_exp = 2 * exp;
    let answer_exp = (double_exp - 1) / 2;

    Ok(Problem {
        id,
        question: format!("#block($ ({coef}{var}^{exp})/sqrt({var}) $)"),
        answer: format!("${coef}{var}^({answer_exp})$"),
        solution: format!("$({coef}{var}^{exp})/sqrt({var}) = ({coef}{var}^{exp})/{var}^(1/2) = 
            {coef}{var}^({exp} - 1/2) = {coef}{var}^({double_exp}/2 - 1/2) = {coef}{var}^({answer_exp})$"),
        identifiers: vec![exp],
        combinations: exp_range.len()
    })
}

/// x^(1/2) * x^(1/3)
/// Absolute difficulty: 7
/// Relative difficulty: 6
#[problem]
fn variable_fraction_times_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (denom_1, denom_1_range) = num_gen::integer().numbers(&[2, 4, 8]).and_random();
    let (denom_2, denom_2_range) = num_gen::integer().numbers(&[3, 5, 7]).and_random();
    let frac_1 = 1 / denom_1;
    let frac_2 = 1 / denom_2;
    let extend_1 = frac_1.extend(denom_2);
    let extend_2 = frac_2.extend(denom_1);
    let total_frac = frac_1 + frac_2;
    let var = symbols::get_unknown()?;

    Ok(Problem {
        id,
        question: format!("${var}^({frac_1}) dot {var}^({frac_2})$"),
        answer: format!("${var}^({total_frac})$"),
        solution: format!(
            "${var}^({frac_1}) dot {var}^({frac_2}) = {var}^({frac_1} + {frac_2}) = 
            {var}^({extend_1} + {extend_2}) = {var}^({total_frac})$"
        ),
        identifiers: vec![denom_1, denom_2],
        combinations: denom_1_range.len() * denom_2_range.len(),
    })
}
