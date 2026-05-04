use crate::{Answer, Question, Solution, common::DbDescRow};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use types::lang::Language;

// ###########################
// Traits
// ###########################
pub trait HasDesc {
    fn desc(&self) -> &DescriptionTranslations;

    fn get_desc(&self, lang: Language) -> String {
        match lang {
            Language::Sv => self.desc().sv.clone(),
            Language::En => self.desc().en.clone(),
        }
    }
}

/// Questions, Answers and Solutions might have template strings in them which will be replaced at
/// runtime. This trait enables you to retrieve the strings and replace the templating
pub trait HasReplacements {
    // Does this need String, or can it be &str?
    fn get_str(&self) -> &String;
    /// Used in problems with dynamic text questions, for example:
    /// "Use the function {f} to solve..."
    fn replace_placeholders(&self, values: &[(&str, impl Display)]) -> String {
        let mut result = self.get_str().to_owned();
        for (key, value) in values {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value.to_string());
        }
        result
    }
}

// ###########################
// Nested structs
// ###########################

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptionTranslations {
    pub sv: String,
    pub en: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemTranslations {
    pub sv: TranslatedProblem,
    pub en: TranslatedProblem,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranslatedProblem {
    pub question: Question,
    pub answer: Answer,
    pub solution: Solution,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixTranslations {
    pub sv: TranslatedPrefix,
    pub en: TranslatedPrefix,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranslatedPrefix {
    pub text: String,
    pub group_text: String,
}

// ###########################
// Entry structs
// ###########################

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
    pub chapter_ids: Vec<i32>,
}
impl HasDesc for CourseEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for CourseEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        CourseEntry {
            id,
            name,
            desc,
            chapter_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
    pub course_ids: Vec<i32>,
    pub topic_ids: Vec<i32>,
}
impl HasDesc for ChapterEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for ChapterEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        ChapterEntry {
            id,
            name,
            desc,
            course_ids: Vec::new(),
            topic_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
    pub chapter_ids: Vec<i32>,
    pub problem_ids: Vec<i32>,
}
impl HasDesc for TopicEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for TopicEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        TopicEntry {
            id,
            name,
            desc,
            chapter_ids: Vec::new(),
            problem_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
    pub difficulty: i32,
    pub module: String,
    pub prefix_id: Option<i32>,
    pub translations: ProblemTranslations,
    pub topic_ids: Vec<i32>,
}
impl HasDesc for ProblemEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl ProblemEntry {
    pub fn get_question(&self, lang: Language) -> &Question {
        match lang {
            Language::Sv => &self.translations.sv.question,
            Language::En => &self.translations.en.question,
        }
    }
    pub fn get_answer(&self, lang: Language) -> &Answer {
        match lang {
            Language::Sv => &self.translations.sv.answer,
            Language::En => &self.translations.en.answer,
        }
    }
    pub fn get_solution(&self, lang: Language) -> &Solution {
        match lang {
            Language::Sv => &self.translations.sv.solution,
            Language::En => &self.translations.en.solution,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixEntry {
    pub id: i32,
    pub name: String,
    pub translations: PrefixTranslations,
}
impl PrefixEntry {
    pub fn get_text(&self, lang: Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.text,
            Language::En => &self.translations.en.text,
        }
    }
    pub fn get_group_text(&self, lang: Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.group_text,
            Language::En => &self.translations.en.group_text,
        }
    }
}
