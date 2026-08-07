use anyhow::Result;
use macros::problem;
use math::{
    Number,
    num_gen::{self, NumberGenerator},
};
use registry::{get_question, get_solution};
use types::{
    format_strings::HasReplacements,
    lang::Language,
    problems::{Problem, ProblemParameters},
};
use typst_writer::graphing::{Axes, Graph};

/// Find y if x = 2
/// Absolute difficulty: 2
/// Relative difficulty: 1
#[problem]
fn find_y(id: i32, lang: Language) -> Result<Problem> {
    let k = Number::from((-1, 2));
    let m = num_gen::integer().range(4, 7).exclude(6).random();

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
                .build_string()?,
        ),
        ("x", x.to_string()),
    ]);
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$y = {y}$"),
        solution,
        identifiers: x,
        combinations: x_range,
    }))
}
