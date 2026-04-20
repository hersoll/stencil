mod generation;
pub use generation::*;
use std::fmt::Display;

use crate::{Number, Term};

// Some Symbols will be created a *lot* during problem generation.
// These consts prevent creating the same data multiple times
pub const X: Symbol = Symbol("x");
pub const Y: Symbol = Symbol("y");
pub const PI: Symbol = Symbol("pi");
pub const E: Symbol = Symbol("e");

/// Represents symbolic values.
///
/// Contains a `&str` since symbols can be "pi", "alpha", etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub &'static str);

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Multiplying two Symbols will net a Term consisting of those symbols:
/// x * y = xy
/// x * x = x^2
impl std::ops::Mul for Symbol {
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
/// 3 * x = 3x
impl std::ops::Mul<Symbol> for i32 {
    type Output = Term;
    fn mul(self, rhs: Symbol) -> Self::Output {
        self * Term::from_var(rhs)
    }
}

/// Multiplying a Symbol with a number will net a Term consisting of those two:
/// 3 * x = 3x
impl std::ops::Mul<Symbol> for Number {
    type Output = Term;
    fn mul(self, rhs: Symbol) -> Self::Output {
        self * Term::from_var(rhs)
    }
}
