use macros::problem;

use crate::{
    math::{self, graphing::PlotType, IntRange},
    problems::Problem,
    Language,
};
use anyhow::Result;

/// Test linear graph
/// Difficulty: 2
#[problem]
fn linear_graph(name: String, _lang: &Language) -> Result<Problem> {
    let (k, k_range) = IntRange::with_zero(-3, 3)?.and_random();
    let (m, m_range) = IntRange::with_zero(-3, 3)?.and_random();
    let x_min = -1;
    let x_max = 2;
    let graph = math::graphing::Graph::new()
        .x_range(x_min, x_max)
        .add_plot(PlotType::Linear(k, m))
        .auto_y_range()?
        .render()?;

    let question = format!("Här är en rät linje: \n {graph}");
    let answer = format!("$y = {k}x {m:+}$");
    let solution = String::from("look at the fucking line");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    })
}

/// Test exponential graph
/// Difficulty: 5
#[problem]
fn exponential_graph(name: String, _lang: &Language) -> Result<Problem> {
    let (c, c_range) = IntRange::with_zero(1, 3)?.and_random();
    let (a, a_range) = IntRange::with_zero(2, 3)?.and_random();
    let x_min = 0;
    let x_max = 2;
    let graph = math::graphing::Graph::new()
        .x_range(x_min as i32, x_max as i32)
        .add_plot(PlotType::Exponential(c, a))
        .auto_y_range()?
        .render()?;

    let question = format!("Här är en exponentialfunktion: \n {graph}");
    let answer = format!("$y = {c} dot {a}^x$");
    let solution = String::from("look at the fucking line");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![c, a],
        combinations: c_range.len() * a_range.len(),
    })
}
