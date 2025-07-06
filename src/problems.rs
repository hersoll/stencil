//#################################
//#           IMPORTS             #
//#################################
pub mod ma1;
pub mod range;
pub mod set_builder;

//#################################
//#          FLATTENING           #
//#################################
pub use ma1::*;
pub use range::*;
pub use set_builder::*;
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]

//#################################
//#          DIFFICULTY           #
//#################################

pub enum Difficulty {
    Intro,
    #[default]
    Easy,
    Medium,
    Hard,
}

//#################################
//#   PROBLEM ENUMS AND STRUCTS   #
//#################################

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProblemId {
    pub name: String,
    pub identifiers: Vec<i32>,
    pub combinations: usize,
}

impl ProblemId {
    pub fn new<T: Into<String>>(name: T, identifiers: Vec<i32>, combinations: usize) -> ProblemId {
        ProblemId {
            name: name.into(),
            identifiers,
            combinations,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub question: String,
    pub answer: String,
    pub solution: String,
    pub id: ProblemId,
}

impl Problem {
    pub fn new(question: impl ToString, answer: impl ToString) -> Problem {
        Problem {
            question: question.to_string(),
            answer: answer.to_string(),
            solution: String::new(),
            id: ProblemId::default(),
        }
    }
}

pub trait ProblemArea {
    fn get_problem_types() -> &'static [&'static ProblemType];
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct ProblemType {
    pub difficulty: Difficulty,
    pub weight: u8,
    pub generator: fn() -> Problem,
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
                question: String::from("question"),
                answer: String::from("answer"),
                solution: String::new(),
                id: ProblemId {
                    name: String::new(),
                    identifiers: Vec::new(),
                    combinations: 0
                },
            }
        )
    }
}
