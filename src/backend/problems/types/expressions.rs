use rand::{rng, seq::SliceRandom};
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
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn random_order(terms: Vec<&Term>) -> Self {
        let mut owned_terms: Vec<Term> = terms.iter().map(|&t| t.clone()).collect();
        owned_terms.shuffle(&mut rng());
        Self { terms: owned_terms }
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
        result.sort();
        result
    }

    /// Sorts the expression in-place
    pub fn sort(&mut self) {
        if self.terms.len() < 2 {
        } else if self.terms.len() == 2 {
            // If we have 3y - 2x or -2x^2 + z we want them in variable order
            if self.terms[0].variables.list.len() > 0 && self.terms[1].variables.list.len() > 0 {
                self.sort_by_variables();
            } else {
                // If we have 3x^2 - 5 or 6 - y we want the positive first
                self.place_positive_first();
            }
        } else {
            self.sort_by_variables();
        }
    }

    /// Returns a sorted clone of the expression
    pub fn sorted(&self) -> Self {
        let mut cloned = self.clone();
        cloned.sort();
        cloned
    }

    fn place_positive_first(&mut self) {
        assert_eq!(self.terms.len(), 2);
        match (self.terms[0].coefficient > 0, self.terms[1].coefficient > 1) {
            (true, false) => {}
            (false, true) => {
                let temp = self.terms[0].clone();
                self.terms[0] = self.terms[1].clone();
                self.terms[1] = temp.clone();
            }
            (_, _) => self.sort_by_variables(),
        }
    }

    fn sort_by_variables(&mut self) {
        self.terms.sort_by(|a, b| b.variables.cmp(&a.variables));
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
        result
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
            if !f.sign_plus() && !first_value_written {
                if term.coefficient != 0 {
                    first_value_written = true;
                    write!(f, "{term}")?;
                }
            } else {
                write!(f, "{term:+}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_creation() {
        let t1: Term = (3, 'x').into();
        let t2: Term = (2, ('x', 2)).into();
        let t3: Term = (-3, 'x').into();
        let t4: Term = 'a'.into();
        let ref_expression: Expression = vec![&t1, &t2, &t3, &t4].into();
        let expression: Expression = vec![t2, t1, t4, t3].into();
        assert_eq!(ref_expression.to_string(), "3x+2x^2-3x+a");
        assert_eq!(expression.to_string(), "2x^2+3x+a-3x");
    }

    #[test]
    fn expression_display() {
        let t1: Term = (3, 'x').into();
        let t2: Term = (2, ('x', 2)).into();
        let t3: Term = (-3, 'x').into();
        let t4: Term = 'a'.into();
        let expression: Expression = vec![t1, t2, t3, t4].into();
        assert_eq!(format!("{expression}"), "3x+2x^2-3x+a");
        assert_eq!(format!("{expression:+}"), "+3x+2x^2-3x+a");
    }

    #[test]
    fn expression_addition() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let t3: Term = (4, 'a').into();
        let exp: Expression = vec![t1, t2, t3].into();
        let t1: Term = ('x', 2).into();
        let t2: Term = (3, 'x').into();
        let t3: Term = ('a', 2).into();
        let exp_2: Expression = vec![t1, t2, t3].into();
        assert_eq!((exp + exp_2).to_string(), "5x-2x^2+4a+a^2");
    }

    #[test]
    fn expression_subtraction() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let t3: Term = (4, 'a').into();
        let exp: Expression = vec![t1, t2, t3].into();
        let t1: Term = ('x', 2).into();
        let t2: Term = (3, 'x').into();
        let t3: Term = ('a', 2).into();
        let exp_2: Expression = vec![t1, t2, t3].into();
        assert_eq!((exp - exp_2).to_string(), "-x-4x^2+4a-a^2");
    }

    #[test]
    fn expression_multiplication() {
        let t1: Term = 'x'.into();
        let t2: Term = (2, ('y', 2)).into();
        let t3: Term = (-3).into();
        let exp_1: Expression = vec![&t1, &t2].into();
        let exp_2: Expression = vec![t1, t2, t3].into();
        assert_eq!((3 * exp_2.clone()).to_string(), "3x+6y^2-9");
        let mult_exp = exp_1 * exp_2;
        assert_eq!(mult_exp.to_string(), "x^2+2x y^2-3x+2x y^2+4y^4-6y^2");
        assert_eq!(mult_exp.simplify().to_string(), "4y^4+4x y^2+x^2-6y^2-3x");
    }
}
