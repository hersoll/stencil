use std::fmt::Display;

use crate::{difficulty::DifficultyCategory, lang::Language};
use serde::{Deserialize, Deserializer, Serialize};

// QuestionSetFormattingOptions::default()
const DEFAULT_QUESTION_COLUMNS: u8 = 2;
const DEFAULT_HEADING: Option<SanitizedTypstString> = None;
const DEFAULT_PROBLEM_SPACING: Option<u16> = None;
const DEFAULT_PAGEBREAK_AFTER: bool = false;

// DocumentOptions::default()
const DEFAULT_TITLE: Option<SanitizedTypstString> = None;
const DEFAULT_SUBTITLE: Option<SanitizedTypstString> = None;
const DEFAULT_NAME_FIELD: bool = false;
const DEFAULT_ANSWER_COLUMNS: u8 = 3;
const DEFAULT_FONT_SIZE: u8 = 10;
const DEFAULT_X_MARGIN: u8 = 20;
const DEFAULT_Y_MARGIN: u8 = 20;
const DEFAULT_LANG: Language = Language::Sv;
const DEFAULT_MAX_PREFIX_GROUP: u8 = 3;
const DEFAULT_PAPER_SIZE: PaperSize = PaperSize::A4;
const DEFAULT_WRITE_SOLUTIONS: WriteSolutions = WriteSolutions::First;
const DEFAULT_PAR_SPACING: Option<u8> = None;
const DEFAULT_COLORS: bool = true;
const DEFAULT_PAGE_BREAK_BEFORE_ANSWERS: bool = true;

// ProblemOptions::default()
const DEFAULT_STARTING_DIFFICULTY: DifficultyCategory = DifficultyCategory::Intro;
const DEFAULT_ENDING_DIFFICULTY: DifficultyCategory = DifficultyCategory::Hard;
const DEFAULT_PROBLEM_COUNT: u8 = 20;

/// Denotes which chars are removed from any incoming strings
const DISALLOWED_CHARS: &str = "[]#\"'";

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SanitizedTypstString(pub String);

impl<'de> Deserialize<'de> for SanitizedTypstString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut s = String::deserialize(deserializer)?;
        s.retain(|c| !DISALLOWED_CHARS.contains(c));
        Ok(Self(s))
    }
}

/// Which problems should we write out the solutions for?
///
/// All/None - Self-explanatory
/// First - Only the first problem of each problem kind
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WriteSolutions {
    All,
    None,
    First,
}

impl Display for WriteSolutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteSolutions::All => write!(f, "always"),
            WriteSolutions::None => write!(f, "never"),
            WriteSolutions::First => write!(f, "first"),
        }
    }
}

/// The valid paper sizes.
///
/// Having an enum for this makes it easier to validate correct sizes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaperSize {
    A4,
    A5,
}

impl PaperSize {
    pub fn to_str(&self) -> &str {
        match self {
            PaperSize::A4 => "a4",
            PaperSize::A5 => "a5",
        }
    }
}

impl Display for PaperSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaperSize::A4 => write!(f, "a4"),
            PaperSize::A5 => write!(f, "a5"),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub enum SolutionDecoration {
    #[default]
    Fill,
    Border,
    None,
}

impl SolutionDecoration {
    pub fn to_typst(&self) -> &str {
        use SolutionDecoration::*;
        match self {
            Fill => "fill: solution_color,",
            Border => "stroke: solution_color,",
            None => "",
        }
    }
}

/// Information about what to include in the problem set
///
/// Should be included in the HTTP request in the form of a Vec<ProblemSetSpec>
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProblemOptions {
    /// Topics to draw problems from
    pub topics: Vec<i32>,
    /// Which problems to exclude from the generator
    #[serde(default)]
    pub exclusions: Vec<i32>,
    pub starting_difficulty: DifficultyCategory,
    pub ending_difficulty: DifficultyCategory,
    /// Number of problems
    pub n: u8,
}

impl Default for ProblemOptions {
    /// Mostly used for the /pdf/example endpoint
    fn default() -> ProblemOptions {
        ProblemOptions {
            topics: Vec::new(),
            exclusions: Vec::new(),
            starting_difficulty: DEFAULT_STARTING_DIFFICULTY,
            ending_difficulty: DEFAULT_ENDING_DIFFICULTY,
            n: DEFAULT_PROBLEM_COUNT,
        }
    }
}

/// Options that dictate how a QuestionSet should be formatted
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionSetFormattingOptions {
    pub question_columns: u8,
    pub heading: Option<SanitizedTypstString>,
    pub spacing: Option<u16>,
    pub pagebreak_after: bool,
}
impl Default for QuestionSetFormattingOptions {
    fn default() -> Self {
        QuestionSetFormattingOptions {
            question_columns: DEFAULT_QUESTION_COLUMNS,
            heading: DEFAULT_HEADING,
            spacing: DEFAULT_PROBLEM_SPACING,
            pagebreak_after: DEFAULT_PAGEBREAK_AFTER,
        }
    }
}

/// Options that concerns the document as a whole
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOptions {
    pub title: Option<SanitizedTypstString>,
    pub subtitle: Option<SanitizedTypstString>,
    pub name_field: bool,
    pub font_size: u8,
    pub answer_columns: u8,
    pub lang: Language,
    pub write_solutions: WriteSolutions,
    pub color: bool,
    pub paper_size: PaperSize,
    pub x_margin: u8,
    pub y_margin: u8,
    pub par_spacing: Option<u8>,
    pub max_prefix_group: u8,
    pub page_break_before_answers: bool,
    pub solution_decoration: SolutionDecoration,
}

impl Default for DocumentOptions {
    fn default() -> Self {
        DocumentOptions {
            title: DEFAULT_TITLE,
            subtitle: DEFAULT_SUBTITLE,
            name_field: DEFAULT_NAME_FIELD,
            font_size: DEFAULT_FONT_SIZE,
            answer_columns: DEFAULT_ANSWER_COLUMNS,
            lang: DEFAULT_LANG,
            write_solutions: DEFAULT_WRITE_SOLUTIONS,
            color: DEFAULT_COLORS,
            paper_size: DEFAULT_PAPER_SIZE,
            x_margin: DEFAULT_X_MARGIN,
            y_margin: DEFAULT_Y_MARGIN,
            par_spacing: DEFAULT_PAR_SPACING,
            max_prefix_group: DEFAULT_MAX_PREFIX_GROUP,
            page_break_before_answers: DEFAULT_PAGE_BREAK_BEFORE_ANSWERS,
            solution_decoration: SolutionDecoration::default(),
        }
    }
}

/// Information about what to include in the problem set
///
/// Should be included in the HTTP request in the form of a Vec<ProblemSetSpec>
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProblemSetSpec {
    pub problem_options: ProblemOptions,
    /// Typst rendering options
    #[serde(default)]
    pub formatting_options: QuestionSetFormattingOptions,
}

impl ProblemSetSpec {
    /// Mostly used for the /pdf/example endpoint
    pub fn new() -> ProblemSetSpec {
        ProblemSetSpec {
            problem_options: ProblemOptions::default(),
            formatting_options: QuestionSetFormattingOptions::default(),
        }
    }
}

/// What the HTTP request is deserialized into
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PDFRequest {
    /// If another pdf was generated by the user in the same session, this is Some()
    pub previous_pdf: Option<i32>,
    pub sets: Vec<ProblemSetSpec>,
    #[serde(default)]
    pub document_options: DocumentOptions,
}
