use serde::{Deserialize, Serialize};

use crate::backend::{self, Difficulty};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemSetData {
    pub ids: Vec<String>,
    pub starting_difficulty: backend::Difficulty,
    pub ending_difficulty: backend::Difficulty,
    pub n: u8,
}

impl ProblemSetData {
    pub fn new() -> ProblemSetData {
        ProblemSetData {
            ids: Vec::new(),
            starting_difficulty: Difficulty::Intro,
            ending_difficulty: Difficulty::Hard,
            n: 5,
        }
    }
}

pub type Sets = Vec<ProblemSetData>;
