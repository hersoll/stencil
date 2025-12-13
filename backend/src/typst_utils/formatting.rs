use anyhow::{Context, Result};
use num_traits::{Signed, Zero};
use std::fmt::Write;
use std::{collections::HashMap, fmt::Display};
use tracing::debug;

use crate::Term;
use crate::typst_utils::typst_file_builder::{DEFAULT_QUESTION_COLUMNS, SetOptions};

static OPERATOR_SPACE: f32 = 0.25;

/// Used for setting colors in the typst file
#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%, {}%, {}%", self.r, self.g, self.b)
    }
}

pub fn colors(has_color: bool) -> String {
    let colored: Color;
    // Graphing colors
    let primary: Color;
    let secondary: Color;
    let tertiary: Color;
    if has_color {
        colored = Color::new(22, 10, 33); // Purple
        primary = Color::new(9, 3, 18); // Dark purple
        secondary = colored.clone();
        tertiary = Color::new(30, 23, 39); // Light purple
    } else {
        colored = Color::new(10, 10, 10); // Gray
        primary = Color::new(0, 0, 0); // Black
        secondary = Color::new(8, 8, 8); // Gray?
        tertiary = Color::new(16, 16, 16); // Grayer?
    };

    format!(
        "
#let colored(x) = text(fill: color.linear-rgb({colored}), $#x$)
#let primary(x) = text(fill: color.linear-rgb({primary}), $#x$)
#let secondary(x) = text(fill: color.linear-rgb({secondary}), $#x$)
#let tertiary(x) = text(fill: color.linear-rgb({tertiary}), $#x$)"
    )
}

pub fn list_item(s: &String) -> String {
    String::from("block(breakable: false)[") + s + "],"
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
pub fn build_solution(
    answer: String,
    solution: String,
    i18n: &HashMap<String, String>,
) -> Result<String> {
    let solution_label = i18n
        .get("solution")
        .context("Unable to get key \"solution\" from i18n")?;
    let mut out = String::with_capacity(1024);
    writeln!(out, "{answer}")?;
    // NOTE: This is the value to adjust to fix nested enum inset
    writeln!(out, "#block(inset: (left: -1.2em))[")?;
    writeln!(out, "#set text(size: 0.8em)")?;
    writeln!(out, "#emph([{solution_label}])\n")?;
    writeln!(out, "{solution}]")?;
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
