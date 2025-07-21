use serde::{Deserialize, Serialize};

use crate::Result;
use crate::backend::document::*;
use crate::backend::problems::Problem;
use crate::backend::translations::GENERAL_TRANSLATIONS;
use crate::frontend_types::SetRenderingOptions;

#[derive(Debug)]
pub struct DocumentBuilder {
    question_sets: Vec<Vec<String>>,
    answer_sets: Vec<Vec<String>>,
    options: DocumentOptions,
    set_options: Vec<SetRenderingOptions>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentOptions {
    pub lang: String,
    pub write_solutions: WriteSolutions,
    pub file_name: String,
    pub color: bool,
    pub heading: String,
    pub paper_size: String,
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
            write_solutions: WriteSolutions::None,
            file_name: String::from("stencil"),
            color: true,
            heading: String::new(),
            paper_size: "a4".to_string(),
            x_margin: 20,
            y_margin: 20,
            par_spacing: 6,
            enum_spacing: 6,
            answer_columns: 2,
        }
    }
}

pub struct FinishedFile {
    file_path: String,
}

impl FinishedFile {
    pub fn new(file_path: String) -> FinishedFile {
        let typst_file_path = file_helpers::to_typst_file_name(&file_path);
        FinishedFile {
            file_path: typst_file_path,
        }
    }

    pub fn compile(&self) -> std::io::Result<String> {
        compile_handler::compile(&self.file_path)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WriteSolutions {
    All,
    None,
    First,
}

impl DocumentBuilder {
    pub fn new(set_options: Vec<SetRenderingOptions>, options: DocumentOptions) -> DocumentBuilder {
        DocumentBuilder {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            set_options,
            options,
        }
    }

    pub fn write_solutions(&mut self, option: WriteSolutions) -> &mut Self {
        self.options.write_solutions = option;
        self
    }

    // TODO: Refactor so it checks added problems in entire document
    pub fn add_problem_set(&mut self, problem_set: Vec<Problem>) -> Result<&mut Self> {
        let mut added_problem_names: Vec<String> = Vec::new();
        let results: Result<Vec<(String, String)>> = problem_set
            .into_iter()
            .map(|problem| match self.options.write_solutions {
                WriteSolutions::None => Ok(self.add_answer_to_set(problem)),
                WriteSolutions::All => self.add_solution_to_set(problem),
                WriteSolutions::First => {
                    if added_problem_names.contains(&problem.id) {
                        Ok(self.add_answer_to_set(problem))
                    } else {
                        added_problem_names.push(problem.id.clone());
                        self.add_solution_to_set(problem)
                    }
                }
            })
            .collect();
        let (question_set, answer_set): (Vec<String>, Vec<String>) = results?.into_iter().unzip();
        self.question_sets.push(question_set);
        self.answer_sets.push(answer_set);
        Ok(self)
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
            "#block(inset: (left: -1.2em))[\n  #emph([{}])\n  #v(-0.5em)",
            GENERAL_TRANSLATIONS.get_phrase("solution", &self.options.lang)?
        );
        let closing_bracket = String::from("]");
        Ok([answer, heading, solution, closing_bracket].join("\n"))
    }

    /// Writes the set to columns with equal height
    fn sets_to_balanced_columns(&self, sets: &Vec<Vec<String>>) -> String {
        let mut collection = String::new();
        for (i, set) in sets.iter().enumerate() {
            let mut set_string = String::from("#let problem_set = (");
            set_string += set
                .iter()
                .map(|entry| typst_formatting::to_list_item(entry))
                .collect::<Vec<String>>()
                .join("\n")
                .as_str();

            set_string += ")\n";

            set_string += &format!(
                "#context{{balanced({}, problem_set,{}mm, here().position().y{})}}\n",
                self.set_options[i].question_columns,
                self.set_options[i].spacing,
                if self.set_options[i].title.is_empty() {
                    String::new()
                } else {
                    format!(
                        ", title: [{}]",
                        typst_formatting::reformat_newlines(&self.set_options[i].title)
                    )
                }
            );
            if i != sets.len() - 1 {
                set_string += &typst_formatting::empty_line();
                set_string += format!("#v({}mm)\n", self.options.par_spacing).as_str();
            }
            collection += &set_string;
        }
        collection
    }

    ///Writes the set to a flow from one filled the column to the next
    fn sets_to_columns(&self, sets: &Vec<Vec<String>>, columns: &u8) -> String {
        let mut collection = format!("#columns({},enum(", columns);
        sets.iter().for_each(|set| {
            collection += (set
                .iter()
                .map(|entry| typst_formatting::to_list_item(entry))
                .collect::<Vec<String>>()
                .join("\n")
                + "\n")
                .as_str();
        });
        collection += "))";
        collection
    }

    pub fn build(&self) -> Result<FinishedFile> {
        let preamble = self.build_preamble();
        let question_string = self.sets_to_balanced_columns(&self.question_sets);
        let answer_preamble = typst_formatting::page_break() + &typst_formatting::reset_enum();
        let answer_string = self.sets_to_columns(&self.answer_sets, &self.options.answer_columns);

        let typst_file_name = file_helpers::to_typst_file_name(&self.options.file_name);
        let typst_file = file_handler::create_typst_file(&typst_file_name)?;
        file_handler::write(&typst_file, preamble)?;
        file_handler::write(&typst_file, question_string)?;
        file_handler::write(&typst_file, answer_preamble)?;
        file_handler::write(&typst_file, answer_string)?;
        Ok(FinishedFile::new(typst_file_name))
    }

    fn build_preamble(&self) -> String {
        //Adjust order of preamble here if required
        let mut preamble = vec![
            String::from(typst_preamble::PREAMBLE_STR),
            self.build_page_size(),
            self.build_enum_spacing(),
        ];

        if !self.options.heading.is_empty() {
            let heading_string = typst_formatting::to_heading(&self.options.heading) + "\n";
            preamble.push(heading_string);
        }
        preamble.join("\n")
    }

    fn build_page_size(&self) -> String {
        format!(
            "#set page(paper: \"{}\", margin: (x: {}mm, y: {}mm))",
            self.options.paper_size, self.options.x_margin, self.options.y_margin
        )
    }

    fn build_enum_spacing(&self) -> String {
        format!("#set enum(spacing: {}mm)\n", self.options.enum_spacing)
    }
}
