use crate::backend::IntRange;
use crate::backend::problems::{Problem, ProblemId};
use crate::backend::typst_formatting;
use macros::problem;
use crate::Result;

#[problem(id = "add_sub_only", difficulty = 0)]
fn only_addition_or_subtraction() -> Result<Problem> {
    let (answer, _answer_range) = IntRange::with_zero(0, 9)?.and_random();
    let (constant, constant_range) = IntRange::without_zero(-answer, 9)?.and_random();

    let solution = format!(
        "x {c:+} &= {rhs}\\ {i:+} \\
              x &= {a}\\",
        c = constant,
        a = answer,
        rhs = answer + constant,
        i = -constant
    );

    let problem = Problem {
        question: format!("$x {constant:+} = {}$", answer + constant),
        answer: format!("$x = {answer}$"),
        solution: typst_formatting::equation_solution(solution),
        id: ProblemId::new("1-e-simp-add-sub", vec![constant], constant_range.len()),
    };
    Ok(problem)
}

#[problem(id = "mult_only", difficulty = 0)]
fn only_multiplication() -> Result<Problem> {
    let (answer, _answer_range) = IntRange::without_zero(2, 5)?.and_random();
    let (coefficient, coefficient_range) = IntRange::without_zero(3, 9)?.and_random();

    let solution = format!(
        "{c}x &= {rhs} \\ div {c}\\
              x &= {a} \\",
        c = coefficient,
        a = answer,
        rhs = answer * coefficient,
    );

    let problem = Problem {
        question: format!("${}x = {}$", coefficient, answer * coefficient),
        answer: format!("$x = {}$", answer),
        solution: typst_formatting::equation_solution(solution),
        id: ProblemId::new("1-e-simp-mul", vec![coefficient], coefficient_range.len()),
    };

    Ok(problem)
}

#[problem(id = "up_to_5", difficulty = 1)]
fn default_equation_positive_up_to_5() -> Result<Problem> {
    let (answer, _answer_range) = IntRange::without_zero(0, 5)?.and_random();
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 5)?.and_random();
    let (constant, constant_range) =
        IntRange::without_zero((-coefficient * answer).max(-5), 5)?.and_random();

    let solution = format!(
        "{cf}x {co:+} = {rhs} \\ {m_co:+} \\
                            {cf}x = {cf_a} \\ div {cf} \\
                                x = {a} \\",
        cf = coefficient,
        co = constant,
        rhs = coefficient * answer + constant,
        m_co = -constant,
        cf_a = coefficient * answer,
        a = answer
    );

    let problem = Problem {
        question: format!(
            "${cf}x {co:+} = {rhs}$",
            cf = coefficient,
            co = constant,
            rhs = coefficient * answer + constant
        ),
        answer: format!("$x = {}$", answer),
        solution: typst_formatting::equation_solution(solution),
        id: ProblemId::new(
            "1-e-simp-default-positive",
            vec![coefficient, constant],
            coefficient_range.len() * constant_range.len(),
        ),
    };
    Ok(problem)
}

#[problem(id = "default_positive", difficulty = 2)]
fn default_equation_positive_answers() -> Result<Problem> {
    let (answer, _answer_range) = IntRange::without_zero(0, 10)?.and_random();
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 9)?.and_random();
    let (constant, constant_range) =
        IntRange::without_zero((-coefficient * answer).max(-10), 10)?.and_random();

    let solution = format!(
        "{cf}x {co:+} = {rhs} \\ {m_co:+} \\
                            {cf}x = {cf_a} \\ div {cf} \\
                                x = {a} \\",
        cf = coefficient,
        co = constant,
        rhs = coefficient * answer + constant,
        m_co = -constant,
        cf_a = coefficient * answer,
        a = answer
    );

    let problem = Problem {
        question: format!(
            "${cf}x {co:+} = {rhs}$",
            cf = coefficient,
            co = constant,
            rhs = coefficient * answer + constant
        ),
        answer: format!("$x = {}$", answer),
        solution: typst_formatting::equation_solution(solution),
        id: ProblemId::new(
            "1-e-simp-default-positive",
            vec![coefficient, constant],
            coefficient_range.len() * constant_range.len(),
        ),
    };
    Ok(problem)
}
