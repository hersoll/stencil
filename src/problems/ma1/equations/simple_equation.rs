use crate::problems::{Difficulty, IntRange, Problem, ProblemId, ProblemType};

crate::collect_into!(
    SimpleLinearEquations {
        ONLY_ADDITION_OR_SUBTRACTION = ProblemType {
            difficulty: Difficulty::Intro,
            weight: 1,
            generator: only_addition_or_subtraction,
        },
        ONLY_MULTIPLICATION = ProblemType {
            difficulty: Difficulty::Intro,
            weight: 2,
            generator: only_multiplication,
        },
    }
);

fn only_addition_or_subtraction() -> Problem {
    let answer_range = IntRange::with_zero(0, 9);
    let answer = answer_range.random();
    let constant_range = IntRange::without_zero(-answer, 9);
    let constant = constant_range.random();

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
        solution,
        id: ProblemId::new("1-e-s-add-sub", vec![constant], constant_range.len()),
    }
}

fn only_multiplication() -> Problem {
    let answer_range = IntRange::without_zero(2, 5);
    let coefficient_range = IntRange::without_zero(3, 9);
    let answer = answer_range.random();
    let coefficient = coefficient_range.random();

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
        solution,
        id: ProblemId::new("1-e-s-mul", vec![coefficient], coefficient_range.len()),
    }
}
