use anyhow::Result;
use macros::problem;
use math::{
    Number, Term,
    formatting::{self, divide_number, show_simplification, subtract_number, subtract_term},
    num_gen::{self, NumberGenerator},
    symbols,
};
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// 3(x + 2) = 9
/// Absolute difficulty: 4
/// Relative difficulty: 1
#[problem]
fn par_with_x(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(1, 9).random();
    let (par_coef, par_range) = num_gen::integer().range(2, 6).and_random();
    let (constant, const_range) = num_gen::integer()
        .range(-answer + 1, 5)
        .exclude(0)
        .and_random();
    let rhs = par_coef * (answer + constant);
    let var = symbols::get_unknown()?;

    let inner_expr = var + constant;
    let lhs = format!("{par_coef}({inner_expr})");
    let multiplied_const = par_coef * constant;

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, rhs)
        .aligned(par_coef * inner_expr, rhs)
        .step(formatting::subtract_number(multiplied_const))
        .aligned(par_coef * var, par_coef * answer)
        .step(divide_number(par_coef))
        .aligned(var, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![par_coef, constant],
        combinations: par_range.len() * const_range.len(),
    }))
}

/// 3(3x + 2) = 10
/// Absolute difficulty: 4
/// Relative difficulty: 2
#[problem]
fn par_with_kx(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(1, 9).random();
    let (par_coef, par_range) = num_gen::integer().range(2, 3).and_random();
    let coef = num_gen::integer().range(2, 10 / par_coef.as_i32()).random();
    let (constant, const_range) = num_gen::integer().range(-5, 5).exclude(0).and_random();

    let var = symbols::get_unknown()?;
    let var_term = coef * var;
    let rhs = par_coef * (coef * answer + constant);

    let inner_expr = var_term.and(&constant);
    let lhs = format!("{par_coef}({inner_expr})");
    let multiplied_const = par_coef * constant;

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, rhs)
        .aligned(par_coef * inner_expr, rhs)
        .step(formatting::subtract_number(multiplied_const))
        .aligned(par_coef * var_term, par_coef * coef * answer)
        .step(divide_number(par_coef * coef))
        .aligned(var, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![par_coef, coef, constant],
        combinations: par_range.len() * const_range.len(),
    }))
}

/// 2 + 3(x + 2) = 11
/// Absolute difficulty: 4
/// Relative difficulty: 2
#[problem]
fn par_with_const(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(1, 9).random();
    let (par_coef, par_range) = num_gen::integer().range(2, 6).and_random();
    let (par_const, const_range) = num_gen::integer()
        .range(-answer + 1, 5)
        .exclude(0)
        .and_random();
    let lhs_const = num_gen::integer().range(1, 10).random();
    let rhs = lhs_const + par_coef * (answer + par_const);
    let var = symbols::get_unknown()?;

    let inner_expr = var + par_const;
    let lhs = format!("{lhs_const} + {par_coef}({inner_expr})");
    let multiplied_const = par_coef * par_const;
    let total_const = lhs_const + multiplied_const;
    let var_term = par_coef * var;

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, rhs)
        .aligned(format!("{lhs_const} + {}", par_coef * inner_expr), rhs)
        .aligned(format!("{var_term} {total_const:+}"), rhs)
        .step(formatting::subtract_number(total_const))
        .aligned(var_term, par_coef * answer)
        .step(divide_number(par_coef))
        .aligned(var, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![par_coef, par_const],
        combinations: par_range.len() * const_range.len(),
    }))
}

/// 2 - 3(x - 4) = 11
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn neg_par_with_const(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(-5, 5).random();
    let (par_coef, par_range) = num_gen::integer().range(-6, -2).and_random();
    let (par_const, const_range) = num_gen::integer()
        .range(-5, 5)
        .exclude(0)
        .exclude(-answer)
        .and_random();
    let lhs_const = num_gen::integer().range(1, 10).random();
    let rhs = lhs_const + par_coef * (answer + par_const);
    let var = symbols::get_unknown()?;

    let inner_expr = var + par_const;
    let lhs = format!("{lhs_const} {par_coef}({inner_expr})");
    let multiplied_const = par_coef * par_const;
    let total_const = lhs_const + multiplied_const;
    let var_term = par_coef * var;

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, rhs)
        .aligned(format!("{lhs_const} {}", par_coef * inner_expr), rhs)
        .aligned(format!("{var_term} {total_const:+}"), rhs)
        .step(formatting::subtract_number(total_const))
        .aligned(var_term, par_coef * answer)
        .step(divide_number(par_coef))
        .aligned(var, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![par_coef, par_const],
        combinations: par_range.len() * const_range.len(),
    }))
}

/// 3(x + 1) = 2x + 2
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn one_par_with_const(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(-9, 9).random();
    let (par_coef, par_range) = num_gen::integer().range(2, 3).and_random();
    let (lhs_coef, lhs_range) = num_gen::integer().range(1, 3).and_random();
    // Keep largest x coef on LHS
    let (rhs_coef, rhs_range) = num_gen::integer()
        .range(-4, par_coef * lhs_coef - 1)
        .exclude(0)
        .and_random();
    let (lhs_const, const_range) = num_gen::integer().range(-4, 4).exclude(0).and_random();
    // Determine RHS const from all the other numbers
    let rhs_const = par_coef * (lhs_coef * answer + lhs_const) - rhs_coef * answer;
    let var = symbols::get_unknown()?;

    let lhs_term = lhs_coef * var;
    let lhs_poly = lhs_term.and(&lhs_const);
    let rhs_term = rhs_coef * var;
    let lhs = format!("{par_coef}({lhs_poly})");
    let rhs = rhs_term.and(&rhs_const).simplify();
    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, &rhs)
        .aligned(par_coef * lhs_poly.clone(), &rhs)
        .step(subtract_term(&rhs_term))
        .aligned((par_coef * lhs_poly).and(&-rhs_term).simplify(), rhs_const)
        .step(subtract_number(par_coef * lhs_const))
        .aligned(
            (par_coef * lhs_coef - rhs_coef) * var,
            rhs_const - par_coef * lhs_const,
        );
    if (par_coef * lhs_coef - rhs_coef) != 1 {
        solution
            .step(divide_number(par_coef * lhs_coef - rhs_coef))
            .aligned(var, answer);
    }

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {answer}$"),
        solution: solution.to_string(),
        identifiers: vec![par_coef, lhs_coef, rhs_coef, lhs_const],
        combinations: par_range.len() * lhs_range.len() * const_range.len() * rhs_range.len(),
    }))
}

/// 3(x + 1) = 2x
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn one_par_no_const(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(-9, 9).exclude(0).random();
    let (par_coef, par_range) = num_gen::integer().range(2, 4).and_random();
    let (lhs_coef, lhs_range) = num_gen::integer().range(1, 3).and_random();
    let (rhs_coef, rhs_range) = num_gen::integer()
        .range(1, 3)
        .exclude(lhs_coef)
        .and_random();
    let rhs_coef = rhs_coef * par_coef;
    let lhs_const = (rhs_coef - par_coef * lhs_coef) * answer / par_coef;
    let var = symbols::get_unknown()?;

    let lhs_term = lhs_coef * var;
    let lhs_poly = lhs_term.and(&lhs_const);
    let rhs = rhs_coef * var;
    let lhs = format!("{par_coef}({lhs_poly})");
    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, &rhs)
        .aligned(par_coef * lhs_poly, &rhs)
        .step(subtract_term(&(par_coef * lhs_term.clone())))
        .aligned(
            par_coef * lhs_const,
            (rhs.and(&(-par_coef * lhs_term))).simplify(),
        );
    if (rhs_coef - par_coef * lhs_coef) != 1 {
        solution
            .step(divide_number(rhs_coef - par_coef * lhs_coef))
            .aligned(answer, var);
    }

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {answer}$"),
        solution: solution.to_string(),
        identifiers: vec![par_coef, lhs_coef, rhs_coef],
        combinations: par_range.len() * lhs_range.len() * rhs_range.len(),
    }))
}

/// 2 - 3(2 - 3x) = 11
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn neg_par_with_neg_coef(id: i32, _lang: Language) -> Result<Problem> {
    let answer = num_gen::integer().range(-5, 5).random();
    let (par_coef, par_range) = num_gen::integer().range(-5, -2).and_random();
    let coef = num_gen::integer()
        .range(10 / par_coef.as_i32(), -1)
        .random();
    let (par_const, const_range) = num_gen::integer().range(1, 5).exclude(-answer).and_random();
    let lhs_const = num_gen::integer().range(1, 10).random();
    let rhs = lhs_const + par_coef * (coef * answer + par_const);
    let var = symbols::get_unknown()?;

    let var_term = coef * var;
    let inner_expr = var_term.and(&par_const).simplify();
    let lhs = format!("{lhs_const} {par_coef}({inner_expr})");
    let multiplied_const = par_coef * par_const;
    let total_const = lhs_const + multiplied_const;
    let final_var_term = par_coef * var_term;

    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, rhs)
        .aligned(format!("{lhs_const} {}", par_coef * inner_expr), rhs)
        .aligned(format!("{final_var_term} {total_const:+}"), rhs)
        .step(formatting::subtract_number(total_const))
        .aligned(final_var_term, par_coef * coef * answer)
        .step(divide_number(par_coef * coef))
        .aligned(var, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {answer}$"),
        solution,
        identifiers: vec![par_coef, par_const],
        combinations: par_range.len() * const_range.len(),
    }))
}

/// 3(2x - 1) = 2(x + 4)
/// Absolute difficulty: 5
/// Relative difficulty: 7
#[problem]
fn two_pars(id: i32, _lang: Language) -> Result<Problem> {
    let (par1, par_range) = num_gen::integer().range(2, 4).and_random();
    let par2 = 2;
    let (coef1, coef_range) = num_gen::integer().range(2, 3).and_random();
    // We want to assert that LHS has the greatest amount of x
    let max_coef2 = par1.as_i32() * coef1.as_i32() / par2 - 1;
    let coef2 = num_gen::integer().range(1, max_coef2).random();

    let const1 = num_gen::integer().range(-5, -1).random();
    let const2 = num_gen::integer().range(1, 5).random();
    let final_const = par2 * const2 - par1 * const1;
    let final_coef = par1 * coef1 - par2 * coef2;
    let answer = Number::fraction(final_const, final_coef);
    let var = symbols::get_unknown()?;

    let term1 = coef1 * var;
    let poly1 = term1.and(&const1);
    let term2 = coef2 * var;
    let final_term2 = par2 * term2.clone();
    let poly2 = term2.and(&const2);

    let lhs = format!("{par1}({poly1})");
    let rhs = format!("{par2}({poly2})");
    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, &rhs)
        .aligned(par1 * poly1.clone(), par2 * poly2)
        .step(subtract_term(&final_term2))
        .aligned((par1 * poly1).and(&-final_term2).simplify(), par2 * const2)
        .step(subtract_number(par1 * const1))
        .aligned(final_coef * var, final_const)
        .step(divide_number(final_coef));
    if answer.can_be_simplified() {
        let simplification = format!("{} = {}", show_simplification(answer), answer.simplify());
        solution.aligned(var, simplification);
    } else {
        solution.aligned(var, answer);
    }
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {}$", answer.simplify()),
        solution,
        identifiers: vec![par1, coef1],
        combinations: par_range.len() * coef_range.len(),
    }))
}

/// 5(2x - 1) - 2x = 2(x + 4)
/// Absolute difficulty: 6
/// Relative difficulty: 8
#[problem]
fn two_pars_with_term(id: i32, _lang: Language) -> Result<Problem> {
    let (par1, par_range) = num_gen::integer().range(3, 5).and_random();
    let (coef1, coef_range) = num_gen::integer().range(2, 4).and_random();
    let total_coef1 = par1 * coef1;
    let coef2 = num_gen::integer().range(-total_coef1 + 3, -2).random();
    let total_coef_lhs = total_coef1 + coef2;
    let par2 = num_gen::integer().range(2, total_coef_lhs - 1).random();

    let const1 = num_gen::integer().range(-3, -1).random();
    let const2 = num_gen::integer().range(1, 6).random();
    let final_const = par2 * const2 - par1 * const1;
    let final_coef = total_coef_lhs - par2;
    let answer = Number::fraction(final_const, final_coef);
    let var = symbols::get_unknown()?;

    let term1 = coef1 * var;
    let term2 = coef2 * var;
    let poly1 = term1.and(&const1);
    let poly2 = Term::from_var(var).and(&const2);
    let final_term_rhs = par2 * var;
    let lhs_after_mult = (par1 * poly1.clone()).and(&term2);

    let lhs = format!("{par1}({poly1}){term2:+}");
    let rhs = format!("{par2}({poly2})");
    let mut solution = Solution::with_steps();
    solution
        .aligned(&lhs, &rhs)
        .aligned(&lhs_after_mult, par2 * poly2.clone())
        .aligned(lhs_after_mult.simplify(), par2 * poly2)
        .step(subtract_term(&final_term_rhs))
        .aligned(
            lhs_after_mult.and(&-final_term_rhs).simplify(),
            par2 * const2,
        )
        .step(subtract_number(par1 * const1))
        .aligned(final_coef * var, final_const);
    if final_coef != 1 {
        solution.step(divide_number(final_coef));
        if answer.can_be_simplified() {
            let simplification = format!("{} = {}", show_simplification(answer), answer.simplify());
            solution.aligned(var, simplification);
        } else {
            solution.aligned(var, answer);
        }
    }
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {}$", answer.simplify()),
        solution,
        identifiers: vec![par1, coef1],
        combinations: par_range.len() * coef_range.len(),
    }))
}

/// 5(2x + 1) - 2(3 - 2x) = 2(x + 4)
/// Absolute difficulty: 7
/// Relative difficulty: 9
#[problem]
fn three_pars(id: i32, _lang: Language) -> Result<Problem> {
    let (par1, par1_range) = num_gen::integer().range(2, 5).and_random();
    let (par2, par2_range) = num_gen::integer().range(-4, -2).and_random();
    let (coef1, coef_range) = num_gen::integer().range(-4, 4).exclude(0).and_random();
    let coef2 = coef_range.random();
    let total_coef1 = par1 * coef1;
    let total_coef2 = par2 * coef2;
    let total_coef_lhs = total_coef1 + total_coef2;
    let par3 = par1_range.random();
    let coef3 = num_gen::integer()
        .range(
            total_coef_lhs.as_i32() / par3.as_i32() + 1,
            total_coef_lhs.as_i32() / par3.as_i32() + 6,
        )
        .exclude(0)
        .random();
    let total_coef_rhs = par3 * coef3;

    let (const1, const_range) = num_gen::integer().range(1, 8).and_random();
    let const2 = const_range.random();
    let const3 = const_range.random();
    let final_const = par1 * const1 + par2 * const2 - par3 * const3;
    let final_coef = total_coef_rhs - total_coef_lhs;
    let answer = Number::fraction(final_const, final_coef);
    let var = symbols::get_unknown()?;

    let term1 = coef1 * var;
    let term2 = coef2 * var;
    let term3 = coef3 * var;
    let poly1 = term1.and(&const1).simplify();
    let poly2 = term2.and(&const2).simplify();
    let poly3 = term3.and(&const3).simplify();

    let lhs = format!("{par1}({poly1}){par2:+}({poly2})");
    let rhs = format!("{par3}({poly3})");
    let mut solution = Solution::with_steps();
    solution
        .aligned(
            format!("{} {:+}", par1 * poly1.clone(), par2 * poly2.clone()),
            par3 * poly3.clone(),
        )
        .aligned((par1 * poly1 + par2 * poly2).simplify(), par3 * poly3)
        .step(subtract_term(&(total_coef_lhs * var)))
        .aligned(
            par1 * const1 + par2 * const2,
            (final_coef * var).and(&Term::from_num(par3 * const3)),
        )
        .step(subtract_number(par3 * const3))
        .aligned(final_const, final_coef * var);
    if final_coef != 1 {
        solution.step(divide_number(final_coef));
        if answer.can_be_simplified() {
            let simplification = format!("{var} = {}", show_simplification(answer));
            solution.aligned(simplification, answer.simplify());
        } else {
            solution.aligned(answer, var);
        }
    }
    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} = {rhs}$"),
        answer: format!("${var} = {}$", answer.simplify()),
        solution: format!("${lhs} = {rhs}$ \\ {solution}"),
        identifiers: vec![par1, par2, par3],
        combinations: par1_range.len().pow(2) * par2_range.len(),
    }))
}
