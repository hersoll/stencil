use math::Term;
use num_traits::{Signed, Zero};
use std::fmt::Display;

/// The space between the operator and number in the solution step-by-step
static OPERATOR_SPACE: f32 = 0.25;

pub fn subtract_number<T: PartialOrd + Zero + Signed + Display>(val: T) -> String {
    if val < T::zero() {
        format!("+ #h({OPERATOR_SPACE}em) {}", val.abs())
    } else if val > T::zero() {
        format!("- #h({OPERATOR_SPACE}em) {}", val)
    } else {
        String::new()
    }
}

pub fn subtract_term(term: &Term) -> String {
    if term < &Term::zero() {
        format!("+ #h({OPERATOR_SPACE}em) {}", term.abs())
    } else if term > &Term::zero() {
        format!("- #h({OPERATOR_SPACE}em) {}", term)
    } else {
        String::new()
    }
}

pub fn add_number<T: PartialOrd + Zero + Signed + Display>(val: T) -> String {
    if val > T::zero() {
        format!("+ #h({OPERATOR_SPACE}em) {}", val)
    } else if val < T::zero() {
        format!("- #h({OPERATOR_SPACE}em) {}", val.abs())
    } else {
        String::new()
    }
}

pub fn add_term(term: &Term) -> String {
    if term > &Term::zero() {
        format!("+ #h({OPERATOR_SPACE}em) {}", term)
    } else if term < &Term::zero() {
        format!("- #h({OPERATOR_SPACE}em) {}", term.abs())
    } else {
        String::new()
    }
}

pub fn divide_number<T: PartialOrd + Zero + Display>(val: T) -> String {
    format!("div {}", parentheses(val))
}

pub fn multiply_number<T: PartialOrd + Zero + Display>(val: T) -> String {
    format!("dot {}", parentheses(val))
}

pub fn parentheses<T: PartialOrd + Zero + Display>(val: T) -> String {
    if val < T::zero() {
        format!("({val})")
    } else {
        format!("{val}")
    }
}
