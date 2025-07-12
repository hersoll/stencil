use serde::{Deserialize, Serialize};

use crate::backend::problems::Problem;
use crate::backend::translations::GENERAL_TRANSLATIONS;
use crate::backend::{ProblemGenerator, document::*};
use crate::{Error, Result};

#[derive(Debug)]
pub struct DocumentBuilder {
    question_sets: Vec<Vec<String>>,
    answer_sets: Vec<Vec<String>>,
    options: DocumentOptions,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentOptions {
    //Config options
    write_solutions: WriteSolutions,
    file_name: String,
    heading: String,
    paper_size: String,
    x_margin: u8,
    y_margin: u8,
    enum_spacing: u8,
    par_spacing: u8,
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
    pub fn new() -> DocumentBuilder {
        let options = DocumentOptions {
            write_solutions: WriteSolutions::None,
            file_name: String::from("stencil"),
            heading: String::new(),
            paper_size: "a4".to_string(),
            x_margin: 20,
            y_margin: 20,
            par_spacing: 6,
            enum_spacing: 6,
        };
        DocumentBuilder {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            options,
        }
    }

    pub fn file_name<T: Into<String>>(&mut self, file_name: T) -> &mut Self {
        self.options.file_name = file_name.into();
        self
    }

    pub fn write_solutions(&mut self, option: WriteSolutions) -> &mut Self {
        self.options.write_solutions = option;
        self
    }

    pub fn heading<T: Into<String>>(&mut self, heading: T) -> &mut Self {
        self.options.heading = heading.into();
        self
    }

    pub fn paper_size<T: Into<String>>(&mut self, size: T) -> &mut Self {
        self.options.paper_size = size.into();
        self
    }

    pub fn enum_spacing<T: Into<u8>>(&mut self, size: T) -> &mut Self {
        self.options.enum_spacing = size.into();
        self
    }

    pub fn x_margin<T: Into<u8>>(&mut self, margin: T) -> &mut Self {
        self.options.x_margin = margin.into();
        self
    }

    pub fn y_margin<T: Into<u8>>(&mut self, margin: T) -> &mut Self {
        self.options.y_margin = margin.into();
        self
    }

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
        let translation = GENERAL_TRANSLATIONS
            .lock()
            .map_err(|_| Error::RegistryMutexIsPoisoned)?;
        let heading = format!(
            "  #v(0pt)\n  #emph([{}])\n  #v(-6pt)",
            translation.get_phrase("solution")?
        );
        Ok([answer, heading, solution].join("\n"))
    }

    fn sets_to_string(&self, sets: &Vec<Vec<String>>) -> String {
        let mut collection = String::new();
        for set in sets {
            let mut set_string: String = set
                .iter()
                .map(|entry| typst_formatting::to_list_item(entry))
                .collect::<Vec<String>>()
                .join("\n");

            set_string += &typst_formatting::empty_line();
            set_string += format!("#v({}mm)\n", self.options.par_spacing).as_str();
            collection += &set_string;
        }
        collection
    }

    pub fn build(&self) -> Result<FinishedFile> {
        let preamble = self.build_preamble();
        let question_string = self.sets_to_string(&self.question_sets);
        let answer_preamble = typst_formatting::page_break() + &typst_formatting::reset_enum();
        let answer_string = self.sets_to_string(&self.answer_sets);

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
            "#set page(paper: \"{}\", margin: (x: {}mm, y: {}mm), columns: 2)",
            self.options.paper_size, self.options.x_margin, self.options.y_margin
        )
    }

    fn build_enum_spacing(&self) -> String {
        format!(
            "#set enum(start: 0, spacing: {}mm)",
            self.options.enum_spacing
        )
    }
}
