use std::fmt::Display;

use num_traits::{Signed, Zero};

pub fn to_list_item(s: &String) -> String {
    String::from("block(breakable: false)[") + s + "],"
}

pub fn to_heading<T: Into<String>>(heading: T) -> String {
    let heading_str: String = heading.into();
    format!("#align(center, text(1.5em)[*{}*])", heading_str)
}

pub fn line_break() -> String {
    String::from("\\")
}

pub fn empty_line() -> String {
    String::from("\n\\\n")
}

pub fn page_break() -> String {
    String::from("\n#pagebreak()\n")
}

pub fn reset_enum() -> String {
    String::from("#item-counter.update(0)\n")
}
pub fn reformat_newlines(input: &str) -> String {
    input.replace('\n', r" \ ")
}



pub fn add<T: PartialOrd + Zero + Signed + Display>(first: T, second: T) -> String {
    format!("{} + {}", parentheses(first), parentheses(second))
}

pub fn subtract<T: PartialOrd + Zero + Signed + Display>(first: T, second: T) -> String {
    format!("{} - {}", parentheses(first), parentheses(second))
}

pub fn multiply<T: PartialOrd + Zero + Signed + Display>(first: T, second: T) -> String {
    format!("{} dot.op {}", parentheses(first), parentheses(second))
}

pub fn divide<T: PartialOrd + Zero + Signed + Display>(first: T, second: T) -> String {
    format!("({})/({})", parentheses(first), parentheses(second))
}

static OPERATOR_SPACE: f32 = 0.25;

pub fn step_subtract<T: PartialOrd + Zero + Signed + Display>(val: T) -> String {
    if val < T::zero() {
        format!("+ #h({OPERATOR_SPACE}em) {}", val.abs())
    } else if val > T::zero() {
        format!("- #h({OPERATOR_SPACE}em) {}", val)
    } else {
        String::new()
    }
}

pub fn step_add<T: PartialOrd + Zero + Signed + Display>(val: T) -> String {
    if val > T::zero() {
        format!("+ #h({OPERATOR_SPACE}em) {}", val)
    } else if val < T::zero() {
        format!("- #h({OPERATOR_SPACE}em) {}", val.abs())
    } else {
        String::new()
    }
}

pub fn step_divide<T: PartialOrd + Zero + Display>(val: T) -> String {
    format!("div {}", parentheses(val))
}

pub fn step_multiply<T: PartialOrd + Zero + Display>(val: T) -> String {
    format!("dot.op {}", parentheses(val))
}

pub fn parentheses<T: PartialOrd + Zero + Display>(val: T) -> String {
    if val < T::zero() {
        format!("({val})")
    } else {
        format!("{val}")
    }
}

/// Formats the solution of an equation, step by step.
///
/// Do not include dollar signs ($) for math formatting. The function makes sure the left column is
/// a block type ($ 2x + 1 $) and the right column is inline ($+2x$).
/// Remember to include empty steps if nothing happens in a step
pub fn equation_solution(equation_string: String) -> String {
    let mut equations: Vec<&str> = Vec::new();
    let mut steps: Vec<&str> = Vec::new();

    for (i, val) in equation_string.split(r#"\"#).enumerate() {
        if i % 2 == 0 {
            equations.push(val);
        } else {
            steps.push(val);
        }
    }

    let combined_equations = equations
        .into_iter()
        .map(|e| format!("$ {} $", e.trim_matches('$').trim()))
        .collect::<Vec<String>>()
        .join(", ");
    let combined_steps = steps
        .into_iter()
        .map(|s| format!("${}$", s.trim_matches('$').trim()))
        .collect::<Vec<String>>()
        .join(", ");

    format!("#equation-solution(({combined_equations}),({combined_steps}),)")
}
