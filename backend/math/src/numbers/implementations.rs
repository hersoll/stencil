use crate::{HasCoef, get_decimal_divisor};

use super::Number;
use std::fmt::Display;

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() && self.value() >= 0.0 {
            write!(f, "+")?;
        }
        match self {
            Number::Integer(int) => {
                if let Some(decimals) = f.precision() {
                    write!(f, "num(\"{int:.*}\")", decimals)
                } else if *int >= 1000 {
                    write!(f, "num(\"{int}\")")
                } else {
                    write!(f, "{int}")
                }
            }
            Number::Decimal { integer, decimals } => {
                if let Some(decimals) = f.precision() {
                    // Write with the given number of decimals
                    write!(f, "num(\"{:.*}\")", decimals, self.value())
                }
                // The decimal value is actually an integer, don't include decimals
                else if self.is_integer() {
                    write!(f, "{}", *integer / 10i32.pow(*decimals as u32))
                } else {
                    write!(f, "num(\"{}\")", self.value())
                }
            }
            Number::Fraction {
                numerator,
                denominator,
            } => write!(f, "{numerator}/{denominator}"),
            Number::Irrational { symbol, .. } => write!(f, "{symbol}"),
        }
    }
}

/// These implementations let us do `1.into()` or `(1, 3).into()`.
/// Improves ergonomics like `Term::from_num_and_vars(3.5, X)`;
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}
impl From<(i32, i32)> for Number {
    fn from(value: (i32, i32)) -> Self {
        Self::Fraction {
            numerator: value.0,
            denominator: value.1,
        }
    }
}

/// Prefer `Number::decimal_from_f64()` whenever possible
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::decimal_from_f64(value, 3)
    }
}
impl From<(f64, &'static str)> for Number {
    fn from(value: (f64, &'static str)) -> Self {
        Self::Irrational {
            value: value.0,
            symbol: value.1,
        }
    }
}

impl PartialEq<Self> for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialEq<f64> for Number {
    fn eq(&self, other: &f64) -> bool {
        self.value() == *other
    }
}

impl PartialEq<Number> for f64 {
    fn eq(&self, other: &Number) -> bool {
        other.value() == *self
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Number::Integer(val) => val == other,
            Number::Decimal { integer, decimals } => {
                *integer == other * get_decimal_divisor(*decimals)
            }
            Number::Fraction {
                numerator,
                denominator,
            } => *numerator == other * denominator,
            Number::Irrational { .. } => false,
        }
    }
}

impl PartialEq<Number> for i32 {
    fn eq(&self, other: &Number) -> bool {
        match other {
            Number::Integer(val) => val == self,
            Number::Decimal { integer, decimals } => {
                *integer == self * get_decimal_divisor(*decimals)
            }
            Number::Fraction {
                numerator,
                denominator,
            } => *numerator == self * denominator,
            Number::Irrational { .. } => false,
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value()
            .partial_cmp(&other.value())
            .unwrap_or_else(|| panic!("Cannot compare NaN values"))
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<f64> for Number {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(other)
    }
}

impl PartialOrd<i32> for Number {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        match self {
            Number::Integer(i) => i.partial_cmp(other),
            Number::Decimal { integer, decimals } => {
                integer.partial_cmp(&(other * get_decimal_divisor(*decimals)))
            }
            Number::Fraction {
                numerator,
                denominator,
            } => numerator.partial_cmp(&(denominator * other)),
            Number::Irrational { value, .. } => value.partial_cmp(&(*other as f64)),
        }
    }
}

impl Eq for Number {}

impl HasCoef for Number {
    fn coef(&self) -> Number {
        *self
    }
}

#[cfg(test)]
mod tests {
    use crate::PI;

    use super::*;

    #[test]
    fn creation_and_display() {
        let integer: Number = 3.into();
        let decimal: Number = 1.39.into();
        let fraction: Number = (3, 4).into();
        let irrational = PI;
        let negative: Number = (-2).into();

        assert_eq!(format!("{integer}"), "3");
        assert_eq!(format!("{integer:+}"), "+3");
        assert_eq!(format!("{decimal}"), "num(\"1.39\")");
        assert_eq!(format!("{decimal:.3}"), "num(\"1.390\")");
        assert_eq!(format!("{decimal:.1}"), "num(\"1.4\")");
        assert_eq!(format!("{decimal:+}"), "+num(\"1.39\")");
        assert_eq!(format!("{fraction}"), "3/4");
        assert_eq!(format!("{fraction:+}"), "+3/4");
        assert_eq!(format!("{irrational}"), "pi");
        assert_eq!(format!("{irrational:+}"), "+pi");
        assert_eq!(format!("{negative}"), "-2");
        assert_eq!(format!("{negative:+}"), "-2");
    }

    #[test]
    fn comparison() {
        let integer = Number::Integer(3);
        let decimal_lower = Number::decimal_from_f64(2.9, 1);
        let decimal_higher = Number::decimal_from_f64(3.1, 1);
        let fraction_lowest = Number::from((8, 3));
        let fraction_highest = Number::from((10, 3));
        let actual_integer = 3i32;

        assert!(integer > decimal_lower);
        assert!(integer < decimal_higher);
        assert!(integer > fraction_lowest);
        assert!(integer < fraction_highest);
        assert!(decimal_lower > fraction_lowest);
        assert!(decimal_higher < fraction_highest);
        assert!(decimal_lower < actual_integer);
        assert!(decimal_higher > actual_integer);
        assert!(fraction_highest > actual_integer);
        assert!(fraction_lowest < actual_integer);
        assert_eq!(integer, actual_integer);
    }
}
