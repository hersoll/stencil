use anyhow::Result;
use std::fmt::Write;
/// Background color for the solutions
static SOLUTION_COLOR: &str = "oklch(95.25%, 0.0285, 73deg, 50%)";
static SOLUTION_HEADING_SPACE: &str = "0.3em";
static SOLUTION_RADIUS: &str = "0.5em";
static SOLUTION_FONT_SIZE: &str = "0.8em";
static SOLUTION_INSET: &str = "1.2em";
static SOLUTION_NESTED_INSET: &str = "2.5em";
static SOLUTION_OUTSET: &str = "1.7em";
static SOLUTION_NESTED_OUTSET: &str = "3em";
static SOLUTION_BACKGROUND_PADDING: &str = "0.5em";

/// Formats the answer and solution strings to show up as a proper solution in the Typst file
pub fn build_solution(answer: &str, solution: &str) -> Result<String> {
    let mut out = String::with_capacity(1024);
    writeln!(out, "{answer}")?;
    writeln!(out, "#solution[")?;
    writeln!(out, "{solution}")?;
    writeln!(out, "]")?;
    Ok(out)
}

pub fn solution_rules(solution_label: &str) -> Result<String> {
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
