use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

//#################################
//#       COURSE STRUCTURE        #
//#################################

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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WriteSolutions {
    All,
    None,
    First,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentOptions {
    pub font_size: u8,
    pub heading: String,
    pub answer_columns: u8,
    pub lang: String,
    pub write_solutions: WriteSolutions,
    pub file_name: String,
    pub color: bool,
    pub paper_size: PaperSize,
    pub x_margin: u8,
    pub y_margin: u8,
    pub par_spacing: Option<u8>,
    pub max_prefix_group: Option<u8>,
}

impl Default for DocumentOptions {
    fn default() -> Self {
        DocumentOptions {
            font_size: 10,
            lang: "sv".to_string(),
            write_solutions: WriteSolutions::First,
            file_name: String::from("stencil"),
            color: true,
            heading: String::new(),
            paper_size: PaperSize::A4,
            x_margin: 20,
            y_margin: 20,
            par_spacing: None,
            answer_columns: 3,
            max_prefix_group: Some(3),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProblemSetData {
    pub key: usize,
    pub topics: Vec<i32>,
    pub exclusions: Vec<i32>,
    pub starting_difficulty: Difficulty,
    pub ending_difficulty: Difficulty,
    pub n: u8,
    pub options: SetRenderingOptions,
}

impl ProblemSetData {
    pub fn new(key: usize) -> ProblemSetData {
        ProblemSetData {
            key,
            topics: Vec::new(),
            exclusions: Vec::new(),
            starting_difficulty: Difficulty::Intro,
            ending_difficulty: Difficulty::Hard,
            n: 10,
            options: SetRenderingOptions {
                question_columns: 2,
                title: String::new(),
                spacing: None,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SetRenderingOptions {
    pub question_columns: u8,
    pub title: String,
    pub spacing: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendableProblemSetData {
    pub topics: Vec<i32>,
    #[serde(default)]
    pub exclusions: Vec<i32>,
    pub starting_difficulty: Difficulty,
    pub ending_difficulty: Difficulty,
    pub n: u8,
    pub options: SetRenderingOptions,
}

impl From<ProblemSetData> for SendableProblemSetData {
    fn from(data: ProblemSetData) -> Self {
        SendableProblemSetData {
            topics: data.topics,
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
            _ => Err(anyhow!(format!(
                "Invalid difficulty number: {difficulty_number}"
            ))),
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
