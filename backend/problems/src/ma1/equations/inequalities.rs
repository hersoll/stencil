use anyhow::Result;
use macros::problem;
use math::{
    MathDisplay, formatting,
    num_gen::{self, NumberGenerator},
    symbols::{self, inequality_sign::InequalitySign},
};
use registry::{get_answer, get_question, get_solution};
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters, Solution},
};

/// x - 5 > 8
/// Absolute difficulty: 3
/// Relative difficulty: 1
#[problem]
fn addition_subtraction_only(id: i32, _lang: Language) -> Result<Problem> {
    let (answer, ans_range) = num_gen::integer().range(1, 9).and_random();
    let constant = num_gen::integer().range(-answer + 1, 9).exclude(0).random();
    let rhs = answer + constant;
    let var = symbols::get_unknown()?;
    let sign = InequalitySign::strict();

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(var + constant, sign, rhs)
        .step(formatting::subtract_number(constant))
        .aligned_inequality(var, sign, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${var}{constant:+}{sign}{rhs}$"),
        answer: format!("${var}{sign}{answer}$"),
        solution,
        identifiers: answer,
        combinations: ans_range,
    }))
}

/// 2x > 8
/// Absolute difficulty: 3
/// Relative difficulty: 1
#[problem]
fn multiplication_only(id: i32, _lang: Language) -> Result<Problem> {
    let (answer, ans_range) = num_gen::integer().range(2, 9).and_random();
    let (coef, coef_range) = num_gen::integer().range(2, 9).and_random();
    let rhs = answer * coef;
    let var = symbols::get_unknown()?;
    let sign = InequalitySign::strict();
    let term = coef * var;

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(coef * var, sign, rhs)
        .step(formatting::divide_number(coef))
        .aligned_inequality(var, sign, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${term}{sign}{rhs}$"),
        answer: format!("${var}{sign}{answer}$"),
        solution,
        identifiers: vec![answer, coef],
        combinations: ans_range.len() * coef_range.len(),
    }))
}

/// 2x + 1 >= 9
/// Absolute difficulty: 4
/// Relative difficulty: 2
#[problem]
fn standard(id: i32, _lang: Language) -> Result<Problem> {
    let (answer, ans_range) = num_gen::integer().range(2, 9).and_random();
    let (coef, coef_range) = num_gen::integer().range(2, 5).and_random();
    let constant = ans_range.random();
    let rhs = answer * coef + constant;
    let var = symbols::get_unknown()?;
    let sign = InequalitySign::random();
    let lhs = (coef * var).and(&constant);

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(&lhs, sign, rhs)
        .step(formatting::subtract_number(constant))
        .aligned_inequality(coef * var, sign, rhs - constant)
        .step(formatting::divide_number(coef))
        .aligned_inequality(var, sign, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} {sign} {rhs}$"),
        answer: format!("${var} {sign} {answer}$"),
        solution,
        identifiers: vec![answer, coef],
        combinations: ans_range.len() * coef_range.len(),
    }))
}

/// -2x > 8
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn negative_division(id: i32, lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(-9, -2).and_random();
    let answer = coef_range.random();

    let var = symbols::get_unknown()?;
    let sign = InequalitySign::random();
    let swapped_sign = sign.swapped();
    let rhs = coef * answer;
    let lhs = coef * var;

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(&lhs, sign, rhs)
        .step(formatting::divide_number(coef))
        .aligned_inequality(var, swapped_sign, answer);

    let solution = get_solution(id, lang)?.replace_one("solution", solution);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} {sign} {rhs}$"),
        answer: format!("${var} {swapped_sign} {answer}$"),
        solution,
        identifiers: vec![coef, answer],
        combinations: coef_range.len().pow(2),
    }))
}

/// 2x > -8
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn positive_division(id: i32, lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(2, 9).and_random();
    let answer = -coef_range.random();

    let var = symbols::get_unknown()?;
    let sign = InequalitySign::random();
    let rhs = coef * answer;
    let lhs = coef * var;

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(&lhs, sign, rhs)
        .step(formatting::divide_number(coef))
        .aligned_inequality(var, sign, answer);

    let solution = get_solution(id, lang)?.replace_one("solution", solution);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} {sign} {rhs}$"),
        answer: format!("${var} {sign} {answer}$"),
        solution,
        identifiers: vec![coef, answer],
        combinations: coef_range.len().pow(2),
    }))
}

/// 10 - 2x > 2
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn constant_negative_coef(id: i32, _lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(-5, -2).and_random();
    let (constant, const_range) = num_gen::integer().range(2, 20).and_random();
    let (answer, ans_range) = num_gen::integer().range(-5, 5).exclude(0).and_random();

    let var = symbols::get_unknown()?;
    let sign = InequalitySign::random();
    let swapped_sign = sign.swapped();
    let rhs = constant + coef * answer;
    let lhs = (coef * var).and(&constant).simplify();

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(&lhs, sign, rhs)
        .step(formatting::subtract_number(constant))
        .aligned_inequality(coef * var, sign, rhs - constant)
        .step(formatting::divide_number(coef))
        .aligned_inequality(var, swapped_sign, answer);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} {sign} {rhs}$"),
        answer: format!("${var} {swapped_sign} {answer}$"),
        solution,
        identifiers: vec![coef, constant, answer],
        combinations: coef_range.len() * const_range.len() * ans_range.len(),
    }))
}

/// 10 - 2x > 6x + 2
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn x_on_both_sides(id: i32, lang: Language) -> Result<Problem> {
    let (coef_1, coef_1_range) = num_gen::integer().range(-5, -2).and_random();
    let (coef_2, coef_2_range) = num_gen::integer().range(2, 5).and_random();
    let (const_1, const_range) = num_gen::integer().range(2, 20).and_random();
    let (answer, ans_range) = num_gen::integer().range(-5, 5).exclude(0).and_random();
    let const_2 = const_1 + coef_1 * answer - coef_2 * answer;

    let var = symbols::get_unknown()?;
    let sign = InequalitySign::random();
    let swapped_sign = sign.swapped();
    let lhs = (coef_1 * var).and(&const_1).simplify();
    let rhs = (coef_2 * var).and(&const_2).simplify();
    // 6 - (-2) = 6 + 2 = 8(x)
    let total_coef = coef_2 - coef_1;
    // 10 - 2
    let total_const = const_1 - const_2;

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(&lhs, sign, &rhs)
        .step(formatting::subtract_term(&(coef_1 * var)))
        .aligned_inequality(const_1, sign, (total_coef * var).and(&const_2).simplify())
        .step(formatting::subtract_number(const_2))
        .aligned_inequality(total_const, sign, total_coef * var)
        .step(formatting::divide_number(total_coef))
        .aligned_inequality(answer, sign, var);

    let solution = get_solution(id, lang)?
        .replace_multiple(&[("solution", solution.to_string()), ("var", var.as_math())]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("${lhs} {sign} {rhs}$"),
        answer: format!("${var} {swapped_sign} {answer}$"),
        solution,
        identifiers: vec![coef_1, coef_2, const_1, answer],
        combinations: coef_1_range.len() * coef_2_range.len() * const_range.len() * ans_range.len(),
    }))
}

/// Largest integer that solves 10 - 2x > 2
/// Absolute difficulty: 6
/// Relative difficulty: 6
#[problem]
fn largest_integer(id: i32, lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(-5, -2).and_random();
    let (constant, const_range) = num_gen::integer().range(2, 20).and_random();
    let (answer, ans_range) = num_gen::integer().range(-5, 5).and_random();

    let var = symbols::get_unknown()?;
    let sign = InequalitySign::Greater;
    let swapped_sign = sign.swapped();
    let rhs = constant + coef * answer;
    let lhs = (coef * var).and(&constant).simplify();

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(&lhs, sign, rhs)
        .step(formatting::subtract_number(constant))
        .aligned_inequality(coef * var, sign, rhs - constant)
        .step(formatting::divide_number(coef))
        .aligned_inequality(var, swapped_sign, answer);

    let question = format!("${lhs} {sign} {rhs}$");
    let question = get_question(id, lang)?.replace_one("eq", question);

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("solution", solution.to_string()),
        ("answer", format!("${var} {swapped_sign} {answer}$")),
        ("largest", (answer - 1).as_math()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer - 1,
        solution,
        identifiers: vec![coef, constant, answer],
        combinations: coef_range.len() * const_range.len() * ans_range.len(),
    }))
}

/// Smallest integer that solves 10 - 2x < 2
/// Absolute difficulty: 6
/// Relative difficulty: 6
#[problem]
fn smallest_integer(id: i32, lang: Language) -> Result<Problem> {
    let (coef, coef_range) = num_gen::integer().range(-5, -2).and_random();
    let (constant, const_range) = num_gen::integer().range(2, 20).and_random();
    let (answer, ans_range) = num_gen::integer().range(-5, 5).and_random();

    let var = symbols::get_unknown()?;
    let sign = InequalitySign::Less;
    let swapped_sign = sign.swapped();
    let rhs = constant + coef * answer;
    let lhs = (coef * var).and(&constant).simplify();

    let mut solution = Solution::with_steps();
    solution
        .aligned_inequality(&lhs, sign, rhs)
        .step(formatting::subtract_number(constant))
        .aligned_inequality(coef * var, sign, rhs - constant)
        .step(formatting::divide_number(coef))
        .aligned_inequality(var, swapped_sign, answer);

    let question = format!("${lhs} {sign} {rhs}$");
    let question = get_question(id, lang)?.replace_one("eq", question);

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("solution", solution.to_string()),
        ("answer", format!("${var} {swapped_sign} {answer}$")),
        ("smallest", (answer + 1).as_math()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: answer + 1,
        solution,
        identifiers: vec![coef, constant, answer],
        combinations: coef_range.len() * const_range.len() * ans_range.len(),
    }))
}

/// 2 < 5x + 1 < 10
/// Absolute difficulty: 7
/// Relative difficulty: 7
#[problem]
fn double_inequality(id: i32, lang: Language) -> Result<Problem> {
    let lower_limit = num_gen::integer().range(-5, 5).random();
    let higher_limit = num_gen::integer()
        .range(lower_limit + 5, lower_limit + 15)
        .random();
    let (coef, coef_range) = num_gen::integer().range(2, 5).and_random();
    let (constant, const_range) = num_gen::integer().range(-9, 9).exclude(0).and_random();

    let sign = InequalitySign::Less;
    let var = symbols::get_unknown()?;
    let expression = (coef * var).and(&constant);
    let lower_answer = (lower_limit - constant) / coef;
    let higher_answer = (higher_limit - constant) / coef;

    let question = format!("${lower_limit} {sign} {expression} {sign} {higher_limit}$");
    let answer = format!("${lower_answer} {sign} {var} {sign} {higher_answer}$");

    let mut solution = Solution::with_steps();
    solution
        .line(format!(
            "{lower_limit} {sign} {expression} {sign} {higher_limit}"
        ))
        .step(formatting::subtract_number(constant))
        .line(format!(
            "{} {sign} {} {sign} {}",
            lower_limit - constant,
            coef * var,
            higher_limit - constant
        ))
        .step(formatting::divide_number(coef))
        .line(format!(
            "{lower_answer} {sign} {var} {sign} {higher_answer}"
        ));
    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("solution", solution.to_string()),
        ("eq1", format!("${lower_limit} {sign} {expression}$")),
        ("eq2", format!("${expression} {sign} {higher_limit}$")),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![constant, coef],
        combinations: const_range.len() * coef_range.len(),
    }))
}

/// 3x - 10 < 14 < 4x + 6
/// Absolute difficulty: 8
/// Relative difficulty: 8
#[problem]
fn double_split_inequality(id: i32, lang: Language) -> Result<Problem> {
    let lower_answer = num_gen::integer().range(-5, 3).random();
    let higher_answer = num_gen::integer()
        .range(lower_answer, lower_answer + 7)
        .random();
    let (coef_1, coef_range) = num_gen::integer()
        .range(-4, 4)
        .exclude_multiple(&[-1, 0, 1])
        .and_random();
    let coef_2 = coef_range.random();
    let var = symbols::get_unknown()?;
    let sign = InequalitySign::Less;

    // To choose our consts correctly, we need to adapt them to difference between the two sides
    let total_lower = coef_1 * lower_answer;
    let total_higher = coef_2 * higher_answer;
    let diff = total_higher - total_lower;
    let const_1 = num_gen::integer().range(1, 9).random();
    let const_2 = const_1 - diff;

    let middle_number = total_lower + const_1;

    let expr_1 = (coef_1 * var).and(&const_1);
    let expr_2 = (coef_2 * var).and(&const_2);

    let question = format!("${expr_2} < {middle_number} < {expr_1}$");
    let answer = format!("${lower_answer} < {var} < {higher_answer}$");

    let mut solution_1 = Solution::with_steps();
    solution_1
        .aligned_inequality(&expr_2, sign, middle_number)
        .step(formatting::subtract_number(const_2))
        .aligned_inequality(coef_2 * var, sign, middle_number - const_2)
        .step(formatting::divide_number(coef_2))
        .aligned_inequality(var, sign, higher_answer);
    let mut solution_2 = Solution::with_steps();
    solution_2
        .aligned_inequality(middle_number, sign, &expr_1)
        .step(formatting::subtract_number(const_1))
        .aligned_inequality(middle_number - const_1, sign, coef_1 * var)
        .step(formatting::divide_number(coef_1))
        .aligned_inequality(lower_answer, sign, var);

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("eq_1", format!("${expr_2} {sign} {middle_number}$")),
        ("eq_2", format!("${middle_number} {sign} {expr_1}$")),
        ("solution_1", solution_1.to_string()),
        ("solution_2", solution_2.to_string()),
        ("answer", answer.clone()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![coef_1, coef_2],
        combinations: coef_range.len().pow(2),
    }))
}

/// x^2 < 16
/// Absolute difficulty: 10
/// Relative difficulty: 10
#[problem]
fn square_less(id: i32, lang: Language) -> Result<Problem> {
    let (square, sq_range) = num_gen::integer()
        .numbers(&[1, 4, 9, 16, 25, 36, 49, 64, 81, 100])
        .and_random();
    let root = square.sqrt();
    let sign = InequalitySign::random_less();
    let answer = format!("$-{root} {sign} x {sign} {root}$");

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("eq", format!("$x^2 = {square}$")),
        ("eq_answer", format!("$x = plus.minus {root}$")),
        ("ineq_answer", answer.clone()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$x^2 {sign} {square}$"),
        answer,
        solution,
        identifiers: square,
        combinations: sq_range,
    }))
}

/// x^2 > 16
/// Absolute difficulty: 10
/// Relative difficulty: 11
#[problem]
fn square_more(id: i32, lang: Language) -> Result<Problem> {
    let (square, sq_range) = num_gen::integer()
        .numbers(&[1, 4, 9, 16, 25, 36, 49, 64, 81, 100])
        .and_random();
    let root = square.sqrt();
    let sign = InequalitySign::random_greater();
    let swapped_sign = sign.swapped();
    let answer = get_answer(id, lang)?.replace_multiple(&[
        ("less", format!("$x {swapped_sign} -{root}$")),
        // less is more! :)
        ("more", format!("$x {sign} {root}$")),
    ]);

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("eq", format!("$x^2 = {square}$")),
        ("eq_answer", format!("$x = plus.minus {root}$")),
        ("ineq_answer", answer.clone()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$x^2 {sign} {square}$"),
        answer,
        solution,
        identifiers: square,
        combinations: sq_range,
    }))
}
