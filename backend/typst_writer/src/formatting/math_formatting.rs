use math::{Number, Term};
use num_traits::Zero;
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
    match term.cmp(&Term::zero()) {
        Greater => format!("+ #h({OPERATOR_SPACE}em) {}", term),
        Less => format!("- #h({OPERATOR_SPACE}em) {}", term.abs()),
        Equal => String::new(),
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

pub fn subtract_term(term: &Term) -> String {
    use std::cmp::Ordering::*;
    match term.cmp(&Term::zero()) {
        Greater => format!("- #h({OPERATOR_SPACE}em) {}", term),
        Less => format!("+ #h({OPERATOR_SPACE}em) {}", term.abs()),
        Equal => String::new(),
    }
}

pub fn divide_number(val: impl Into<Number>) -> String {
    let val = val.into();
    format!("div {}", parentheses(&val))
}

pub fn multiply_number(val: impl Into<Number>) -> String {
    let val = val.into();
    format!("dot {}", parentheses(&val))
}

pub fn parentheses<T: PartialOrd + Zero + Display>(val: &T) -> String {
    if val < &T::zero() {
        format!("({val})")
    } else {
        format!("{val}")
    }
}
