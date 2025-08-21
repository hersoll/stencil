use std::{cmp::Ordering, collections::HashSet, fmt::Display};

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd)]
pub struct Variable {
    pub symbol: char,
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
impl From<char> for Variable {
    fn from(value: char) -> Self {
        Self {
            symbol: value,
            exponent: 1,
        }
    }
}
impl From<(char, i32)> for Variable {
    fn from(value: (char, i32)) -> Self {
        Self {
            symbol: value.0,
            exponent: value.1,
        }
    }
}

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        other.symbol.cmp(&self.symbol)
    }
}
impl std::ops::Neg for Variable {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            symbol: self.symbol,
            exponent: -self.exponent,
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Variables {
    pub list: Vec<Variable>,
}

impl Variables {
    pub fn new() -> Self {
        Self { list: Vec::new() }
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
        let mut variables: Vec<Variable> = list.into_iter().map(|v| v.into()).collect();
        variables.sort_by_key(|v| v.symbol);
        Self { list: variables }
    }
}
impl PartialEq for Variables {
    fn eq(&self, other: &Self) -> bool {
        let set1: HashSet<(char, i32)> = self.list.iter().map(|v| (v.symbol, v.exponent)).collect();
        let set2: HashSet<(char, i32)> =
            other.list.iter().map(|v| (v.symbol, v.exponent)).collect();
        set1 == set2
    }
}
impl std::ops::Mul for Variables {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut final_variables = self.list.clone();
        for var in rhs.list {
            match final_variables.iter().position(|v| v.symbol == var.symbol) {
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
        let mut final_variables = self.list.clone();
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
        if self.list.len() == 0 && other.list.len() == 0 {
            Ordering::Equal
        } else if self.list.len() == 0 {
            Ordering::Less
        } else if other.list.len() == 0 {
            Ordering::Greater
        } else {
            let total_exponent_first: i32 = self.list.iter().map(|v| v.exponent).sum();
            let total_exponent_second: i32 = other.list.iter().map(|v| v.exponent).sum();

            total_exponent_first
                .cmp(&total_exponent_second)
                .then_with(|| self.list.cmp(&other.list))
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

    #[test]
    fn variable_creations() {
        let v1: Variable = 'x'.into();
        let v2: Variable = ('x', 2).into();
        let v_const: Variable = ('a', 0).into();
        assert_eq!(v1.exponent, 1);
        assert_eq!(v1.symbol, 'x');
        assert_eq!(v1.to_string(), "x");
        assert_eq!(v2.to_string(), "x^2");
        assert_eq!(v_const.to_string(), "");
    }

    #[test]
    fn variable_ordering() {
        let v1: Variable = 'a'.into();
        let v2: Variable = 'b'.into();
        let v3: Variable = 'x'.into();
        let v4: Variable = 'x'.into();
        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
        assert!(v3 == v4);
    }

    #[test]
    fn variable_negation() {
        let v1: Variable = ('x', 2).into();
        let v2: Variable = ('a', -3).into();
        assert_eq!((-v1).to_string(), "x^-2");
        assert_eq!((-v2).to_string(), "a^3");
    }

    #[test]
    fn variables_creation() {
        let v1: Variables = 'x'.into();
        let v2: Variables = vec!['x', 'y'].into();
        let v3: Variables = vec![('x', 3), ('y', 5)].into();

        assert_eq!(v1.to_string(), "x");
        assert_eq!(v2.to_string(), "x y");
        assert_eq!(v3.to_string(), "x^3 y^5");
    }

    #[test]
    fn variables_equality() {
        let v1: Variables = vec![('a', 3), ('x', 5)].into();
        let v2: Variables = vec![('x', 5), ('a', 3)].into();

        assert_eq!(v1, v2);
    }

    #[test]
    fn variables_ordering() {
        let v1: Variables = vec![('x', 2), ('y', 1), ('z', 3)].into();
        let v2: Variables = vec!['x', 'y', 'z'].into();
        let v3: Variables = ('x', 10).into();
        let v4 = Variables::new();

        assert!(v3 > v1);
        assert!(v1 > v2);
        assert!(v2 > v4);
    }

    #[test]
    fn variables_operations() {
        let v1: Variables = vec!['x', 'y', 'z'].into();
        let v2: Variables = vec!['x', 'y'].into();
        let v3: Variables = ('x', 2).into();
        let v4: Variables = ('y', 2).into();

        assert_eq!((v1.clone() * v2.clone()).to_string(), "x^2 y^2 z");
        assert_eq!((v1.clone() * v3.clone()).to_string(), "x^3 y z");
        assert_eq!((v4.clone() * v3.clone()).to_string(), "x^2 y^2");
        assert_eq!((v3.clone() / v4.clone()).to_string(), "x^2 y^-2");
        assert_eq!((v4.clone() / v3.clone()).to_string(), "x^-2 y^2");
    }
}
