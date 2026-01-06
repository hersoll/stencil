use crate::{
    Language,
    math::IntRange,
    problems::Problem,
    registry,
    typst_utils::graphing::{Axes, Graph},
};
use anyhow::Result;
use macros::problem;

/// Find m in graph
/// Difficulty: 0
#[problem]
fn find_m(name: String, lang: &Language) -> Result<Problem> {
    // k = -1 makes x-intersect and y-intersect the same - not ideal for learning
    let (k, k_range) = IntRange::without_zero(-3, 3)?.exclude(-1).and_random();
    // We want the x-intersect to be an integer, easiest way is to make sure m is a multiple of k
    let multiplier = IntRange::with_zero(-2, 2)?.random();
    let m = k * multiplier;

    // Always show the intersection with the x-axis.
    let x_intersect = -multiplier;
    let x_min = if x_intersect < 0 { x_intersect - 1 } else { -1 };
    let x_max = if x_intersect > 0 { x_intersect + 1 } else { 1 };

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .padding(1)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m).with_dot_at(0, m))
        .build_string()?;

    let problem_data = registry::get_problem_data(&name)?;
    let question = problem_data.get_question(lang);
    let solution = problem_data.get_solution(lang);

    let question = format!("{question}\n{question_graph}");
    let answer = format!("$m = {m}$");
    let solution = format!("{solution}\n{solution_graph}");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}
