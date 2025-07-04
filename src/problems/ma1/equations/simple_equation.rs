use crate::problems::{Difficulty, Problem, ProblemType};
use rand::prelude::*;

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
    let answer_range = 2..=10;
    let constant_range = -9..10;
    let mut rng = rand::rng();
    let answer: i8 = rng.random_range(answer_range);
    let mut constant: i8 = rng.random_range(constant_range.clone());
    if constant == 0 {
        constant = 9;
    }
    Problem {
        question: format!("$x {:+} = {}$", constant, answer + constant),
        answer: format!("$x = {}$", answer),
        solution: String::new(),
        id: (
            "lin-eq-add-sub".to_string(),
            vec![constant as i32],
            constant_range.count() - 1,
        ),
    }
}

fn only_multiplication() -> Problem {
    let answer_range = 2..6;
    let coefficient_range = 3..10;
    let mut rng = rand::rng();
    let answer: u8 = rng.random_range(answer_range);
    let coefficient: u8 = rng.random_range(coefficient_range.clone());
    Problem {
        question: format!("${}x = {}$", coefficient, answer * coefficient),
        answer: format!("$x = {}$", answer),
        solution: String::new(),
        id: (
            "lin-eq-only-mult".to_string(),
            vec![coefficient as i32],
            coefficient_range.count(),
        ),
    }
}
