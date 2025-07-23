use std::fmt::Display;

use crate::Result;
use crate::backend::document::*;
use crate::backend::problems::Problem;
use crate::backend::translations::GENERAL_TRANSLATIONS;
use crate::shared::{DocumentOptions, SetRenderingOptions, WriteSolutions};

#[derive(Debug)]
pub struct DocumentBuilder {
    question_sets: Vec<Vec<String>>,
    answer_sets: Vec<Vec<String>>,
    options: DocumentOptions,
    set_options: Vec<SetRenderingOptions>,
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
                        typst_formatting::reformat_newlines(&self.set_options[i].title)
                    )
                }
            );
            if i != sets.len() - 1 {
                if let Some(spacing) = self.options.par_spacing {
                    set_string += format!("#v({}mm)\n", spacing).as_str();
                } else {
                    set_string += &String::from("#v(1.33em)\n");
                }
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

    fn answer_heading(&self) -> String {
        let heading: &str;
        if self.options.lang == "sv" {
            heading = "Facit";
        } else {
            heading = "Answer key";
        }
        typst_formatting::to_heading(heading)
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
        file_handler::write(&typst_file, self.answer_heading())?;
        file_handler::write(&typst_file, answer_string)?;
        Ok(FinishedFile::new(typst_file_name))
    }

    fn build_preamble(&self) -> String {
        //Adjust order of preamble here if require
        let preamble = vec![
            self.set_colors(),
            self.set_page_size(),
            self.set_font_size(),
            String::from(typst_preamble::PREAMBLE_STR),
            self.set_heading(),
        ];

        preamble.join("\n") + "\n" // join only adds \n between items, not at the end
    }

    fn set_heading(&self) -> String {
        if !self.options.heading.is_empty() {
            typst_formatting::to_heading(&self.options.heading)
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
