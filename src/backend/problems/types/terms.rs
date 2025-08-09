use std::fmt::Display;

use crate::backend::problems::types::variables::Variable;
use crate::backend::problems::types::variables::Variables;

#[derive(Clone, Debug)]
pub struct Term {
    pub coefficient: i32,
    pub variables: Variables,
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            if self.coefficient == 1 {
                if self.variables.list.is_empty() {
                    write!(f, "+1")?;
                } else {
                    write!(f, "+{}", self.variables)?;
                }
            } else if self.coefficient == -1 {
                if self.variables.list.is_empty() {
                    write!(f, "-1")?;
                } else {
                    write!(f, "-{}", self.variables)?;
                }
            } else if self.coefficient == 0 {
                write!(f, "")?;
            } else {
                write!(f, "{:+}{}", self.coefficient, self.variables)?;
            }
        } else {
            if self.coefficient == 1 {
                if self.variables.list.is_empty() {
                    write!(f, "1")?;
                } else {
                    write!(f, "{}", self.variables)?;
                }
            } else if self.coefficient == -1 {
                if self.variables.list.is_empty() {
                    write!(f, "-1")?;
                } else {
                    write!(f, "-{}", self.variables)?;
                }
            } else if self.coefficient == 0 {
                write!(f, "")?;
            } else {
                write!(f, "{}{}", self.coefficient, self.variables)?;
            }
        }
        Ok(())
    }
}

impl<T> From<(i32, T)> for Term
where
    T: Into<Variable>,
{
    fn from(value: (i32, T)) -> Self {
        Self {
            coefficient: value.0,
            variables: Variables::from(value.1),
        }
    }
}
impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Self {
            coefficient: value,
            variables: Variables::new(),
        }
    }
}

impl std::ops::Neg for Term {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            coefficient: -self.coefficient,
            variables: self.variables,
        }
    }
}
// Do not add terms manually, only meant to be used inside Expression implementation
impl std::ops::Add for Term {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.variables, rhs.variables);
        Self {
            coefficient: self.coefficient + rhs.coefficient,
            variables: self.variables,
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
        }
    }
}
impl std::ops::MulAssign<i32> for Term {
    fn mul_assign(&mut self, rhs: i32) {
        self.coefficient *= rhs;
    }
}
