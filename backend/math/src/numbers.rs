/// The numbers module handles calculations between different types of numbers
/// (Integers, Decimals, Fractions, Irrationals) and formats them for Typst.
///
/// The main point was to handle decimal numbers (Rust doesn't even have a round(3) method),
/// but since fractions also need to be formatted it became suitable to handle it all in one place.
///
/// The `Number::Irrational` variant is used when their values are actually needed for calcuations,
/// otherwise you're better of just treating pi as a `Symbol` in the problem.
mod operations;
use crate::utils::simplified_fraction;
use num_traits::{Signed, Zero};
use std::fmt::Display;

// This limits all numbers to 3 decimals.
pub const DECIMAL_FACTOR: i32 = 1_000;
pub const PI: Number = Number::Irrational(std::f64::consts::PI, "pi");
pub const E: Number = Number::Irrational(std::f64::consts::E, "e");
pub const ZERO: Number = Number::Integer(0);

/// The `Number` enum is used to properly display numbers in Typst while
/// still being able to do calculations.
/// Note that decimal numbers are limited to display (and use) 3 decimals.
#[derive(Debug, Clone, Copy)]
pub enum Number {
    Integer(i32),
    /// The decimal value multiplied by `DECIMAL_FACTOR` (1 000)
    Decimal(i32),
    Fraction(i32, i32),
    Irrational(f64, &'static str),
}

/// These implementations lets us do 1.into() or (1, 3).into(),
/// but calling the variant, like `Number::Fraction(1, 3)`, is preferred.
///
/// This does improve ergonomics like `Term::from_num_and_vars(3.5, X)`;
///
/// Note that the signature is different for `Number::Decimal(1300)` and `1.3.into().`
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

/// Prefer `Number::decimal_from_f64()` whenever possible
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::decimal_from_f64(value)
    }
}
impl From<(f64, &'static str)> for Number {
    fn from(value: (f64, &'static str)) -> Self {
        Self::Irrational(value.0, value.1)
    }
}

impl Number {
    /// Creates a `Number::Decimal` from the given float.
    ///
    /// Note that all floats will be rounded to the amount of decimals provided in
    /// `Number::DECIMAL_FACTOR`
    pub fn decimal_from_f64(num: f64) -> Number {
        Self::Decimal(float_to_int(num))
    }
    pub fn to_decimal(self) -> Number {
        use Number::*;
        match self {
            Integer(i) => Decimal(i * DECIMAL_FACTOR),
            Decimal(d) => Decimal(d),
            Fraction(num, denom) => {
                let approximation = num as f64 / denom as f64;
                Number::decimal_from_f64(approximation)
            }
            Irrational(val, _) => Number::decimal_from_f64(val),
        }
    }

    /// Converts the current `Number` to a `Number::Fraction`.
    /// Converting a `Decimal` to a `Fraction` returns the most simplified version.
    ///
    /// Please don't convert an `Irrational` to a `Fraction`!
    pub fn to_fraction(self) -> Number {
        use Number::*;
        match self {
            Integer(i) => Fraction(i, 1),
            // If the decimal number is 1.6 (represented by d = 1600),
            // we create the fraction 1600 / 1000 and simplify it => 8 / 5
            Decimal(d) => Fraction(d, DECIMAL_FACTOR).simplify(),
            Fraction(_, _) => self,
            Irrational(val, _char) => {
                tracing::error!("Please don't call .to_fraction() on an Irrational!");
                // We can approximate it anyway :) pi = 22 / 7 amirite?
                Number::decimal_from_f64(val).to_fraction()
            }
        }
    }

    /// Calling value() is useful even for integers, since it lets us do things like
    /// num.value().pow(-2), which will be a float.
    pub fn value(&self) -> f64 {
        use Number::*;
        match self {
            Integer(val) => *val as f64,
            Decimal(val) => *val as f64 / DECIMAL_FACTOR as f64,
            Fraction(num, denom) => *num as f64 / *denom as f64,
            Irrational(val, _) => *val,
        }
    }

    /// If the Number is a Fraction, simplifies it (to an Integer if possible)
    pub fn simplify(self) -> Number {
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
        use Number::*;
        match self {
            Integer(val) => Integer(val.abs()),
            Decimal(val) => Decimal(val.abs()),
            Fraction(num, denom) => Fraction(num.abs(), denom.abs()),
            Irrational(val, s) => Irrational(val.abs(), s),
        }
    }

    /// Get the numerator of the number (as a `Number`), if it is a fraction.
    ///
    /// Returns the Number itself if it isn't a fraction.
    pub fn numerator(&self) -> Number {
        match self {
            Number::Fraction(n, _) => Number::Integer(*n),
            _ => *self,
        }
    }

    /// Get the denominator of the number (as a `Number`), if it is a fraction.
    ///
    /// Returns 1 if it isn't a fraction.
    pub fn denominator(&self) -> Number {
        match self {
            Number::Fraction(_, d) => Number::Integer(*d),
            _ => Number::Integer(1),
        }
    }

    pub fn as_i32(self) -> i32 {
        match self {
            Number::Integer(val) => val,
            _ => {
                tracing::error!("Called into<i32> on a non-Integer Number!");
                0
            }
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

/// Returns the i32 that's used for `Number::Decimal` representation.
///
/// # Examples
/// ```
/// use math::float_to_int;
/// let float = 3.14;
/// assert_eq!(float_to_int(float), 3_140);
/// ```
pub fn float_to_int(float: f64) -> i32 {
    (float * DECIMAL_FACTOR as f64).round() as i32
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

impl Zero for Number {
    fn zero() -> Self {
        Number::Integer(0)
    }
    fn is_zero(&self) -> bool {
        match self {
            Number::Integer(l) => *l == 0,
            Number::Decimal(l) => *l == 0,
            Number::Fraction(n, _) => *n == 0,
            Number::Irrational(f, _) => *f == 0.0,
        }
    }
}

impl PartialEq<Self> for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Number::Integer(l) => l == other,
            Number::Decimal(l) => *l == other * DECIMAL_FACTOR,
            Number::Fraction(n, d) => *n == other * d,
            Number::Irrational(_, _) => false,
        }
    }
}

impl PartialEq<Number> for i32 {
    fn eq(&self, other: &Number) -> bool {
        match other {
            Number::Integer(l) => l == self,
            Number::Decimal(l) => *l == self * DECIMAL_FACTOR,
            Number::Fraction(n, d) => *n == self * d,
            Number::Irrational(_, _) => false,
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

impl PartialOrd<i32> for Number {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        match self {
            Number::Integer(i) => i.partial_cmp(other),
            Number::Decimal(d) => d.partial_cmp(&(other * DECIMAL_FACTOR)),
            Number::Fraction(n, d) => n.partial_cmp(&(d * other)),
            Number::Irrational(val, _) => val.partial_cmp(&(*other as f64)),
        }
    }
}

impl Eq for Number {}

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

    #[test]
    fn decimal_from_f64_works_for_positive() {
        let decimal = Number::decimal_from_f64(6.76);
        let long_decimal_round_up = Number::decimal_from_f64(6.7676767676767);
        let long_decimal_round_down = Number::decimal_from_f64(6.7671111111111);
        assert_eq!(decimal.to_string(), "num(\"6.76\")");
        assert_eq!(long_decimal_round_up.to_string(), "num(\"6.768\")");
        assert_eq!(long_decimal_round_down.to_string(), "num(\"6.767\")");
    }

    #[test]
    fn decimal_from_f64_works_for_negative() {
        let decimal = Number::decimal_from_f64(-6.76);
        let long_decimal_round_up = Number::decimal_from_f64(-6.7676767676767);
        let long_decimal_round_down = Number::decimal_from_f64(-6.7671111111111);
        assert_eq!(decimal.to_string(), "num(\"-6.76\")");
        assert_eq!(long_decimal_round_up.to_string(), "num(\"-6.768\")"); // Or will it?
        assert_eq!(long_decimal_round_down.to_string(), "num(\"-6.767\")");
    }

    #[test]
    fn to_decimal() {
        use Number::*;
        let decimal_target = Number::decimal_from_f64(1.2);
        let integer_target = Number::decimal_from_f64(1.0);

        let integer = Integer(1);
        let fraction = Fraction(6, 5);
        let irrational = Irrational(1.2, "test");
        assert_eq!(integer.to_decimal(), integer_target);
        assert_eq!(fraction.to_decimal(), decimal_target);
        assert_eq!(irrational.to_decimal(), decimal_target);
    }

    #[test]
    fn to_fraction() {
        use Number::*;
        let fraction_target = Fraction(6, 5);
        let integer_target = Fraction(2, 1);

        let integer = Integer(2);
        let decimal = Number::decimal_from_f64(1.2);
        let irrational = Irrational(1.2, "test");
        assert_eq!(integer.to_fraction(), integer_target);
        assert_eq!(decimal.to_fraction(), fraction_target);
        assert_eq!(irrational.to_fraction(), fraction_target);
    }
}
