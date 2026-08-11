mod generation;
mod statics;
pub use generation::*;
pub use statics::*;
pub mod inequality_sign;
use std::fmt::Display;

use crate::{HasCoef, Number, Polynomial, Term};

/// Represents symbolic values.
///
/// For memory efficiency, Symbols should **never** be constructed manually, there are static references
/// available either through direct access (like `symbols::X`) or by the generator (`symbols::get_unknown()`)
/// which both return references.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub &'static str);

impl Symbol {
    /// Note that exp will be an integer
    pub fn powi(&'static self, exp: impl Into<Number>) -> Term {
        Term::from_var((self, exp.into().as_i32()))
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Multiplying two Symbols will net a Term consisting of those symbols:
/// x * y = Term(xy)
/// x * x = Term(x^2)
impl std::ops::Mul for &'static Symbol {
    type Output = Term;
    fn mul(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Term::from_var((self, 2))
        } else {
            Term::from_var(vec![self, rhs])
        }
    }
}

/// Multiplying a Symbol with a number will net a Term consisting of those two:
/// 3 * x = Term(3x)
impl std::ops::Mul<&'static Symbol> for i32 {
    type Output = Term;
    fn mul(self, rhs: &'static Symbol) -> Self::Output {
        self * Term::from_var(rhs)
    }
}

/// Multiplying a Symbol with a number will net a Term consisting of those two:
/// x * 3 = Term(3x)
impl std::ops::Mul<i32> for &'static Symbol {
    type Output = Term;
    fn mul(self, rhs: i32) -> Self::Output {
        rhs * Term::from_var(self)
    }
}

/// Multiplying a Symbol with a number will net a Term consisting of those two:
/// 3 * x = Term(3x)
impl std::ops::Mul<&'static Symbol> for Number {
    type Output = Term;
    fn mul(self, rhs: &'static Symbol) -> Self::Output {
        self * Term::from_var(rhs)
    }
}

/// Multiplying a Symbol with a number will net a Term consisting of those two:
/// x * 3 = Term(3x)
impl std::ops::Mul<Number> for &'static Symbol {
    type Output = Term;
    fn mul(self, rhs: Number) -> Self::Output {
        rhs * Term::from_var(self)
    }
}

/// Adding two symbols turns it into a polynomial
impl std::ops::Add for &'static Symbol {
    type Output = Polynomial;
    fn add(self, rhs: Self) -> Self::Output {
        let t1 = Term::from_var(self);
        let t2 = Term::from_var(rhs);
        let p = t1.and(&t2);
        if self == rhs { p.simplify() } else { p }
    }
}

/// Adding a Symbol and a Number turns it into a polynomial
impl std::ops::Add<Number> for &'static Symbol {
    type Output = Polynomial;
    fn add(self, rhs: Number) -> Self::Output {
        let t1 = Term::from_var(self);
        let t2 = Term::from_num(rhs);
        t1.and(&t2)
    }
}
impl std::ops::Add<i32> for &'static Symbol {
    type Output = Polynomial;
    fn add(self, rhs: i32) -> Self::Output {
        let t1 = Term::from_var(self);
        let t2 = Term::from_num(rhs);
        t1.and(&t2)
    }
}

/// Adding a Number and a Symbol turns it into a polynomial
impl std::ops::Add<&'static Symbol> for Number {
    type Output = Polynomial;
    fn add(self, rhs: &'static Symbol) -> Self::Output {
        let t1 = Term::from_num(self);
        let t2 = Term::from_var(rhs);
        t1.and(&t2)
    }
}
impl std::ops::Add<&'static Symbol> for i32 {
    type Output = Polynomial;
    fn add(self, rhs: &'static Symbol) -> Self::Output {
        let t1 = Term::from_num(self);
        let t2 = Term::from_var(rhs);
        t1.and(&t2)
    }
}

/// Subtracting a Number and a Symbol turns it into a polynomial
impl std::ops::Sub<&'static Symbol> for Number {
    type Output = Polynomial;
    fn sub(self, rhs: &'static Symbol) -> Self::Output {
        let t1 = Term::from_num(self);
        let t2 = Term::from_var(rhs);
        t1.and(&-t2)
    }
}
impl std::ops::Sub<&'static Symbol> for i32 {
    type Output = Polynomial;
    fn sub(self, rhs: &'static Symbol) -> Self::Output {
        let t1 = Term::from_num(self);
        let t2 = Term::from_var(rhs);
        t1.and(&-t2)
    }
}
impl std::ops::Sub<Number> for &'static Symbol {
    type Output = Polynomial;
    fn sub(self, rhs: Number) -> Self::Output {
        let t1 = Term::from_var(self);
        let t2 = Term::from_num(rhs);
        t1.and(&-t2)
    }
}
impl std::ops::Sub<i32> for &'static Symbol {
    type Output = Polynomial;
    fn sub(self, rhs: i32) -> Self::Output {
        let t1 = Term::from_var(self);
        let t2 = Term::from_num(rhs);
        t1.and(&-t2)
    }
}

impl HasCoef for &'static Symbol {
    fn coef(&self) -> Number {
        Number::Integer(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponentiation() {
        assert_eq!((X.powi(2)).to_string(), "x^2");
        assert_eq!((X.powi(3)).to_string(), "x^3");
        assert_eq!((X.powi(1)).to_string(), "x");
        assert_eq!((X.powi(-1)).to_string(), "x^(-1)");
        assert_eq!((X.powi(0)).to_string(), "1");
    }

    #[test]
    fn multiplication_between_symbols() {
        assert_eq!((X * X).to_string(), "x^2");
        assert_eq!((X * Y).to_string(), "x y");
        assert_eq!((Y * X).to_string(), "x y");
        assert_eq!((X * X * X).to_string(), "x^3");
        assert_eq!((X * Y * X).to_string(), "x^2 y");
    }

    #[test]
    fn addition_between_symbols() {
        assert_eq!((X + X).to_string(), "2x");
        assert_eq!((X + Y).to_string(), "x+y");
        assert_eq!((Y + X).to_string(), "y+x");
        assert_eq!((X + X + X).to_string(), "3x");
        assert_eq!((X + Y + X).to_string(), "2x+y");
    }

    #[test]
    fn multiplying_with_number() {
        assert_eq!((3 * X).to_string(), "3x");
        assert_eq!((X * 3).to_string(), "3x");
        assert_eq!((X * 1).to_string(), "x");
        assert_eq!((-1 * X).to_string(), "-x");
        let num = Number::Integer(-2);
        assert_eq!((num * X).to_string(), "-2x");
        assert_eq!((X * num).to_string(), "-2x");
    }

    #[test]
    fn adding_with_number() {
        assert_eq!((3 + X).to_string(), "3+x");
        assert_eq!((X + 3).to_string(), "x+3");
        assert_eq!((X + 0).to_string(), "x");
        let num = Number::Integer(-2);
        assert_eq!((num + X).to_string(), "-2+x");
        assert_eq!((X + num).to_string(), "x-2");
    }

    #[test]
    fn subtracting_with_number() {
        assert_eq!((3 - X).to_string(), "3-x");
        assert_eq!((X - 3).to_string(), "x-3");
        assert_eq!((X - 0).to_string(), "x");
        let num = Number::Integer(-2);
        assert_eq!((num - X).to_string(), "-2-x");
        assert_eq!((X - num).to_string(), "x+2");
    }

    #[test]
    fn coef_returns_one() {
        assert_eq!(X.coef(), 1);
        assert_eq!(LAMBDA_CAPS.coef(), 1);
    }
}
