pub use super::solution_with_steps::*;
mod columns;
pub mod evaluables;
mod math_formatting;
mod solutions;
pub use columns::*;
pub use math_formatting::*;
pub use solutions::*;

// item = unbreakable, block = breakable
pub fn list_item(s: &str) -> String {
    String::from("item[") + s + "],"
}

pub fn font_size(font_size: u8) -> String {
    format!("#set text(size: {}pt)", font_size)
}
pub fn page_size(paper_size: &str, x_margin: u8, y_margin: u8) -> String {
    format!(
        "#set page(paper: \"{}\", margin: (x: {}mm, y: {}mm))",
        paper_size, x_margin, y_margin
    )
}

/// Formats a &str into the "`<h1>`" of the page, so to speak
pub fn heading(heading: &str) -> String {
    format!("#align(center, text(1.5em)[*{}*])", heading)
}
/// Formats a &str into the "`<h2>`" of the page, so to speak.
///
/// Usually positioned below a [`heading()`].
pub fn subheading(subheading: &str) -> String {
    format!("#v(-0.8em)\n#align(center, text(1.1em)[{}])", subheading)
}

pub fn name_field(i18n_name: &str) -> String {
    format!(
        "#place(top + right, dx: 30pt, dy: -35pt)[{i18n_name}: #underline[#box(width: 35%, repeat(\" \"))]]"
    )
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

// Formats the solution of an equation, step by step.
//
// Do not include dollar signs ($) manually before calling this!
// The function makes sure the left column is
// a block type ($ 2x + 1 $) and the right column is inline ($+2x$).
// Remember to include empty steps if nothing happens in a step
pub fn equation_solution(equation_string: &str) -> String {
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

    format!("#v(-0.5em)\n#equation-solution(({combined_equations}),({combined_steps}),)")
}
