use crate::{Error, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// A map between problem names (simple-equations-default) and ProblemTypes
pub static PROBLEM_REGISTRY: Lazy<Mutex<HashMap<String, super::ProblemType>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

//#################################
//#       COURSE STRUCTURE        #
//#################################

#[derive(Debug, Clone, PartialEq)]
pub struct ProblemRegistry {
    pub courses: Vec<CourseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub chapters: Vec<ChapterData>,
}
impl HasDesc for CourseData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub topics: Vec<TopicData>,
}
impl HasDesc for ChapterData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub problems: Vec<ProblemData>,
}
impl HasDesc for TopicData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}
pub trait HasDesc {
    fn desc(&self) -> &HashMap<String, String>;
    fn name(&self) -> &String;
    fn get_desc(&self, lang: String) -> Result<String> {
        let desc = self
            .desc()
            .get(&lang)
            .ok_or(Error::NoDescriptionForLang {
                name: self.name().clone(),
                lang,
            })?
            .clone();
        Ok(desc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub question: HashMap<String, String>,
    pub answer: HashMap<String, String>,
    pub solution: HashMap<String, String>,
}
impl ProblemData {
    pub fn get_question(&self, lang: String) -> Result<String> {
        let question = self
            .question            
            .get(&lang)
            .ok_or(Error::NoQuestionForLang {
                name: self.name().clone(),
                lang,
            })?
            .clone();
        Ok(question)
    }

    pub fn get_answer(&self, lang: String) -> Result<String> {
        let answer = self
            .answer            
            .get(&lang)
            .ok_or(Error::NoAnswerForLang {
                name: self.name().clone(),
                lang,
            })?
            .clone();
        Ok(answer)
    }

    pub fn get_solution(&self, lang: String) -> Result<String> {
        let solution = self
            .solution            
            .get(&lang)
            .ok_or(Error::NoSolutionForLang {
                name: self.name().clone(),
                lang,
            })?
            .clone();
        Ok(solution)
    }
}
impl HasDesc for ProblemData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}
