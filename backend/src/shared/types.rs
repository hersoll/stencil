use serde::{Deserialize, Serialize};

//#################################
//#       COURSE STRUCTURE        #
//#################################

/*
 TODO: Suggestion:

pub struct ProblemTemplate {
    pub id: i32,
    pub name: String,
    pub difficulty: i32,
    pub module: String,
    pub prefix_id: Option<i32>,
    pub localizations: ProblemLocalizations,
}

pub struct ProblemLocalizations {
    pub sv: LocalizedContent,
    pub en: LocalizedContent,
}

pub struct LocalizedContent {
    pub desc: String,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub solution: Option<String>,
}
*/

#[derive(Debug, sqlx::FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedCourseData {
    pub id: i32,
    pub name: String,
    pub desc: String,
}
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
pub struct ParsedChapterData {
    pub id: i32,
    pub name: String,
    pub desc: String,
}
#[derive(Debug, Clone, Serialize, sqlx::FromRow, Deserialize, PartialEq)]
pub struct ParsedTopicData {
    pub id: i32,
    pub name: String,
    pub desc: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, sqlx:: FromRow, PartialEq)]
pub struct ParsedProblemData {
    pub id: i32,
    pub name: String,
    pub difficulty: i32,
    pub desc: String,
    pub question: String,
    pub answer: String,
    pub solution: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct ParsedPrefixData {
    pub id: i32,
    pub name: String,
    pub text: String,
    pub group_text: String,
}
//###############################
//#          API TYPES          #
//###############################
