use dioxus::signals::Signal;
use serde::{Deserialize, Serialize};

use crate::backend::{self, Difficulty, TopicData};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SetRenderingOptions {
    pub question_columns: u8,
    pub title: String,
    pub spacing: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProblemSetData {
    pub key: usize,
    pub ids: Vec<TopicData>,
    pub exclusions: Vec<String>,
    pub starting_difficulty: backend::Difficulty,
    pub ending_difficulty: backend::Difficulty,
    pub n: u8,
    pub options: SetRenderingOptions,
}

impl ProblemSetData {
    pub fn new(key: usize) -> ProblemSetData {
        ProblemSetData {
            key,
            ids: Vec::new(),
            exclusions: Vec::new(),
            starting_difficulty: Difficulty::Intro,
            ending_difficulty: Difficulty::Intro,
            n: 5,
            options: SetRenderingOptions {
                question_columns: 2,
                title: String::new(),
                spacing: 6,
            },
        }
    }
}

pub type Sets = Vec<Signal<ProblemSetData>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendableProblemSetData {
    pub ids: Vec<String>,
    #[serde(default)]
    pub exclusions: Vec<String>,
    pub starting_difficulty: backend::Difficulty,
    pub ending_difficulty: backend::Difficulty,
    pub n: u8,
    pub options: SetRenderingOptions,
}

impl From<ProblemSetData> for SendableProblemSetData {
    fn from(data: ProblemSetData) -> Self {
        SendableProblemSetData {
            ids: data.ids.into_iter().map(|topic| topic.name).collect(),
            exclusions: data.exclusions,
            starting_difficulty: data.starting_difficulty,
            ending_difficulty: data.ending_difficulty,
            n: data.n,
            options: data.options,
        }
    }
}
