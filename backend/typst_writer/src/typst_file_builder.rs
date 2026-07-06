use crate::{colors, formatting, preamble::PREAMBLE_STR, prefix_handler};
use anyhow::Result;
use db;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use tracing::{error, warn};
use types::{lang::Language, problems::Problem};

pub const DEFAULT_QUESTION_COLUMNS: u8 = 2;
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

impl AsRef<str> for SanitizedTypstString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A set of questions grouped together in the final PDF
#[derive(Debug, Default)]
pub struct QuestionSet {
    pub questions: Vec<String>,
}
/// A set of answers grouped together in the final PDF
#[derive(Debug, Default)]
pub struct AnswerSet {
    pub answers: Vec<String>,
}

/// Options that dictate how a QuestionSet should be formatted
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionSetOptions {
    pub question_columns: u8,
    pub heading: Option<SanitizedTypstString>,
    pub spacing: Option<u16>,
    pub pagebreak_after: bool,
}
impl Default for QuestionSetOptions {
    fn default() -> Self {
        QuestionSetOptions {
            question_columns: DEFAULT_QUESTION_COLUMNS,
            heading: None,
            spacing: None,
            pagebreak_after: false,
        }
    }
}

/// Options that concerns the document as a whole
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOptions {
    pub font_size: u8,
    pub title: Option<SanitizedTypstString>,
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
}

impl Default for DocumentOptions {
    fn default() -> Self {
        DocumentOptions {
            title: None,
            lang: DEFAULT_LANG,
            font_size: DEFAULT_FONT_SIZE,
            write_solutions: DEFAULT_WRITE_SOLUTIONS,
            color: DEFAULT_COLORS,
            paper_size: DEFAULT_PAPER_SIZE,
            x_margin: DEFAULT_X_MARGIN,
            y_margin: DEFAULT_Y_MARGIN,
            par_spacing: DEFAULT_PAR_SPACING,
            answer_columns: DEFAULT_ANSWER_COLUMNS,
            max_prefix_group: DEFAULT_MAX_PREFIX_GROUP,
            page_break_before_answers: DEFAULT_PAGE_BREAK_BEFORE_ANSWERS,
        }
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

impl WriteSolutions {
    pub fn from(s: &str) -> WriteSolutions {
        match s.to_lowercase().as_str() {
            "all" => WriteSolutions::All,
            "none" => WriteSolutions::None,
            "first" => WriteSolutions::First,
            _ => {
                error!("Invalid WriteSolutions: {s}");
                WriteSolutions::None
            }
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
    pub fn from(size: &str) -> PaperSize {
        match size.to_lowercase().as_str() {
            "a4" => PaperSize::A4,
            "a5" => PaperSize::A5,
            _ => {
                error!("Invalid PaperSize: {size}");
                PaperSize::A4
            }
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            PaperSize::A4 => "a4",
            PaperSize::A5 => "a5",
        }
    }
}

/// The data structure that tracks everything needed to build the PDF.
///
/// Uses the builder pattern.
#[derive(Debug)]
pub struct TypstFileBuilder {
    question_sets: Vec<QuestionSet>,
    answer_sets: Vec<AnswerSet>,
    group_prefixes: Vec<Option<String>>,
    /// Keeps track of which problems has a written solution,
    /// if we have WriteSolutions::First. Not used otherwise.
    seen_problem_ids: HashSet<i32>,
    options: DocumentOptions,
    set_options: Vec<QuestionSetOptions>,
    i18n_strings: HashMap<String, String>,
}

impl TypstFileBuilder {
    /// Create a new builder. Some i18n keys are fetched up-front.
    pub async fn new(
        set_options: Vec<QuestionSetOptions>,
        options: DocumentOptions,
    ) -> Result<TypstFileBuilder> {
        let i18n_keys = vec!["solution", "answer_key"];
        let i18n_strings = db::i18n::get_multiple(i18n_keys, &options.lang).await?;
        Ok(TypstFileBuilder {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            group_prefixes: Vec::new(),
            seen_problem_ids: HashSet::new(),
            options,
            set_options,
            i18n_strings,
        })
    }

    /// Read a problem set and convert it to a QuestionSet and AnswerSet.
    ///
    /// All of the formatting within the set is handled here. The sets are grouped
    /// into nested problems (a, b, c, ...) and have their prefixes applied.
    pub fn parse_problem_set(&mut self, problem_set: Vec<Problem>) -> Result<&mut Self> {
        if problem_set.is_empty() {
            warn!("Trying to add_problem_set() an empty problem set!");
            self.question_sets.push(QuestionSet::default());
            self.answer_sets.push(AnswerSet::default());
            self.group_prefixes.push(None);
            return Ok(self);
        }

        // Problem names are collected to get their prefixes from the DB later
        let mut problem_ids = Vec::new();
        let mut new_question_set = QuestionSet::default();
        let mut new_answer_set = AnswerSet::default();

        // Split the problem into a question and either an answer or (answer + solution)
        for problem in problem_set {
            let include_solution = self.should_include_solution(problem.id);

            let (q, a) = if include_solution {
                let answer_with_solution =
                    formatting::build_solution(&problem.answer, &problem.solution)?;
                (problem.question, answer_with_solution)
            } else {
                (problem.question, problem.answer)
            };

            new_question_set.questions.push(q);
            new_answer_set.answers.push(a);
            problem_ids.push(problem.id);
        }

        let (prefixed_question_set, prefixed_answer_set) = prefix_handler::apply_prefixes(
            new_question_set,
            new_answer_set,
            &mut self.group_prefixes,
            &self.options,
            &problem_ids,
        )?;
        self.question_sets.push(prefixed_question_set);
        self.answer_sets.push(prefixed_answer_set);
        Ok(self)
    }

    /// Construct the entire Typst file and return it as one long String
    pub fn build_to_string(&self) -> Result<String> {
        let mut typst_content = String::with_capacity(32 * 1024);

        let preamble = self.build_preamble()?;
        let question_string = formatting::questions_to_balanced_columns(
            &self.question_sets,
            &self.group_prefixes,
            &self.set_options,
            &self.options.par_spacing,
        )?;
        let answer_heading = self
            .i18n_strings
            .get("answer_key")
            .expect("Unable to get answer_key translation from i18n");
        let answer_preamble = if self.options.page_break_before_answers {
            formatting::page_break()
        } else {
            String::from("\n")
        } + &formatting::reset_enum()
            + &formatting::heading(answer_heading);
        let answer_string =
            formatting::answers_to_columns(&self.answer_sets, &self.options.answer_columns)?;

        writeln!(typst_content, "{preamble}")?;
        writeln!(typst_content, "{question_string}")?;
        writeln!(typst_content, "{answer_preamble}")?;
        writeln!(typst_content, "{answer_string}")?;
        Ok(typst_content)
    }

    /// Decides if a particular problem should have its solution included
    fn should_include_solution(&mut self, problem_id: i32) -> bool {
        match self.options.write_solutions {
            WriteSolutions::All => true,
            WriteSolutions::None => false,
            // Returns true if it was inserted (and thus is the first occurence)
            WriteSolutions::First => self.seen_problem_ids.insert(problem_id),
        }
    }

    /// Constructs the preamble of the Typst file depending on the options provided.
    ///
    /// Some things are the same no matter the user's options. These are included in preamble.typ.
    fn build_preamble(&self) -> Result<String> {
        let mut parts = Vec::with_capacity(7);
        parts.push(colors::get_color_preamble(self.options.color));
        parts.push(formatting::page_size(
            self.options.paper_size.to_str(),
            self.options.x_margin,
            self.options.y_margin,
        ));
        parts.push(formatting::font_size(self.options.font_size));
        parts.push(formatting::solution_rules(&self.i18n_strings)?);
        parts.push(String::from(PREAMBLE_STR));
        if let Some(title) = &self.options.title {
            parts.push(formatting::heading(title.as_ref()));
        }
        Ok(parts.join("\n") + "\n") // join only adds \n between items, not at the end
    }
}
