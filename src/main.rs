use std::time;

use stencil::{problems, typst_writer};
fn main() {
    let now = time::SystemTime::now();
    println!("Generating problems...");
    let problems = stencil::SetBuilder::new()
        .area(problems::SimpleLinearEquations)
        .batch(stencil::Difficulty::Intro, 10)
        .build();
    println!(
        "Problems generated in {} s",
        now.elapsed().unwrap_or_default().as_secs_f32()
    );

    let message = problems.iter().next().unwrap().question();
    match typst_writer::write(message) {
        Ok(_) => println!(
            "Finished the program. Duration: {} s",
            now.elapsed().unwrap_or_default().as_secs_f32()
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
}
