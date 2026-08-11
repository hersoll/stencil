use anyhow::Result;
use macros::problem;
use math::{
    Number, Polynomial, Term,
    formatting::{divide_number, subtract_number, subtract_term},
    num_gen::{self, NumberGenerator},
    symbols,
};
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// 4x + 1 = 2x + 3
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn two_positive_coefs_lhs_greater(id: i32, _lang: Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(1, 6).random();
    let lhs_range = num_gen::integer().range(3, 9);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(1, lhs_coef - 1).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_const = num_gen::integer()
        .range(1, 9)
        .exclude((rhs_coef - lhs_coef) * answer) // Prevents other const from becoming 0
        .random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]);
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]);

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let mut sol = Solution::with_steps();
    sol.aligned(lhs_pol, rhs_pol) // 4x + 1 = 2x + 3
        .step(subtract_term(&rhs_term));

    let subtracted_term = lhs_term - rhs_term;

    sol.aligned(format!("{subtracted_term}{lhs_const:+}"), rhs_const) // 2x + 1 = 3
        .step(subtract_number(lhs_const));
    if subtracted_term.coefficient.value() > 1.0 {
        sol.aligned(&subtracted_term, rhs_const - lhs_const) // 2x = 2
            .step(divide_number(subtracted_term.coefficient));
    }
    sol.aligned(unknown, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution: sol,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range,
    }))
}

/// 2x + 1 = 4x + 3
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn two_positive_coefs_rhs_greater(id: i32, _lang: Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(1, 6).random();
    let rhs_range = num_gen::integer().range(3, 9);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(1, rhs_coef - 1).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_const = num_gen::integer()
        .range(1, 9)
        .exclude((lhs_coef - rhs_coef) * answer) // Prevents other const from becoming 0
        .random();
    let lhs_const = (rhs_coef - lhs_coef) * answer + rhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]);
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]);

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let mut sol = Solution::with_steps();
    sol.aligned(lhs_pol, rhs_pol) // 2x + 1 = 4x + 3
        .step(subtract_term(&lhs_term));

    let subtracted_term = rhs_term - lhs_term;

    sol.aligned(lhs_const, format!("{subtracted_term}{rhs_const:+}")) // 1 = 2x + 3
        .step(subtract_number(rhs_const));
    if subtracted_term.coefficient.value() > 1.0 {
        sol.aligned(lhs_const - rhs_const, &subtracted_term) // -2 = 2x
            .step(divide_number(subtracted_term.coefficient));
    }
    sol.aligned(answer, unknown); // -1 = x

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution: sol,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range,
    }))
}

/// 4x + 1 = 4 - 2x
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn one_negative_coef_lhs_greater(id: i32, _lang: Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let lhs_range = num_gen::integer().range(1, 9);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(lhs_coef - 10, -1).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let rhs_const = num_gen::integer()
        .range(1, 9)
        .exclude((lhs_coef - rhs_coef) * answer) // Prevents other const from becoming 0
        .random();
    let lhs_const = (rhs_coef - lhs_coef) * answer + rhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]).sorted();
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]).sorted();

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let mut sol = Solution::with_steps();

    let subtracted_term = lhs_term - rhs_term.clone();

    sol.aligned(lhs_pol, rhs_pol) // 4x + 1 = 4 - 2x
        .step(subtract_term(&rhs_term))
        .aligned(format!("{subtracted_term}{lhs_const:+}"), rhs_const) // 6x + 1 = 4
        .step(subtract_number(lhs_const))
        .aligned(&subtracted_term, rhs_const - lhs_const) // 6x = 3
        .step(divide_number(subtracted_term.coefficient))
        .aligned(unknown, answer); // x = 0.5

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution: sol,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range,
    }))
}

/// 4 - 2x = 1 + 4x
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn one_negative_coef_rhs_greater(id: i32, _lang: Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let rhs_range = num_gen::integer().range(1, 9);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(rhs_coef - 10, -1).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let lhs_const = num_gen::integer()
        .range(1, 9)
        .exclude((rhs_coef - lhs_coef) * answer) // Prevents other const from becoming 0
        .random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]).sorted();
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]).sorted();

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let subtracted_term = rhs_term - lhs_term.clone();
    let mut sol = Solution::with_steps();
    sol.aligned(lhs_pol, rhs_pol) // 4 - 2x = 1 + 4x
        .step(subtract_term(&lhs_term))
        .aligned(lhs_const, format!("{subtracted_term}{rhs_const:+}")) // 4 = 6x + 1
        .step(subtract_number(rhs_const))
        .aligned(lhs_const - rhs_const, &subtracted_term) // 3 = 6x
        .step(divide_number(subtracted_term.coefficient))
        .aligned(answer, unknown); // 0.5 = x

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution: sol,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range,
    }))
}

// 4 - 4x = 8 - 2x
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn two_negative_coefs_rhs_greater(id: i32, _lang: Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let lhs_range = num_gen::integer().range(-9, -3);
    let lhs_coef = lhs_range.random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_coef = num_gen::integer().range(lhs_coef + 1, -1).random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_const = num_gen::integer()
        .range(-9, 9)
        .exclude((rhs_coef - lhs_coef) * answer) // Prevents other const from becoming 0
        .random();
    let rhs_const = (lhs_coef - rhs_coef) * answer + lhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]).sorted();
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]).sorted();

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let mut sol = Solution::with_steps();
    let subtracted_term = rhs_term - lhs_term.clone();
    sol.aligned(lhs_pol, rhs_pol) // 4 - 4x = 8 - 2x
        .step(subtract_term(&lhs_term))
        .aligned(lhs_const, format!("{subtracted_term}{rhs_const:+}")) // 4 = 2x + 8
        .step(subtract_number(rhs_const));
    if subtracted_term.coefficient.value() > 1.0 {
        sol.aligned(lhs_const - rhs_const, &subtracted_term) // -4 = 2x
            .step(divide_number(subtracted_term.coefficient));
    }
    sol.aligned(answer, unknown); // -2 = x

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution: sol,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range,
    }))
}

// 4 - 2x = 8 - 4x
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn two_negative_coefs_lhs_greater(id: i32, _lang: Language) -> Result<Problem> {
    let unknown = symbols::get_unknown()?;
    let answer = num_gen::integer().range(-8, 8).exclude(0).random();
    let rhs_range = num_gen::integer().range(-9, -2);
    let rhs_coef = rhs_range.random();
    let rhs_term = Term::from_num_and_vars(rhs_coef, unknown);
    let lhs_coef = num_gen::integer().range(rhs_coef + 1, -1).random();
    let lhs_term = Term::from_num_and_vars(lhs_coef, unknown);
    let rhs_const = num_gen::integer()
        .range(-9, 9)
        .exclude((lhs_coef - rhs_coef) * answer) // Prevents other const from becoming 0
        .random();
    let lhs_const = (rhs_coef - lhs_coef) * answer + rhs_const;
    let lhs_pol = Polynomial::from_terms(&[&lhs_term, &Term::from_num(lhs_const)]).sorted();
    let rhs_pol = Polynomial::from_terms(&[&rhs_term, &Term::from_num(rhs_const)]).sorted();

    let question = format!("${lhs_pol} = {rhs_pol}$");
    let answer_str = format!("${unknown} = {answer}$");

    let total_var = lhs_term - rhs_term.clone();
    let total_coef = lhs_coef - rhs_coef;
    let mut sol = Solution::with_steps();
    sol.aligned(&lhs_pol, &rhs_pol) // 4 - 2x = 8 - 4x
        .step(subtract_term(&rhs_term))
        .aligned(format!("{total_var}{lhs_const:+}"), rhs_const) // 4 + 2x = 8
        .step(subtract_number(lhs_const));
    if total_coef > 1 {
        sol.aligned(total_var, rhs_const - lhs_const) // 2x = 4
            .step(divide_number(total_coef));
    }
    sol.aligned(unknown, answer); // x = 2

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution: sol,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range,
    }))
}

// 4x = 6x - 6
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn positive_coefs_lhs_has_zero(id: i32, _lang: Language) -> Result<Problem> {
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

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs_term, rhs_pol) // 4x = 6x - 6
        .step(subtract_term(&lhs_term));
    let total_var = rhs_term - lhs_term;
    solution
        .aligned(0, format!("{total_var}{rhs_const:+}")) // 0 = 2x - 6
        .step(subtract_number(rhs_const));
    if total_var.coefficient > Number::Integer(1) {
        solution
            .aligned(-rhs_const, total_var) // 6 = 2x
            .step(divide_number(rhs_coef - lhs_coef));
    }
    solution.aligned(answer, unknown); // 3 = x

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: rhs_range,
    }))
}

// 6x - 6 = 4x
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn positive_coefs_rhs_has_zero(id: i32, _lang: Language) -> Result<Problem> {
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

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs_pol, &rhs_term) // 6x - 6 = 4x
        .step(subtract_term(&rhs_term));
    let total_var = lhs_term - rhs_term;
    solution
        .aligned(format!("{total_var}{lhs_const:+}"), 0) // 2x - 6 = 0
        .step(subtract_number(lhs_const));
    if total_var.coefficient > Number::Integer(1) {
        solution
            .aligned(total_var, -lhs_const) // 2x = 6
            .step(divide_number(lhs_coef - rhs_coef));
    }
    solution.aligned(unknown, answer); // x = 3

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer_str,
        solution,
        identifiers: vec![lhs_coef, rhs_coef],
        combinations: lhs_range,
    }))
}
