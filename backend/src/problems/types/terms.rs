use crate::problems::{IntRange, Number, types::variables::Variable, types::variables::Variables};
use num_traits::Pow;
use num_traits::Zero;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Term {
    pub coefficient: Number,
    pub variables: Variables,
    pub colored: bool,
}

impl Term {
    pub fn abs(&self) -> Self {
        Self {
            coefficient: self.coefficient.abs(),
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }

    pub fn evaluate(&self, replacements: &Vec<(char, Number)>) -> Number {
        let mut result = self.coefficient.clone();
        self.variables.list.iter().for_each(|v| {
            match replacements.iter().find(|pair| pair.0 == v.symbol) {
                Some(pair) => result *= pair.1.value().pow(v.exponent as i32).into(),
                None => panic!("Variable {v} not in replacements {replacements:#?}. (Panic should not be reached if called from polynomial.evaluate())"),
            }
        });
        result
    }

    pub fn assert_one_positive(term1: &mut Term, term2: &mut Term) {
        if *term1 < Term::zero() && *term2 < Term::zero() {
            let random = IntRange::with_zero(0, 1).unwrap().random();
            if random == 0 {
                *term1 = -term1.clone();
            } else {
                *term2 = -term2.clone();
            }
        }
    }
}
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.colored && self.coefficient != 0.into() {
            write!(f, " colored(")?;
        }
        if f.sign_plus() && self.coefficient.value() > 0.0 {
            write!(f, "+")?;
        }

        if self.coefficient.value() == 1.0 {
            if self.variables.list.is_empty() {
                write!(f, "1")?;
            } else {
                write!(f, "{}", self.variables)?;
            }
        } else if self.coefficient.value() == -1.0 {
            if self.variables.list.is_empty() {
                write!(f, "-1")?;
            } else {
                write!(f, "-{}", self.variables)?;
            }
        } else if self.coefficient.value() == 0.0 {
            write!(f, "")?;
        } else {
            match self.coefficient {
                Number::Integer(_) | Number::Decimal(_) => {
                    write!(f, "{}{}", self.coefficient, self.variables)?
                }
                Number::Fraction(num, denom) => write!(
                    f,
                    "({}{})/{denom}",
                    if num != 1 {
                        num.to_string()
                    } else {
                        String::new()
                    },
                    self.variables
                )?,
                Number::Irrational(_, s) => write!(f, "{s} {}", self.variables)?,
            };
        }
        if self.colored && self.coefficient != 0.into() {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Self {
            coefficient: value.into(),
            variables: Variables::new(),
            colored: false,
        }
    }
}

impl From<(i32, i32)> for Term {
    fn from(value: (i32, i32)) -> Self {
        Self {
            coefficient: value.into(),
            variables: Variables::new(),
            colored: false,
        }
    }
}

impl From<f64> for Term {
    fn from(value: f64) -> Self {
        Self {
            coefficient: value.into(),
            variables: Variables::new(),
            colored: false,
        }
    }
}

impl From<(char, i32)> for Term {
    fn from(value: (char, i32)) -> Self {
        Self {
            coefficient: 1.into(),
            variables: Variables::from((value.0, value.1)),
            colored: false,
        }
    }
}

impl From<char> for Term {
    fn from(value: char) -> Self {
        let variable: Variable = value.into();
        Self {
            coefficient: 1.into(),
            variables: Variables::from(variable),
            colored: false,
        }
    }
}

impl<T> From<(T, Variables)> for Term
where
    T: Into<Number>,
{
    fn from(value: (T, Variables)) -> Self {
        Term {
            coefficient: value.0.into(),
            variables: value.1,
            colored: false,
        }
    }
}

impl<T, U> From<(T, U)> for Term
where
    T: Into<Number>,
    U: Into<Variable>,
{
    fn from(value: (T, U)) -> Self {
        Self {
            coefficient: value.0.into(),
            variables: Variables::from(value.1),
            colored: false,
        }
    }
}

// NOTE: The Ord/Eq is not meant for sorting or anything like that.
// It's primarily to determine whether a Term is negative or positive for the typst_formatting::step_...

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.coefficient.cmp(&other.coefficient)
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.coefficient == other.coefficient
    }
}
impl Eq for Term {}

impl Zero for Term {
    fn zero() -> Self {
        Self {
            coefficient: 0.into(),
            colored: false,
            variables: Variables::new(),
        }
    }
    fn is_zero(&self) -> bool {
        self.coefficient == 0.into()
    }
}

impl std::ops::Neg for Term {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            coefficient: -self.coefficient,
            variables: self.variables,
            colored: self.colored,
        }
    }
}

impl std::ops::Neg for &Term {
    type Output = Term;
    fn neg(self) -> Self::Output {
        Term {
            coefficient: -self.coefficient.clone(),
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }
}
// Do not add terms manually, only meant to be used inside Expression implementation
impl std::ops::Add for Term {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.variables, rhs.variables);
        Self {
            coefficient: self.coefficient + &rhs.coefficient,
            variables: self.variables,
            colored: self.colored,
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
            coefficient: self.coefficient - &rhs.coefficient,
            variables: self.variables,
            colored: self.colored,
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
            coefficient: self.coefficient * &rhs.coefficient,
            variables: self.variables * rhs.variables,
            colored: self.colored,
        }
    }
}
impl std::ops::MulAssign for Term {
    fn mul_assign(&mut self, rhs: Self) {
        self.coefficient = &self.coefficient * &rhs.coefficient;
        self.variables = self.variables.clone() * rhs.variables;
    }
}
impl std::ops::Mul<Term> for i32 {
    type Output = Term;
    fn mul(self, rhs: Term) -> Self::Output {
        Term {
            coefficient: rhs.coefficient * &self.into(),
            variables: rhs.variables.clone(),
            colored: rhs.colored,
        }
    }
}
impl std::ops::Mul<i32> for Term {
    type Output = Term;
    fn mul(self, rhs: i32) -> Self::Output {
        Term {
            coefficient: self.coefficient * &rhs.into(),
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }
}
impl std::ops::MulAssign<i32> for Term {
    fn mul_assign(&mut self, rhs: i32) {
        self.coefficient *= rhs.into();
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::PI;

    use super::*;

    #[test]
    fn term_creation() {
        let t1: Term = (3, 'x').into();
        let t2: Term = ('x', 3).into();
        let t3: Term = 6.into();

        assert_eq!(t1.to_string(), "3x");
        assert_eq!(t2.to_string(), "x^3");
        assert_eq!(t3.to_string(), "6");
    }

    #[test]
    fn term_displays() {
        let t_a: Term = (1, 'a').into();
        let t_one: Term = 1.into();
        let t_m_one: Term = (-1).into();
        let t_zero: Term = 0.into();
        let mut t_color: Term = (-3, 'x').into();
        let fractional_term: Term = ((3, 5), 'x').into();
        t_color.colored = true;
        assert_eq!(format!("{t_a}"), "a");
        assert_eq!(format!("{t_a:+}"), "+a");
        assert_eq!(format!("{t_one}"), "1");
        assert_eq!(format!("{t_one:+}"), "+1");
        assert_eq!(format!("{t_m_one}"), "-1");
        assert_eq!(format!("{t_m_one:+}"), "-1");
        assert_eq!(format!("{t_zero}"), "");
        assert_eq!(format!("{t_zero:+}"), "");
        assert_eq!(format!("{t_color}"), " colored(-3x)");
        assert_eq!(format!("{fractional_term}"), "(3x)/5");
    }

    #[test]
    fn term_addition() {
        let t1: Term = (3, ('x', 2)).into();
        let t2: Term = (2, ('x', 2)).into();
        assert_eq!((t1 + t2).to_string(), "5x^2");
        // += assignment
        let mut t3: Term = ('x', 4).into();
        let t4: Term = (4, ('x', 4)).into();
        t3 += t4.clone();
        assert_eq!(t3.to_string(), "5x^4");

        let t5: Term = ((2, 3), ('x', 4)).into();
        assert_eq!((t4 + t5).to_string(), "(14x^4)/3");

        let t6: Term = (1.3, 'x').into();
        let t7: Term = (PI, 'x').into();
        assert_eq!((t6 + t7).to_string(), "num(\"4.442\")x");
    }

    #[test]
    #[should_panic]
    fn cant_add_different_terms() {
        let t1: Term = (3, ('x', 2)).into();
        let t2: Term = (2, 'x').into();
        assert_eq!((t1 + t2).to_string(), "throws");
    }

    #[test]
    fn term_subtraction() {
        let t1: Term = (3, ('x', 2)).into();
        let t2: Term = (2, ('x', 2)).into();
        assert_eq!((t1 - t2).to_string(), "x^2");
        // -= assignment
        let mut t3: Term = ('x', 4).into();
        let t4: Term = (4, ('x', 4)).into();
        t3 -= t4;
        assert_eq!(t3.to_string(), "-3x^4");
    }

    #[test]
    #[should_panic]
    fn cant_subtract_different_terms() {
        let t1: Term = (3, ('x', 2)).into();
        let t2: Term = (2, 'x').into();
        assert_eq!((t1 - t2).to_string(), "throws");
    }

    #[test]
    fn term_multiplication() {
        // Term and number
        let t1: Term = (12, ('x', 4)).into();
        let factor = 3;
        assert_eq!((factor * t1.clone()).to_string(), "36x^4");
        assert_eq!((t1 * factor).to_string(), "36x^4");
        // Term and term
        let mut t2: Term = (3, 'x').into();
        let t3: Term = (2, ('x', 2)).into();
        let t4: Term = ('a', 2).into();
        let t5: Term = 'y'.into();
        assert_eq!((t2.clone() * t3.clone()).to_string(), "6x^3");
        assert_eq!((t2.clone() * t4.clone()).to_string(), "3a^2 x");
        assert_eq!((t2.clone() * t5.clone()).to_string(), "3x y");
        t2 *= t3.clone();
        assert_eq!(t2.to_string(), "6x^3");
        assert_eq!((t2 * t3 * t4 * t5).to_string(), "12a^2 x^5 y");
    }
}
