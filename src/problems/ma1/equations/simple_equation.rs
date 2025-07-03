use crate::{metadata, problems::ProblemType};
use rand::prelude::*;

metadata! {
    map SimpleEquation to SimpleEquations {
        OnlyAdditionOrSubtraction: (only_addition_or_subtraction, Difficulty::Intro),
        OnlyMultiplication: (only_multiplication, Difficulty::Intro, 2),
    }
}

impl SimpleEquation {
    fn only_addition_or_subtraction(&self) -> Problem {
        let mut rng = rand::rng();
        let answer: i8 = rng.random_range(2..=10);
        let constant: i8 = rng.random_range(-9..=9);
        Problem {
            question: format!("$x {:+} = {}$", constant, answer + constant),
            answer: format!("$x = {}$", answer),
            solution: String::new(),
        }
    }
    fn only_multiplication(&self) -> Problem {
        let mut rng = rand::rng();
        let answer: u8 = rng.random_range(2..=5);
        let coefficient: u8 = rng.random_range(3..=9);
        Problem {
            question: format!("${}x = {}$", coefficient, answer * coefficient),
            answer: format!("$x = {}", answer),
            solution: String::new(),
        }
    }
}
