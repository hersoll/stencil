use crate::typst_file_builder::{DEFAULT_QUESTION_COLUMNS, SetOptions};
use anyhow::{Context, Result};
use math::Term;
use num_traits::{Signed, Zero};
use std::fmt::Write;
use std::{collections::HashMap, fmt::Display};
use tracing::debug;

pub use super::solution_with_steps::*;

/// The space between the operator and number in the solution step-by-step
static OPERATOR_SPACE: f32 = 0.25;
/// Background color for the solutions
static SOLUTION_COLOR: &'static str = "oklch(95.25%, 0.0285, 73deg, 50%)";
static SOLUTION_HEADING_SPACE: &'static str = "0.3em";
static SOLUTION_RADIUS: &'static str = "0.5em";
static SOLUTION_FONT_SIZE: &'static str = "0.8em";
static SOLUTION_INSET: &'static str = "1.2em";
static SOLUTION_NESTED_INSET: &'static str = "2.5em";
static SOLUTION_OUTSET: &'static str = "1.7em";
static SOLUTION_NESTED_OUTSET: &'static str = "3em";
static SOLUTION_BACKGROUND_PADDING: &'static str = "0.5em";

// item = unbreakable, block = breakable
pub fn list_item(s: &String) -> String {
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

pub fn heading(heading: &str) -> String {
    if heading.is_empty() {
        String::new()
    } else {
        format!("#align(center, text(1.5em)[*{}*])", heading)
    }
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

// TODO: Maybe move the math functions to another module?
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

//NOTE: Idea for new way to write solutions:
// Struct which takes a list of Rows
// Each Row has a line (String) and a step (Option<String>)
// solution.add_line(format!("{x} + {y} &= {}", x + y))
// solution.add_line(s).next_step(step) (or .add_line_with_step(s, step))

/// Formats the solution of an equation, step by step.
///
/// Do not include dollar signs ($) manually before calling this!
/// The function makes sure the left column is
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

    format!("#v(-0.5em)\n#equation-solution(({combined_equations}),({combined_steps}),)")
}

/// Formats the answer and solution strings to show up as a proper solution in the Typst file
pub fn build_solution(answer: String, solution: String) -> Result<String> {
    let mut out = String::with_capacity(1024);
    writeln!(out, "{answer}")?;
    writeln!(out, "#solution[")?;
    writeln!(out, "{solution}")?;
    writeln!(out, "]")?;
    Ok(out)
}

pub fn solution_rules(i18n: &HashMap<String, String>) -> Result<String> {
    let solution_label = i18n
        .get("solution")
        .context("Unable to get key \"solution\" from i18n")?;
    // String is about 500 bytes
    let mut out = String::with_capacity(512);
    // Two space indentation for Typst legibility
    writeln!(out, "#let solution(content) = block(")?;
    writeln!(out, "  inset: (left: -{SOLUTION_INSET}), ")?;
    writeln!(
        out,
        "  outset: (left: {SOLUTION_OUTSET}, rest: {SOLUTION_BACKGROUND_PADDING}), "
    )?;
    writeln!(out, "  fill: {SOLUTION_COLOR}, ")?;
    writeln!(out, "  radius: {SOLUTION_RADIUS}")?;
    writeln!(out, ")[")?;
    writeln!(out, "  #set text(size: {SOLUTION_FONT_SIZE})")?;
    writeln!(out, "  #align(center)[#emph([{solution_label}])]")?;
    writeln!(out, "  #v({SOLUTION_HEADING_SPACE})")?;
    writeln!(out, "  #content\n]")?;

    writeln!(out, "#let nested_solution(content) = block(")?;
    writeln!(out, "  inset: (left: -{SOLUTION_NESTED_INSET}), ")?;
    writeln!(
        out,
        "  outset: (left: {SOLUTION_NESTED_OUTSET}, rest: {SOLUTION_BACKGROUND_PADDING}), "
    )?;
    writeln!(out, "  fill: {SOLUTION_COLOR}, ")?;
    writeln!(out, "  radius: {SOLUTION_RADIUS}")?;
    writeln!(out, ")[")?;
    writeln!(out, "  #set text(size: {SOLUTION_FONT_SIZE})")?;
    writeln!(out, "  #align(center)[#emph([{solution_label}])]")?;
    writeln!(out, "  #v({SOLUTION_HEADING_SPACE})")?;
    writeln!(out, "  #content\n]")?;
    Ok(out)
}
/// Formats the sets to columns with equal height
pub fn sets_to_balanced_columns(
    sets: &Vec<Vec<String>>,
    group_prefixes: &[Option<String>],
    set_options: &[SetOptions],
    par_spacing: &Option<u8>,
) -> Result<String> {
    let mut out = String::with_capacity(8 * 1024);

    for (i, set) in sets.iter().enumerate() {
        // Write group prefix (if any)
        if let Some(prefix) = group_prefixes.get(i).and_then(|p| p.clone()) {
            writeln!(out, "{prefix}")?;
        }

        writeln!(out, "\n#let problem_set = (")?;
        // Write each list item
        for item in set.iter() {
            writeln!(out, "{}", list_item(item))?;
        }
        writeln!(out, ")")?;

        let spacing_setting = if let Some(spacing) = set_options.get(i).and_then(|o| o.spacing) {
            format!(", custom_spacing: {spacing}mm")
        } else {
            String::new()
        };

        let heading_setting = if let Some(option) = set_options.get(i) {
            if option.heading.is_empty() {
                String::new()
            } else {
                format!(", title: [{}]", reformat_newlines(&option.heading))
            }
        } else {
            String::new()
        };

        // Call the balanced function in Typst
        writeln!(
            out,
            "#context{{balanced({}, problem_set, here().position().y{}{})}}",
            set_options
                .get(i)
                .map(|o| o.question_columns)
                .unwrap_or(DEFAULT_QUESTION_COLUMNS),
            spacing_setting,
            heading_setting
        )?;

        // Paragraph spacing between sets (except after last)
        if i != sets.len().saturating_sub(1) {
            if let Some(spacing) = par_spacing {
                writeln!(out, "#v({}mm)", spacing)?;
            } else {
                writeln!(out, "#v(1.8em)")?;
            }
        }
    }

    debug!(
        "Allocated 8kb for balanced column set. Final length: {}",
        out.len()
    );
    Ok(out)
}

///Writes the set to a flow from one filled column to the next
pub fn sets_to_columns(sets: &[Vec<String>], columns: &u8) -> Result<String> {
    let mut out = String::with_capacity(sets[0].len() * sets.len() * 128);

    writeln!(out, "#columns({columns}, enum(spacing: 2.5em, ")?;
    for set in sets.iter() {
        for entry in set.iter() {
            writeln!(out, "{}", list_item(entry))?;
        }
    }
    writeln!(out, "))")?;
    Ok(out)
}
