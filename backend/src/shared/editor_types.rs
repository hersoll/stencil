use serde::{Deserialize, Serialize};

use crate::shared::{
    ParsedChapterData, ParsedCourseData, ParsedPrefixData, ParsedProblemData, ParsedTopicData,
};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
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
        if lang == "sv" {
            ParsedProblemData {
                id: self.id,
                name: self.name,
                difficulty: self.difficulty,
                desc: self.desc_sv,
                question: self.question_sv.unwrap_or_default(),
                answer: self.answer_sv.unwrap_or_default(),
                solution: self.solution_sv.unwrap_or_default(),
            }
        } else {
            ParsedProblemData {
                id: self.id,
                name: self.name,
                difficulty: self.difficulty,
                desc: self.desc_en,
                question: self.question_en.unwrap_or_default(),
                answer: self.answer_en.unwrap_or_default(),
                solution: self.solution_en.unwrap_or_default(),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrefixData {
    pub id: i32,
    pub name: String,
    pub text_sv: String,
    pub text_en: String,
    pub group_text_sv: String,
    pub group_text_en: String,
}
impl PrefixData {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            text_sv: String::new(),
            text_en: String::new(),
            group_text_sv: String::new(),
            group_text_en: String::new(),
        }
    }

    pub fn parse(self, lang: &str) -> ParsedPrefixData {
        if lang == "sv" {
            ParsedPrefixData {
                id: self.id,
                name: self.name,
                text: self.text_sv,
                group_text: self.group_text_sv,
            }
        } else {
            ParsedPrefixData {
                id: self.id,
                name: self.name,
                text: self.text_en,
                group_text: self.group_text_en,
            }
        }
    }
}
