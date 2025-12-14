use crate::{
    Language,
    db::I18nDatabase,
    problems::Problem,
    typst_utils::{formatting, preamble::PREAMBLE_STR, prefix_handler},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use tracing::{error, warn};

pub const DEFAULT_QUESTION_COLUMNS: u8 = 2;
const DEFAULT_ANSWER_COLUMNS: u8 = 3;
const DEFAULT_FONT_SIZE: u8 = 10;
const DEFAULT_X_MARGIN: u8 = 20;
const DEFAULT_Y_MARGIN: u8 = 20;
const DEFAULT_LANG: Language = Language::Sv;
const DEFAULT_MAX_PREFIX_GROUP: Option<u8> = Some(3);
const DEFAULT_PAPER_SIZE: PaperSize = PaperSize::A4;
const DEFAULT_WRITE_SOLUTIONS: WriteSolutions = WriteSolutions::First;
const DEFAULT_PAR_SPACING: Option<u8> = None;
const DEFAULT_COLORS: bool = true;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SetOptions {
    pub question_columns: u8,
    pub heading: String,
    pub spacing: Option<u16>,
}
impl Default for SetOptions {
    fn default() -> Self {
        SetOptions {
            question_columns: DEFAULT_QUESTION_COLUMNS,
            heading: String::new(),
            spacing: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentOptions {
    pub font_size: u8,
    pub title: String,
    pub answer_columns: u8,
    pub lang: Language,
    pub write_solutions: WriteSolutions,
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
            title: String::new(),
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

#[derive(Debug)]
pub struct TypstFileBuilder {
    question_sets: Vec<Vec<String>>,
    answer_sets: Vec<Vec<String>>,
    group_prefixes: Vec<Option<String>>,
    /// Keeps track of which problems has a written solution,
    /// if we have WriteSolutions::First. Not used otherwise.
    seen_problem_names: HashSet<String>,
    options: DocumentOptions,
    set_options: Vec<SetOptions>,
    i18n_strings: HashMap<String, String>,
}

impl TypstFileBuilder {
    /// Create a new builder. Some i18n keys are fetched up-front.
    pub async fn new(
        set_options: Vec<SetOptions>,
        options: DocumentOptions,
    ) -> Result<TypstFileBuilder> {
        let i18n_keys = vec!["solution", "answer_key"];
        let i18n_strings = I18nDatabase::get_multiple(i18n_keys, &options.lang).await?;
        Ok(TypstFileBuilder {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            group_prefixes: Vec::new(),
            seen_problem_names: HashSet::new(),
            options,
            set_options,
            i18n_strings,
        })
    }

    /// Add a problem set to the builder. The builder stores formatted question/answer strings
    pub fn add_problem_set(&mut self, problem_set: Vec<Problem>) -> Result<&mut Self> {
        if problem_set.is_empty() {
            warn!("Trying to add_problem_set() an empty problem set!");
            self.question_sets.push(Vec::new());
            self.answer_sets.push(Vec::new());
            self.group_prefixes.push(None);
            return Ok(self);
        }

        // Transform problems into names, questions and answer-or-solutions depending on the config
        let mut names = Vec::with_capacity(problem_set.len());
        let mut questions = Vec::with_capacity(problem_set.len());
        let mut answers = Vec::with_capacity(problem_set.len());

        for problem in problem_set {
            let include_solution = self.should_include_solution(&problem.name);

            let (q, a) = if include_solution {
                let answer_with_solution =
                    formatting::build_solution(problem.answer, problem.solution)?;
                (problem.question, answer_with_solution)
            } else {
                (problem.question, problem.answer)
            };

            questions.push(q);
            answers.push(a);
            names.push(problem.name);
        }

        let (question_set, answer_set) = prefix_handler::handle_prefixes(
            questions,
            answers,
            &mut self.group_prefixes,
            &self.options,
            &names,
        )?;
        self.question_sets.push(question_set);
        self.answer_sets.push(answer_set);
        Ok(self)
    }

    /// Construct the entire Typst file and return it as one long String
    pub fn build_to_string(&self) -> Result<String> {
        let mut typst_content = String::with_capacity(32 * 1024);

        let preamble = self.build_preamble()?;
        let question_string = formatting::sets_to_balanced_columns(
            &self.question_sets,
            &self.group_prefixes,
            &self.set_options,
            &self.options.par_spacing,
        )?;
        let answer_heading = self
            .i18n_strings
            .get("answer_key")
            .expect("Unable to get answer_key translation from i18n");
        let answer_preamble = formatting::page_break()
            + &formatting::reset_enum()
            + &formatting::heading(answer_heading);
        let answer_string =
            formatting::sets_to_columns(&self.answer_sets, &self.options.answer_columns)?;

        writeln!(typst_content, "{preamble}")?;
        writeln!(typst_content, "{question_string}")?;
        writeln!(typst_content, "{answer_preamble}")?;
        writeln!(typst_content, "{answer_string}")?;
        Ok(typst_content)
    }

    fn should_include_solution(&mut self, problem_name: &str) -> bool {
        match self.options.write_solutions {
            WriteSolutions::All => true,
            WriteSolutions::None => false,
            // Returns true if it was inserted (and thus is the first occurence)
            WriteSolutions::First => self.seen_problem_names.insert(problem_name.to_owned()),
        }
    }

    fn build_preamble(&self) -> Result<String> {
        let mut parts = Vec::with_capacity(7);
        parts.push(formatting::colors(self.options.color));
        parts.push(formatting::page_size(
            &self.options.paper_size.to_str(),
            self.options.x_margin,
            self.options.y_margin,
        ));
        parts.push(formatting::font_size(self.options.font_size));
        parts.push(formatting::solution_rules(&self.i18n_strings)?);
        parts.push(String::from(PREAMBLE_STR));
        parts.push(formatting::heading(&self.options.title));
        Ok(parts.join("\n") + "\n") // join only adds \n between items, not at the end
    }
}
