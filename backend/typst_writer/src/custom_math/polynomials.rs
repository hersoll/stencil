use crate::formatting;
use math::{Number, Polynomial};

pub fn show_polynomial_replacements(pol: &Polynomial, replacements: &[(char, i32)]) -> String {
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
    replacements: &[(char, T)],
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

#[cfg(test)]
mod tests {
    use crate::custom_math::polynomials::show_polynomial_replacements;
    use math::{Polynomial, Term, Variables};

    #[test]
    fn expression_evaluation_display() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let vars = Variables::from(vec![('a', 3), ('x', 3), ('y', 4)]);
        let t3: Term = (4, vars).into();
        let exp: Polynomial = vec![&t1, &t2, &t3].into();
        assert_eq!(
            show_polynomial_replacements(&exp, &vec![('x', -1)]),
            "2 dot colored((-1))-3 dot colored((-1))^2+4a^3 dot colored((-1))^3 dot y^4"
        );
        assert_eq!(
            show_polynomial_replacements(&exp, &vec![('x', 4)]),
            "2 dot colored(4)-3 dot colored(4)^2+4a^3 dot colored(4)^3 dot y^4"
        );
    }
}
