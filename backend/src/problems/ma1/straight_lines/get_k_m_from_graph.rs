use crate::{
    Language,
    math::IntRange,
    problems::Problem,
    typst_utils::graphing::{Axes, Graph},
};
use anyhow::Result;
use macros::problem;

/// Find m in graph
/// Difficulty: 0
#[problem]
fn find_m(name: String, _lang: &Language) -> Result<Problem> {
    let (k, k_range) = IntRange::without_zero(-3, 3)?.and_random();
    let (m, m_range) = IntRange::with_zero(-5, 5)?.and_random();

    // Always show the intersection with the x-axis.
    // y = kx + m => x = (y - m) / k with y = 0
    let x_0 = -m / k;
    let mut x_min = if x_0 < 0 { x_0 - 1 } else { -1 };
    let mut x_max = if x_0 > 0 { x_0 + 1 } else { 1 };

    // The graph (currently) looks weird when there is only one step.
    // Will be fixed if I autosize the grid
    if x_min == -1 && x_max == 1 {
        x_min = -2;
        x_max = 2;
    }

    let mut axes = Axes::new();
    axes.x_range(x_min, x_max).padding(1); // Show y-values over/under graph

    let question_graph = axes.add_graph(Graph::linear(k, m)).build_string()?;
    let solution_graph = axes.add_graph(Graph::linear(k, m)).build_string()?;

    let question = question_graph;
    let answer = format!("$m = {m}$");
    let solution = format!("Blabla\n{solution_graph}");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    })
}
