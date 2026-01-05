use macros::problem;

use crate::{
    Language,
    math::{IntRange, Number, Term},
    problems::Problem,
    typst_utils::plotting::{Axes, Plot},
};
use anyhow::Result;

#[problem]
fn empty_graph(name: String, _lang: &Language) -> Result<Problem> {
    let x_min = -1;
    let x_max = 2;
    let graph = Axes::new()
        .x_range(x_min, x_max)
        .y_range(-3, 4)
        .add_plot(Plot::linear(Number::Integer(0), Number::Integer(1000)))
        .build_string()?;

    let question = format!("Här är ett tomt koordinatsystem: \n {graph}");
    let answer = format!("Tom");
    let solution = String::from("look at the fucking line");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![1],
        combinations: 1,
    })
}

#[problem]
fn linear_graph(name: String, _lang: &Language) -> Result<Problem> {
    let (k, k_range) = IntRange::with_zero(-3, 3)?.and_random();
    let (m, m_range) = IntRange::with_zero(-3, 3)?.and_random();
    let x_min = -1;
    let x_max = 2;
    let graph = Axes::new()
        .x_range(x_min, x_max)
        .add_plot(Plot::linear(Number::Integer(k), Number::Integer(m)))
        .build_string()?;
    let graph_solution = Axes::new()
        .x_range(x_min, x_max)
        .add_plot(
            Plot::linear(Number::Integer(k), Number::Integer(m)).with_slope_hint(0, 1, ("x", "y")),
        )
        .build_string()?;

    let question = format!("Här är en rät linje: \n {graph}");
    let answer = format!("$y = {} {:+}$", Term::from((k, 'x')), Term::from(m));
    let solution = format!("{graph_solution}\n$k = (Delta y)/(Delta x) = {k}$");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    })
}

#[problem]
fn linear_graph_with_decimals(name: String, _lang: &Language) -> Result<Problem> {
    let k = Number::Decimal(500); // 0.5
    let m = Number::Decimal(1500); // 1.5
    let x_min = Number::Decimal(-500);
    let x_max = Number::Decimal(1200);
    let graph = Axes::new()
        .x_range(x_min, x_max)
        .add_plot(Plot::linear(k, m))
        .build_string()?;

    let question = format!("Här är en rät linje med decimaltal: \n {graph}");
    let answer = format!("$y = {k}x {m:+}$");
    let solution = String::from("look at the fucking line");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![1],
        combinations: 1,
    })
}

#[problem]
fn linear_graph_with_fractions(name: String, _lang: &Language) -> Result<Problem> {
    let k = Number::Fraction(1, 3);
    let m = Number::Integer(-1); // 1.5
    let x_min = Number::Integer(-1);
    let x_max = Number::Fraction(13, 3);
    let graph = Axes::new()
        .x_range(x_min, x_max)
        .add_plot(Plot::linear(k, m))
        .build_string()?;

    let question = format!("Här är en rät linje med bråk: \n {graph}");
    let answer = format!("$y = {k}x {m:+}$");
    let solution = String::from("look at the fucking line");

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![1],
        combinations: 1,
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
    let graph = Axes::new()
        .x_range(x_min as i32, x_max as i32)
        .add_plot(Plot::exponential(Number::Integer(c), Number::Integer(a))?)
        .build_string()?;

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
