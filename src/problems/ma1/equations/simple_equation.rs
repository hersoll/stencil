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
    let mut rng = rand::rng();
    let answer: i8 = rng.random_range(2..=10);
    let constant: i8 = rng.random_range(-9..=9);
    Problem {
        question: format!("$x {:+} = {}$", constant, answer + constant),
        answer: format!("$x = {}$", answer),
        solution: String::new(),
    }
}

fn only_multiplication() -> Problem {
    let mut rng = rand::rng();
    let answer: u8 = rng.random_range(2..=5);
    let coefficient: u8 = rng.random_range(3..=9);
    Problem {
        question: format!("${}x = {}$", coefficient, answer * coefficient),
        answer: format!("$x = {}", answer),
        solution: String::new(),
    }
}
