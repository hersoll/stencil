use std::fmt::Display;

use crate::backend::problems::types::terms::Term;

#[derive(Debug, Clone)]
pub struct Expression {
    terms: Vec<Term>,
}

impl From<Vec<Term>> for Expression {
    fn from(value: Vec<Term>) -> Self {
        Expression { terms: value }
    }
}

impl From<Vec<&Term>> for Expression {
    fn from(value: Vec<&Term>) -> Self {
        Expression {
            terms: value.iter().map(|&t| t.clone()).collect(),
        }
    }
}

impl Expression {
    fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn simplify(&self) -> Self {
        let mut result = Expression::new();
        for term in &self.terms {
            match result
                .terms
                .iter()
                .position(|t| t.variables == term.variables)
            {
                Some(index) => result.terms[index] += term.clone(),
                None => result.terms.push(term.clone()),
            }
        }
        result
    }
}

impl std::ops::Add for Expression {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.terms.clone();
        for term in rhs.terms {
            match self
                .terms
                .iter()
                .position(|t| t.variables == term.variables)
            {
                Some(index) => result[index] = result[index].clone() + term,
                None => result.push(term),
            }
        }
        Self { terms: result }
    }
}
impl std::ops::Sub for Expression {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.terms.clone();
        for term in rhs.terms {
            match self
                .terms
                .iter()
                .position(|t| t.variables == term.variables)
            {
                Some(index) => result[index] -= term,
                None => result.push(-term),
            }
        }
        Self { terms: result }
    }
}
impl std::ops::Mul for Expression {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Expression::new();
        for lhs_term in self.terms {
            for rhs_term in rhs.terms.clone() {
                result.terms.push(lhs_term.clone() * rhs_term);
            }
        }
        result.simplify()
    }
}
impl std::ops::Mul<Expression> for i32 {
    type Output = Expression;
    fn mul(self, rhs: Expression) -> Self::Output {
        let mut result = rhs.clone();
        for term in &mut result.terms {
            *term *= self;
        }
        result
    }
}
impl std::ops::MulAssign<i32> for Expression {
    fn mul_assign(&mut self, rhs: i32) {
        for term in &mut self.terms {
            *term *= rhs;
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first_value_written = false;
        for term in &self.terms {
            if !first_value_written {
                if term.coefficient != 0 {
                    first_value_written = true;
                    write!(f, "{term} ")?;
                }
            } else {
                write!(f, "{term:+} ")?;
            }
        }
        Ok(())
    }
}
