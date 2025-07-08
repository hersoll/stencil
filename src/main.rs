use std::time;

use stencil::{document, problems};
fn main() {
    let now = time::SystemTime::now();
    println!("Generating problems...");
    let problems_1 = stencil::SetBuilder::new()
        .area(problems::SimpleLinearEquations)
        .batch(problems::Difficulty::Intro, 20)
        .build();
    let problems_2 = stencil::SetBuilder::new()
        .area(problems::SimpleLinearEquations)
        .batch(problems::Difficulty::Intro, 5)
        .build();
    println!(
        "Problems generated in {} s\n",
        now.elapsed().unwrap_or_default().as_secs_f32()
    );

    let write_time = time::SystemTime::now();
    println!("Writing the document...");
    let typst_file = document::DocumentBuilder::new()
        .heading("Equations")
        .write_solutions(document::WriteSolutions::First)
        .add_problem_set(problems_1)
        .add_problem_set(problems_2)
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
