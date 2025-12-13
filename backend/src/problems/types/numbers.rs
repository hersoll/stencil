use std::fmt::Display;

use crate::problems::math_utils::simplified_fraction;

// This limits all numbers to 3 decimals.
const DECIMAL_FACTOR: i32 = 1_000;
pub const PI: Number = Number::Irrational(std::f64::consts::PI, "pi");
pub const E: Number = Number::Irrational(std::f64::consts::E, "e");

#[derive(Debug, PartialOrd, Clone)]
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
    pub fn value(&self) -> f64 {
        match self {
            Number::Integer(val) => *val as f64,
            Number::Decimal(val) => *val as f64 / DECIMAL_FACTOR as f64,
            Number::Fraction(num, denom) => *num as f64 / *denom as f64,
            Number::Irrational(val, _) => *val,
        }
    }

    fn simplify(self) -> Number {
        match self {
            Number::Fraction(num, denom) => {
                let (s_num, s_denom) = simplified_fraction(num, denom);
                if s_num % s_denom == 0 {
                    Number::Integer(s_num / s_denom)
                } else {
                    Number::Fraction(s_num, s_denom)
                }
            }
            n => n,
        }
    }

    pub fn abs(&self) -> Number {
        match self {
            Number::Integer(val) => Number::Integer(val.abs()),
            Number::Decimal(val) => Number::Decimal(val.abs()),
            Number::Fraction(num, denom) => Number::Fraction(num.abs(), denom.abs()),
            Number::Irrational(val, s) => Number::Irrational(val.abs(), s),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() && self.value() >= 0.0 {
            write!(f, "+")?;
        }
        match self {
            Number::Integer(val) => write!(f, "{val}"),
            Number::Decimal(val) => {
                if val % DECIMAL_FACTOR == 0 {
                    write!(f, "{}", *val / DECIMAL_FACTOR)
                } else {
                    write!(f, "num(\"{}\")", *val as f64 / DECIMAL_FACTOR as f64)
                }
            }
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

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value()
            .partial_cmp(&other.value())
            .unwrap_or_else(|| panic!("Cannot compare NaN values"))
    }
}

impl Eq for Number {}

impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(val) => Self::Integer(-val),
            Self::Decimal(val) => Self::Decimal(-val),
            Self::Fraction(num, denom) => Self::Fraction(-num, denom),
            Self::Irrational(val, s) => Self::Irrational(-val, s),
        }
    }
}

impl std::ops::Add<&Number> for &Number {
    type Output = Number;
    fn add(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l + r),
            (Number::Integer(l), Number::Fraction(num, denom)) => {
                Number::Fraction(num + l * denom, *denom).simplify()
            }
            (Number::Fraction(num, denom), Number::Integer(r)) => {
                Number::Fraction(num + r * denom, *denom).simplify()
            }
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                let frac =
                    simplified_fraction(l_num * r_denom + r_num * l_denom, l_denom * r_denom);
                Number::Fraction(frac.0, frac.1).simplify()
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

impl std::ops::AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + &rhs;
    }
}

impl std::ops::Sub<&Number> for &Number {
    type Output = Number;
    fn sub(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l - r),
            (Number::Integer(l), Number::Fraction(num, denom)) => {
                Number::Fraction(l * denom - num, *denom).simplify()
            }
            (Number::Fraction(num, denom), Number::Integer(r)) => {
                Number::Fraction(num - r * denom, *denom).simplify()
            }
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                let frac =
                    simplified_fraction(l_num * r_denom - r_num * l_denom, l_denom * r_denom);
                Number::Fraction(frac.0, frac.1).simplify()
            }
            (l_val, r_val) => Number::from(l_val.value() - r_val.value()),
        }
    }
}

impl std::ops::Sub<&Number> for Number {
    type Output = Number;
    fn sub(self, rhs: &Number) -> Self::Output {
        &self - rhs
    }
}

impl std::ops::SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - &rhs;
    }
}

impl std::ops::Mul<&Number> for &Number {
    type Output = Number;
    fn mul(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l * r),
            (Number::Integer(l), Number::Fraction(num, denom)) => {
                Number::Fraction(l * num, *denom).simplify()
            }
            (Number::Fraction(num, denom), Number::Integer(r)) => {
                Number::Fraction(num * r, *denom).simplify()
            }
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                let frac = simplified_fraction(l_num * r_num, l_denom * r_denom);
                Number::Fraction(frac.0, frac.1).simplify()
            }
            (l_val, r_val) => Number::from(l_val.value() * r_val.value()),
        }
    }
}

impl std::ops::Mul<&Number> for Number {
    type Output = Number;
    fn mul(self, rhs: &Number) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * &rhs;
    }
}

impl std::ops::Div<&Number> for &Number {
    type Output = Number;
    fn div(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Fraction(*l, *r).simplify(),
            (Number::Integer(l), Number::Fraction(num, denom)) => {
                Number::Fraction(l * denom, *num).simplify()
            }
            (Number::Fraction(num, denom), Number::Integer(r)) => {
                Number::Fraction(*num, *denom * r).simplify()
            }
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                let frac = simplified_fraction(l_num * r_denom, l_denom * r_num);
                Number::Fraction(frac.0, frac.1).simplify()
            }
            (l_val, r_val) => Number::from(l_val.value() / r_val.value()),
        }
    }
}

impl std::ops::Div<&Number> for Number {
    type Output = Number;
    fn div(self, rhs: &Number) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / &rhs;
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
        let negative: Number = (-2).into();

        assert_eq!(format!("{integer}"), "3");
        assert_eq!(format!("{integer:+}"), "+3");
        assert_eq!(format!("{decimal}"), "num(\"1.2\")");
        assert_eq!(format!("{decimal:+}"), "+num(\"1.2\")");
        assert_eq!(format!("{fraction}"), "3/4");
        assert_eq!(format!("{fraction:+}"), "+3/4");
        assert_eq!(format!("{irrational}"), "pi");
        assert_eq!(format!("{irrational:+}"), "+pi");
        assert_eq!(format!("{negative}"), "-2");
        assert_eq!(format!("{negative:+}"), "-2");
    }

    #[test]
    fn number_addition() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let decimal_2: Number = 1.8.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((&integer + &integer).to_string(), "6");
        assert_eq!((&integer + &decimal).to_string(), "num(\"4.2\")");
        assert_eq!((&integer + &fraction).to_string(), "15/4");
        assert_eq!((&decimal + &fraction).to_string(), "num(\"1.95\")");
        assert_eq!((&PI + &integer).to_string(), "num(\"6.142\")");
        assert_eq!((&decimal + &decimal_2).to_string(), "3");
    }

    #[test]
    fn number_subtraction() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((&integer - &integer).to_string(), "0");
        assert_eq!((&integer - &decimal).to_string(), "num(\"1.8\")");
        assert_eq!((&integer - &fraction).to_string(), "9/4");
        assert_eq!((&decimal - &fraction).to_string(), "num(\"0.45\")");
        assert_eq!((&PI - &integer).to_string(), "num(\"0.142\")");
    }

    #[test]
    fn number_multiplication() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((&integer * &integer).to_string(), "9");
        assert_eq!((&integer * &decimal).to_string(), "num(\"3.6\")");
        assert_eq!((&integer * &fraction).to_string(), "9/4");
        assert_eq!((&decimal * &fraction).to_string(), "num(\"0.9\")");
        assert_eq!((&PI * &integer).to_string(), "num(\"9.425\")");
    }

    #[test]
    fn number_division() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((&integer / &integer).to_string(), "1");
        assert_eq!((&integer / &decimal).to_string(), "num(\"2.5\")");
        assert_eq!((&integer / &fraction).to_string(), "4");
        assert_eq!((&decimal / &fraction).to_string(), "num(\"1.6\")");
        assert_eq!((&PI / &integer).to_string(), "num(\"1.047\")");
    }
}
