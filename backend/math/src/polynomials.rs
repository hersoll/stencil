mod operations;
use crate::{Evaluable, Number, Replacement, Term, symbols::Symbol};
use rand::{rng, seq::SliceRandom};
use std::fmt::Display;

/// The [`Polynomial`] groups [`Term`]s together and allows for sorting them, performing calculations
/// with other [`Polynomial`]s and maybe most importantly, simplifying [`Term`]s together
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    /// Constructor
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    /// Creates a [`Polynomial`] from an array of [`Term`]s.
    ///
    /// This is more convenient when the [`Polynomial`] consists of multiple [`Term`]s,
    /// otherwise `.and()` is faster to write
    pub fn from_terms(terms: &[&Term]) -> Self {
        let terms: Vec<Term> = terms.iter().map(|&t| t.clone()).collect();
        Self { terms }
    }

    /// Method used to chain multiple [`Term`]s into an ergonomic [`Polynomial`] creation:
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
    /// The first [`.and()`](Term::and) call comes from [`Term::and`], but that call returns a [`Polynomial`] and
    /// therefore this method is needed to do the second [`.and()`](Polynomial::and) call.
    pub fn and(mut self, term: &Term) -> Polynomial {
        self.push(term.clone());
        self
    }

    /// Ergonomic wrapper for pushing into the inner [`Vec`]
    pub fn push(&mut self, term: Term) {
        self.terms.push(term);
    }

    /// Returns a [`Vec`] of every symbol in the [`Polynomial`], sorted alphabetically.
    ///
    /// # Examples
    /// ```rust
    /// use math::symbols::{X, Y, Z};
    /// let t1 = X * Z;
    /// let t2 = 2 * (Y * X);
    /// let poly = t1.and(&t2);
    /// assert_ne!(poly.get_symbols(), vec![X, Z, Y]);
    /// assert_eq!(poly.get_symbols(), vec![X, Y, Z]);
    /// ```
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

    /// Returns a [`Polynomial`] with the order of the [`Term`]s shuffled.
    pub fn random_order(terms: &[&Term]) -> Self {
        let mut owned_terms: Vec<Term> = terms.iter().map(|&t| t.clone()).collect();
        owned_terms.shuffle(&mut rng());
        Self { terms: owned_terms }
    }

    /// Simplifies the [`Polynomial`] using normal math rules.
    ///
    /// Will always [`sort()`](Polynomial::sort) the result.
    ///
    /// # Examples
    /// ```rust
    /// use math::symbols::X;
    /// let t1 = 2 * X;
    /// let t2 = 3 * X;
    /// let polynomial = t1.and(&t2).simplify();
    /// assert_eq!(polynomial.to_string(), "5x");
    /// ```
    ///
    /// ```rust
    /// use math::symbols::X;
    /// let t1 = X * X;
    /// let t2 = 3 * X;
    /// let t3 = 2 * (X * X);
    /// let polynomial = t1.and(&t2).and(&t3).simplify();
    /// assert_eq!(polynomial.to_string(), "3x^2+3x");
    /// ```
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

    /// Sorts the [`Polynomial`] in-place.
    ///
    /// When sorting [`Polynomial`]s, some conventions are enforced:
    /// - If there are two terms where one is positive, the positive term is always placed first
    /// - Otherwise, the terms are sorted by degree first and alphabetically second
    /// # Examples
    /// ```rust
    /// use math::{Polynomial, Term};
    /// use math::symbols::{X, Y};
    /// let xy = X * Y;
    /// let two_x = 2 * X;
    /// let x2 = X * X;
    /// let three_y = 3 * Y;
    /// let constant = Term::from_num(-4);
    /// let y2 = Y * Y;
    /// let mut poly_sorted = Polynomial::from_terms(&[&xy, &two_x, &x2, &three_y, &constant, &y2]);
    /// poly_sorted.sort();
    /// // x^2 comes before xy, which comes before y^2
    /// let poly_target = Polynomial::from_terms(&[&x2, &xy, &y2, &two_x, &three_y, &constant]);
    /// assert_eq!(poly_sorted, poly_target);
    /// ```
    /// Generally, [`sorted()`](Polynomial::sorted) is the more ergonomic method
    /// ```rust
    /// use math::symbols::{X, Y};
    /// let t1 = -(X * X);
    /// let t2 = 3 * Y;
    /// // Positive term is placed first
    /// assert_eq!(t1.and(&t2).sorted(), t2.and(&t1));
    /// ```
    pub fn sort(&mut self) {
        use std::cmp::Ordering::*;
        match self.terms.len().cmp(&2) {
            Greater => self.sort_by_variables(),
            Equal => self.place_positive_first(),
            Less => (),
        }
    }

    /// Returns a sorted clone of the [`Polynomial`].
    ///
    /// To read about the sorting, see [`sort()`](Polynomial::sort).
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
                self.terms[1] = temp;
            }
            (_, _) => self.sort_by_variables(),
        }
    }

    fn sort_by_variables(&mut self) {
        self.terms.sort_by(|a, b| b.variables.cmp(&a.variables));
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

impl Evaluable for Polynomial {
    fn print_replacements(&self, replacements: &[Replacement]) -> String {
        let mut s = String::new();
        self.terms.iter().enumerate().for_each(|(i, term)| {
            if term.coefficient != 0 {
                // Terms don't add signs by themselves when printing replacements
                if i > 0 && term.coefficient > 0 {
                    s += "+";
                }
                s += &term.print_replacements(replacements);
            }
        });

        s
    }

    fn print_evaluation_by_parts(&self, replacements: &[Replacement]) -> String {
        let mut s = String::new();
        self.terms.iter().enumerate().for_each(|(i, term)| {
            if term.coefficient != 0 {
                // Terms don't add signs by themselves when printing evaluations
                if i > 0 && term.coefficient > 0 {
                    s += "+";
                }
                s += &term.print_evaluation_by_parts(replacements);
            }
        });

        s
    }

    fn evaluate(&self, replacements: &[Replacement]) -> Number {
        let mut result: Number = Number::Integer(0);
        self.terms.iter().for_each(|term| {
            result += term.evaluate(replacements);
        });
        result
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
        let x_const_expression = t_x.and(&t_const);
        assert_eq!(x_const_expression.sorted().to_string(), "4-3x");
        let y_const_expression = t_y.and(&t_const);
        assert_eq!(y_const_expression.sorted().to_string(), "2y+4");
        let t_const = Term::from(-4);
        let x_const_expression = t_x.and(&t_const);
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
        let expression = Polynomial::from_terms(&[&t_x, &t_x_2, &t_const, &t_y_3]);
        assert_eq!(expression.sorted().to_string(), "y^3+2x^2-3x+4");
    }

    #[test]
    fn expression_evaluation() {
        let t1: Term = 2 * X;
        let t2: Term = -3 * (X * X);
        let t3: Term = 4 * A;
        let exp = t1.and(&t2).and(&t3);
        let replacements = [(A, &Number::Integer(-2)), (X, &Number::Integer(5))];
        let full_evaluation = exp.evaluate(&replacements);
        assert_eq!(full_evaluation.to_string(), "-73");
    }
}
