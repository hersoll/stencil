use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct CourseData {
    pub id: i32,
    pub name: String,
    pub desc_sv: String,
    pub desc_en: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct ChapterData {
    pub id: i32,
    pub name: String,
    pub desc_sv: String,
    pub desc_en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct TopicData {
    pub id: i32,
    pub name: String,
    pub desc_sv: String,
    pub desc_en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct ProblemData {
    pub id: i32,
    pub name: String,
    pub difficulty: i32,
    pub desc_sv: String,
    pub desc_en: String,
    pub question_sv: Option<String>,
    pub question_en: Option<String>,
    pub answer_sv: Option<String>,
    pub answer_en: Option<String>,
    pub solution_sv: Option<String>,
    pub solution_en: Option<String>,
    pub module: String,
    pub prefix_id: Option<i32>,
}
