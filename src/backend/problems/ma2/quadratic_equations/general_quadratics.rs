use macros::problem;
use crate::backend::{typst_formatting, IntRange, Problem};
use crate::Result;

#[problem]
fn small_numbers_positive_p(id: String, _lang: &str) -> Result<Problem> {
    let (symmetry, sym_range) = IntRange::without_zero(-3, -1)?.and_random();
    let (distance, dist_range) = IntRange::without_zero(-symmetry + 1, 5)?.and_random();
    let x_1 = symmetry + distance;
    let x_2 = symmetry - distance;
    let p = -2*symmetry;
    let q = x_1 * x_2;
    let question = format!("$x^2{p:+}x{q:+}=0$");
    let answer = format!("$x_1={x_1}, #h(0.5em) x_2={x_2}$");
    let solution = format!(
    "$ &x^2{p:+}x{q:+}=0 \\ 
    &x=(-{p_par:+})/2 plus.minus sqrt(({p}/2)^2 - {q_par:+}) \\
    &x={p_m:+}/2 plus.minus sqrt(({p}/2)^2 {q_m:+}) \\
        &x = {symmetry} plus.minus sqrt({p_sq} {q_m:+}) \\
        &x = {symmetry} plus.minus sqrt({total_sq}) \\
        &x = {symmetry} plus.minus {distance} \\
        &x_1 = {x_1}, #h(0.5em) x_2 = {x_2} $",
        p_par = typst_formatting::parentheses(p),
        q_par = typst_formatting::parentheses(q),
        p_m = -p,
        q_m = -q,
        p_sq = (p/2).pow(2),
        total_sq = (p/2).pow(2) - q,
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
