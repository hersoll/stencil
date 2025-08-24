use std::fmt::Display;

use crate::backend::math_utils::simplified_fraction;

// This limits all numbers to 6 decimals.
const DECIMAL_FACTOR: i32 = 1_000_000;
pub const PI: Number = Number::Irrational(std::f64::consts::PI, "pi");
pub const E: Number = Number::Irrational(std::f64::consts::E, "e");

#[derive(Debug)]
pub enum Number {
    Integer(i32),
    Decimal(i32),
    Fraction(i32, i32),
    Irrational(f64, &'static str),
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}
impl From<(i32, i32)> for Number {
    fn from(value: (i32, i32)) -> Self {
        Self::Fraction(value.0, value.1)
    }
}
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Decimal((value * DECIMAL_FACTOR as f64).round() as i32)
    }
}
impl From<(f64, &'static str)> for Number {
    fn from(value: (f64, &'static str)) -> Self {
        Self::Irrational(value.0, value.1)
    }
}

impl Number {
    fn value(&self) -> f64 {
        match self {
            Number::Integer(val) => *val as f64,
            Number::Decimal(val) => *val as f64 / DECIMAL_FACTOR as f64,
            Number::Fraction(num, denom) => *num as f64 / *denom as f64,
            Number::Irrational(val, _) => *val,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(val) => write!(f, "{val}"),
            Number::Decimal(val) => write!(f, "num(\"{}\")", *val as f64 / DECIMAL_FACTOR as f64),
            Number::Fraction(num, denom) => write!(f, "{num}/{denom}"),
            Number::Irrational(_, id) => write!(f, "{id}"),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl std::ops::Add<&Number> for &Number {
    type Output = Number;
    fn add(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l + r),
            (Number::Integer(l), Number::Fraction(num, denom)) => {
                Number::Fraction(num + l * denom, *denom)
            }
            (Number::Fraction(num, denom), Number::Integer(r)) => {
                Number::Fraction(num + r * denom, *denom)
            }
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                let frac =
                    simplified_fraction(l_num * r_denom + r_num * l_denom, l_denom * r_denom);
                Number::Fraction(frac.0, frac.1)
            }
            (l_val, r_val) => Number::from(l_val.value() + r_val.value()),
        }
    }
}

impl std::ops::Add<&Number> for Number {
    type Output = Number;
    fn add(self, rhs: &Number) -> Self::Output {
        &self + rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_creation_and_display() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();
        let irrational = PI;

        assert_eq!(format!("{integer}"), "3");
        assert_eq!(format!("{decimal}"), "num(\"1.2\")");
        assert_eq!(format!("{fraction}"), "3/4");
        assert_eq!(format!("{irrational}"), "pi");
    }

    #[test]
    fn number_addition() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((&integer + &integer).to_string(), "6");
        assert_eq!((&integer + &decimal).to_string(), "num(\"4.2\")");
        assert_eq!((&integer + &fraction).to_string(), "15/4");
        assert_eq!((&decimal + &fraction).to_string(), "num(\"1.95\")");
        assert_eq!((&PI + &integer).to_string(), "num(\"6.141593\")");
    }
}
