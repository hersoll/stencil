use crate::Result;
use crate::backend::{IntRange, Problem, solutions};
use macros::problem;

#[problem]
fn small_numbers_positive_p(id: String, _lang: &str) -> Result<Problem> {
    let (symmetry, sym_range) = IntRange::without_zero(-3, -1)?.and_random();
    let (distance, dist_range) = IntRange::without_zero(-symmetry + 1, 5)?.and_random();
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

    Ok(Problem {
        id,
        question,
        answer,
        solution,
        identifiers: vec![p, q],
        combinations: sym_range.len() * dist_range.len(),
    })
}
