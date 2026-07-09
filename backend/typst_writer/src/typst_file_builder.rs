use crate::{colors, formatting, preamble::PREAMBLE_STR, prefix_handler};
use anyhow::Result;
use db;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use tracing::warn;
use types::pdf::{DocumentOptions, QuestionSetFormattingOptions, WriteSolutions};
use types::problems::Problem;

const TRANSLATION_NOT_FOUND_MESSAGE: &str = "!!! Translation not found !!!";

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
    formatting_options: Vec<QuestionSetFormattingOptions>,
    i18n_strings: HashMap<String, String>,
}

impl TypstFileBuilder {
    /// Create a new builder and load the translations from the DB
    pub async fn new(
        formatting_options: Vec<QuestionSetFormattingOptions>,
        options: DocumentOptions,
    ) -> Result<TypstFileBuilder> {
        let i18n_strings = db::i18n::get_pdf_translations(&options.lang).await?;
        Ok(TypstFileBuilder {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            group_prefixes: Vec::new(),
            seen_problem_ids: HashSet::new(),
            options,
            formatting_options,
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
            &self.formatting_options,
            &self.options.par_spacing,
        )?;
        let answer_heading = self.get_translation("answer_key");
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
        parts.push(formatting::solution_rules(
            self.get_translation("solution"),
        )?);
        parts.push(String::from(PREAMBLE_STR));
        if let Some(title) = &self.options.title {
            parts.push(formatting::heading(title.as_ref()));
        }
        if let Some(subtitle) = &self.options.subtitle {
            parts.push(formatting::subheading(subtitle.as_ref()));
        }
        if self.options.name_field {
            parts.push(formatting::name_field(self.get_translation("name")));
        }
        Ok(parts.join("\n") + "\n") // join only adds \n between items, not at the end
    }

    fn get_translation<'s>(&'s self, key: &str) -> &'s str {
        self.i18n_strings
            .get(key)
            .map(|s| s.as_str())
            .unwrap_or(TRANSLATION_NOT_FOUND_MESSAGE)
    }
}
