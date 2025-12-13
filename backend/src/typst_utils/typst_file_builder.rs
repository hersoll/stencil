use crate::{
    PREFIX_DATA, PROBLEM_DATA, RegistryError,
    db::I18nDatabase,
    problems::Problem,
    typst_utils::{formatting, preamble::PREAMBLE_STR},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use tracing::{error, warn};

const DEFAULT_QUESTION_COLUMNS: u8 = 2;
const DEFAULT_ANSWER_COLUMNS: u8 = 3;
const DEFAULT_FONT_SIZE: u8 = 10;
const DEFAULT_X_MARGIN: u8 = 20;
const DEFAULT_Y_MARGIN: u8 = 20;
const DEFAULT_LANG: &'static str = "sv";
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
    pub lang: String,
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
            lang: DEFAULT_LANG.to_string(),
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
    problem_names: Vec<String>,
    group_prefixes: Vec<Option<String>>,
    options: DocumentOptions,
    set_options: Vec<SetOptions>,
    i18n_strings: HashMap<String, String>,
}

impl TypstFileBuilder {
    /// Create a new builder. Some 18n keys are fetched up-front.
    pub async fn new(
        set_options: Vec<SetOptions>,
        options: DocumentOptions,
    ) -> Result<TypstFileBuilder> {
        let i18n_keys = vec!["solution", "answer_key"];
        let i18n_strings = I18nDatabase::get_multiple(i18n_keys, &options.lang).await?;
        Ok(TypstFileBuilder {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            problem_names: Vec::new(),
            group_prefixes: Vec::new(),
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

        // Save the IDs to use when appending prefixes
        let ids: Vec<String> = problem_set.iter().map(|pr| pr.id.clone()).collect();

        let results: Result<Vec<(String, String)>> = problem_set
            .into_iter()
            .map(|problem| match self.options.write_solutions {
                WriteSolutions::None => Ok(self.add_problem_without_solution(problem)),
                WriteSolutions::All => self.add_problem_with_solution(problem),
                WriteSolutions::First => {
                    if self.problem_names.contains(&problem.id) {
                        Ok(self.add_problem_without_solution(problem))
                    } else {
                        self.problem_names.push(problem.id.clone());
                        self.add_problem_with_solution(problem)
                    }
                }
            })
            .collect();
        let (mut question_set, mut answer_set): (Vec<String>, Vec<String>) =
            results?.into_iter().unzip();
        (question_set, answer_set) = self.handle_prefixes(question_set, answer_set, &ids)?;
        self.question_sets.push(question_set);
        self.answer_sets.push(answer_set);
        Ok(self)
    }

    /// Construct the entire Typst file and return it as one long String
    pub fn build_to_string(&self) -> Result<String> {
        // Estimated 16kb
        let mut typst_content = String::with_capacity(16384);

        let preamble = self.build_preamble();
        let question_string = self.sets_to_balanced_columns(&self.question_sets);
        // Set up to start printing the answer key
        let answer_heading = self
            .i18n_strings
            .get("answer_key")
            .expect("Unable to get answer_key translation from db");
        let answer_preamble = formatting::page_break()
            + &formatting::reset_enum()
            + &formatting::heading(answer_heading);
        let answer_string = self.sets_to_columns(&self.answer_sets, &self.options.answer_columns);

        writeln!(typst_content, "{preamble}")?;
        writeln!(typst_content, "{question_string}")?;
        writeln!(typst_content, "{answer_preamble}")?;
        writeln!(typst_content, "{answer_string}")?;
        Ok(typst_content)
    }

    fn build_preamble(&self) -> String {
        //Adjust order of preamble here if required
        let preamble = vec![
            formatting::colors(self.options.color),
            formatting::page_size(
                &self.options.paper_size.to_str(),
                self.options.x_margin,
                self.options.y_margin,
            ),
            formatting::font_size(self.options.font_size),
            String::from(PREAMBLE_STR),
            formatting::heading(&self.options.title),
        ];

        preamble.join("\n") + "\n" // join only adds \n between items, not at the end
    }

    // TODO: BIG refactor
    //
    /// If all problems share prefix, a group prefix will be designated to the set.
    /// Otherwise, problems may be grouped together into nested enums if they share
    /// a prefix with adjacent problems
    fn handle_prefixes(
        &mut self,
        mut question_set: Vec<String>,
        mut answer_set: Vec<String>,
        problem_ids: &Vec<String>,
    ) -> Result<(Vec<String>, Vec<String>)> {
        let problem_reg =
            PROBLEM_DATA
                .read()
                .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
                    registry: "PROBLEM_DATA".to_string(),
                })?;

        // TODO: Scope this with the above to make it drop the lock ASAP
        let prefix_ids: Vec<Option<i32>> = problem_ids
            .iter()
            .map(|id| match problem_reg.get(id) {
                Some(problem) => problem.prefix_id,
                None => None,
            })
            .collect();

        // TODO: Scope this with the fetch below to make it drop the lock ASAP
        let prefix_reg =
            PREFIX_DATA
                .read()
                .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
                    registry: "PREFIX_DATA".to_string(),
                })?;
        if let Some(first_id) = prefix_ids[0]
            && prefix_ids.iter().all(|&id| id == prefix_ids[0])
        {
            let prefix_fetch = prefix_reg.get(&first_id);
            if let Some(prefix) = prefix_fetch {
                let prefix_text = prefix.clone().parse(&self.options.lang).group_text;
                self.group_prefixes.push(Some(prefix_text + ":"));
            } else {
                self.group_prefixes.push(None);
            }
        } else {
            self.group_prefixes.push(None);
            if let Some(max_grouping) = self.options.max_prefix_group {
                let groups = self.group_related_prefixes(&prefix_ids, max_grouping);
                let mut new_question_set: Vec<String> = Vec::new();
                let mut new_answer_set: Vec<String> = Vec::new();

                let mut i = 0;
                for group in groups {
                    if group == 1 {
                        if let Some(id) = prefix_ids[i] {
                            let prefix_fetch = prefix_reg.get(&id);
                            if let Some(prefix) = prefix_fetch {
                                let prefix_text = prefix.clone().parse(&self.options.lang).text;
                                new_question_set.push(prefix_text + " " + &question_set[i]);
                                new_answer_set.push(answer_set[i].clone());
                            }
                        } else {
                            new_question_set.push(question_set[i].clone());
                            new_answer_set.push(answer_set[i].clone());
                        }
                    } else {
                        if let Some(id) = prefix_ids[i] {
                            let prefix_fetch = prefix_reg.get(&id);
                            if let Some(prefix) = prefix_fetch {
                                let prefix_text =
                                    prefix.clone().parse(&self.options.lang).group_text;
                                let mut grouped_questions = prefix_text;
                                grouped_questions +=
                                    ": \n\n#enum(numbering: \"a)\", indent: -0.8em,\n";

                                let mut grouped_answers =
                                    String::from("\\ #enum(numbering: \"a)\", indent: -1em, \n");

                                for j in i..(i + group as usize) {
                                    grouped_questions += &format!("[{q}],\n", q = &question_set[j]);
                                    grouped_answers += &format!("[{q}],\n", q = &answer_set[j]);
                                }

                                grouped_questions += ")";
                                new_question_set.push(grouped_questions);

                                grouped_answers += ")";
                                new_answer_set.push(grouped_answers);
                            }
                        }
                    }

                    i += group as usize;
                }
                question_set = new_question_set;
                answer_set = new_answer_set;
            } else {
                prefix_ids.into_iter().enumerate().for_each(|(i, id_opt)| {
                    if let Some(id) = id_opt {
                        let prefix_fetch = prefix_reg.get(&id);
                        if let Some(prefix) = prefix_fetch {
                            let prefix_text = prefix.clone().parse(&self.options.lang).text;
                            question_set[i] = prefix_text + " " + &question_set[i];
                        }
                    }
                });
            }
        }
        Ok((question_set, answer_set))
    }

    // TODO: Refactor
    //
    /// Goes through the list of prefix_ids, which might look like:
    /// [None Some(1) Some(2) Some(2) Some(2) None Some(2) Some(2)]
    /// And writes out how many similar ids are adjacent:
    /// [1 1 3 1 2]
    ///
    /// If a chain length is > max_len, new chain is started.
    fn group_related_prefixes(&self, prefix_ids: &[Option<i32>], max_len: u8) -> Vec<u8> {
        if prefix_ids.len() < 2 {
            return vec![prefix_ids.len() as u8];
        }
        let mut latest_id: Option<i32> = prefix_ids[0];
        let mut current_length: u8 = 1;
        let mut groups: Vec<u8> = Vec::new();
        for &id in &prefix_ids[1..] {
            if id.is_some() && id == latest_id {
                current_length += 1;
                if current_length > max_len.into() {
                    groups.push(max_len.into());
                    current_length = 1;
                }
            } else {
                groups.push(current_length);
                latest_id = id;
                current_length = 1;
            }
        }

        groups.push(current_length);
        groups
    }

    /// Get the proper strings from a Problem for the question and answer
    fn add_problem_without_solution(&self, problem: Problem) -> (String, String) {
        (problem.question, problem.answer)
    }

    /// Get the proper strings from a Problem for the question, answer and solution
    fn add_problem_with_solution(&self, problem: Problem) -> Result<(String, String)> {
        Ok((
            problem.question,
            self.build_solution(problem.answer, problem.solution)?,
        ))
    }

    // TODO: Move to formatting module
    //
    /// Formats the answer and solution strings to show up as a proper solution in the Typst file
    fn build_solution(&self, answer: String, solution: String) -> Result<String> {
        let heading = format!(
            "#block(inset: (left: -1.2em))[\n#set text(size: 0.8em)\n #emph([{}])\n\n ",
            self.i18n_strings
                .get("solution")
                .context("Unable to get key \"solution\" from i18n")?
        );
        let closing_bracket = String::from("]");
        Ok([answer, heading, solution, closing_bracket].join("\n"))
    }

    // TODO: Refactor - move formatting parts to formatting module
    //
    /// Writes the sets to columns with equal height
    fn sets_to_balanced_columns(&self, sets: &Vec<Vec<String>>) -> String {
        let mut collection = String::new();
        for (i, set) in sets.iter().enumerate() {
            let mut set_string = self.group_prefixes[i].clone().unwrap_or_default();
            set_string += "\n#let problem_set = (";
            set_string += set
                .iter()
                .map(|entry| formatting::list_item(entry))
                .collect::<Vec<String>>()
                .join("\n")
                .as_str();

            set_string += ")\n";

            set_string += &format!(
                "#context{{balanced({}, problem_set,here().position().y{}{})}}\n",
                self.set_options[i].question_columns,
                if let Some(spacing) = self.set_options[i].spacing {
                    format!(", custom_spacing: {spacing}mm")
                } else {
                    String::new()
                },
                if self.set_options[i].heading.is_empty() {
                    String::new()
                } else {
                    format!(
                        ", title: [{}]",
                        formatting::reformat_newlines(&self.set_options[i].heading)
                    )
                }
            );
            if i != sets.len() - 1 {
                if let Some(spacing) = self.options.par_spacing {
                    set_string += format!("#v({}mm)\n", spacing).as_str();
                } else {
                    set_string += &String::from("#v(1.8em)\n");
                }
            }
            collection += &set_string;
        }
        collection
    }

    // TODO: Refactor - move formatting parts to formatting module
    //
    ///Writes the set to a flow from one filled column to the next
    fn sets_to_columns(&self, sets: &Vec<Vec<String>>, columns: &u8) -> String {
        let mut collection = format!("#columns({},enum(spacing: 2.5em, ", columns);
        sets.iter().for_each(|set| {
            collection += (set
                .iter()
                .map(|entry| formatting::list_item(entry))
                .collect::<Vec<String>>()
                .join("\n")
                + "\n")
                .as_str();
        });
        collection += "))";
        collection
    }
}
