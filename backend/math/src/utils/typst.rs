use crate::Number;

/// Adds parentheses to negative numbers
///
/// When printing for Typst, we want parentheses around negative numbers when they are a part of
/// longer expressions.
pub fn parenthesize(number: &Number) -> String {
    if *number < 0 {
        format!("({number})")
    } else {
        format!("{number}")
    }
}
