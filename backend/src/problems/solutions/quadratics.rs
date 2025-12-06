use crate::typst_formatting;

pub fn pq_long(p: i32, q: i32, unknown: char) -> String {
    let symmetry = -p / 2;
    let distance = ((p / 2).pow(2) - q).isqrt();
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
        p_d = typst_formatting::parentheses(p / 2),
        q_par = typst_formatting::parentheses(q),
        q_m = -q,
        p_sq = (p / 2).pow(2),
        total_sq = (p / 2).pow(2) - q,
    )
}

pub fn pq_short(p: i32, q: i32, unknown: char) -> String {
    let symmetry = -p / 2;
    let distance = ((p / 2).pow(2) - q).isqrt();
    let x_1 = symmetry - distance;
    let x_2 = symmetry + distance;
    format!(
        "&{unknown}^2{p:+}{unknown}{q:+}=0 \\ 
        &{unknown}=-{p}/2 plus.minus sqrt(({p}/2)^2 {q_m:+}) \\
        &{unknown} = {symmetry} plus.minus sqrt({total_sq}) \\
        &{unknown} = {symmetry} plus.minus {distance} \\
        &{unknown}_1 = {x_1}, #h(0.4em) {unknown}_2 = {x_2}",
        q_m = -q,
        total_sq = (p / 2).pow(2) - q,
    )
}
