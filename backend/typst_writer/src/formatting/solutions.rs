mod continuous;
mod with_steps;
use continuous::*;
use with_steps::*;

use anyhow::Result;
use std::fmt::Write;
use types::pdf::SolutionDecoration;

static SOLUTION_HEADING_SPACE: &str = "0.3em";
static SOLUTION_RADIUS: &str = "0.5em";
static SOLUTION_FONT_SIZE: &str = "0.8em";
static SOLUTION_INSET: &str = "1.2em";
static SOLUTION_NESTED_INSET: &str = "2.5em";
static SOLUTION_OUTSET: &str = "1.7em";
static SOLUTION_NESTED_OUTSET: &str = "3em";
static SOLUTION_BACKGROUND_PADDING: &str = "0.5em";

/// A mathematical solution that is formatted to Typst
///
/// Helper struct that acts as a springboard for SolutionWithSteps and ContinousSolution
/// This is simply so the API becomes Solution::with_steps() instead of writing out
/// SolutionWithSteps::new()
pub struct Solution;

impl Solution {
    pub fn with_steps() -> SolutionWithSteps {
        SolutionWithSteps::default()
    }

    pub fn inline() -> ContinuousSolution {
        ContinuousSolution::inline()
    }

    pub fn block() -> ContinuousSolution {
        ContinuousSolution::block()
    }

    pub fn block_with_text() -> ContinuousSolution {
        ContinuousSolution::block_with_text()
    }
}

/// Formats the answer and solution strings to show up as a proper solution in the Typst file
pub fn build_solution(answer: &str, solution: &str) -> Result<String> {
    let mut out = String::with_capacity(1024);
    writeln!(out, "{answer}")?;
    writeln!(out, "#solution[")?;
    writeln!(out, "{solution}")?;
    writeln!(out, "]")?;
    Ok(out)
}

/// Writes out the preamble that formats solutions
///
/// `solution_label` is the language-specific string that denotes a solution
pub fn solution_preamble(
    solution_label: &str,
    solution_decoration: &SolutionDecoration,
) -> Result<String> {
    // String is about 500 bytes
    let mut out = String::with_capacity(512);
    // Two space indentation for Typst legibility
    writeln!(out, "#let solution(content) = block(")?;
    writeln!(out, "  inset: (left: -{SOLUTION_INSET}), ")?;
    writeln!(
        out,
        "  outset: (left: {SOLUTION_OUTSET}, rest: {SOLUTION_BACKGROUND_PADDING}), "
    )?;
    writeln!(out, "  {}", solution_decoration.to_typst())?;
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
    writeln!(out, "  fill: solution_color, ")?;
    writeln!(out, "  radius: {SOLUTION_RADIUS}")?;
    writeln!(out, ")[")?;
    writeln!(out, "  #set text(size: {SOLUTION_FONT_SIZE})")?;
    writeln!(out, "  #align(center)[#emph([{solution_label}])]")?;
    writeln!(out, "  #v({SOLUTION_HEADING_SPACE})")?;
    writeln!(out, "  #content\n]")?;
    Ok(out)
}
