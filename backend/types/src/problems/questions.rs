mod subquestions;
pub use subquestions::*;

use math::{MathDisplay, Number, Polynomial, Term};
use std::fmt::Display;

use crate::format_strings::Subdivision;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Question(pub(crate) String);

impl Question {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Ergonomic constructor to make the pathing look nicer.
    ///
    /// Question::subquestions() looks better when starting a new struct than SubQuestions::default()
    pub fn subquestions() -> SubQuestions {
        SubQuestions::default()
    }
}

impl From<&str> for Question {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Question {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Subdivision> for Question {
    fn from(s: Subdivision) -> Self {
        Self(s.to_string())
    }
}

impl From<Number> for Question {
    fn from(num: Number) -> Self {
        Self(num.as_math())
    }
}

impl From<Polynomial> for Question {
    fn from(pol: Polynomial) -> Self {
        Self(pol.as_math())
    }
}

impl From<Term> for Question {
    fn from(term: Term) -> Self {
        Self(term.as_math())
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
