mod operations;

use rand::{rng, seq::SliceRandom};
use std::fmt::Display;

use crate::{Number, Term, symbols::Symbol};

#[derive(Debug, Clone, Default)]
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
    pub fn from_terms(terms: &[&Term]) -> Self {
        let terms: Vec<Term> = terms.iter().map(|&t| t.clone()).collect();
        Self { terms }
    }

    /// Method used to chain multiple terms during fast Polynomial creation:
    /// ```rust
    /// use math::Term;
    /// use math::symbols::{X, Y};
    ///
    /// let t1 = Term::from_var(X);
    /// let t2 = Term::from_var(Y);
    /// let t3 = 2 * X;
    /// let p1 = t1.and(&t2).and(&t3);
    /// assert_eq!(p1.to_string(), String::from("x+y+2x"));
    /// ```
    ///
    /// The first `.and()` call comes from the Term struct, but that call returns a Polynomial and
    /// therefore this method is needed to do the second `.and()` call.
    pub fn and(mut self, term: &Term) -> Polynomial {
        self.push(term.clone());
        self
    }

    pub fn push(&mut self, term: Term) {
        self.terms.push(term);
    }

    /// Returns a sorted Vec of every symbol in the polynomial
    pub fn get_symbols(&self) -> Vec<&'static Symbol> {
        let mut symbols: Vec<&'static Symbol> = Vec::new();
        self.terms.iter().for_each(|term| {
            term.variables.list.iter().for_each(|var| {
                if !symbols.contains(&var.symbol) {
                    symbols.push(var.symbol);
                }
            })
        });
        symbols.sort();
        symbols
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
        match (self.terms[0].coefficient > 0, self.terms[1].coefficient > 0) {
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

    // p1 = "3x + 1"
    // p1.evaluate("x", 3) => 10
    // p2 = "3x + 2y + 1"
    // p2.evaluate("x", 3) => "10 + 2y"
    // p2.evaluate(&[("x", 3), ("y", 2)]) => 14

    pub fn evaluate<T: Into<Number> + Clone>(&self, replacements: &[(&str, T)]) -> Number {
        let replacement_numbers: Vec<(&str, Number)> = replacements
            .iter()
            .map(|(c, t)| (*c, t.clone().into()))
            .collect();
        let mut symbols: Vec<&str> = replacement_numbers.iter().map(|&(c, _)| c).collect();
        symbols.sort();
        let mut my_symbols: Vec<&str> = self.get_symbols().iter().map(|s| s.0).collect();
        my_symbols.sort();
        assert_eq!(
            symbols, my_symbols,
            "Called evaluate() with a mismatch of variables:"
        );

        let mut result: Number = 0.into();
        self.terms.iter().for_each(|term| {
            result += term.evaluate(&replacement_numbers);
        });
        result
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first_value_written = false;
        if self.terms.len() == 1 && self.terms[0].coefficient == 0 {
            write!(f, "0")?;
        } else {
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
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static X: &Symbol = &Symbol("x");
    static Y: &Symbol = &Symbol("y");
    static A: &Symbol = &Symbol("a");

    #[test]
    fn polynomial_creation() {
        let t1 = 3 * X;
        let t2 = 2 * (X * X);
        let t3 = -3 * X;
        let t4 = Term::from_var(A);
        let p1 = t1.and(&t2).and(&t3).and(&t4);
        let p2 = t2.and(&t1).and(&t4).and(&t3);
        assert_eq!(p1.to_string(), "3x+2x^2-3x+a");
        assert_eq!(p2.to_string(), "2x^2+3x+a-3x");
    }

    #[test]
    fn expression_display() {
        let t1 = 3 * X;
        let t2 = 2 * (X * X);
        let t3 = -3 * X;
        let t4 = Term::from_var(A);
        let polynomial = t1.and(&t2).and(&t3).and(&t4);
        assert_eq!(format!("{polynomial}"), "3x+2x^2-3x+a");
        assert_eq!(format!("{polynomial:+}"), "+3x+2x^2-3x+a");
    }

    #[test]
    fn two_term_sorting() {
        let t_x = -3 * X;
        let t_y = 2 * Y;
        let t_const = Term::from_num(4);
        let x_y_expression = t_x.and(&t_y);
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
        let t_x = -3 * X;
        let t_x_2 = 2 * (X * X);
        let t_const = Term::from(4);
        let expression = t_x.and(&t_x_2).and(&t_const);
        assert_eq!(expression.sorted().to_string(), "2x^2-3x+4");
        let t_y_3 = Term::from_var((Y, 3));
        let expression: Polynomial = vec![&t_x, &t_x_2, &t_const, &t_y_3].into();
        assert_eq!(expression.sorted().to_string(), "y^3+2x^2-3x+4");
    }

    #[test]
    fn expression_evaluation() {
        let t1: Term = 2 * X;
        let t2: Term = -3 * (X * X);
        let t3: Term = 4 * A;
        let exp = t1.and(&t2).and(&t3);
        // NOTE: Currently evaluate() is only available for full number evaluation, not algebraic
        // let partial_evaluation = exp.evaluate(&vec![('x', -1)]);
        // assert_eq!(partial_evaluation.to_string(), "4a-5");
        let full_evaluation = exp.evaluate(&[("a", -2), ("x", 5)]);
        assert_eq!(full_evaluation.to_string(), "-73");
    }
}
