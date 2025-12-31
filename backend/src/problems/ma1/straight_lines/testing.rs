use macros::problem;

use crate::{Language, math, problems::Problem};
use anyhow::Result;

/// Testing
/// Difficulty: 0
#[problem]
fn test_graph(name: String, _lang: &Language) -> Result<Problem> {
    let question = math::graphing::Graph::new().render()?;
    let answer = String::from("$2x + 1$");
    let solution = String::from("Reading the line explains the line");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![1],
        combinations: 1,
    })
}
