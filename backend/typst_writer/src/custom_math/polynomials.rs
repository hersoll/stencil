use std::fmt::Display;

use crate::formatting;
use math::{Number, Polynomial};
use num_traits::Zero;

// TODO: Put these in an impl Polynomial {} or use a trait.
// Write doc comments.

pub fn show_polynomial_replacements<T: Display + Zero + PartialOrd + Clone>(
    pol: &Polynomial,
    replacements: &[(&str, T)],
) -> String {
    use std::fmt::Write;

    let mut s = String::new();
    for (i, term) in pol.terms.iter().enumerate() {
        if i == 0 {
            write!(&mut s, "{}", term.coefficient).unwrap();
        } else {
            write!(&mut s, "{:+}", term.coefficient).unwrap();
        }
        for (j, var) in term.variables.list.iter().enumerate() {
            match replacements.iter().find(|pair| pair.0 == var.symbol.0) {
                Some(pair) => {
                    // Prevent double dot
                    s = s.trim_end_matches(" dot ").to_string();
                    write!(
                        &mut s,
                        " dot colored({}){}{}",
                        formatting::parentheses(&pair.1),
                        if var.exponent > 1 {
                            format!("^{}", var.exponent)
                        } else {
                            String::new()
                        },
                        if j < term.variables.list.len() - 1 {
                            " dot "
                        } else {
                            ""
                        }
                    )
                    .unwrap()
                }
                None => write!(&mut s, "{var}").unwrap(),
            }
        }
    }
    s
}

pub fn show_polynomial_evaluation<T: Into<Number> + Clone>(
    pol: &Polynomial,
    replacements: &[(&str, T)],
) -> String {
    use std::fmt::Write;

    let replacement_numbers: Vec<(&str, Number)> = replacements
        .iter()
        .map(|(c, t)| (*c, t.clone().into()))
        .collect();
    let mut s = String::new();
    for (i, term) in pol.terms.iter().enumerate() {
        if i == 0 {
            write!(&mut s, "{}", term.evaluate(&replacement_numbers)).unwrap();
        } else {
            write!(&mut s, "{:+}", term.evaluate(&replacement_numbers)).unwrap();
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::custom_math::polynomials::show_polynomial_replacements;
    use math::{Term, Variables, symbols::Symbol};
    static A: &Symbol = &Symbol("a");
    static X: &Symbol = &Symbol("x");
    static Y: &Symbol = &Symbol("y");

    #[test]
    fn expression_evaluation_display() {
        let t1 = 2 * X;
        let t2 = -3 * (X * X);
        let vars = Variables::from(vec![(A, 3), (X, 3), (Y, 4)]);
        let t3 = 4 * Term::from_var(vars);
        let exp = t1.and(&t2).and(&t3);
        assert_eq!(
            show_polynomial_replacements(&exp, &[("x", -1)]),
            "2 dot colored((-1))-3 dot colored((-1))^2+4a^3 dot colored((-1))^3 dot y^4"
        );
        assert_eq!(
            show_polynomial_replacements(&exp, &[("x", 4)]),
            "2 dot colored(4)-3 dot colored(4)^2+4a^3 dot colored(4)^3 dot y^4"
        );
    }
}
