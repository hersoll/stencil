use serde::{Deserialize, Serialize};

use crate::backend::{self, Difficulty, TopicData};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemSetData {
    pub ids: Vec<TopicData>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendableProblemSetData {
    pub ids: Vec<String>,
    pub starting_difficulty: backend::Difficulty,
    pub ending_difficulty: backend::Difficulty,
    pub n: u8,
}

impl From<ProblemSetData> for SendableProblemSetData {
    fn from(data: ProblemSetData) -> Self {
        SendableProblemSetData {
            ids: data.ids.into_iter().map(|topic| topic.name).collect(),
            starting_difficulty: data.starting_difficulty,
            ending_difficulty: data.ending_difficulty,
            n: data.n,
        }
    }
}
