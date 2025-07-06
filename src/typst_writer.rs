use crate::problems::Problem;

pub mod file_handler;
pub mod preamble;
mod renderer;
pub mod typst;

pub use preamble::*;

#[derive(Debug)]
pub struct TypstWriter {
    question_sets: Vec<Vec<String>>,
    answer_sets: Vec<Vec<String>>,
    file_name: String,
    write_solutions: WriteSolutions,

    //Config options
    heading: String,
    paper_size: String,
    x_margin: u8,
    y_margin: u8,
    enum_spacing: u8,
    par_spacing: u8,
}

impl Default for TypstWriter {
    fn default() -> Self {
        TypstWriter {
            question_sets: Vec::new(),
            answer_sets: Vec::new(),
            file_name: String::from("stencil"),
            write_solutions: Default::default(),

            heading: String::new(),
            paper_size: "a4".to_string(),
            x_margin: 20,
            y_margin: 20,
            par_spacing: 12,
            enum_spacing: 6,
        }
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
pub enum WriteSolutions {
    All,
    #[default]
    None,
    First,
}

impl TypstWriter {
    pub fn new() -> TypstWriter {
        TypstWriter::default()
    }

    pub fn file_name<T: Into<String>>(&mut self, file_name: T) -> &mut Self {
        self.file_name = file_name.into();
        self
    }

    pub fn write_solutions(&mut self, option: WriteSolutions) -> &mut Self {
        self.write_solutions = option;
        self
    }

    pub fn heading<T: Into<String>>(&mut self, heading: T) -> &mut Self {
        self.heading = heading.into();
        self
    }

    pub fn paper_size<T: Into<String>>(&mut self, size: T) -> &mut Self {
        self.paper_size = size.into();
        self
    }

    pub fn enum_spacing<T: Into<u8>>(&mut self, size: T) -> &mut Self {
        self.enum_spacing = size.into();
        self
    }

    pub fn x_margin<T: Into<u8>>(&mut self, margin: T) -> &mut Self {
        self.x_margin = margin.into();
        self
    }

    pub fn y_margin<T: Into<u8>>(&mut self, margin: T) -> &mut Self {
        self.y_margin = margin.into();
        self
    }

    pub fn add_problem_set(&mut self, problem_set: Vec<Problem>) -> &mut Self {
        let mut added_problem_names: Vec<String> = Vec::new();
        let (question_set, answer_set) = problem_set
            .into_iter()
            .map(|problem| match self.write_solutions {
                WriteSolutions::None => (problem.question, problem.answer),
                WriteSolutions::All => (problem.question, problem.solution),
                WriteSolutions::First => {
                    if added_problem_names.contains(&problem.id.name) {
                        (problem.question, problem.answer)
                    } else {
                        added_problem_names.push(problem.id.name);
                        (problem.question, problem.solution)
                    }
                }
            })
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
                .map(|entry| typst::to_list_item(entry))
                .collect::<Vec<String>>()
                .join("\n");

            set_string += &typst::empty_line();
            collection += &set_string;
        }
        collection
    }

    pub fn build(&self) -> Result<TypstFile, std::io::Error> {
        let preamble = self.build_preamble();
        let question_string = Self::sets_to_string(&self.question_sets);
        let answer_preamble = typst::page_break() + &typst::reset_enum();
        let answer_string = Self::sets_to_string(&self.answer_sets);

        let typst_file_name = file_handler::to_typst_file_name(&self.file_name);
        let typst_file = file_handler::create_typst_file(&typst_file_name)?;
        file_handler::write(&typst_file, preamble)?;
        file_handler::write(&typst_file, question_string)?;
        file_handler::write(&typst_file, answer_preamble)?;
        file_handler::write(&typst_file, answer_string)?;
        Ok(TypstFile::new(typst_file_name))
    }

    fn build_preamble(&self) -> String {
        //Adjust order of preamble here if required
        let mut preamble = vec![
            String::from(PREAMBLE_STR),
            self.build_page_size(),
            self.build_enum_spacing(),
            self.build_par_spacing(),
        ];

        if !self.heading.is_empty() {
            let heading_string = typst::to_heading(&self.heading) + "\n";
            preamble.push(heading_string);
        }
        preamble.join("\n")
    }

    fn build_page_size(&self) -> String {
        format!(
            "#set page(paper: \"{}\", margin: (x: {}mm, y: {}mm))",
            self.paper_size, self.x_margin, self.y_margin
        )
    }

    fn build_enum_spacing(&self) -> String {
        format!("#set enum(start: 0, spacing: {}mm)", self.enum_spacing)
    }

    fn build_par_spacing(&self) -> String {
        format!("#set par(spacing: {}mm)", self.par_spacing)
    }
}

static PREAMBLE_STR: &str = r#"
#import "@preview/equate:0.3.2": equate, share-align
#show: equate.with(debug: false)


//Colors
#let gray(x) = text(fill: color.linear-rgb(10%, 10%, 10%), $#x$)
#let linecolor = color.linear-rgb(20%, 20%, 20%)

//Enum settings
#let item-counter = counter("item-counter")
#set enum(numbering: it => box(width: 1em, text(weight: "bold")[#it)]))
#show enum: it => {
  if it.start != 0 { return it }
  let args = it.fields()
  let items = args.remove("children")
  context enum(..args, start: item-counter.get().first() + 1, ..items)
  item-counter.update(i => i + it.children.len())
}

//Equation solution template
#let equation-solution(equations, operations, linecolor: black) = {
  context {
    let max-eq-width = 0pt
    let max-op-width = 0pt

    for eq in equations {
      let size = measure(eq)
      if size.width > max-eq-width {
        max-eq-width = size.width
      }
    }

    let gray-operations = operations.map(op => if op != $$ { gray(op) } else { op })

    share-align({
      grid(
        columns: (max-eq-width, auto),
        inset: 5pt,
        align: (left, horizon + center),
        grid.vline(x: 1, stroke: (paint: linecolor, thickness: 0.5pt)),
        ..equations.zip(gray-operations).flatten(),
      )
    })
  }
}
"#;

//#equation-solution(
//  ($ 3x +1 &= 16 - 2x $, $ (5x +1)/2 &= 16 $, $ 5x &= 15 $, $ x&=5/7 $),
//  ($+2x$, $dot.op 2$, $div 5$, $$),
//  linecolor: linecolor,
//)
