use std::collections::HashMap;
use std::fmt::Display;

use crate::{
    db::I18nDatabase, problems::Problem, typst_utils, RegistryError, PREFIX_DATA, PROBLEM_DATA,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SetOptions {
    pub question_columns: u8,
    pub title: String,
    pub spacing: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentOptions {
    pub font_size: u8,
    pub heading: String,
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
            font_size: 10,
            lang: "sv".to_string(),
            write_solutions: WriteSolutions::First,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WriteSolutions {
    All,
    None,
    First,
}

// TODO: Is this needed, or is it enough to just get "a4" or "a5"
// from the HTTP request?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaperSize {
    A4,
    A5,
}

// TODO: Check if these are even used
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
    pub async fn new(
        set_options: Vec<SetOptions>,
        options: DocumentOptions,
    ) -> Result<TypstFileBuilder> {
        let i18n_keys = vec!["solution"];
        let i18n_strings = I18nDatabase::get_multiple(i18n_keys, &options.lang).await?;
        Ok(TypstFileBuilder {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            problem_names: Vec::new(),
            group_prefixes: Vec::new(),
            set_options,
            options,
            i18n_strings,
        })
    }

    pub fn write_solutions(&mut self, option: WriteSolutions) -> &mut Self {
        self.options.write_solutions = option;
        self
    }

    pub fn add_problem_set(&mut self, problem_set: Vec<Problem>) -> Result<&mut Self> {
        // Save the IDs to use when appending prefixes
        let ids: Vec<String> = problem_set.iter().map(|pr| pr.id.clone()).collect();
        let results: Result<Vec<(String, String)>> = problem_set
            .into_iter()
            .map(|problem| match self.options.write_solutions {
                WriteSolutions::None => Ok(self.add_answer_to_set(problem)),
                WriteSolutions::All => self.add_solution_to_set(problem),
                WriteSolutions::First => {
                    if self.problem_names.contains(&problem.id) {
                        Ok(self.add_answer_to_set(problem))
                    } else {
                        self.problem_names.push(problem.id.clone());
                        self.add_solution_to_set(problem)
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

        let prefix_ids: Vec<Option<i32>> = problem_ids
            .iter()
            .map(|id| match problem_reg.get(id) {
                Some(problem) => problem.prefix_id,
                None => None,
            })
            .collect();

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

    /// Goes through the list of prefix_ids, which might look like:
    /// None Some(1) Some(2) Some(2) Some(2) None Some(2) Some(2)
    /// And writes out how many similar ids are adjacent:
    /// 1 1 3 1 2
    ///
    /// If a chain length is > max_len, new chain is started.
    fn group_related_prefixes(&self, prefix_ids: &Vec<Option<i32>>, max_len: u8) -> Vec<u8> {
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

    fn add_answer_to_set(&self, problem: Problem) -> (String, String) {
        (problem.question, problem.answer)
    }

    fn add_solution_to_set(&self, problem: Problem) -> Result<(String, String)> {
        Ok((
            problem.question,
            self.build_solution(problem.answer, problem.solution)?,
        ))
    }

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

    /// Writes the sets to columns with equal height
    fn sets_to_balanced_columns(&self, sets: &Vec<Vec<String>>) -> String {
        let mut collection = String::new();
        for (i, set) in sets.iter().enumerate() {
            let mut set_string = self.group_prefixes[i].clone().unwrap_or_default();
            set_string += "\n#let problem_set = (";
            set_string += set
                .iter()
                .map(|entry| typst_utils::formatting::to_list_item(entry))
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
                if self.set_options[i].title.is_empty() {
                    String::new()
                } else {
                    format!(
                        ", title: [{}]",
                        typst_utils::formatting::reformat_newlines(&self.set_options[i].title)
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

    ///Writes the set to a flow from one filled the column to the next
    fn sets_to_columns(&self, sets: &Vec<Vec<String>>, columns: &u8) -> String {
        let mut collection = format!("#columns({},enum(spacing: 2.5em, ", columns);
        sets.iter().for_each(|set| {
            collection += (set
                .iter()
                .map(|entry| typst_utils::formatting::to_list_item(entry))
                .collect::<Vec<String>>()
                .join("\n")
                + "\n")
                .as_str();
        });
        collection += "))";
        collection
    }

    fn answer_heading(&self) -> String {
        let heading: &str;
        if self.options.lang == "sv" {
            heading = "Facit";
        } else {
            heading = "Answer key";
        }
        typst_utils::formatting::to_heading(heading)
    }

    pub fn build_to_string(&self) -> Result<String> {
        let mut typst_content = String::new();

        let preamble = self.build_preamble();
        let question_string = self.sets_to_balanced_columns(&self.question_sets);
        let answer_preamble =
            typst_utils::formatting::page_break() + &typst_utils::formatting::reset_enum();
        let answer_string = self.sets_to_columns(&self.answer_sets, &self.options.answer_columns);

        typst_content += &preamble;
        typst_content += &question_string;
        typst_content += &answer_preamble;
        typst_content += &self.answer_heading();
        typst_content += &answer_string;
        Ok(typst_content)
    }

    fn build_preamble(&self) -> String {
        //Adjust order of preamble here if required
        let preamble = vec![
            self.set_colors(),
            self.set_page_size(),
            self.set_font_size(),
            String::from(typst_utils::preamble::PREAMBLE_STR),
            self.set_heading(),
        ];

        preamble.join("\n") + "\n" // join only adds \n between items, not at the end
    }

    fn set_heading(&self) -> String {
        if !self.options.heading.is_empty() {
            typst_utils::formatting::to_heading(&self.options.heading)
        } else {
            String::new()
        }
    }
    fn set_font_size(&self) -> String {
        format!("#set text(size: {}pt)", self.options.font_size)
    }

    fn set_page_size(&self) -> String {
        format!(
            "#set page(paper: \"{}\", margin: (x: {}mm, y: {}mm))",
            self.options.paper_size.to_typst(),
            self.options.x_margin,
            self.options.y_margin
        )
    }

    fn set_colors(&self) -> String {
        let colored: Color;
        // Graphing colors
        let primary: Color;
        let secondary: Color;
        let tertiary: Color;
        if self.options.color {
            colored = Color::new(22, 10, 33); // Purple
            primary = Color::new(9, 3, 18); // Dark purple
            secondary = colored.clone();
            tertiary = Color::new(30, 23, 39); // Light purple
        } else {
            colored = Color::new(10, 10, 10); // Gray
            primary = Color::new(0, 0, 0); // Black
            secondary = Color::new(8, 8, 8); // Gray?
            tertiary = Color::new(16, 16, 16); // Grayer?
        };

        format!(
            "
#let colored(x) = text(fill: color.linear-rgb({colored}), $#x$)
#let primary(x) = text(fill: color.linear-rgb({primary}), $#x$)
#let secondary(x) = text(fill: color.linear-rgb({secondary}), $#x$)
#let tertiary(x) = text(fill: color.linear-rgb({tertiary}), $#x$)"
        )
    }
}

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%, {}%, {}%", self.r, self.g, self.b)
    }
}
