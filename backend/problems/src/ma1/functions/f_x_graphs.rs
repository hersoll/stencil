use std::cmp::Ordering::{Greater, Less};

use anyhow::Result;
use macros::problem;
use math::{
    Number,
    num_gen::{self, NumberGenerator},
    symbols,
};
use registry::{get_question, get_solution};
use types::{
    format_strings::{HasReplacements, HasSubdivisions},
    lang::Language,
    problems::{Answer, Problem, ProblemParameters, Question},
};
use typst_writer::graphing::{Axes, Direction, Graph};

/// Find y if x = 2
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn find_y(id: i32, lang: Language) -> Result<Problem> {
    let k = Number::from((-1, 2));
    let (m, m_range) = num_gen::integer().range(4, 7).exclude(6).and_random();

    let (x, x_range) = num_gen::integer().range_step(2, 6, 2).and_random();
    let f = |x: Number| (k * x + m).simplify();
    let y = f(x);

    let mut axes = Axes::new();
    axes.x_range(-1, 7);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("x", x.to_string()),
    ]);
    axes.clear_graphs();
    let solution = get_solution(id, lang)?.replace_multiple(&[
        (
            "graph",
            axes.add_graph(Graph::linear(k, m).dot_with_lines(x, y))
                .solution_size()
                .build_string()?,
        ),
        ("x", x.to_string()),
    ]);
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$y = {y}$"),
        solution,
        identifiers: vec![x, m],
        combinations: x_range.len() * m_range.len(),
    }))
}

/// Find x if y = 4
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn find_x(id: i32, lang: Language) -> Result<Problem> {
    let k = Number::from((-1, 2));
    let m = num_gen::integer().range(4, 7).exclude(6).random();

    let (x, x_range) = num_gen::integer().range_step(2, 6, 2).and_random();
    let f = |x: Number| (k * x + m).simplify();
    let y = f(x);

    let mut axes = Axes::new();
    axes.x_range(-1, 7);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("y", y.to_string()),
    ]);
    axes.clear_graphs();
    let solution = get_solution(id, lang)?.replace_multiple(&[
        (
            "graph",
            axes.add_graph(Graph::linear(k, m).dot_with_lines(x, y))
                .solution_size()
                .build_string()?,
        ),
        ("y", y.to_string()),
    ]);
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x = {x}$"),
        solution,
        identifiers: vec![y],
        combinations: x_range,
    }))
}

/// Find f(2)
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn find_y_with_f(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(-2, 2).exclude(0).and_random();
    let (m, m_range) = num_gen::integer().range(-3, 3).and_random();
    let zero = -m / k;
    let x = num_gen::integer()
        .range(-4, 4)
        .exclude(0)
        .exclude(zero)
        .random();

    let y = k * x + m;
    let f_name = symbols::get_function_name()?;

    let mut axes = Axes::new();
    axes.x_range(-3, 3).fit_x(x);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("f(x)", format!("${f_name}(x)$")),
        ("f(a)", format!("${f_name}({x})$")),
    ]);
    axes.clear_graphs();
    let solution = get_solution(id, lang)?.replace_multiple(&[
        (
            "graph",
            axes.add_graph(Graph::linear(k, m).dot_with_lines(x, y))
                .solution_size()
                .build_string()?,
        ),
        ("f(a)", format!("${f_name}({x})$")),
        ("x", x.to_string()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${f_name}({x}) = {y}$"),
        solution,
        identifiers: vec![k, m, x],
        combinations: k_range.len() * m_range.len(),
    }))
}

/// Solve f(x) = 2
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn find_x_with_f(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(-2, 2).exclude(0).and_random();
    let (m, m_range) = num_gen::integer().range(-3, 3).and_random();
    let zero = -m / k;
    let x = num_gen::integer()
        .range(-4, 4)
        .exclude(0)
        .exclude(zero)
        .random();

    let y = k * x + m;
    let f_name = symbols::get_function_name()?;

    let mut axes = Axes::new();
    axes.x_range(-3, 3).fit_x(x);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("f(x)", format!("${f_name}(x)$")),
        ("f(x) = a", format!("${f_name}(x) = {y}$")),
    ]);
    axes.clear_graphs();
    let solution = get_solution(id, lang)?.replace_multiple(&[
        (
            "graph",
            axes.add_graph(Graph::linear(k, m).dot_with_lines(x, y))
                .solution_size()
                .build_string()?,
        ),
        ("f(x)", format!("{f_name}(x)")),
        ("y", y.to_string()),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x = {x}$"),
        solution,
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    }))
}

/// a) Evaluate f(2) b) Solve f(x) = 2
/// Absolute difficulty: 5
/// Relative difficulty: 5
#[problem]
fn find_x_and_y_with_f(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(-2, 2).exclude(0).and_random();
    let (m, m_range) = num_gen::integer().range(-3, 3).and_random();

    let (x1, x_range) = num_gen::integer().range(-4, 4).and_random();
    let x2 = x_range.exclude(x1).random();
    let y1 = k * x1 + m;
    let y2 = k * x2 + m;

    let f_name = symbols::get_function_name()?;

    let mut axes = Axes::new();
    axes.x_range(-3, 3).fit_x(x1).fit_x(x2);

    let mut questions = get_question(id, lang)?.to_subdivisions();
    let mut question = Question::subquestions();
    question
        .pre(
            questions
                .pre()
                .replace_one("f(x)", format!("${f_name}(x)$")),
        )
        .subquestion(
            questions
                .sub()
                .replace_one("f(a)", format!("${f_name}({x1})$")),
        )
        .subquestion(
            questions
                .sub()
                .replace_one("f(x) = a", format!("${f_name}(x) = {y2}$")),
        )
        .post(
            questions
                .post()
                .replace_one("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        );

    axes.clear_graphs();
    let solution = get_solution(id, lang)?.replace_multiple(&[
        (
            "graph",
            axes.add_graph(Graph::linear(k, m).dots(vec![(x1, y1), (x2, y2)]))
                .solution_size()
                .build_string()?,
        ),
        ("f(x) = a", format!("${f_name}(x) = {y2}$")),
        ("f(a)", format!("${f_name}({x1})$")),
        ("x", x1.to_string()),
        ("y", y2.to_string()),
    ]);

    let mut answer = Answer::subanswers();
    answer
        .subanswer(format!("${f_name}({x1}) = {y1}$"))
        .subanswer(format!("$x = {x2}$"));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    }))
}

/// Find f(0)
/// Absolute difficulty: 5
/// Relative difficulty: 5
#[problem]
fn find_y_zero(id: i32, lang: Language) -> Result<Problem> {
    let k = num_gen::integer().range(-2, 2).exclude(0).random();
    let (m, m_range) = num_gen::integer().range(-3, 3).exclude(0).and_random();
    let x = Number::Integer(0);
    let y = m;
    let f_name = symbols::get_function_name()?;

    let zero = -m / k;
    let mut axes = Axes::new();
    axes.x_range(-3, 3).fit_x(zero);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("f(x)", format!("${f_name}(x)$")),
        ("f(a)", format!("${f_name}({x})$")),
    ]);
    axes.clear_graphs();
    let solution = get_solution(id, lang)?.replace_multiple(&[
        (
            "graph",
            axes.add_graph(Graph::linear(k, m).dot_with_lines(x, y))
                .solution_size()
                .build_string()?,
        ),
        ("f(a)", format!("${f_name}({x})$")),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${f_name}({x}) = {y}$"),
        solution,
        identifiers: vec![m],
        combinations: m_range,
    }))
}

/// Solve f(x) = 0
/// Absolute difficulty: 5
/// Relative difficulty: 5
#[problem]
fn find_x_zero(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(-2, 2).exclude(0).and_random();
    let (m, m_range) = num_gen::integer()
        .range_step(-4, 4, 2)
        .exclude(0)
        .and_random();
    let x = -m / k;
    let y = 0;
    let f_name = symbols::get_function_name()?;

    let mut axes = Axes::new();
    axes.x_range(-3, 3).fit_x(x);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("f(x)", format!("${f_name}(x)$")),
        ("f(x) = a", format!("${f_name}(x) = {y}$")),
    ]);
    axes.clear_graphs();
    let solution = get_solution(id, lang)?.replace_multiple(&[
        (
            "graph",
            axes.add_graph(Graph::linear(k, m).dot_with_lines(x, y))
                .solution_size()
                .build_string()?,
        ),
        ("f(x) = a", format!("${f_name}(x) = {y}$")),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x = {x}$"),
        solution,
        identifiers: vec![k, m],
        combinations: m_range.len() * k_range.len(),
    }))
}

/// Solve f(x) = g(x)
/// Absolute difficulty: 6
/// Relative difficulty: 6
#[problem]
fn f_equals_g_linear(id: i32, lang: Language) -> Result<Problem> {
    // Start by determining the intersect point
    let (x, x_range) = num_gen::integer().range(-4, 4).exclude(0).and_random();
    let (y, y_range) = num_gen::integer().range(-4, 4).and_random();
    // Choose two different m, doesn't matter that k gets fractional
    let m1 = num_gen::integer().range(1, 5).random();
    let m2 = num_gen::integer().range(-5, -1).random();

    let k1 = ((y - m1) / x).simplify();
    let k2 = ((y - m2) / x).simplify();

    let legend_dir = match (0.cmp(&x.as_i32()), 0.cmp(&y.as_i32())) {
        (Greater, Greater) => Direction::NorthWest,
        (Less, Greater) => Direction::NorthEast,
        (Greater, Less) => Direction::SouthWest,
        (Less, Less) => Direction::SouthEast,
        (Less, _) => Direction::NorthEast,
        (Greater, _) => Direction::NorthWest,
        (_, _) => Direction::NorthEast,
    };

    let graph = Axes::new()
        .x_range(-3, 3)
        .fit_x(x)
        .add_graph(Graph::linear(k1, m1).label("$f(x)$"))
        .add_graph(Graph::linear(k2, m2).label("$g(x)$"))
        .legend(Direction::South)
        .build_string()?;

    let question = get_question(id, lang)?.replace_one("graph", graph);
    let solution = get_solution(id, lang)?.to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x = {x}$"),
        solution,
        identifiers: vec![x, y],
        combinations: x_range.len() * y_range.len(),
    }))
}
