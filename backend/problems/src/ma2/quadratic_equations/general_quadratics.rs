use anyhow::Result;
use macros::problem;
use math::{Number, num_gen};
use types::{lang::Language, problems::Problem};
use typst_writer::custom_math::solutions;

fn quadratics_template(symmetry: Number, distance: Number) -> (String, String, String) {
    let x_1 = symmetry - distance;
    let x_2 = symmetry + distance;
    let p = -2 * symmetry;
    let q = x_1 * x_2;
    let question = format!("$x^2{p:+}x{q:+}=0$");
    let answer = format!("$x_1={x_1}, #h(0.4em) x_2={x_2}$");
    let solution = format!(
        "$ {sol} $",
        sol = solutions::quadratics::pq_short(p, q, 'x')
    );

    (question, answer, solution)
}

/// Absolute difficulty: 4
/// Relative difficulty: 1
#[problem]
fn small_numbers_positive_p(id: i32, _lang: Language) -> Result<Problem> {
    let (symmetry, sym_range) = num_gen::integer().range(-3, -1).and_random();
    let distance = num_gen::integer().range(-symmetry + 1, 5).random();
    let (question, answer, solution) = quadratics_template(symmetry, distance);

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![symmetry, distance],
        combinations: sym_range.len() * 2,
    })
}

/// Absolute difficulty: 4
/// Relative difficulty: 2
#[problem]
fn positive_p(id: i32, _lang: Language) -> Result<Problem> {
    let (symmetry, sym_range) = num_gen::integer().range(-8, -1).and_random();
    let distance = num_gen::integer().range(1, 10).random();
    let (question, answer, solution) = quadratics_template(symmetry, distance);

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![symmetry, distance],
        combinations: sym_range.len() * 10,
    })
}
