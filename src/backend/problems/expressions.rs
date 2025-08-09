use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Debug, Copy)]
pub struct Variable {
    symbol: char,
    exponent: i32,
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
    list: Vec<Variable>,
}

impl Variables {
    fn new() -> Self {
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

#[derive(Clone, Debug)]
pub struct Term {
    coefficient: i32,
    variables: Variables,
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

pub struct Expression {
    terms: Vec<Term>,
}

impl From<Vec<Term>> for Expression {
    fn from(value: Vec<Term>) -> Self {
        Expression { terms: value }
    }
}

impl Expression {
    fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn simplify(self) -> Self {
        let mut result = Expression::new();
        for term in self.terms {
            match result
                .terms
                .iter()
                .position(|t| t.variables == term.variables)
            {
                Some(index) => result.terms[index] += term,
                None => result.terms.push(term.clone()),
            }
        }
        result
    }
}

impl std::ops::Add for Expression {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.terms.clone();
        for term in rhs.terms {
            match self
                .terms
                .iter()
                .position(|t| t.variables == term.variables)
            {
                Some(index) => result[index] = result[index].clone() + term,
                None => result.push(term),
            }
        }
        Self { terms: result }
    }
}
impl std::ops::Sub for Expression {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.terms.clone();
        for term in rhs.terms {
            match self
                .terms
                .iter()
                .position(|t| t.variables == term.variables)
            {
                Some(index) => result[index] -= term,
                None => result.push(-term),
            }
        }
        Self { terms: result }
    }
}
impl std::ops::Mul for Expression {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Expression::new();
        for lhs_term in self.terms {
            for rhs_term in rhs.terms.clone() {
                result.terms.push(lhs_term.clone() * rhs_term);
            }
        }
        result.simplify()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first_value_written = false;
        for term in &self.terms {
            if !first_value_written {
                first_value_written = true;
                if term.coefficient == 1 {
                    write!(f, "{}", term.variables)?;
                } else if term.coefficient == -1 {
                    write!(f, "-{}", term.variables)?;
                } else if term.coefficient == 0 {
                    first_value_written = false;
                } else {
                    write!(f, "{}{}", term.coefficient, term.variables)?;
                }
            } else {
                if term.coefficient == 1 {
                    write!(f, "+{}", term.variables)?;
                } else if term.coefficient == -1 {
                    write!(f, "-{}", term.variables)?;
                } else if term.coefficient == 0 {
                } else {
                    write!(f, "{:+}{}", term.coefficient, term.variables)?;
                }
            }
        }
        Ok(())
    }
}
