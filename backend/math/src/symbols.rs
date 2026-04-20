mod generation;
mod statics;
pub use generation::*;
pub use statics::*;
use std::fmt::Display;

use crate::{Number, Term};

/// Represents symbolic values.
///
/// For memory efficiency, Symbols should **always** be used by refererence, either by directly
/// accessing one of the statics by reference (like `&symbols::X`) or by the generator (`symbols::get_unknown()`)
/// which returns references.
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
/// 3 * x = 3x
impl std::ops::Mul<&'static Symbol> for i32 {
    type Output = Term;
    fn mul(self, rhs: &'static Symbol) -> Self::Output {
        self * Term::from_var(rhs)
    }
}

/// Multiplying a Symbol with a number will net a Term consisting of those two:
/// 3 * x = 3x
impl std::ops::Mul<&'static Symbol> for Number {
    type Output = Term;
    fn mul(self, rhs: &'static Symbol) -> Self::Output {
        self * Term::from_var(rhs)
    }
}
