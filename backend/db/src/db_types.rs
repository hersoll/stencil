use crate::{Answer, Question, Solution, common::DbDescRow};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use types::lang::Language;

/// Trait for all structs with a [`DescriptionTranslations`] field.
///
/// Used to easily access the description in the [`Language`] you want.
pub trait HasDesc {
    /// Accesses the [`DescriptionTranslations`] field.
    ///
    /// Required for `get_desc()`, not intended for external use.
    fn desc(&self) -> &DescriptionTranslations;

    /// Get the description in the specified [`Language`].
    fn get_desc(&self, lang: Language) -> String {
        match lang {
            Language::Sv => self.desc().sv.clone(),
            Language::En => self.desc().en.clone(),
        }
    }
}

/// Trait for structs with a [`String`] that contains `{placeholders}`
///
/// Questions, Answers and Solutions might have template strings in them which will be replaced at
/// runtime. This trait enables you to retrieve the strings and replace the templating.
pub trait HasReplacements {
    /// Accesses the templated [`String`] used for replacement.
    fn get_str(&self) -> &String;
    /// Takes a template string containing `{key}` patterns and replaces those keys with values.
    ///
    /// Used in problems with dynamic text questions, for example:
    /// `"Use the function {f} to solve..."`
    fn replace_placeholders(&self, key_value_pairs: &[(&str, impl Display)]) -> String {
        let mut result = self.get_str().to_owned();
        for (key, value) in key_value_pairs {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value.to_string());
        }
        result
    }
}

/// The texts associated with a specific problem in a certain [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemTexts {
    pub question: Question,
    pub answer: Answer,
    pub solution: Solution,
}

/// The texts associated with a specific prefix in a certain [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixTexts {
    pub text: String,
    pub group_text: String,
}

/// Contains [`ProblemTexts`] for every [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemTranslations {
    pub sv: ProblemTexts,
    pub en: ProblemTexts,
}

/// Contains [`PrefixTexts`] for every [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixTranslations {
    pub sv: PrefixTexts,
    pub en: PrefixTexts,
}

/// Contains a description for every [`Language`]
///
/// Descriptions are used on multiple structs. They explain to the user what that
/// specific problem, topic, etc. is.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptionTranslations {
    pub sv: String,
    pub en: String,
}

/// Representation of data about a course from the DB
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

/// Representation of data about a chapter from the DB
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

/// Representation of data about a topic from the DB
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

/// Representation of data about a problem from the DB
///
/// A common pattern is to read this data during problem generation to get question/answer/solution text
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

/// Representation of prefix data in the DB
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixEntry {
    pub id: i32,
    pub name: String,
    pub translations: PrefixTranslations,
}
impl PrefixEntry {
    /// Get the text in a specific [`Language`] for a prefix in its singular form.
    pub fn get_text(&self, lang: Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.text,
            Language::En => &self.translations.en.text,
        }
    }

    /// Get the text in a specific [`Language`] for a prefix in its group form.
    pub fn get_group_text(&self, lang: Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.group_text,
            Language::En => &self.translations.en.group_text,
        }
    }
}
