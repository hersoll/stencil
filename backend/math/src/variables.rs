use crate::{Number, symbols::Symbol};
use std::{cmp::Ordering, collections::HashSet, fmt::Display};

/// An internal representation of variables.
/// Since Symbols are statics that have a place in memory, we can always use a reference to that
/// place instad!
///
/// Will almost never be constructed explicitly, you probably want Term instead!
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Variable {
    pub symbol: &'static Symbol,
    pub exponent: i32,
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.exponent == 0 {
            write!(f, "")
        } else if self.exponent == 1 {
            write!(f, "{}", self.symbol)
        } else {
            write!(f, "{}^{}", self.symbol, self.exponent)
        }
    }
}

/// Simple variable
impl From<&'static Symbol> for Variable {
    fn from(symbol: &'static Symbol) -> Self {
        Self {
            symbol,
            exponent: 1,
        }
    }
}

/// Includes an exponent
impl From<(&'static Symbol, i32)> for Variable {
    fn from(value: (&'static Symbol, i32)) -> Self {
        Self {
            symbol: value.0,
            exponent: value.1,
        }
    }
}

impl From<(&'static Symbol, Number)> for Variable {
    fn from(value: (&'static Symbol, Number)) -> Self {
        if let Number::Integer(val) = value.1 {
            Self {
                symbol: value.0,
                exponent: val,
            }
        } else {
            tracing::error!(
                "Don't use a non-integer Number (used {}) as an exponent",
                value.1
            );
            Self {
                symbol: value.0,
                exponent: 1,
            }
        }
    }
}

/// Ordering makes it easier to sort multivariate terms by convention:
/// 3ab should be printed as such, not 3ba.
impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.symbol.cmp(other.symbol)
    }
}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implementing Neg this way makes it easier to define division of variables:
/// a / b = a * (-b)
///
/// Note that negative variables aren't the same as negative Terms - those are negative in the
/// normal sense. Negative variables should only be used internally
impl std::ops::Neg for Variable {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            symbol: self.symbol,
            exponent: -self.exponent,
        }
    }
}

#[derive(Debug, Clone, Eq, Default)]
pub struct Variables {
    pub list: Vec<Variable>,
}

impl Variables {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn from_symbol(s: &'static Symbol) -> Self {
        Self {
            list: vec![Variable::from(s)],
        }
    }
}

impl Display for Variables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, var) in self.list.iter().enumerate() {
            if i == &self.list.len() - 1 {
                write!(f, "{var}")?;
            } else {
                write!(f, "{var} ")?;
            }
        }
        Ok(())
    }
}
impl<T> From<T> for Variables
where
    T: Into<Variable>,
{
    fn from(variable: T) -> Self {
        Self {
            list: vec![variable.into()],
        }
    }
}
impl<T> From<Vec<T>> for Variables
where
    T: Into<Variable>,
{
    fn from(list: Vec<T>) -> Self {
        let mut variables: Vec<Variable> = list
            .into_iter()
            .map(|v| v.into())
            .filter(|v| v.exponent != 0)
            .collect();
        variables.sort_by_key(|v| v.symbol);
        Self { list: variables }
    }
}
impl PartialEq for Variables {
    fn eq(&self, other: &Self) -> bool {
        let set1: HashSet<(&Symbol, i32)> =
            self.list.iter().map(|v| (v.symbol, v.exponent)).collect();
        let set2: HashSet<(&Symbol, i32)> =
            other.list.iter().map(|v| (v.symbol, v.exponent)).collect();
        set1 == set2
    }
}
impl std::ops::Mul for Variables {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut final_variables = self.list;
        for var in rhs.list {
            match final_variables.iter().position(|v| v.symbol == var.symbol) {
                // Yes, we actually want to add the exponents, clippy :)
                #[allow(clippy::suspicious_arithmetic_impl)]
                Some(index) => final_variables[index].exponent += var.exponent,
                None => final_variables.push(var),
            }
        }
        final_variables.sort_by_key(|v| v.symbol);
        Self {
            list: final_variables,
        }
    }
}

impl std::ops::Div for Variables {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut final_variables = self.list;
        for var in rhs.list {
            match final_variables.iter().position(|v| v.symbol == var.symbol) {
                Some(index) => final_variables[index].exponent -= var.exponent,
                None => final_variables.push(-var),
            }
        }
        final_variables.sort_by_key(|v| v.symbol);
        Self {
            list: final_variables,
        }
    }
}

impl Ord for Variables {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.list.is_empty() && other.list.is_empty() {
            Ordering::Equal
        } else if self.list.is_empty() {
            Ordering::Less
        } else if other.list.is_empty() {
            Ordering::Greater
        } else {
            let total_exponent_first: i32 = self.list.iter().map(|v| v.exponent.abs()).sum();
            let total_exponent_second: i32 = other.list.iter().map(|v| v.exponent.abs()).sum();
            let variable_list_first: String = self
                .list
                .iter()
                .map(|v| {
                    v.symbol
                        .to_string()
                        .repeat(v.exponent.unsigned_abs().try_into().unwrap())
                })
                .collect();
            let variable_list_second: String = other
                .list
                .iter()
                .map(|v| {
                    v.symbol
                        .to_string()
                        .repeat(v.exponent.unsigned_abs().try_into().unwrap())
                })
                .collect();

            total_exponent_first
                .cmp(&total_exponent_second)
                .then_with(|| variable_list_second.cmp(&variable_list_first))
        }
    }
}

impl PartialOrd for Variables {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static X: Symbol = Symbol("x");
    static Y: Symbol = Symbol("y");
    static Z: Symbol = Symbol("z");
    static A: Symbol = Symbol("a");
    static B: Symbol = Symbol("b");

    #[test]
    fn variable_creations() {
        let v1 = Variable::from(&X);
        let v2 = Variable::from((&X, 2));
        let v_const = Variable::from((&A, 0));
        assert_eq!(v1.exponent, 1);
        assert_eq!(v1.symbol.0, "x");
        assert_eq!(v1.to_string(), "x");
        assert_eq!(v2.to_string(), "x^2");
        assert_eq!(v_const.to_string(), "");
    }

    #[test]
    fn variable_ordering() {
        let v1 = Variable::from(&A);
        let v2 = Variable::from(&B);
        let v3 = Variable::from(&X);
        let v4 = Variable::from(&X);
        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
        assert!(v3 == v4);
    }

    #[test]
    fn variable_negation() {
        let v1 = Variable::from((&X, 2));
        let v2 = Variable::from((&A, -3));
        assert_eq!((-v1).to_string(), "x^-2");
        assert_eq!((-v2).to_string(), "a^3");
    }

    #[test]
    fn variables_creation() {
        let v1 = Variables::from(&X);
        let v2 = Variables::from(vec![&X, &Y]);
        let v3 = Variables::from(vec![(&X, 3), (&Y, 5)]);

        assert_eq!(v1.to_string(), "x");
        assert_eq!(v2.to_string(), "x y");
        assert_eq!(v3.to_string(), "x^3 y^5");
    }

    #[test]
    fn variables_equality() {
        let v1 = Variables::from(vec![(&A, 3), (&X, 5)]);
        let v2 = Variables::from(vec![(&X, 5), (&A, 3)]);

        assert_eq!(v1, v2);
    }

    #[test]
    fn variables_ordering() {
        let v1 = Variables::from(vec![(&X, 2), (&Y, 1), (&Z, 3)]);
        let v2 = Variables::from(vec![&X, &Y, &Z]);
        let v3 = Variables::from((&X, 10));
        let v4 = Variables::new();

        assert!(v3 > v1);
        assert!(v1 > v2);
        assert!(v2 > v4);
    }

    #[test]
    fn variables_operations() {
        let v1 = Variables::from(vec![&X, &Y, &Z]);
        let v2 = Variables::from(vec![&X, &Y]);
        let v3 = Variables::from((&X, 2));
        let v4 = Variables::from((&Y, 2));

        assert_eq!((v1.clone() * v2).to_string(), "x^2 y^2 z");
        assert_eq!((v1 * v3.clone()).to_string(), "x^3 y z");
        assert_eq!((v4.clone() * v3.clone()).to_string(), "x^2 y^2");
        assert_eq!((v3.clone() / v4.clone()).to_string(), "x^2 y^-2");
        assert_eq!((v4 / v3).to_string(), "x^-2 y^2");
    }
}
