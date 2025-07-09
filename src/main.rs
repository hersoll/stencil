use std::time;
use stencil::translations::{self, Translations};
use stencil::{DocumentBuilder, WriteSolutions};

use stencil::{document, problems};
fn main() {
    let translations: translations::Translations = Translations::new("en");

    let now = time::SystemTime::now();
    println!("Generating problems...");
    let problems_1 = stencil::SetBuilder::new()
        .area(problems::SimpleLinearEquations)
        .batch(problems::Difficulty::Intro, 5)
        .batch(problems::Difficulty::Easy, 10)
        .build();
    let _problems_2 = stencil::SetBuilder::new()
        .area(problems::SimpleLinearEquations)
        .batch(problems::Difficulty::Intro, 5)
        .build();
    println!(
        "Problems generated in {} s\n",
        now.elapsed().unwrap_or_default().as_secs_f32()
    );

    let write_time = time::SystemTime::now();
    println!("Writing the document...");
    let typst_file = DocumentBuilder::new(translations)
        .heading("Equations")
        .write_solutions(WriteSolutions::First)
        .add_problem_set(problems_1)
        .build()
        .unwrap();
    println!(
        "Finished writing in {} s\n",
        write_time.elapsed().unwrap_or_default().as_secs_f32()
    );

    let compile_time = time::SystemTime::now();
    println!("Compiling the document...");
    let pdf_path = typst_file.compile().unwrap();
    println!(
        "Finished compiling in {} s\n",
        compile_time.elapsed().unwrap_or_default().as_secs_f32()
    );

    document::file_handler::open_pdf(pdf_path);
}
