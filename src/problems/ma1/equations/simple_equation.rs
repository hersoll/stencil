use crate::problems::{Difficulty, IntRange, Problem, ProblemType};

crate::collect_into!(SimpleLinearEquations {
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
});

fn only_addition_or_subtraction() -> Problem {
    let answer_range = IntRange::with_zero(0, 9);
    let constant_range = IntRange::without_zero(-9, 9);
    let answer = answer_range.random();
    let constant = constant_range.random();
    Problem {
        question: format!("$x {:+} = {}$", constant, answer + constant),
        answer: format!("$x = {}$", answer),
        solution: String::new(),
        id: (
            "lin-eq-add-sub".to_string(),
            vec![constant],
            constant_range.len(),
        ),
    }
}

fn only_multiplication() -> Problem {
    let answer_range = IntRange::without_zero(2, 5);
    let coefficient_range = IntRange::without_zero(3, 9);
    let answer = answer_range.random();
    let coefficient = coefficient_range.random();
    Problem {
        question: format!("${}x = {}$", coefficient, answer * coefficient),
        answer: format!("$x = {}$", answer),
        solution: String::new(),
        id: (
            "lin-eq-only-mult".to_string(),
            vec![coefficient],
            coefficient_range.len(),
        ),
    }
}
