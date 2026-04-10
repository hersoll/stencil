use anyhow::Result;
use macros::problem;
use math::{Number, Polynomial, Term, num_gen, symbols};
use types::{lang::Language, problems::Problem};
use typst_writer::formatting::{
    self, StructuredSolution, divide_number, equation_solution, subtract_number, subtract_term,
};

/// 4x + 1 = 2x + 3
/// Difficulty: 1
#[problem]
fn two_positive_coefs_lhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(1, 6).random();
    let lhs_range = num_gen::integer().range(3, 9);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(1, lhs_coef - 2).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_const = num_gen::integer().range(1, 9).random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;

    let question = format!("${lhs_term}{lhs_const:+} = {rhs_term}{rhs_const:+}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_term}{lhs_const:+} &= {rhs_term}{rhs_const:+} \\ {sub_rhs} \\
        {total_var}{lhs_const:+} &= {rhs_const} \\ {sub_lhs} \\
        {total_var} &= {total_const} \\ {div_coef} \\
        {unknown} &= {answer} \\ \\",
        sub_rhs = formatting::subtract_term(&rhs_term),
        sub_lhs = formatting::subtract_number(lhs_const),
        total_var = lhs_term.clone() - rhs_term.clone(),
        div_coef = formatting::divide_number(lhs_coef - rhs_coef),
        total_const = rhs_const - lhs_const
    ));

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range.len(),
    })
}

/// 2x + 1 = 4x + 3
/// Difficulty: 1
#[problem]
fn two_positive_coefs_rhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(1, 6).random();
    let rhs_range = num_gen::integer().range(3, 9);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(1, rhs_coef - 2).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_const = num_gen::integer().range(1, 9).random();
    let lhs_const = (rhs_coef - lhs_coef) * answer + rhs_const;

    let question = format!("${lhs_term}{lhs_const:+} = {rhs_term}{rhs_const:+}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_term}{lhs_const:+} &= {rhs_term}{rhs_const:+} \\ {sub_lhs} \\
        {lhs_const} &= {total_var}{rhs_const:+} \\ {sub_rhs} \\
        {total_const} &= {total_var} \\ {div_coef} \\
        {answer} &= {unknown} \\ \\",
        sub_lhs = formatting::subtract_term(&lhs_term),
        sub_rhs = formatting::subtract_number(rhs_const),
        total_var = rhs_term.clone() - lhs_term.clone(),
        div_coef = formatting::divide_number(rhs_coef - lhs_coef),
        total_const = lhs_const - rhs_const
    ));

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range.len(),
    })
}

/// 4x + 1 = 4 - 2x
/// Difficulty: 2
#[problem]
fn one_negative_coef_lhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let lhs_range = num_gen::integer().range(1, 9);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(lhs_coef - 10, -1).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let rhs_const = num_gen::integer().range(1, 9).random();
    let lhs_const = (rhs_coef - lhs_coef) * answer + rhs_const;

    let question = format!("${lhs_term}{lhs_const:+} = {rhs_const}{rhs_term:+}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_term}{lhs_const:+} &= {rhs_const}{rhs_term:+} \\ {sub_rhs} \\
        {total_var}{lhs_const:+} &= {rhs_const} \\ {sub_lhs} \\
        {total_var} &= {total_const} \\ {div_coef} \\
        {unknown} &= {answer} \\ \\",
        sub_rhs = formatting::subtract_term(&rhs_term),
        sub_lhs = formatting::subtract_number(lhs_const),
        total_var = lhs_term.clone() - rhs_term.clone(),
        div_coef = formatting::divide_number(lhs_coef - rhs_coef),
        total_const = rhs_const - lhs_const
    ));

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range.len(),
    })
}

/// 4 - 2x = 1 + 4x
/// Difficulty: 2
#[problem]
fn one_negative_coef_rhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let rhs_range = num_gen::integer().range(1, 9);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(rhs_coef - 10, -1).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let lhs_const = num_gen::integer().range(1, 9).random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;

    let question = format!("${lhs_const}{lhs_term:+} = {rhs_term}{rhs_const:+}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_const}{lhs_term:+} &= {rhs_term}{rhs_const:+} \\ {sub_lhs} \\
        {lhs_const} &= {total_var}{rhs_const:+} \\ {sub_rhs} \\
        {total_const} &= {total_var} \\ {div_coef} \\
        {answer} &= {unknown} \\ \\",
        sub_lhs = formatting::subtract_term(&lhs_term),
        sub_rhs = formatting::subtract_number(rhs_const),
        total_var = rhs_term.clone() - lhs_term.clone(),
        div_coef = formatting::divide_number(rhs_coef - lhs_coef),
        total_const = lhs_const - rhs_const
    ));

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range.len(),
    })
}

// 4 - 4x = 8 - 2x
/// Difficulty: 3
#[problem]
fn two_negative_coefs_rhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let lhs_range = num_gen::integer().range(-9, -3);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(lhs_coef + 2, -1).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_const = num_gen::integer().range(-9, 9).random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]).sorted();
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]).sorted();

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");
    let solution = equation_solution(format!(
        "{lhs_pol} &= {rhs_pol} \\ {sub_lhs} \\
        {lhs_const} &= {total_var}{rhs_const:+} \\ {sub_rhs} \\
        {total_const} &= {total_var} \\ {div_coef} \\
        {answer} &= {unknown} \\ \\",
        sub_lhs = formatting::subtract_term(&lhs_term),
        sub_rhs = formatting::subtract_number(rhs_const),
        total_var = rhs_term.clone() - lhs_term.clone(),
        div_coef = formatting::divide_number(rhs_coef - lhs_coef),
        total_const = lhs_const - rhs_const
    ));

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range.len(),
    })
}

// 4 - 2x = 8 - 4x
/// Difficulty: 3
#[problem]
fn two_negative_coefs_lhs_greater(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let rhs_range = num_gen::integer().range(-9, -2);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(rhs_coef + 1, -1).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_const = num_gen::integer().range(-9, 9).random();
    let lhs_const = (rhs_coef - lhs_coef) * answer + rhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]).sorted();
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]).sorted();

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let total_var = lhs_term.clone() - rhs_term.clone();
    let total_coef = lhs_coef - rhs_coef;
    let mut sol = StructuredSolution::new();
    sol.add_aligned(&lhs_pol, &rhs_pol) // 4 - 2x = 8 - 4x
        .with_step(subtract_term(&rhs_term))
        .add_aligned(format!("{total_var}{lhs_const:+}"), rhs_const) // 4 + 2x = 8
        .with_step(subtract_number(lhs_const));
    if total_coef > 1 {
        sol.add_aligned(total_var, rhs_const - lhs_const) // 2x = 4
            .with_step(divide_number(total_coef));
    }
    sol.add_aligned(unknown, answer); // x = 2

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution: sol.to_string(),
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range.len(),
    })
}

// 4x = 6x - 6
/// Difficulty: 3
#[problem]
fn positive_coefs_lhs_has_zero(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let rhs_range = num_gen::integer().range(2, 9);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(1, rhs_coef - 1).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_const = (lhs_coef - rhs_coef) * answer;
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]).sorted();

    let question = format!("${lhs_term} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let mut solution = StructuredSolution::new();
    solution
        .add_aligned(&lhs_term, rhs_pol) // 4x = 6x - 6
        .with_step(subtract_term(&lhs_term));
    let total_var = rhs_term - lhs_term;
    solution
        .add_aligned(0, format!("{total_var}{rhs_const:+}")) // 0 = 2x - 6
        .with_step(subtract_number(rhs_const));
    if total_var.coefficient > Number::Integer(1) {
        solution
            .add_aligned(-rhs_const, total_var) // 6 = 2x
            .with_step(divide_number(rhs_coef - lhs_coef));
    }
    solution.add_aligned(answer, unknown); // 3 = x

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution: solution.to_string(),
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range.len(),
    })
}

// 6x - 6 = 4x
/// Difficulty: 3
#[problem]
fn positive_coefs_rhs_has_zero(name: String, _lang: &Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let lhs_range = num_gen::integer().range(2, 9);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(1, lhs_coef - 1).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_const = (rhs_coef - lhs_coef) * answer;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]).sorted();

    let question = format!("${lhs_pol} = {rhs_term}$");
    let answer_str = format!("${unknown} = {answer}$");

    let mut solution = StructuredSolution::new();
    solution
        .add_aligned(&lhs_pol, &rhs_term) // 6x - 6 = 4x
        .with_step(subtract_term(&rhs_term));
    let total_var = lhs_term - rhs_term;
    solution
        .add_aligned(format!("{total_var}{lhs_const:+}"), 0) // 2x - 6 = 0
        .with_step(subtract_number(lhs_const));
    if total_var.coefficient > Number::Integer(1) {
        solution
            .add_aligned(total_var, -lhs_const) // 2x = 6
            .with_step(divide_number(lhs_coef - rhs_coef));
    }
    solution.add_aligned(unknown, answer); // x = 3

    Ok(Problem {
        name,
        question,
        answer: answer_str,
        solution: solution.to_string(),
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range.len(),
    })
}
