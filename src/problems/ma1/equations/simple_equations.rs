use crate::IntRange;
use crate::problems::{Problem, ProblemId, ProblemType};

crate::collect_into!(
    SimpleLinearEquations {
        ONLY_ADDITION_OR_SUBTRACTION = ProblemType {
            difficulty: 0,
            generator: only_addition_or_subtraction,
        },
        ONLY_MULTIPLICATION = ProblemType {
            difficulty: 0,
            generator: only_multiplication,
        },
        DEFAULT_POSITIVE_UP_TO_5 = ProblemType {
            difficulty: 1,
            generator: default_equation_positive_up_to_5,
        },
        DEFAULT_POSITIVE = ProblemType {
            difficulty: 2,
            generator: default_equation_positive_answers,
        },
    }
);

fn only_addition_or_subtraction() -> Problem {
    let (answer, _answer_range) = IntRange::with_zero(0, 9).and_random();
    let (constant, constant_range) = IntRange::without_zero(-answer, 9).and_random();

    let solution = format!(
        "x {c:+} &= {rhs}\\ {i:+} \\
              x &= {a}\\",
        c = constant,
        a = answer,
        rhs = answer + constant,
        i = -constant
    );

    Problem {
        question: format!("$x {constant:+} = {}$", answer + constant),
        answer: format!("$x = {answer}$"),
        solution: crate::equation_solution(solution),
        id: ProblemId::new("1-e-simp-add-sub", vec![constant], constant_range.len()),
    }
}

fn only_multiplication() -> Problem {
    let (answer, _answer_range) = IntRange::without_zero(2, 5).and_random();
    let (coefficient, coefficient_range) = IntRange::without_zero(3, 9).and_random();

    let solution = format!(
        "{c}x &= {rhs} \\ div {c}\\
              x &= {a} \\",
        c = coefficient,
        a = answer,
        rhs = answer * coefficient,
    );

    Problem {
        question: format!("${}x = {}$", coefficient, answer * coefficient),
        answer: format!("$x = {}$", answer),
        solution: crate::equation_solution(solution),
        id: ProblemId::new("1-e-simp-mul", vec![coefficient], coefficient_range.len()),
    }
}

fn default_equation_positive_up_to_5() -> Problem {
    let (answer, _answer_range) = IntRange::without_zero(0, 5).and_random();
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 5).and_random();
    let (constant, constant_range) =
        IntRange::without_zero((-coefficient * answer).max(-5), 5).and_random();

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

    Problem {
        question: format!(
            "${cf}x {co:+} = {rhs}$",
            cf = coefficient,
            co = constant,
            rhs = coefficient * answer + constant
        ),
        answer: format!("$x = {}$", answer),
        solution: crate::equation_solution(solution),
        id: ProblemId::new(
            "1-e-simp-default-positive",
            vec![coefficient, constant],
            coefficient_range.len() * constant_range.len(),
        ),
    }
}

fn default_equation_positive_answers() -> Problem {
    let (answer, _answer_range) = IntRange::without_zero(0, 10).and_random();
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 9).and_random();
    let (constant, constant_range) =
        IntRange::without_zero((-coefficient * answer).max(-10), 10).and_random();

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

    Problem {
        question: format!(
            "${cf}x {co:+} = {rhs}$",
            cf = coefficient,
            co = constant,
            rhs = coefficient * answer + constant
        ),
        answer: format!("$x = {}$", answer),
        solution: crate::equation_solution(solution),
        id: ProblemId::new(
            "1-e-simp-default-positive",
            vec![coefficient, constant],
            coefficient_range.len() * constant_range.len(),
        ),
    }
}
