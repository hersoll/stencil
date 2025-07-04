use std::time;

use stencil::{problems, typst_writer};
fn main() {
    let now = time::SystemTime::now();
    println!("Generating problems...");
    let problems_1 = stencil::SetBuilder::new()
        .exclude(problems::SimpleLinearEquations::ONLY_MULTIPLICATION)
        .area(problems::SimpleLinearEquations)
        .batch(stencil::Difficulty::Intro, 5)
        .build();
    let problems_2 = stencil::SetBuilder::new()
        .exclude(problems::SimpleLinearEquations::ONLY_ADDITION_OR_SUBTRACTION)
        .area(problems::SimpleLinearEquations)
        .batch(stencil::Difficulty::Intro, 5)
        .build();
    println!(
        "Problems generated in {} s\n",
        now.elapsed().unwrap_or_default().as_secs_f32()
    );

    // TODO: Refactor. Should be baked in to SetBuilder, should include logic about when to use answer and when to use solution
    let processed_problems_1: Vec<problems::ProcessedProblem> = problems_1
        .into_iter()
        .map(|problem| problem.process())
        .collect();
    let processed_problems_2: Vec<problems::ProcessedProblem> = problems_2
        .into_iter()
        .map(|problem| problem.process())
        .collect();

    let write_time = time::SystemTime::now();
    println!("Writing the document...");
    let typst_file = typst_writer::TypstWriter::new()
        .add_problem_set(processed_problems_1)
        .add_problem_set(processed_problems_2)
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

    crate::typst_writer::file_handler::open_pdf(pdf_path);
}
