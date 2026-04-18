use super::super::Number;
use super::Term;

impl std::ops::Neg for Term {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            coefficient: -self.coefficient,
            variables: self.variables,
            colored: self.colored,
        }
    }
}

impl std::ops::Neg for &Term {
    type Output = Term;
    fn neg(self) -> Self::Output {
        Term {
            coefficient: -self.coefficient,
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }
}
/// Do not add terms manually, only meant to be used inside Polynomial implementation
///
/// Use `t1.and(t2)` to quickly create Polynomials
impl std::ops::Add for Term {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.variables, rhs.variables);
        Self {
            coefficient: self.coefficient + rhs.coefficient,
            variables: self.variables,
            colored: self.colored,
        }
    }
}
impl std::ops::AddAssign for Term {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.variables, rhs.variables);
        self.coefficient += rhs.coefficient;
    }
}
impl std::ops::Sub for Term {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.variables, rhs.variables);
        Self {
            coefficient: self.coefficient - rhs.coefficient,
            variables: self.variables,
            colored: self.colored,
        }
    }
}
impl std::ops::SubAssign for Term {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.variables, rhs.variables);
        self.coefficient -= rhs.coefficient;
    }
}
impl std::ops::Mul for Term {
    type Output = Term;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            coefficient: self.coefficient * rhs.coefficient,
            variables: self.variables * rhs.variables,
            colored: self.colored,
        }
    }
}
impl std::ops::MulAssign for Term {
    fn mul_assign(&mut self, rhs: Self) {
        self.coefficient = self.coefficient * rhs.coefficient;
        self.variables = self.variables.clone() * rhs.variables;
    }
}
impl std::ops::Mul<Term> for i32 {
    type Output = Term;
    fn mul(self, rhs: Term) -> Self::Output {
        Term {
            coefficient: rhs.coefficient * self,
            variables: rhs.variables.clone(),
            colored: rhs.colored,
        }
    }
}
impl std::ops::Mul<i32> for Term {
    type Output = Term;
    fn mul(self, rhs: i32) -> Self::Output {
        Term {
            coefficient: self.coefficient * rhs,
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }
}
impl std::ops::MulAssign<i32> for Term {
    fn mul_assign(&mut self, rhs: i32) {
        self.coefficient *= rhs;
    }
}

impl std::ops::Mul<Term> for Number {
    type Output = Term;
    fn mul(self, rhs: Term) -> Self::Output {
        Term {
            coefficient: rhs.coefficient * self,
            variables: rhs.variables.clone(),
            colored: rhs.colored,
        }
    }
}
impl std::ops::Mul<Number> for Term {
    type Output = Term;
    fn mul(self, rhs: Number) -> Self::Output {
        Term {
            coefficient: self.coefficient * rhs,
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }
}
impl std::ops::MulAssign<Number> for Term {
    fn mul_assign(&mut self, rhs: Number) {
        self.coefficient *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::{PI, symbols::Symbol};

    use super::*;
    const X: Symbol = Symbol("x");
    #[test]
    fn addition() {
        let t1 = 3 * (X * X);
        let t2 = 2 * (X * X);
        assert_eq!((t1 + t2).to_string(), "5x^2");
        // += assignment
        let mut t3 = Term::from_var((X, 4));
        let t4 = 4 * t3.clone();
        t3 += t4.clone();
        assert_eq!(t3.to_string(), "5x^4");

        let t5 = Term::from_num_and_vars((2, 3), (X, 4));
        assert_eq!((t4 + t5).to_string(), "(14x^4)/3");

        let t6 = Term::from_num_and_vars(1.3, X);
        let t7 = Term::from_num_and_vars(PI, X);
        assert_eq!((t6 + t7).to_string(), "num(\"4.442\")x");
    }

    #[test]
    #[should_panic]
    fn cant_add_different_terms() {
        let t1 = 3 * (X * X);
        let t2 = 2 * X;
        assert_eq!((t1 + t2).to_string(), "throws");
    }

    #[test]
    fn subtraction() {
        let t1 = 3 * (X * X);
        let t2 = 2 * (X * X);
        assert_eq!((t1 - t2).to_string(), "x^2");
        // -= assignment
        let mut t3 = Term::from_var((X, 4));
        let t4 = 4 * t3.clone();
        t3 -= t4.clone();
        assert_eq!(t3.to_string(), "-3x^4");
    }

    #[test]
    #[should_panic]
    fn cant_subtract_different_terms() {
        let t1 = 3 * (X * X);
        let t2 = 2 * X;
        assert_eq!((t1 - t2).to_string(), "throws");
    }

    #[test]
    fn multiplication() {
        // Term and number
        let t1 = 12 * Term::from_var((X, 4));
        let factor = 3;
        assert_eq!((factor * t1.clone()).to_string(), "36x^4");
        assert_eq!((t1 * factor).to_string(), "36x^4");
        // Term and term
        let mut t2 = 3 * X;
        let t3 = 2 * (X * X);
        let a = Symbol("a");
        let y = Symbol("y");
        let t4 = a * a;
        let t5 = Term::from_var(y);
        assert_eq!((t2.clone() * t3.clone()).to_string(), "6x^3");
        assert_eq!((t2.clone() * t4.clone()).to_string(), "3a^2 x");
        assert_eq!((t2.clone() * t5.clone()).to_string(), "3x y");
        t2 *= t3.clone();
        assert_eq!(t2.to_string(), "6x^3");
        assert_eq!((t2 * t3 * t4 * t5).to_string(), "12a^2 x^5 y");
    }
}
