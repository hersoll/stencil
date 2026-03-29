use anyhow::Result;
use macros::problem;
use math::IntRange;
use types::{lang::Language, problems::Problem};
use typst_writer::custom_math::solutions;

fn quadratics_template(symmetry: i32, distance: i32) -> (String, String, String) {
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

#[problem]
fn small_numbers_positive_p(name: String, _lang: &Language) -> Result<Problem> {
    let (symmetry, sym_range) = IntRange::without_zero(-3, -1)?.and_random();
    let distance = IntRange::without_zero(-symmetry + 1, 5)?.random();
    let (question, answer, solution) = quadratics_template(symmetry, distance);

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![symmetry, distance],
        combinations: sym_range.len() * 2,
    })
}

#[problem]
fn positive_p(name: String, _lang: &Language) -> Result<Problem> {
    let (symmetry, sym_range) = IntRange::without_zero(-8, -1)?.and_random();
    let distance = IntRange::without_zero(1, 10)?.random();
    let (question, answer, solution) = quadratics_template(symmetry, distance);

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![symmetry, distance],
        combinations: sym_range.len() * 10,
    })
}
