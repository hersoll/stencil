use serde::{Deserialize, Serialize};

use crate::shared::{ParsedChapterData, ParsedCourseData, ParsedProblemData, ParsedTopicData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct CourseData {
    pub id: i32,
    pub name: String,
    pub desc_sv: String,
    pub desc_en: String,
}
impl CourseData {
    pub fn parse(self, lang: &str) -> ParsedCourseData {
        ParsedCourseData {
            id: self.id,
            name: self.name,
            desc: if lang == "sv" {
                self.desc_sv
            } else {
                self.desc_en
            },
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct ChapterData {
    pub id: i32,
    pub name: String,
    pub desc_sv: String,
    pub desc_en: String,
}
impl ChapterData {
    pub fn parse(self, lang: &str) -> ParsedChapterData {
        ParsedChapterData {
            id: self.id,
            name: self.name,
            desc: if lang == "sv" {
                self.desc_sv
            } else {
                self.desc_en
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct TopicData {
    pub id: i32,
    pub name: String,
    pub desc_sv: String,
    pub desc_en: String,
}
impl TopicData {
    pub fn parse(self, lang: &str) -> ParsedTopicData {
        ParsedTopicData {
            id: self.id,
            name: self.name,
            desc: if lang == "sv" {
                self.desc_sv
            } else {
                self.desc_en
            },
        }
    }
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
impl ProblemData {
    pub fn parse(self, lang: &str) -> ParsedProblemData {
        ParsedProblemData {
            id: self.id,
            name: self.name,
            difficulty: self.difficulty,
            desc: if lang == "sv" {
                self.desc_sv
            } else {
                self.desc_en
            },
        }
    }
}
