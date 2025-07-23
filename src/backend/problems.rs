use crate::{Error, Result};
//#################################
//#           IMPORTS             #
//#################################
pub mod int_range;
pub mod ma1;
pub mod math_utils;
pub mod solutions;
mod symbols;

//#################################
//#   PROBLEM ENUMS AND STRUCTS   #
//#################################

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub solution: String,
    pub identifiers: Vec<i32>,
    pub combinations: usize,
}

impl Problem {
    pub fn new(question: impl ToString, answer: impl ToString) -> Problem {
        Problem {
            question: question.to_string(),
            answer: answer.to_string(),
            ..Default::default()
        }
    }
}

pub type ProblemGenerator = fn(String, &str) -> Result<Problem>;

#[derive(Debug, PartialEq, Clone)]
pub struct ProblemId {
    pub name: String,
    pub identifiers: Vec<i32>,
}

#[derive(Debug, Clone, Eq)]
pub struct ProblemType {
    pub name: String,
    pub difficulty: u8,
    pub generator: ProblemGenerator,
}

impl PartialEq for ProblemType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
//#################################
//#             TESTS             #
//#################################

#[cfg(test)]
mod tests {
    use super::*;

    // PROBLEM STRUCT
    #[test]
    fn problem_initialisation() {
        assert_eq!(
            Problem::new("question", "answer"),
            Problem {
                id: String::new(),
                question: String::from("question"),
                answer: String::from("answer"),
                solution: String::new(),
                identifiers: Vec::new(),
                combinations: 0,
            }
        )
    }
}
