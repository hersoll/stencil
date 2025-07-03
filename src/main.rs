use std::time;

use stencil::problems::{Difficulty, Problem, ProblemBuilder};
use stencil::{problems, typst_writer};
fn main() {
    let now = time::SystemTime::now();
    let problems = problems::SimpleEquation::new()
        .add(Difficulty::Intro, 2)
        .add(Difficulty::Intro, 3)
        .build();

    let message = problems.iter().next().unwrap().question();
    match typst_writer::write(message) {
        Ok(_) => println!(
            "Finished the program. Duration: {} s",
            now.elapsed().unwrap_or_default().as_secs_f32()
        ),
        Err(e) => eprintln!("Error: {}", e),
    }

    let equations = problems::SimpleEquation::new()
        .add(Difficulty::Intro, 1)
        .build();
    let equation = equations.first().unwrap();
    println!("{}", equation.question());
    println!("{}", equation.answer());
}
