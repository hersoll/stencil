use math::Number;

use crate::formatting;

pub fn pq_long(p: impl Into<Number>, q: impl Into<Number>, unknown: char) -> String {
    let p = p.into();
    let q = q.into();
    let symmetry = -p / 2;
    let distance = (p / 2)
        .pow(Number::Integer(2) - q)
        .pow(Number::Fraction(1, 2));
    let x_1 = symmetry - distance;
    let x_2 = symmetry + distance;
    format!(
        "&{unknown}^2{p:+}{unknown}{q:+}=0 \\ 
    &{unknown}=-{p}/2 plus.minus sqrt(({p}/2)^2 - {q_par:+}) \\
    &{unknown}={symmetry} plus.minus sqrt({p_d}^2 {q_m:+}) \\
        &{unknown} = {symmetry} plus.minus sqrt({p_sq} {q_m:+}) \\
        &{unknown} = {symmetry} plus.minus sqrt({total_sq}) \\
        &{unknown} = {symmetry} plus.minus {distance} \\
        &{unknown}_1 = {x_1}, #h(0.4em) {unknown}_2 = {x_2}",
        p_d = formatting::parentheses(p / 2),
        q_par = formatting::parentheses(q),
        q_m = -q,
        p_sq = (p / 2).pow(Number::Integer(2)),
        total_sq = (p / 2).pow(Number::Integer(2)) - q,
    )
}

pub fn pq_short(p: impl Into<Number>, q: impl Into<Number>, unknown: char) -> String {
    let p = p.into();
    let q = q.into();
    let symmetry = -p / 2;
    let distance = (p / 2)
        .pow(Number::Integer(2) - q)
        .pow(Number::Fraction(1, 2));
    let x_1 = symmetry - distance;
    let x_2 = symmetry + distance;
    format!(
        "&{unknown}^2{p:+}{unknown}{q:+}=0 \\ 
        &{unknown}=-{p}/2 plus.minus sqrt(({p}/2)^2 {q_m:+}) \\
        &{unknown} = {symmetry} plus.minus sqrt({total_sq}) \\
        &{unknown} = {symmetry} plus.minus {distance} \\
        &{unknown}_1 = {x_1}, #h(0.4em) {unknown}_2 = {x_2}",
        q_m = -q,
        total_sq = (p / 2).pow(Number::Integer(2)) - q,
    )
}
