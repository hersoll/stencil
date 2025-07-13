use serde::{Deserialize, Serialize};

use crate::backend;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemSetData {
    ids: Vec<String>,
    starting_difficulty: backend::Difficulty,
    ending_difficulty: backend::Difficulty,
    n: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetData(pub Vec<ProblemSetData>);
