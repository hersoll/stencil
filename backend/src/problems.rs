use anyhow::{Result, anyhow};
//#################################
//#           IMPORTS             #
//#################################
pub mod ma1;
pub mod ma2;
pub mod problem_generator;
pub mod problem_picker;
pub mod solutions;

use serde::{Deserialize, Serialize};

//#################################
//#   PROBLEM ENUMS AND STRUCTS   #
//#################################

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub name: String,
    pub question: String,
    pub answer: String,
    pub solution: String,
    pub identifiers: Vec<i32>,
    pub combinations: usize,
}

//#################################
//#          DIFFICULTY           #
//#################################

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, PartialOrd)]
pub enum Difficulty {
    Intro,
    #[default]
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn all() -> [Difficulty; 4] {
        [
            Difficulty::Intro,
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
        ]
    }

    pub fn num_to_enum(difficulty_number: u8) -> Result<Difficulty> {
        match difficulty_number {
            0 | 1 => Ok(Difficulty::Intro),
            2 | 3 | 4 => Ok(Difficulty::Easy),
            5 | 6 | 7 => Ok(Difficulty::Medium),
            8 | 9 | 10 => Ok(Difficulty::Hard),
            _ => Err(anyhow!(format!(
                "Invalid difficulty number: {difficulty_number}"
            ))),
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

    pub fn enums_to_nums(
        starting_difficulty: Difficulty,
        ending_difficulty: Difficulty,
    ) -> Vec<u8> {
        let minimum_number = match starting_difficulty {
            Difficulty::Intro => 0,
            Difficulty::Easy => 2,
            Difficulty::Medium => 5,
            Difficulty::Hard => 8,
        };

        let maximum_number = match ending_difficulty {
            Difficulty::Intro => 1,
            Difficulty::Easy => 4,
            Difficulty::Medium => 7,
            Difficulty::Hard => 10,
        };

        (minimum_number..=maximum_number).collect()
    }
    pub fn str_to_enum(s: &str) -> Result<Difficulty> {
        match s {
            "difficulty_intro" => Ok(Difficulty::Intro),
            "difficulty_easy" => Ok(Difficulty::Easy),
            "difficulty_medium" => Ok(Difficulty::Medium),
            "difficulty_hard" => Ok(Difficulty::Hard),
            _ => Err(anyhow!("Invalid difficulty string: {s}")),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Difficulty::Intro => "difficulty_intro",
            Difficulty::Easy => "difficulty_easy",
            Difficulty::Medium => "difficulty_medium",
            Difficulty::Hard => "difficulty_hard",
        }
    }
}
