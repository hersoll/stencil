use crate::{Language, db::common::DbDescRow};
use serde::{Deserialize, Serialize};
use tracing::error;

// ###########################
// Traits
// ###########################
pub trait HasDesc {
    fn desc(&self) -> &DescriptionTranslations;

    fn get_desc(&self, lang: &Language) -> String {
        match lang {
            Language::Sv => self.desc().sv.clone(),
            Language::En => self.desc().en.clone(),
        }
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
    pub question: Option<String>,
    pub answer: Option<String>,
    pub solution: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct CourseEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
}
impl HasDesc for CourseEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for CourseEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        CourseEntry { id, name, desc }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChapterEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
}
impl HasDesc for ChapterEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for ChapterEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        ChapterEntry { id, name, desc }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
}
impl HasDesc for TopicEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for TopicEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        TopicEntry { id, name, desc }
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
}
impl HasDesc for ProblemEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl ProblemEntry {
    pub fn get_question(&self, lang: &Language) -> &str {
        let question = match lang {
            Language::Sv => &self.translations.sv.question,
            Language::En => &self.translations.en.question,
        };

        question.as_deref().unwrap_or_else(|| {
            error!(
                "get_question() was called on problem {}, but there is no question.",
                self.name
            );
            ""
        })
    }
    pub fn get_answer(&self, lang: &Language) -> &str {
        let answer = match lang {
            Language::Sv => &self.translations.sv.answer,
            Language::En => &self.translations.en.answer,
        };

        answer.as_deref().unwrap_or_else(|| {
            error!(
                "get_answer() was called on problem {}, but there is no question.",
                self.name
            );
            ""
        })
    }
    pub fn get_solution(&self, lang: &Language) -> &str {
        let solution = match lang {
            Language::Sv => &self.translations.sv.solution,
            Language::En => &self.translations.en.solution,
        };

        solution.as_deref().unwrap_or_else(|| {
            error!(
                "get_solution() was called on problem {}, but there is no question.",
                self.name
            );
            ""
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixEntry {
    pub id: i32,
    pub name: String,
    pub translations: PrefixTranslations,
}
impl PrefixEntry {
    pub fn get_text(&self, lang: &Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.text,
            Language::En => &self.translations.en.text,
        }
    }
    pub fn get_group_text(&self, lang: &Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.group_text,
            Language::En => &self.translations.en.group_text,
        }
    }
}
