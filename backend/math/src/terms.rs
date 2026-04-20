mod operations;

use crate::Polynomial;
use crate::symbols::Symbol;
use crate::{Number, Variable, Variables, num_gen};
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
    // ########## CONSTRUCTORS ###########

    pub fn from_var<T: Into<Variables>>(var: T) -> Self {
        let var = var.into();
        Self {
            coefficient: Number::Integer(1),
            variables: var,
            colored: false,
        }
    }

    pub fn from_num<T: Into<Number>>(num: T) -> Self {
        let num = num.into();
        Self {
            coefficient: num,
            variables: Variables::new(),
            colored: false,
        }
    }

    pub fn from_num_and_vars<T: Into<Number>, U: Into<Variables>>(num: T, vars: U) -> Self {
        let num = num.into();
        let vars = vars.into();
        Self {
            coefficient: num,
            variables: vars,
            colored: false,
        }
    }
    // ###################################

    /// Alias method to quickly create a `Polynomial`.
    ///
    /// Example:
    /// ```rust
    /// use math::Term;
    /// use math::symbols::X;
    /// let k_term = 3 * X;
    /// let m_term = Term::from_num(-2);
    /// let function = k_term.and(&m_term);
    /// assert_eq!(function.to_string(), String::from("3x-2"));
    /// ```
    pub fn and(&self, other: &Term) -> Polynomial {
        Polynomial::from_terms(&[self, other])
    }

    pub fn abs(&self) -> Self {
        Self {
            coefficient: self.coefficient.abs(),
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }

    pub fn evaluate(&self, replacements: &[(&str, Number)]) -> Number {
        let mut result = self.coefficient;
        self.variables.list.iter().for_each(|v| {
            match replacements.iter().find(|pair| pair.0 == v.symbol.0) {
                Some(pair) => result *= Number::from(pair.1.value().pow(v.exponent)),
                None => panic!("Variable {v} not in replacements {replacements:#?}. (Panic should not be reached if called from polynomial.evaluate())"),
            }
        });
        result
    }

    pub fn assert_one_positive(term1: &mut Term, term2: &mut Term) {
        if *term1 < Term::zero() && *term2 < Term::zero() {
            let random = num_gen::integer().range(0, 1).random();
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
        if self.colored && self.coefficient != 0 {
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
                    "{sign}({}{})/{}",
                    if num.abs() != 1 || self.variables.list.is_empty() {
                        num.abs().to_string()
                    } else {
                        String::new()
                    },
                    self.variables,
                    denom.abs(),
                    sign = if self.coefficient.value() < 0.0 {
                        "-"
                    } else {
                        ""
                    }
                )?,
                Number::Irrational(_, s) => write!(f, "{s} {}", self.variables)?,
            };
        }
        if self.colored && self.coefficient != 0 {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl From<Number> for Term {
    fn from(value: Number) -> Self {
        Self {
            coefficient: value,
            variables: Variables::new(),
            colored: false,
        }
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

impl From<(&'static Symbol, i32)> for Term {
    fn from(value: (&'static Symbol, i32)) -> Self {
        Self {
            coefficient: 1.into(),
            variables: Variables::from((value.0, value.1)),
            colored: false,
        }
    }
}

impl From<&'static Symbol> for Term {
    fn from(value: &'static Symbol) -> Self {
        Self {
            coefficient: 1.into(),
            variables: Variables::from(value),
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
        self.coefficient == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    static X: Symbol = Symbol("x");
    static A: Symbol = Symbol("a");

    #[test]
    fn term_creation() {
        let t1 = 3 * Term::from_var(&X);
        let t2 = Term::from_var((&X, 3));
        let t3 = Term::from_num(6);

        assert_eq!(t1.to_string(), "3x");
        assert_eq!(t2.to_string(), "x^3");
        assert_eq!(t3.to_string(), "6");
    }

    #[test]
    fn term_displays() {
        let t_a = Term::from_var(&A);
        let t_one = Term::from_num(1);
        let t_m_one = Term::from_num(-1);
        let t_zero = Term::from_num(0);
        let mut t_color = -3 * Term::from_var(&X);
        let fractional_term = Term::from_num_and_vars((3, 5), &X);
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
}
