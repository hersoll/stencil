use rand::{rng, seq::SliceRandom};
use std::fmt::Display;

use crate::{Number, Term};

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl From<Term> for Polynomial {
    fn from(value: Term) -> Self {
        Polynomial { terms: vec![value] }
    }
}

impl<T> From<Vec<T>> for Polynomial
where
    T: Into<Term>,
{
    fn from(value: Vec<T>) -> Self {
        Polynomial {
            terms: value.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl From<Vec<&Term>> for Polynomial {
    fn from(value: Vec<&Term>) -> Self {
        Polynomial {
            terms: value.iter().map(|&t| t.clone()).collect(),
        }
    }
}

impl Polynomial {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn push(&mut self, term: Term) {
        self.terms.push(term);
    }

    /// Returns a sorted Vec of every variable in the polynomial
    pub fn get_variables(&self) -> Vec<char> {
        let mut variables: Vec<char> = Vec::new();
        self.terms.iter().for_each(|term| {
            term.variables.list.iter().for_each(|var| {
                if !variables.contains(&var.symbol) {
                    variables.push(var.symbol);
                }
            })
        });
        variables.sort();
        variables
    }

    pub fn random_order(terms: Vec<&Term>) -> Self {
        let mut owned_terms: Vec<Term> = terms.iter().map(|&t| t.clone()).collect();
        owned_terms.shuffle(&mut rng());
        Self { terms: owned_terms }
    }

    pub fn simplify(&self) -> Self {
        let mut result = Polynomial::new();
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
            self.place_positive_first();
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
        match (
            self.terms[0].coefficient > 0.into(),
            self.terms[1].coefficient > 0.into(),
        ) {
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

    pub fn evaluate<T: Into<Number> + Clone>(&self, replacements: &Vec<(char, T)>) -> Number {
        let replacement_numbers: Vec<(char, Number)> = replacements
            .iter()
            .map(|(c, t)| (*c, t.clone().into()))
            .collect();
        let mut variables: Vec<char> = replacement_numbers.iter().map(|&(c, _)| c).collect();
        variables.sort();
        assert_eq!(
            variables,
            self.get_variables(),
            "Called evaluate() with a mismatch of variables:"
        );

        let mut result: Number = 0.into();
        self.terms.iter().for_each(|term| {
            result += term.evaluate(&replacement_numbers);
        });
        result
    }
}

impl std::ops::Add for Polynomial {
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

impl std::ops::Add<Term> for Polynomial {
    type Output = Self;
    fn add(self, rhs: Term) -> Self::Output {
        let mut result = self.terms.clone();
        match self.terms.iter().position(|t| t.variables == rhs.variables) {
            Some(index) => result[index] = result[index].clone() + rhs,
            None => result.push(rhs),
        }
        Self { terms: result }
    }
}

impl std::ops::Add<i32> for Polynomial {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        let term_rhs: Term = rhs.into();
        self + term_rhs
    }
}

impl std::ops::AddAssign<Term> for Polynomial {
    fn add_assign(&mut self, rhs: Term) {
        *self = self.clone() + rhs;
    }
}

impl std::ops::AddAssign<i32> for Polynomial {
    fn add_assign(&mut self, rhs: i32) {
        let term_rhs: Term = rhs.into();
        *self = self.clone() + term_rhs;
    }
}
impl std::ops::Sub for Polynomial {
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
impl std::ops::Mul<Polynomial> for Polynomial {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Polynomial::new();
        for lhs_term in self.terms {
            for rhs_term in rhs.terms.clone() {
                result.terms.push(lhs_term.clone() * rhs_term);
            }
        }
        result
    }
}
impl std::ops::Mul<Polynomial> for Term {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        let lhs: Polynomial = self.into();
        lhs * rhs
    }
}
impl std::ops::Mul<Polynomial> for i32 {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        let mut result = rhs.clone();
        for term in &mut result.terms {
            *term *= self;
        }
        result
    }
}
impl std::ops::MulAssign<i32> for Polynomial {
    fn mul_assign(&mut self, rhs: i32) {
        for term in &mut self.terms {
            *term *= rhs;
        }
    }
}

impl std::ops::Neg for &Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Self::Output {
        let mut new_exp = Polynomial::new();
        for term in &self.terms {
            new_exp.push(-term);
        }
        new_exp
    }
}

impl std::ops::Neg for Polynomial {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut new_exp = Polynomial::new();
        for term in self.terms {
            new_exp.push(-term);
        }
        new_exp
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first_value_written = false;
        if self.terms.len() == 1 && self.terms[0].coefficient == 0.into() {
            write!(f, "0")?;
        } else {
            for term in &self.terms {
                if !f.sign_plus() && !first_value_written {
                    if term.coefficient != 0.into() {
                        first_value_written = true;
                        write!(f, "{term}")?;
                    }
                } else {
                    write!(f, "{term:+}")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variables;

    #[test]
    fn expression_creation() {
        let t1: Term = (3, 'x').into();
        let t2: Term = (2, ('x', 2)).into();
        let t3: Term = (-3, 'x').into();
        let t4: Term = 'a'.into();
        let ref_expression: Polynomial = vec![&t1, &t2, &t3, &t4].into();
        let expression: Polynomial = vec![t2, t1, t4, t3].into();
        assert_eq!(ref_expression.to_string(), "3x+2x^2-3x+a");
        assert_eq!(expression.to_string(), "2x^2+3x+a-3x");
    }

    #[test]
    fn expression_display() {
        let t1: Term = (3, 'x').into();
        let t2: Term = (2, ('x', 2)).into();
        let t3: Term = (-3, 'x').into();
        let t4: Term = 'a'.into();
        let expression: Polynomial = vec![t1, t2, t3, t4].into();
        assert_eq!(format!("{expression}"), "3x+2x^2-3x+a");
        assert_eq!(format!("{expression:+}"), "+3x+2x^2-3x+a");
    }

    #[test]
    fn two_term_sorting() {
        let t_x = Term::from((-3, 'x'));
        let t_y = Term::from((2, 'y'));
        let t_const = Term::from(4);
        let x_y_expression: Polynomial = vec![&t_x, &t_y].into();
        assert_eq!(x_y_expression.to_string(), "-3x+2y");
        assert_eq!(x_y_expression.sorted().to_string(), "2y-3x");
        let x_const_expression: Polynomial = vec![&t_x, &t_const].into();
        assert_eq!(x_const_expression.sorted().to_string(), "4-3x");
        let y_const_expression: Polynomial = vec![&t_y, &t_const].into();
        assert_eq!(y_const_expression.sorted().to_string(), "2y+4");
        let t_const = Term::from(-4);
        let x_const_expression: Polynomial = vec![&t_x, &t_const].into();
        assert_eq!(x_const_expression.sorted().to_string(), "-3x-4");
    }

    #[test]
    fn multiple_term_sorting() {
        let t_x = Term::from((-3, 'x'));
        let t_x_2 = Term::from((2, ('x', 2)));
        let t_const = Term::from(4);
        let expression: Polynomial = vec![&t_x, &t_x_2, &t_const].into();
        assert_eq!(expression.sorted().to_string(), "2x^2-3x+4");
        let t_y_3: Term = ('y', 3).into();
        let expression: Polynomial = vec![&t_x, &t_x_2, &t_const, &t_y_3].into();
        assert_eq!(expression.sorted().to_string(), "y^3+2x^2-3x+4");
    }

    #[test]
    fn expression_evaluation() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let t3: Term = (4, 'a').into();
        let exp: Polynomial = vec![&t1, &t2, &t3].into();
        // NOTE: Currently evaluate() is only available for full number evaluation, not algebraic
        // let partial_evaluation = exp.evaluate(&vec![('x', -1)]);
        // assert_eq!(partial_evaluation.to_string(), "4a-5");
        let full_evaluation = exp.evaluate(&vec![('a', -2), ('x', 5)]);
        assert_eq!(full_evaluation.to_string(), "-73");
    }

    #[test]
    fn expression_addition() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let t3: Term = (4, 'a').into();
        let exp: Polynomial = vec![t1, t2, t3].into();
        let t1: Term = ('x', 2).into();
        let t2: Term = (3, 'x').into();
        let t3: Term = ('a', 2).into();
        let exp_2: Polynomial = vec![t1, t2, t3].into();
        assert_eq!((exp + exp_2).to_string(), "5x-2x^2+4a+a^2");
    }

    #[test]
    fn expression_subtraction() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let t3: Term = (4, 'a').into();
        let exp: Polynomial = vec![t1, t2, t3].into();
        let t1: Term = ('x', 2).into();
        let t2: Term = (3, 'x').into();
        let t3: Term = ('a', 2).into();
        let exp_2: Polynomial = vec![t1, t2, t3].into();
        assert_eq!((exp - exp_2).to_string(), "-x-4x^2+4a-a^2");
    }

    #[test]
    fn expression_multiplication() {
        let t1: Term = 'x'.into();
        let t2: Term = (2, ('y', 2)).into();
        let t3: Term = (-3).into();
        let exp_1: Polynomial = vec![&t1, &t2].into();
        let exp_2: Polynomial = vec![t1, t2, t3].into();
        assert_eq!((3 * exp_2.clone()).to_string(), "3x+6y^2-9");
        let mult_exp = exp_1 * exp_2;
        assert_eq!(mult_exp.to_string(), "x^2+2x y^2-3x+2x y^2+4y^4-6y^2");
        assert_eq!(mult_exp.simplify().to_string(), "4y^4+4x y^2+x^2-6y^2-3x");
    }
}
