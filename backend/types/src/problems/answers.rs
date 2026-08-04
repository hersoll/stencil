use math::{MathDisplay, Number, Polynomial, PolynomialVariable, Term};
use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Answer(String);

impl Answer {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Answer {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Number> for Answer {
    fn from(num: Number) -> Self {
        Self(num.as_math())
    }
}

impl From<PolynomialVariable> for Answer {
    fn from(pol: PolynomialVariable) -> Self {
        Self(pol.as_math())
    }
}

impl From<Term> for Answer {
    fn from(term: Term) -> Self {
        Self(term.as_math())
    }
}

impl From<Polynomial> for Answer {
    fn from(pol: Polynomial) -> Self {
        Self(pol.as_math())
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
