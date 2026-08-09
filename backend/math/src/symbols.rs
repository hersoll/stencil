mod generation;
mod statics;
pub use generation::*;
pub use statics::*;
pub mod inequality_sign;
use std::fmt::Display;

use crate::{Number, Polynomial, Term};

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
/// 3 * x = Term(3x)
impl std::ops::Mul<&'static Symbol> for Number {
    type Output = Term;
    fn mul(self, rhs: &'static Symbol) -> Self::Output {
        self * Term::from_var(rhs)
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

/// Adding a Number and a Symbol turns it into a polynomial
impl std::ops::Add<&'static Symbol> for Number {
    type Output = Polynomial;
    fn add(self, rhs: &'static Symbol) -> Self::Output {
        let t1 = Term::from_var(rhs);
        let t2 = Term::from_num(self);
        t1.and(&t2)
    }
}
