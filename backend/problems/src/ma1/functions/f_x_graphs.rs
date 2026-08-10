use anyhow::Result;
use macros::problem;
use math::{
    Number,
    functions::Function,
    num_gen::{self, NumberGenerator},
    symbols::{self, F, Y, inequality_sign::InequalitySign},
};
use registry::{get_answer, get_question, get_solution};
use types::{
    format_strings::{HasReplacements, HasSubdivisions},
    lang::Language,
    problems::{Answer, Problem, ProblemParameters, Question},
};
use typst_writer::graphing::{Axes, Graph};

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
    // k = -1 makes y-intersect and x-intersect the same
    let k = num_gen::integer()
        .range(-2, 2)
        .exclude(0)
        .exclude(-1)
        .random();
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
    // k = -1 makes y-intersect and x-intersect the same
    let (k, k_range) = num_gen::integer()
        .range(-2, 2)
        .exclude(0)
        .exclude(-1)
        .and_random();
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

    let graph = Axes::new()
        .x_range(-3, 3)
        .fit_x(x)
        .add_graph(Graph::linear(k1, m1).label("$f(x)$"))
        .add_graph(Graph::linear(k2, m2).label("$g(x)$"))
        .legend()
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

/// Solve f(x) < 2
/// Absolute difficulty: 6
/// Relative difficulty: 6
#[problem]
fn positive_linear_inequality(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(1, 2).and_random();
    let (m, m_range) = num_gen::integer().range(-3, 3).and_random();
    let x = num_gen::integer().range(-4, 4).random();

    let y = k * x + m;
    let f_name = symbols::get_function_name()?;

    let mut axes = Axes::new();
    axes.x_range(-3, 3).fit_x(x);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("f(x)", format!("{f_name}(x)")),
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
        ("f(x)", format!("{f_name}(x)")),
        ("y", y.to_string()),
        ("x", x.to_string()),
    ]);
    let answer = get_answer(id, lang)?.replace_one("x", x);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    }))
}

/// Solve f(x) < 2
/// Absolute difficulty: 6
/// Relative difficulty: 6
#[problem]
fn negative_linear_inequality(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(-2, -1).and_random();
    let (m, m_range) = num_gen::integer().range(-3, 3).and_random();
    let x = num_gen::integer().range(-4, 4).random();

    let y = k * x + m;
    let f_name = symbols::get_function_name()?;

    let mut axes = Axes::new();
    axes.x_range(-3, 3).fit_x(x);

    let question = get_question(id, lang)?.replace_multiple(&[
        ("graph", axes.add_graph(Graph::linear(k, m)).build_string()?),
        ("f(x)", format!("{f_name}(x)")),
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
        ("f(x)", format!("{f_name}(x)")),
        ("y", y.to_string()),
        ("x", x.to_string()),
    ]);
    let answer = get_answer(id, lang)?.replace_one("x", x);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    }))
}

/// Solve f(x) = g(x)
/// Absolute difficulty: 7
/// Relative difficulty: 7
#[problem]
fn f_equals_g_quadratic(id: i32, lang: Language) -> Result<Problem> {
    // Start by determining the intersect points
    let (x1, x1_range) = num_gen::integer().range(1, 4).and_random();
    let (x2, x2_range) = num_gen::integer().range(-4, -1).and_random();
    let (y1, y_range) = num_gen::integer().range(-2, 3).and_random();
    let y2 = y_range.exclude(y1).random();

    let k = (y2 - y1) / (x2 - x1);
    let m = y2 - k * x2;

    let graph = Axes::new()
        .x_range(-2, 2)
        .fit_x(x1)
        .fit_x(x2)
        .add_graph(Graph::linear(k, m).label("$f(x)$"))
        .add_graph(
            Graph::default()
                .function(Function::quadratic_from_points((x1, y1), (x2, y2), (0, -4)))
                .label("$g(x)$"),
        )
        .legend()
        .build_string()?;

    let question = get_question(id, lang)?.replace_one("graph", graph);
    let solution = get_solution(id, lang)?.to_string();

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x_1 = {x2}, thick x_2 = {x1}$"),
        solution,
        identifiers: vec![x1, x2],
        combinations: x1_range.len() * x2_range.len(),
    }))
}

/// Solve f(x) < g(x) (linear)
/// Absolute difficulty: 7
/// Relative difficulty: 7
#[problem]
fn f_less_than_g_linear(id: i32, lang: Language) -> Result<Problem> {
    // Start by determining the intersect point
    let (x, x_range) = num_gen::integer().range(1, 4).and_random();
    let (y, y_range) = num_gen::integer().range(-4, 4).and_random();
    // Choose two different m, doesn't matter that k gets fractional
    let m1 = num_gen::integer().range(1, 5).random();
    let m2 = num_gen::integer().range(-5, -1).random();

    let k1 = ((y - m1) / x).simplify();
    let k2 = ((y - m2) / x).simplify();

    let sign = InequalitySign::random_less();
    let swapped = sign.swapped();

    let graph = Axes::new()
        .x_range(-1, 3)
        .fit_x(x)
        .add_graph(Graph::linear(k1, m1).label("$f(x)$"))
        .add_graph(Graph::linear(k2, m2).label("$g(x)$"))
        .legend()
        .build_string()?;

    let question =
        get_question(id, lang)?.replace_multiple(&[("graph", graph), ("sign", sign.to_string())]);

    let graph = Axes::new()
        .x_range(-1, 3)
        .fit_x(x)
        .add_graph(Graph::linear(k1, m1).label("$f(x)$").with_name("f"))
        .add_graph(Graph::linear(k2, m2).label("$g(x)$").with_name("g"))
        .legend()
        .custom(format!("plot.add-fill-between(domain: ({x}, 5), f, g)"))
        .build_string()?;
    let solution =
        get_solution(id, lang)?.replace_multiple(&[("graph", graph), ("x", x.to_string())]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x {swapped} {x}$"),
        solution,
        identifiers: vec![x, y],
        combinations: x_range.len() * y_range.len(),
    }))
}

/// Solve f(x) > g(x) (linear)
/// Absolute difficulty: 7
/// Relative difficulty: 7
#[problem]
fn f_greater_than_g_linear(id: i32, lang: Language) -> Result<Problem> {
    // Start by determining the intersect point
    let (x, x_range) = num_gen::integer().range(1, 4).and_random();
    let (y, y_range) = num_gen::integer().range(-4, 4).and_random();
    // Choose two different m, doesn't matter that k gets fractional
    let m1 = num_gen::integer().range(1, 5).random();
    let m2 = num_gen::integer().range(-5, -1).random();

    let k1 = ((y - m1) / x).simplify();
    let k2 = ((y - m2) / x).simplify();

    let sign = InequalitySign::random_greater();
    let swapped = sign.swapped();

    let graph = Axes::new()
        .x_range(-1, 3)
        .fit_x(x)
        .add_graph(Graph::linear(k1, m1).label("$f(x)$"))
        .add_graph(Graph::linear(k2, m2).label("$g(x)$"))
        .legend()
        .build_string()?;

    let question =
        get_question(id, lang)?.replace_multiple(&[("graph", graph), ("sign", sign.to_string())]);

    let graph = Axes::new()
        .x_range(-1, 3)
        .fit_x(x)
        .add_graph(Graph::linear(k1, m1).label("$f(x)$").with_name("f"))
        .add_graph(Graph::linear(k2, m2).label("$g(x)$").with_name("g"))
        .legend()
        .custom(format!("plot.add-fill-between(domain: (-1, {x}), f, g)"))
        .build_string()?;
    let solution =
        get_solution(id, lang)?.replace_multiple(&[("graph", graph), ("x", x.to_string())]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x {swapped} {x}$"),
        solution,
        identifiers: vec![x, y],
        combinations: x_range.len() * y_range.len(),
    }))
}

/// Solve f(x) < 4 (quadratic)
/// Absolute difficulty: 8
/// Relative difficulty: 8
#[problem]
fn quadratic_inequality(id: i32, lang: Language) -> Result<Problem> {
    let (symmetry, sym_range) = num_gen::integer().range(-3, 3).and_random();
    let (distance, dist_range) = num_gen::integer().range(1, 3).and_random();
    let f = symbols::get_function_name()?;
    let func = Function::quadratic_from_sym_dist(symmetry, distance);
    // We choose an integer x which will guarantee integer y
    let x_distance = dist_range.random();
    let x1 = symmetry - x_distance;
    let x2 = symmetry + x_distance;
    let y = func.get_y(&x1).unwrap();
    let sign = InequalitySign::random_less();

    let question_graph = Axes::new()
        .x_range(-1, 1)
        .fit_x(x1)
        .fit_x(x2)
        .add_graph(Graph::from_function(func))
        .build_string()?;
    let question = get_question(id, lang)?.replace_multiple(&[
        ("y", y.to_string()),
        ("sign", sign.to_string()),
        ("f", f.to_string()),
        ("graph", question_graph),
    ]);
    let solution_graph = Axes::new_solution()
        .x_range(-2, 2)
        .fit_x(x1)
        .fit_x(x2)
        .add_graph(
            Graph::from_function(func)
                .with_name("quad")
                .dots(vec![(x1, y), (x2, y)]),
        )
        .custom(format!(
            "plot.add-fill-between(domain: ({x1}, {x2}), quad, ( ({x1}, {y}), ({x2}, {y}) ))"
        ))
        .build_string()?;

    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("f", f.to_string()),
        ("y", y.to_string()),
        ("x1", x1.to_string()),
        ("x2", x2.to_string()),
        ("sign", sign.to_string()),
        ("graph", solution_graph),
    ]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${x1} {sign} x {sign} {x2}$"),
        solution,
        identifiers: vec![symmetry, x_distance],
        combinations: sym_range.len() * dist_range.len(),
    }))
}

/// Solve f(x) > g(x) (quadratic)
/// Absolute difficulty: 8
/// Relative difficulty: 8
#[problem]
fn f_greater_than_g_quadratic(id: i32, lang: Language) -> Result<Problem> {
    // Start by determining the intersect points
    let (x1, x1_range) = num_gen::integer().range(-4, -1).and_random();
    let (x2, x2_range) = num_gen::integer().range(1, 4).and_random();
    let (y1, y_range) = num_gen::integer().range(-2, 3).and_random();
    let y2 = y_range.exclude(y1).random();

    let k = (y2 - y1) / (x2 - x1);
    let m = y2 - k * x2;
    let sign = InequalitySign::random_greater();
    let swap_sign = sign.swapped();

    let graph = Axes::new()
        .x_range(-2, 2)
        .fit_x(x1)
        .fit_x(x2)
        .add_graph(Graph::linear(k, m).label("$f(x)$"))
        .add_graph(
            Graph::default()
                .function(Function::quadratic_from_points((x1, y1), (x2, y2), (0, -4)))
                .label("$g(x)$"),
        )
        .legend()
        .build_string()?;

    let question =
        get_question(id, lang)?.replace_multiple(&[("sign", sign.to_string()), ("graph", graph)]);

    let graph = Axes::new_solution()
        .x_range(-2, 2)
        .fit_x(x1)
        .fit_x(x2)
        .add_graph(Graph::linear(k, m).label("$f(x)$").with_name("f"))
        .add_graph(
            Graph::default()
                .function(Function::quadratic_from_points((x1, y1), (x2, y2), (0, -4)))
                .label("$g(x)$")
                .with_name("g"),
        )
        .custom(format!("plot.add-fill-between(domain: ({x1}, {x2}), f, g)"))
        .legend()
        .build_string()?;
    let solution = get_solution(id, lang)?.replace_multiple(&[("graph", graph)]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${x1} {swap_sign} x {swap_sign} {x2}$"),
        solution,
        identifiers: vec![x1, x2],
        combinations: x1_range.len() * x2_range.len(),
    }))
}

/// Solve f(x) < g(x) (quadratic)
/// Absolute difficulty: 8
/// Relative difficulty: 8
#[problem]
fn f_less_than_g_quadratic(id: i32, lang: Language) -> Result<Problem> {
    // Start by determining the intersect points
    let (x1, x1_range) = num_gen::integer().range(-4, -1).and_random();
    let (x2, x2_range) = num_gen::integer().range(1, 4).and_random();
    let (y1, y_range) = num_gen::integer().range(-2, 3).and_random();
    let y2 = y_range.exclude(y1).random();

    let k = (y2 - y1) / (x2 - x1);
    let m = y2 - k * x2;
    let sign = InequalitySign::random_less();
    let swap_sign = sign.swapped();

    let graph = Axes::new()
        .x_range(-2, 2)
        .fit_x(x1)
        .fit_x(x2)
        .add_graph(Graph::linear(k, m).label("$f(x)$"))
        .add_graph(
            Graph::default()
                .function(Function::quadratic_from_points((x1, y1), (x2, y2), (0, -4)))
                .label("$g(x)$"),
        )
        .legend()
        .build_string()?;

    let question =
        get_question(id, lang)?.replace_multiple(&[("sign", sign.to_string()), ("graph", graph)]);

    let graph = Axes::new_solution()
        .x_range(-2, 2)
        .fit_x(x1)
        .fit_x(x2)
        .add_graph(Graph::linear(k, m).label("$f(x)$").with_name("f"))
        .add_graph(
            Graph::default()
                .function(Function::quadratic_from_points((x1, y1), (x2, y2), (0, -4)))
                .label("$g(x)$")
                .with_name("g"),
        )
        .custom(format!("plot.add-fill-between(domain: (-5, {x1}), f, g)"))
        .custom(format!("plot.add-fill-between(domain: ({x2}, 5), f, g)"))
        .legend()
        .build_string()?;
    let solution = get_solution(id, lang)?.replace_multiple(&[("graph", graph)]);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x {sign} {x1}, thick thick x {swap_sign} {x2}$"),
        solution,
        identifiers: vec![x1, x2],
        combinations: x1_range.len() * x2_range.len(),
    }))
}

/// Solve the equation f(x) = kx + m
/// Absolute difficulty: 10
/// Relative difficulty: 10
#[problem]
fn f_equals_formula(id: i32, lang: Language) -> Result<Problem> {
    // Start with the rhs formula
    let (k, k_range) = num_gen::integer().range(1, 3).and_random();
    let (m, m_range) = num_gen::integer().range(-4, 1).and_random();
    // Intersection point
    let x = num_gen::integer().range(1, 4).random();
    let func_rhs = Function::linear(k, m).with_function_notation().with_name(F);
    let y = func_rhs.get_y(&x).unwrap();
    // Lhs
    let m_lhs = num_gen::integer().range(2, 6).exclude(y).random();
    let k_lhs = (y - m_lhs) / x;

    let graph = Axes::new()
        .x_range(-1, 3)
        .y_min(m - 1)
        .fit_x(x)
        .add_graph(Graph::linear(k_lhs, m_lhs))
        .build_string()?;
    let question =
        get_question(id, lang)?.replace_multiple(&[("graph", graph), ("eq", func_rhs.to_string())]);

    let graph = Axes::new()
        .x_range(-1, 3)
        .y_min(m - 1)
        .fit_x(x)
        .add_graph(Graph::linear(k_lhs, m_lhs))
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution = get_solution(id, lang)?.replace_multiple(&[
        ("graph", graph),
        (
            "eq",
            func_rhs
                .with_name(Y)
                .without_function_notation()
                .to_string(),
        ),
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
