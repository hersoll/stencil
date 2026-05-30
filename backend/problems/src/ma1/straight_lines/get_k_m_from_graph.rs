use anyhow::Result;
use db::HasReplacements;
use macros::problem;
use math::{Number, Term, num_gen, symbols::X};
use registry::get_problem_data;
use types::{lang::Language, problems::Problem};
use typst_writer::graphing::{Axes, Graph};

/// Find m in graph
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn find_m(id: i32, lang: Language) -> Result<Problem> {
    // k = -1 makes x-intersect and y-intersect the same - not ideal for learning
    let k_range = num_gen::integer().range(-3, 3).exclude_multiple(&[0, -1]);
    let k = k_range.random();
    // We want the x-intersect to be an integer, easiest way is to make sure m is a multiple of k
    let multiplier = num_gen::integer().range(-2, 2).random();
    let m = k * multiplier;

    // Always show the intersection with the x-axis.
    let x_intersect = -multiplier;
    let x_min = if x_intersect < 0 {
        x_intersect - 1
    } else {
        Number::Integer(-1)
    };
    let x_max = if x_intersect > 0 {
        x_intersect + 1
    } else {
        Number::Integer(1)
    };

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m).with_dot_at(0, m))
        .build_string()?;

    let problem_data = registry::get_problem_data(id)?;
    let question = problem_data.get_question(lang);
    let solution = problem_data.get_solution(lang);

    let question = format!("{question}\n{question_graph}");
    let answer = format!("$m = {m}$");
    let solution = format!("{solution}\n{solution_graph}");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}

/// Find k in graph
/// Absolute difficulty: 3
/// Relative difficulty: 2
#[problem]
fn find_k(id: i32, lang: Language) -> Result<Problem> {
    let k_range = num_gen::integer().range(-3, 3).exclude(0);
    let k = k_range.random();
    // We want the x-intersect to be an integer, easiest way is to make sure m is a multiple of k
    let multiplier = num_gen::integer().range(-2, 2).random();
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

    let problem_data = registry::get_problem_data(id)?;
    let question = problem_data.get_question(lang);
    let solution = problem_data.get_solution(lang);

    let question = format!("{question}\n{question_graph}");
    let answer = format!("$k = {k}$");
    let solution = format!("{solution}\n{solution_graph}");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}

/// Find k and m in graph (k and m are integers)
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn find_k_and_m_integers(id: i32, _lang: Language) -> Result<Problem> {
    let k_range = num_gen::integer().range(-3, 3).exclude(0);
    let k = k_range.random();
    let m = num_gen::integer().range(-3, 3).random();

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

    let k_term = k * X;
    let m_term = Term::from_num(m);
    let expr = k_term.and(&m_term);

    let question = question_graph;
    let answer = format!("$y = {}$", expr.sorted());
    let solution = format!("$k = {k}$, $m = {m}$\n{solution_graph}");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}

/// Draw the graph of 3x - 1
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn draw_own_easy_integers(id: i32, lang: Language) -> Result<Problem> {
    let k_range = num_gen::integer()
        .range(-3, 3)
        .exclude_multiple(&[-1, 0, 1]);
    let k = k_range.random();
    let m = num_gen::integer().range(-3, 3).exclude(0).random();

    let x_min = -3;
    let x_max = 3;

    let y_min = -6;
    let y_max = 6;

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .x_tick(1)
        .y_tick(1)
        .build_string()?;
    let answer_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .x_tick(1)
        .y_tick(1)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .x_tick(1)
        .y_tick(1)
        .add_graph(
            Graph::linear(k, m)
                .with_simple_slope_hint()
                .with_dot_at(0, m),
        )
        .build_string()?;

    let k_term = k * X;
    let m_term = Term::from_num(m);
    let expr = k_term.and(&m_term);

    let question_text = get_problem_data(id)?
        .get_question(lang)
        .replace_placeholders(&[("fn", format!("y = {expr}"))]);
    let question = format!("{question_text}\n{question_graph}");
    let solution = format!("$k = {k}$, $m = {m}$\n{solution_graph}");

    Ok(Problem {
        id,
        question,
        answer: answer_graph,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}

/// Find k and m in graph (k and m are large numbers), y = 40x + 300
/// Absolute difficulty: 5
/// Relative difficulty: 5
#[problem]
fn find_k_and_m_large_numbers(id: i32, _lang: Language) -> Result<Problem> {
    let k_range = num_gen::integer().range(-3, 3).exclude(0);
    let k = k_range.random() * 20;
    let m = num_gen::integer().range(1, 5).random() * 100;

    let x_min = 0;
    let x_max = 6;

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m).with_slope_hint(0, 5, ("x", "y")))
        .build_string()?;

    let k_term = k * X;
    let m_term = Term::from_num(m);
    let expr = k_term.and(&m_term);

    let question = question_graph;
    let answer = format!("$y = {}$", expr.sorted());
    let solution = format!("$k = {five_k}/5 = {k}$\n{solution_graph}", five_k = k * 5);

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}

/// Draw the graph of y = x + 1
/// Absolute difficulty: 5
/// Relative difficulty: 5
#[problem]
fn draw_own_unit_k(id: i32, lang: Language) -> Result<Problem> {
    let k_range = num_gen::integer().numbers(&[-1, 1]);
    let k = k_range.random();
    let m = num_gen::integer().range(-2, 2).random();

    let x_min = -2;
    let x_max = 2;

    let y_min = -3;
    let y_max = 3;

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .build_string()?;
    let answer_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;

    let k_term = k * X;
    let m_term = Term::from_num(m);
    let expr = k_term.and(&m_term);

    let problem_data = get_problem_data(id)?;
    let question_text = problem_data
        .get_question(lang)
        .replace_placeholders(&[("fn", format!("y = {}", expr.sorted()))]);
    let question = format!("{question_text}\n{question_graph}");
    let solution = problem_data.get_solution(lang).to_string();

    Ok(Problem {
        id,
        question,
        answer: answer_graph,
        solution,
        identifiers: vec![k],
        combinations: k_range.len(),
    })
}

/// Draw the graph of y = 2
/// Absolute difficulty: 5
/// Relative difficulty: 6
#[problem]
fn draw_own_horizontal(id: i32, lang: Language) -> Result<Problem> {
    let k = 0;
    let m_range = num_gen::integer().range(-2, 2);
    let m = m_range.random();

    let x_min = -3;
    let x_max = 3;

    let y_min = -4;
    let y_max = 4;

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .build_string()?;
    let answer_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;

    let problem_data = get_problem_data(id)?;
    let question_text = problem_data
        .get_question(lang)
        .replace_placeholders(&[("fn", format!("y = {m}"))]);
    let question = format!("{question_text}\n{question_graph}");
    let solution = problem_data.get_solution(lang).to_string();

    Ok(Problem {
        id,
        question,
        answer: answer_graph,
        solution,
        identifiers: vec![m],
        combinations: m_range.len(),
    })
}

/// Find k and m in graph (k is a fraction)
/// Absolute difficulty: 6
/// Relative difficulty: 7
#[problem]
fn find_k_m_fraction(id: i32, _lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::fraction()
        .denom_range(3, 5)
        .min(-1)
        .max(1)
        .and_random();
    let m = num_gen::integer().range(-3, 3).exclude(0).random();

    let x_min = -1;
    // With some random padding
    let x_max = k.denominator() + num_gen::integer().range(0, 3).random();

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .add_graph(Graph::linear(k, m).with_slope_hint(0, k.denominator(), ("x", "y")))
        .build_string()?;

    let k_term = k * X;
    let m_term = Term::from_num(m);
    let expr = k_term.and(&m_term);

    let question = question_graph;
    let answer = format!("$y = {}$", expr.sorted());
    let solution = format!("$k = {k}$, $m = {m}$\n{solution_graph}");

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k.numerator(), k.denominator()],
        combinations: k_range.len(),
    })
}

/// Draw the graph of y = 3x/5 + 2
/// Absolute difficulty: 7
/// Relative difficulty: 8
#[problem]
fn draw_own_fraction(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::fraction().denoms(&[3, 5, 7]).and_random();
    let m = num_gen::integer().range(-2, 2).exclude(0).random();
    let num = k.numerator();
    let denom = k.denominator();
    let function = (k * X).and(&Term::from_num(m));

    let x_min = -1;
    let x_max = denom + 1;

    let y_min = m.min(Number::Integer(0)) - 1;
    let y_max = num * x_max / denom + 2;

    let question_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .build_string()?;
    let answer_graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .add_graph(Graph::linear(k, m))
        .build_string()?;
    let solution_graph = Axes::new_solution()
        .x_range(x_min, x_max)
        .y_range(y_min, y_max)
        .add_graph(Graph::linear(k, m).with_slope_hint(0, denom, ("x", "y")))
        .build_string()?;

    let problem_data = get_problem_data(id)?;
    let question_text = problem_data
        .get_question(lang)
        .replace_placeholders(&[("fn", format!("y = {function}"))]);
    let question = format!("{question_text}\n{question_graph}");
    let solution = format!("{}\n{solution_graph}", problem_data.get_solution(lang));

    Ok(Problem {
        id,
        question,
        answer: answer_graph,
        solution,
        identifiers: vec![num, denom],
        combinations: k_range.len(),
    })
}
