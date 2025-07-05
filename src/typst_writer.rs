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
            preamble: Preamble::new(),
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

            set_string += "\n\\\n";
            collection += &set_string;
        }
        collection
    }

    pub fn build(&self) -> Result<TypstFile, std::io::Error> {
        let preamble = self.preamble.build();
        let question_string = Self::sets_to_string(&self.question_sets);
        let answer_preamble = String::from("\n#pagebreak()\n#item-counter.update(0)\n");
        let answer_string = Self::sets_to_string(&self.answer_sets);

        let typst_file_name = file_handler::to_typst_file_name(&self.file_name);
        let typst_file = file_handler::create_typst_file(&typst_file_name)?;
        file_handler::write(&typst_file, preamble)?;
        file_handler::write(&typst_file, question_string)?;
        file_handler::write(&typst_file, answer_preamble)?;
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

#[derive(Debug)]
pub struct Preamble {
    pub heading: String,
    pub paper: String,
    pub x_margin: u8,
    pub y_margin: u8,
}

impl Preamble {
    fn new() -> Preamble {
        Preamble {
            heading: String::new(),
            paper: "a4".to_string(),
            x_margin: 2,
            y_margin: 2,
        }
    }
    fn build(&self) -> String {
        let page_setup = format!(
            "#set page(paper: \"{}\", margin: (x: {}cm, y: {}cm))\n",
            self.paper, self.x_margin, self.y_margin
        );

        let color_macro = "#let col(x) = text(fill: color.linear-rgb(10%, 10%, 10%), $#x$)\n";
        let enum_setup = "#let item-counter = counter(\"item-counter\")
            #set enum(numbering: it => box(width: 1em, text(weight: \"bold\")[#it)]))
            #set enum(start: 0, spacing: 20pt)

            #show enum: it => {
              if it.start != 0 { return it }
              let args = it.fields()
              let items = args.remove(\"children\")
              context enum(..args, start: item-counter.get().first() + 1, ..items)
              item-counter.update(i => i + it.children.len())
            }\n";

        //Adjust order of preamble here if required
        let mut preamble = String::new() + &page_setup + color_macro + enum_setup;

        if !self.heading.is_empty() {
            let heading_string = String::from("= ") + &self.heading + "\n";
            preamble += &heading_string;
        }
        preamble
    }
}
