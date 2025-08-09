use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Debug, Copy)]
pub struct Variable {
    pub symbol: char,
    pub exponent: i32,
}
impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.exponent == 1 {
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

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
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

#[derive(Debug, Clone)]
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
        for var in &self.list {
            write!(f, "{var} ")?;
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
        Self {
            list: list.into_iter().map(|v| v.into()).collect(),
        }
    }
}
impl PartialEq for Variables {
    fn eq(&self, other: &Self) -> bool {
        let set1: HashSet<char> = self.list.iter().map(|v| &v.symbol).cloned().collect();
        let set2: HashSet<char> = other.list.iter().map(|v| &v.symbol).cloned().collect();
        set1 == set2
    }
}
impl std::ops::Mul for Variables {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut final_variables = self.list.clone();
        for var in rhs.list {
            match final_variables.iter().position(|v| v == &var) {
                Some(index) => final_variables[index].exponent += var.exponent,
                None => final_variables.push(var),
            }
        }
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
            match final_variables.iter().position(|v| v == &var) {
                Some(index) => final_variables[index].exponent -= var.exponent,
                None => final_variables.push(-var),
            }
        }
        Self {
            list: final_variables,
        }
    }
}
