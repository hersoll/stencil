use crate::Number;

use super::{Polynomial, Term};

/// poly_1 + poly_2
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

/// poly + t1
impl std::ops::Add<Term> for Polynomial {
    type Output = Self;
    fn add(self, rhs: Term) -> Self::Output {
        let mut result = self.terms.clone();
        match self.terms.iter().position(|t| t.variables == rhs.variables) {
            Some(index) => result[index] += rhs,
            None => result.push(rhs),
        }
        Self { terms: result }
    }
}

/// poly + 3
impl std::ops::Add<i32> for Polynomial {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        let term_rhs: Term = rhs.into();
        self + term_rhs
    }
}

/// poly += t1
impl std::ops::AddAssign<Term> for Polynomial {
    fn add_assign(&mut self, rhs: Term) {
        *self = self.clone() + rhs;
    }
}

/// poly += 3
impl std::ops::AddAssign<i32> for Polynomial {
    fn add_assign(&mut self, rhs: i32) {
        let term_rhs: Term = rhs.into();
        *self = self.clone() + term_rhs;
    }
}

/// poly_1 - poly_2
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

/// poly_1 * poly_2
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

/// 3 * poly
impl std::ops::Mul<Polynomial> for i32 {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        let mut result = rhs;
        for term in &mut result.terms {
            *term *= self;
        }
        result
    }
}

/// num * poly
impl std::ops::Mul<Polynomial> for Number {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        let mut result = rhs;
        for term in &mut result.terms {
            *term *= self;
        }
        result
    }
}

/// t1 * poly
impl std::ops::Mul<Polynomial> for Term {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        let lhs: Polynomial = self.into();
        lhs * rhs
    }
}

/// poly *= 3
impl std::ops::MulAssign<i32> for Polynomial {
    fn mul_assign(&mut self, rhs: i32) {
        for term in &mut self.terms {
            *term *= rhs;
        }
    }
}

/// -poly
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

/// -poly
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

#[cfg(test)]
mod tests {
    use crate::symbols::Symbol;

    use super::*;

    const X: &Symbol = &Symbol("x");
    const Y: &Symbol = &Symbol("y");
    const A: &Symbol = &Symbol("a");

    #[test]
    fn addition() {
        let t1: Term = 2 * X;
        let t2: Term = -3 * (X * X);
        let t3: Term = 4 * A;
        let exp = Polynomial::from_terms(&[&t1, &t2, &t3]);
        let t1 = X * X;
        let t2 = 3 * X;
        let t3 = A * A;
        let exp_2 = Polynomial::from_terms(&[&t1, &t2, &t3]);
        assert_eq!((exp + exp_2).to_string(), "5x-2x^2+4a+a^2");
    }

    #[test]
    fn subtraction() {
        let t1: Term = 2 * X;
        let t2: Term = -3 * (X * X);
        let t3: Term = 4 * A;
        let exp = Polynomial::from_terms(&[&t1, &t2, &t3]);
        let t1 = X * X;
        let t2 = 3 * X;
        let t3 = A * A;
        let exp_2 = Polynomial::from_terms(&[&t1, &t2, &t3]);
        assert_eq!((exp - exp_2).to_string(), "-x-4x^2+4a-a^2");
    }

    #[test]
    fn multiplication() {
        let t1 = Term::from_var(X);
        let t2 = 2 * (Y * Y);
        let t3 = Term::from_num(-3);
        let exp_1 = Polynomial::from_terms(&[&t1, &t2]);
        let exp_2 = Polynomial::from_terms(&[&t1, &t2, &t3]);
        assert_eq!((3 * exp_2.clone()).to_string(), "3x+6y^2-9");
        let mult_exp = exp_1 * exp_2;
        assert_eq!(mult_exp.to_string(), "x^2+2x y^2-3x+2x y^2+4y^4-6y^2");
        assert_eq!(mult_exp.simplify().to_string(), "4y^4+4x y^2+x^2-6y^2-3x");
    }
}
