use crate::{
    Language,
    math::{IntRange, Number, Polynomial, Term},
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
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
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

/// Find k in graph
/// Difficulty: 1
#[problem]
fn find_k(name: String, lang: &Language) -> Result<Problem> {
    let (k, k_range) = IntRange::without_zero(-3, 3)?.and_random();
    // We want the x-intersect to be an integer, easiest way is to make sure m is a multiple of k
    let multiplier = IntRange::with_zero(-2, 2)?.random();
    let m = k * multiplier;

    let x_min = -1;
    let x_max = 2;

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m).with_simple_slope_hint())
        .build_string()?;

    let problem_data = registry::get_problem_data(&name)?;
    let question = problem_data.get_question(lang);
    let solution = problem_data.get_solution(lang);

    let question = format!("{question}\n{question_graph}");
    let answer = format!("$k = {k}$");
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

/// Find k and m in graph (k and m are integers)
/// Difficulty: 2
#[problem]
fn find_k_and_m_integers(name: String, _lang: &Language) -> Result<Problem> {
    let (k, k_range) = IntRange::without_zero(-3, 3)?.and_random();
    let m = IntRange::with_zero(-3, 3)?.random();

    let x_min = -1;
    let x_max = 2;

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .add_graph(
            Graph::linear(k, m)
                .with_simple_slope_hint()
                .with_dot_at(0, m),
        )
        .build_string()?;

    let k_term = Term::from((k, 'x'));
    let m_term = Term::from(m);
    let expr: Polynomial = vec![k_term, m_term].into();

    let question = question_graph;
    let answer = format!("$y = {}$", expr.sorted());
    let solution = format!("$k = {k}$, $m = {m}$\n{solution_graph}");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}

/// Find k and m in graph (k is a fraction)
/// Difficulty: 4
#[problem]
fn find_k_m_fraction(name: String, _lang: &Language) -> Result<Problem> {
    let (k_num, k_num_range) = IntRange::without_zero(-5, 5)?.and_random();
    let (k_denom, k_denom_range) = IntRange::without_zero(3, 5)?.and_random();
    let m = IntRange::with_zero(-3, 3)?.random();
    let k = Number::Fraction(k_num, k_denom);

    let x_min = -1;
    // With some random padding
    let x_max = k_denom + IntRange::with_zero(0, 3)?.random();

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m).with_slope_hint(0, k_denom, ("x", "y")))
        .build_string()?;

    let k_term = Term::from((k, 'x'));
    let m_term = Term::from(m);
    let expr: Polynomial = vec![k_term, m_term].into();

    let question = question_graph;
    let answer = format!("$y = {}$", expr.sorted());
    let solution = format!("$k = {k}$, $m = {m}$\n{solution_graph}");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k_num, k_denom],
        combinations: k_num_range.len() * k_denom_range.len(),
    })
}
