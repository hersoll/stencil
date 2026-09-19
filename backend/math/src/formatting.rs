use crate::{
    HasCoef,
    Number::{self, Fraction},
    Term,
    utils::gcd,
};
use std::fmt::Display;

/// The space between the operator and number in the solution step-by-step
static OPERATOR_SPACE: f32 = 0.25;

pub fn add_number(val: impl Into<Number>) -> String {
    use std::cmp::Ordering::*;
    let val = val.into();
    match val.cmp(&Number::Integer(0)) {
        Greater => format!("+ #h({OPERATOR_SPACE}em) {}", val),
        Less => format!("- #h({OPERATOR_SPACE}em) {}", val.abs()),
        Equal => String::new(),
    }
}

pub fn add_term(term: &Term) -> String {
    use std::cmp::Ordering::*;
    match term.partial_cmp(&0) {
        Some(Greater) => format!("+ #h({OPERATOR_SPACE}em) {}", term),
        Some(Less) => format!("- #h({OPERATOR_SPACE}em) {}", term.abs()),
        Some(Equal) => String::new(),
        None => {
            tracing::error!("Unable to compare term {term} to 0");
            String::new()
        }
    }
}

pub fn subtract_number(val: impl Into<Number>) -> String {
    use std::cmp::Ordering::*;
    let val = val.into();
    match val.cmp(&Number::Integer(0)) {
        Greater => format!("- #h({OPERATOR_SPACE}em) {}", val),
        Less => format!("+ #h({OPERATOR_SPACE}em) {}", val.abs()),
        Equal => String::new(),
    }
}

pub fn subtract_term<T>(term: &T) -> String
where
    T: Into<Term> + Clone,
{
    let term = term.clone().into();
    use std::cmp::Ordering::*;
    match term.partial_cmp(&0) {
        Some(Greater) => format!("- #h({OPERATOR_SPACE}em) {}", term),
        Some(Less) => format!("+ #h({OPERATOR_SPACE}em) {}", term.abs()),
        Some(Equal) => String::new(),
        None => {
            tracing::error!("Unable to compare term {term} to 0");
            String::new()
        }
    }
}

pub fn divide(val: impl Display) -> String {
    format!("div {val}")
}

pub fn divide_number(val: impl Into<Number>) -> String {
    let val = val.into();
    format!("div {}", parentheses(&val))
}

pub fn multiply(val: impl Display) -> String {
    format!("dot {val}")
}

pub fn multiply_number(val: impl Into<Number>) -> String {
    let val = val.into();
    format!("dot {}", parentheses(&val))
}

pub fn parentheses<T: HasCoef + Display>(val: &T) -> String {
    if val.coef() < Number::Integer(0) {
        format!("({val})")
    } else {
        format!("{val}")
    }
}

/// Shows which number the numerator and denominator is divided by when simplified
pub fn show_simplification(fraction: Number) -> String {
    if let Fraction {
        numerator,
        denominator,
    } = fraction
    {
        // If this is actually an integer result, don't print the "divisions"
        if fraction.is_integer() {
            format!("{numerator} / {denominator}")
        } else {
            let gcd = gcd(numerator, denominator);
            format!("{numerator}_(colored(div {gcd})) / {denominator}_(colored(div {gcd}))")
        }
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_numbers() {
        assert_eq!(add_number(2), format!("+ #h({OPERATOR_SPACE}em) 2"));
        assert_eq!(add_number(-2), format!("- #h({OPERATOR_SPACE}em) 2"));
        assert_eq!(add_number(0), "");
        assert_eq!(
            add_number(2.01),
            format!("+ #h({OPERATOR_SPACE}em) num(\"2.01\")")
        );
        assert_eq!(
            add_number(-2.01),
            format!("- #h({OPERATOR_SPACE}em) num(\"2.01\")")
        );
    }

    #[test]
    fn subtracting_numbers() {
        assert_eq!(subtract_number(2), format!("- #h({OPERATOR_SPACE}em) 2"));
        assert_eq!(subtract_number(-2), format!("+ #h({OPERATOR_SPACE}em) 2"));
        assert_eq!(subtract_number(0), "");
        assert_eq!(
            subtract_number(2.01),
            format!("- #h({OPERATOR_SPACE}em) num(\"2.01\")")
        );
        assert_eq!(
            subtract_number(-2.01),
            format!("+ #h({OPERATOR_SPACE}em) num(\"2.01\")")
        );
    }

    #[test]
    fn adding_terms() {
        use crate::symbols::X;
        assert_eq!(add_term(&(2 * X)), format!("+ #h({OPERATOR_SPACE}em) 2x"));
        assert_eq!(add_term(&(-2 * X)), format!("- #h({OPERATOR_SPACE}em) 2x"));
        assert_eq!(add_term(&(0 * X)), "");
    }

    #[test]
    fn subtracting_terms() {
        use crate::symbols::X;
        assert_eq!(
            subtract_term(&(2 * X)),
            format!("- #h({OPERATOR_SPACE}em) 2x")
        );
        assert_eq!(
            subtract_term(&(-2 * X)),
            format!("+ #h({OPERATOR_SPACE}em) 2x")
        );
        assert_eq!(subtract_term(&(0 * X)), "");
    }
}
