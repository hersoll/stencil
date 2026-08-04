use std::fmt::Display;

use math::{MathDisplay, Number, Polynomial, Term};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Question(String);

impl Question {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Question {
    fn from(s: String) -> Self {
        Self(s)
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
