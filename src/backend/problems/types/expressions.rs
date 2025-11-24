use rand::{rng, seq::SliceRandom};
use std::fmt::Display;

use crate::backend::{problems::types::terms::Term, typst_formatting};

#[derive(Debug, Clone)]
pub struct Expression {
    pub terms: Vec<Term>,
}

impl From<Term> for Expression {
    fn from(value: Term) -> Self {
        Expression { terms: vec![value] }
    }
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

    pub fn push(&mut self, term: Term) {
        self.terms.push(term);
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

    pub fn evaluate(&self, replacements: &Vec<(char, i32)>) -> Expression {
        let mut new_expression = Expression::new();
        self.terms.iter().for_each(|term| {
            new_expression.terms.push(term.evaluate(replacements));
        });
        new_expression.simplify()
    }

    pub fn show_replacements(&self, replacements: &Vec<(char, i32)>) -> String {
        use std::fmt::Write;

        let mut s = String::new();
        for (i, term) in self.terms.iter().enumerate() {
            if i == 0 {
                write!(&mut s, "{}", term.coefficient).unwrap();
            } else {
                write!(&mut s, "{:+}", term.coefficient).unwrap();
            }
            for (j, var) in term.variables.list.iter().enumerate() {
                match replacements.iter().find(|pair| pair.0 == var.symbol) {
                    Some(pair) => {
                        // Prevent double dot
                        s = s.trim_end_matches(" dot ").to_string();
                        write!(
                            &mut s,
                            " dot colored({}){}{}",
                            typst_formatting::parentheses(pair.1),
                            if var.exponent > 1 {
                                format!("^{}", var.exponent)
                            } else {
                                String::new()
                            },
                            if j < term.variables.list.len() - 1 {
                                " dot "
                            } else {
                                ""
                            }
                        )
                        .unwrap()
                    }
                    None => write!(&mut s, "{var}").unwrap(),
                }
            }
        }
        s
    }

    pub fn show_evaluation(&self, replacements: &Vec<(char, i32)>) -> String {
        use std::fmt::Write;

        let mut s = String::new();
        for (i, term) in self.terms.iter().enumerate() {
            if i == 0 {
                write!(&mut s, "{}", term.evaluate(replacements)).unwrap();
            } else {
                write!(&mut s, "{:+}", term.evaluate(replacements)).unwrap();
            }
        }
        s
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

impl std::ops::Add<Term> for Expression {
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

impl std::ops::Add<i32> for Expression {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        let term_rhs: Term = rhs.into();
        self + term_rhs
    }
}

impl std::ops::AddAssign<Term> for Expression {
    fn add_assign(&mut self, rhs: Term) {
        *self = self.clone() + rhs;
    }
}

impl std::ops::AddAssign<i32> for Expression {
    fn add_assign(&mut self, rhs: i32) {
        let term_rhs: Term = rhs.into();
        *self = self.clone() + term_rhs;
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
impl std::ops::Mul<Expression> for Expression {
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
impl std::ops::Mul<Expression> for Term {
    type Output = Expression;
    fn mul(self, rhs: Expression) -> Self::Output {
        let lhs: Expression = self.into();
        lhs * rhs
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

impl std::ops::Neg for &Expression {
    type Output = Expression;
    fn neg(self) -> Self::Output {
        let mut new_exp = Expression::new();
        for term in &self.terms {
            new_exp.push(-term);
        }
        new_exp
    }
}

impl std::ops::Neg for Expression {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut new_exp = Expression::new();
        for term in self.terms {
            new_exp.push(-term);
        }
        new_exp
    }
}

impl Display for Expression {
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
    use crate::backend::Variables;

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
    fn two_term_sorting() {
        let t_x = Term::from((-3, 'x'));
        let t_y = Term::from((2, 'y'));
        let t_const = Term::from(4);
        let x_y_expression: Expression = vec![&t_x, &t_y].into();
        assert_eq!(x_y_expression.to_string(), "-3x+2y");
        assert_eq!(x_y_expression.sorted().to_string(), "-3x+2y");
        let x_const_expression: Expression = vec![&t_x, &t_const].into();
        assert_eq!(x_const_expression.sorted().to_string(), "4-3x");
        let y_const_expression: Expression = vec![&t_y, &t_const].into();
        assert_eq!(y_const_expression.sorted().to_string(), "2y+4");
        let t_const = Term::from(-4);
        let x_const_expression: Expression = vec![&t_x, &t_const].into();
        assert_eq!(x_const_expression.sorted().to_string(), "-3x-4");
    }

    #[test]
    fn multiple_term_sorting() {
        let t_x = Term::from((-3, 'x'));
        let t_x_2 = Term::from((2, ('x', 2)));
        let t_const = Term::from(4);
        let expression: Expression = vec![&t_x, &t_x_2, &t_const].into();
        assert_eq!(expression.sorted().to_string(), "2x^2-3x+4");
        let t_y_3: Term = ('y', 3).into();
        let expression: Expression = vec![&t_x, &t_x_2, &t_const, &t_y_3].into();
        assert_eq!(expression.sorted().to_string(), "y^3+2x^2-3x+4");
    }

    #[test]
    fn expression_evaluation() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let t3: Term = (4, 'a').into();
        let exp: Expression = vec![&t1, &t2, &t3].into();
        let partial_evaluation = exp.evaluate(&vec![('x', -1)]);
        assert_eq!(partial_evaluation.to_string(), "4a-5");
        let full_evaluation = exp.evaluate(&vec![('a', -2), ('x', 5)]);
        assert_eq!(full_evaluation.to_string(), "-73");
    }

    #[test]
    fn expression_evaluation_display() {
        let t1: Term = (2, 'x').into();
        let t2: Term = (-3, ('x', 2)).into();
        let vars = Variables::from(vec![('a', 3), ('x', 3), ('y', 4)]);
        let t3: Term = (4, vars).into();
        let exp: Expression = vec![&t1, &t2, &t3].into();
        assert_eq!(
            exp.show_replacements(&vec![('x', -1)]),
            "2 dot colored((-1))-3 dot colored((-1))^2+4a^3 dot colored((-1))^3 dot y^4"
        );
        assert_eq!(
            exp.show_replacements(&vec![('x', 4)]),
            "2 dot colored(4)-3 dot colored(4)^2+4a^3 dot colored(4)^3 dot y^4"
        );
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
