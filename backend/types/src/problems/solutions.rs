mod continuous;
mod with_steps;
use continuous::*;
use with_steps::*;

use math::{MathDisplay, Number};
use std::fmt::Display;

use crate::format_strings::{SolutionString, Subdivision};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Solution(String);

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

impl Solution {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Solution {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Subdivision> for Solution {
    fn from(s: Subdivision) -> Self {
        Self(s.to_string())
    }
}

impl From<Number> for Solution {
    fn from(num: Number) -> Self {
        Self(num.as_math())
    }
}

impl From<ContinuousSolution> for Solution {
    fn from(solution: ContinuousSolution) -> Self {
        Self(solution.to_string())
    }
}

impl From<SolutionWithSteps> for Solution {
    fn from(solution: SolutionWithSteps) -> Self {
        Self(solution.to_string())
    }
}

impl From<SolutionString> for Solution {
    fn from(s: SolutionString) -> Self {
        Self(s.to_string())
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
