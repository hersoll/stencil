use crate::{Error, Result};
use std::collections::HashMap;
//#################################
//#           IMPORTS             #
//#################################
pub mod int_range;
pub mod ma1;
mod registry;
mod symbols;

//#################################
//#          FLATTENING           #
//#################################
pub use ma1::*;
pub use registry::PROBLEM_REGISTRY;
use serde::Deserialize;
use serde::Serialize;

//#################################
//#       COURSE STRUCTURE        #
//#################################

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemRegistry {
    pub courses: Vec<Course>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Course {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chapter {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub topics: Vec<Topic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Topic {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub problems: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

//#################################
//#          DIFFICULTY           #
//#################################

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum Difficulty {
    Intro,
    #[default]
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn num_to_enum(difficulty_number: u8) -> Result<Difficulty> {
        match difficulty_number {
            0 | 1 => Ok(Difficulty::Intro),
            2 | 3 | 4 => Ok(Difficulty::Easy),
            5 | 6 | 7 => Ok(Difficulty::Medium),
            8 | 9 | 10 => Ok(Difficulty::Hard),
            _ => Err(Error::InvalidDifficulty {
                difficulty: difficulty_number,
            }),
        }
    }

    pub fn enum_to_nums(difficulty: Difficulty) -> Vec<u8> {
        match difficulty {
            Difficulty::Intro => vec![0, 1],
            Difficulty::Easy => vec![2, 3, 4],
            Difficulty::Medium => vec![5, 6, 7],
            Difficulty::Hard => vec![8, 9, 10],
        }
    }
}

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

pub type ProblemGenerator = fn(String) -> Result<Problem>;

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
        self.generator as usize == other.generator as usize
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
