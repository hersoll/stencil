use crate::formatting;
use math::{Number, Polynomial};

pub fn show_polynomial_replacements(pol: &Polynomial, replacements: &Vec<(char, i32)>) -> String {
    use std::fmt::Write;

    let mut s = String::new();
    for (i, term) in pol.terms.iter().enumerate() {
        if i == 0 {
            write!(&mut s, "{}", term.coefficient).unwrap();
        } else {
            write!(&mut s, "{:+}", term.coefficient).unwrap();
        }
        for (j, var) in term.variables.list.iter().enumerate() {
            match replacements.iter().find(|pair| pair.0 == var.symbol) {
                Some(pair) => {
                    // Prevent double dot
                    s = s.trim_end_matches(" dot ").to_string();
                    write!(
                        &mut s,
                        " dot colored({}){}{}",
                        formatting::parentheses(pair.1),
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
    replacements: &Vec<(char, T)>,
) -> String {
    use std::fmt::Write;

    let replacement_numbers: Vec<(char, Number)> = replacements
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
