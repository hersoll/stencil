use crate::math::utils::simplified_fraction;
/// The numbers module handles calculations between different types of numbers
/// (Integers, Decimals, Fractions, Irrationals) and formats them for Typst.
///
/// The main point was to handle decimal numbers (Rust doesn't even have a round(3) method),
/// but since fractions also need to be formatted it became suitable to handle it all in one place.
///
/// The Number::Irrational variant is used when their values are actually needed for calcuations,
/// otherwise you're better of just treating pi as a variable in the problem.
use std::fmt::Display;

// This limits all numbers to 3 decimals.
const DECIMAL_FACTOR: i32 = 1_000;
pub const PI: Number = Number::Irrational(std::f64::consts::PI, "pi");
pub const E: Number = Number::Irrational(std::f64::consts::E, "e");
pub const ZERO: Number = Number::Integer(0);

/// The Number enum is used to properly display numbers in Typst while
/// still being able to do calculations.
/// Note that decimal numbers are limited to display (and use) 3 decimals.
#[derive(Debug, Clone, Copy)]
pub enum Number {
    Integer(i32),
    /// The decimal value multiplied by DECIMAL_FACTOR (1 000)
    Decimal(i32),
    Fraction(i32, i32),
    Irrational(f64, &'static str),
}

/// These implementations lets us do 1.into() or (1, 3).into(),
/// but calling the variant, like Number::Fraction(1, 3), is preferred.
///
/// Note that the signature is different for Number::Decimal(1300) and 1.3.into().
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
    /// Calling value() is useful even for integers, since it lets us do things like
    /// num.value().pow(-2), which will be a float.
    pub fn value(&self) -> f64 {
        match self {
            Number::Integer(val) => *val as f64,
            Number::Decimal(val) => *val as f64 / DECIMAL_FACTOR as f64,
            Number::Fraction(num, denom) => *num as f64 / *denom as f64,
            Number::Irrational(val, _) => *val,
        }
    }

    /// If the Number is a Fraction, simplifies it (to an Integer if possible)
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

    /// Inside graph strings we need actual numbers, decimals can't be output
    /// as num("1.2"), like they normally do in Display. This function accounts for that.
    ///
    /// Is (probably) only used in Graph.to_typst()
    pub fn for_graphs(&self) -> String {
        match self {
            Number::Decimal(_) => strip_num(format!("{self}")),
            _ => format!("{self}"),
        }
    }
}

fn strip_num(s: String) -> String {
    match s
        .strip_prefix("num(\"")
        .and_then(|strip| strip.strip_suffix("\")"))
    {
        Some(stripped) => stripped.to_string(),
        None => s,
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() && self.value() >= 0.0 {
            write!(f, "+")?;
        }
        match self {
            Number::Integer(int) => write!(f, "{int}"),
            Number::Decimal(large_val) => {
                // The decimal value is actually an integer
                if large_val % DECIMAL_FACTOR == 0 {
                    write!(f, "{}", *large_val / DECIMAL_FACTOR)
                } else {
                    // num() is a formatting library which outputs the decimals with commas
                    write!(f, "num(\"{}\")", *large_val as f64 / DECIMAL_FACTOR as f64)
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

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
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

impl std::ops::Add<i32> for Number {
    type Output = Number;
    fn add(self, rhs: i32) -> Self::Output {
        self + &Number::Integer(rhs)
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

impl std::ops::Sub<i32> for Number {
    type Output = Number;
    fn sub(self, rhs: i32) -> Self::Output {
        self - &Number::Integer(rhs)
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

impl std::ops::Mul<i32> for Number {
    type Output = Number;
    fn mul(self, rhs: i32) -> Self::Output {
        self * &Number::Integer(rhs)
    }
}
impl std::ops::Mul<f64> for &Number {
    type Output = Number;
    fn mul(self, rhs: f64) -> Self::Output {
        self * &rhs.into()
    }
}

impl std::ops::Mul<f64> for Number {
    type Output = Number;
    fn mul(self, rhs: f64) -> Self::Output {
        self * &rhs.into()
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
        &self / rhs
    }
}

impl std::ops::Div<i32> for Number {
    type Output = Number;
    fn div(self, rhs: i32) -> Self::Output {
        self / &Number::Integer(rhs)
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
    fn creation_and_display() {
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
    fn comparison() {
        let integer = Number::Integer(3);
        let decimal_lower = Number::Decimal(2900);
        let decimal_higher = Number::Decimal(3100);
        let fraction_lowest = Number::Fraction(8, 3);
        let fraction_highest = Number::Fraction(10, 3);

        assert!(integer > decimal_lower);
        assert!(integer < decimal_higher);
        assert!(integer > fraction_lowest);
        assert!(integer < fraction_highest);
        assert!(decimal_lower > fraction_lowest);
        assert!(decimal_higher < fraction_highest);
    }

    #[test]
    fn addition() {
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
    fn subtraction() {
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
    fn multiplication() {
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
    fn division() {
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
