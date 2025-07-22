use std::collections::HashMap;

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

//#################################
//#       COURSE STRUCTURE        #
//#################################

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    fn get_desc<T: Into<String>>(&self, lang: T) -> Result<String> {
        let lang_str: String = lang.into();
        let desc = self
            .desc()
            .get(&lang_str)
            .ok_or(Error::NoDescriptionForLang {
                name: self.name().clone(),
                lang: lang_str,
            })?
            .clone();
        Ok(desc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemData {
    pub name: String,
    pub desc: HashMap<String, String>,
    #[serde(default)]
    pub question: HashMap<String, String>,
    #[serde(default)]
    pub answer: HashMap<String, String>,
    #[serde(default)]
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

//###############################
//#          API TYPES          #
//###############################
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WriteSolutions {
    All,
    None,
    First,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentOptions {
    pub lang: String,
    pub write_solutions: WriteSolutions,
    pub file_name: String,
    pub color: bool,
    pub heading: String,
    pub paper_size: PaperSize,
    pub x_margin: u8,
    pub y_margin: u8,
    pub enum_spacing: u8,
    pub par_spacing: u8,
    pub answer_columns: u8,
}

impl Default for DocumentOptions {
    fn default() -> Self {
        DocumentOptions {
            lang: "sv".to_string(),
            write_solutions: WriteSolutions::First,
            file_name: String::from("stencil"),
            color: true,
            heading: String::new(),
            paper_size: PaperSize::A4,
            x_margin: 20,
            y_margin: 20,
            par_spacing: 6,
            enum_spacing: 6,
            answer_columns: 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProblemSetData {
    pub key: usize,
    pub ids: Vec<TopicData>,
    pub exclusions: Vec<String>,
    pub starting_difficulty: Difficulty,
    pub ending_difficulty: Difficulty,
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
            n: 10,
            options: SetRenderingOptions {
                question_columns: 2,
                title: String::new(),
                spacing: 6,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SetRenderingOptions {
    pub question_columns: u8,
    pub title: String,
    pub spacing: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendableProblemSetData {
    pub ids: Vec<String>,
    #[serde(default)]
    pub exclusions: Vec<String>,
    pub starting_difficulty: Difficulty,
    pub ending_difficulty: Difficulty,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaperSize {
    A4,
    A5,
}
impl PaperSize {
    pub fn from(name: &str) -> PaperSize {
        match name {
            "a4" => PaperSize::A4,
            "a5" => PaperSize::A5,
            _ => PaperSize::A4,
        }
    }
    pub fn to_typst(&self) -> &str {
        match self {
            PaperSize::A4 => "a4",
            PaperSize::A5 => "a5",
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            PaperSize::A4 => "A4",
            PaperSize::A5 => "A5",
        }
    }
}

//#################################
//#          DIFFICULTY           #
//#################################

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, PartialOrd)]
pub enum Difficulty {
    Intro,
    #[default]
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn num_to_enum(difficulty_number: u8) -> Result<Difficulty> {
        match difficulty_number {
            0 | 1 => Ok(Difficulty::Intro),
            2 | 3 | 4 => Ok(Difficulty::Easy),
            5 | 6 | 7 => Ok(Difficulty::Medium),
            8 | 9 | 10 => Ok(Difficulty::Hard),
            _ => Err(Error::InvalidDifficultyNumber {
                difficulty: difficulty_number,
            }),
        }
    }

    pub fn enum_to_nums(difficulty: Difficulty) -> Vec<u8> {
        match difficulty {
            Difficulty::Intro => vec![0, 1],
            Difficulty::Easy => vec![2, 3, 4],
            Difficulty::Medium => vec![5, 6, 7],
            Difficulty::Hard => vec![8, 9, 10],
        }
    }

    pub fn enums_to_nums(
        starting_difficulty: Difficulty,
        ending_difficulty: Difficulty,
    ) -> Vec<u8> {
        let minimum_number = match starting_difficulty {
            Difficulty::Intro => 0,
            Difficulty::Easy => 2,
            Difficulty::Medium => 5,
            Difficulty::Hard => 8,
        };

        let maximum_number = match ending_difficulty {
            Difficulty::Intro => 1,
            Difficulty::Easy => 4,
            Difficulty::Medium => 7,
            Difficulty::Hard => 10,
        };

        (minimum_number..=maximum_number).collect()
    }
    pub fn str_to_enum(s: &str) -> Difficulty {
        match s {
            "difficulty_intro" => Difficulty::Intro,
            "difficulty_easy" => Difficulty::Easy,
            "difficulty_medium" => Difficulty::Medium,
            "difficulty_hard" => Difficulty::Hard,
            _ => panic!("Don't call str_to_enum with another string you dummy"),
        }
    }

    pub fn to_str(&self) -> String {
        let s = match self {
            Difficulty::Intro => "difficulty_intro",
            Difficulty::Easy => "difficulty_easy",
            Difficulty::Medium => "difficulty_medium",
            Difficulty::Hard => "difficulty_hard",
        };
        s.to_string()
    }
}
