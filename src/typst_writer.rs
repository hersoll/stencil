#![allow(dead_code)]

use crate::problems::ProcessedProblem;

pub mod file_handler;
mod renderer;

#[derive(Debug)]
pub struct TypstWriter {
    preamble: Preamble,
    question_sets: Vec<Vec<String>>,
    answer_sets: Vec<Vec<String>>,
    file_name: String,
}

impl TypstWriter {
    pub fn new() -> TypstWriter {
        TypstWriter {
            preamble: Preamble::default(),
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            file_name: String::from("stencil"),
        }
    }

    pub fn file_name<T: ToString>(&mut self, file_name: T) -> &mut Self {
        self.file_name = file_name.to_string();
        self
    }

    pub fn heading<T: ToString>(&mut self, heading: T) -> &mut Self {
        self.preamble.heading = heading.to_string();
        self
    }

    pub fn add_problem_set(&mut self, problem_set: Vec<ProcessedProblem>) -> &mut Self {
        let (question_set, answer_set) = problem_set
            .into_iter()
            .map(|problem| (problem.question, problem.answer))
            .unzip();
        self.question_sets.push(question_set);
        self.answer_sets.push(answer_set);
        self
    }

    fn sets_to_string(sets: &Vec<Vec<String>>) -> String {
        let mut collection = String::new();
        for set in sets {
            let mut set_string: String = set
                .iter()
                .map(|entry| String::from("+ ") + entry)
                .collect::<Vec<String>>()
                .join("\n");

            set_string += "\n";
            collection += &set_string;
        }
        collection
    }

    pub fn build(&self) -> Result<TypstFile, std::io::Error> {
        let preamble_string = self.preamble.build();
        let question_string = Self::sets_to_string(&self.question_sets);
        let answer_string = Self::sets_to_string(&self.answer_sets);

        let typst_file_name = file_handler::to_typst_file_name(&self.file_name);
        let typst_file = file_handler::create_typst_file(&typst_file_name)?;
        file_handler::write(&typst_file, preamble_string)?;
        file_handler::write(&typst_file, question_string)?;
        file_handler::write(&typst_file, answer_string)?;
        Ok(TypstFile::new(typst_file_name))
    }
}

pub struct TypstFile {
    file_path: String,
}

impl TypstFile {
    pub fn new(file_path: String) -> TypstFile {
        let typst_file_path = file_handler::to_typst_file_name(&file_path);
        TypstFile {
            file_path: typst_file_path,
        }
    }

    pub fn compile(&self) -> std::io::Result<String> {
        renderer::compile(&self.file_path)
    }
}

#[derive(Debug, Default)]
pub struct Preamble {
    pub heading: String,
}

impl Preamble {
    fn new() -> Preamble {
        Preamble::default()
    }
    fn build(&self) -> String {
        let preamble_string = String::from("= ") + self.heading.as_str() + "\n";
        preamble_string
    }
}
