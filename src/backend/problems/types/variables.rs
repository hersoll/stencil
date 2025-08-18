use std::{cmp::Ordering, collections::HashSet, fmt::Display};

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd)]
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

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.symbol.cmp(&other.symbol)
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
            Ordering::Greater
        } else if other.list.len() == 0 {
            Ordering::Less
        } else {
            let total_exponent_first: i32 = self.list.iter().map(|v| v.exponent).sum();
            let total_exponent_second: i32 = other.list.iter().map(|v| v.exponent).sum();

            total_exponent_second
                .cmp(&total_exponent_first)
                .then_with(|| self.list.cmp(&other.list))
        }
    }
}

impl PartialOrd for Variables {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
