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

pub const PI: Number = Number::Irrational {
    value: std::f64::consts::PI,
    symbol: "pi",
};
pub const E: Number = Number::Irrational {
    value: std::f64::consts::E,
    symbol: "e",
};
pub const ZERO: Number = Number::Integer(0);

/// The `Number` enum is used to properly display numbers in Typst while
/// still being able to do calculations.
/// Note that decimal numbers are limited to display (and use) 3 decimals.
#[derive(Debug, Clone, Copy)]
pub enum Number {
    Integer(i32),
    /// (123, 1) = 12,3, (1234, 1) = 123,4, (123, 3) = 0,123
    Decimal {
        integer: i32,
        decimals: u8,
    },
    Fraction {
        numerator: i32,
        denominator: i32,
    },
    Irrational {
        value: f64,
        symbol: &'static str,
    },
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

impl Number {
    /// Creates a `Number::Decimal` from the given float, rounded to the provided number of decimal
    /// places.
    pub fn decimal_from_f64(num: f64, places: u8) -> Number {
        Self::Decimal {
            integer: float_to_int(num, places),
            decimals: places,
        }
    }
    pub fn to_decimal(self) -> Number {
        use Number::*;
        match self {
            Integer(i) => Decimal {
                integer: i,
                decimals: 0,
            },
            Decimal { .. } => self,
            Fraction {
                numerator: num,
                denominator: denom,
            } => {
                let approximation = num as f64 / denom as f64;
                Number::decimal_from_f64(approximation, 3)
            }
            Irrational { value: val, .. } => Number::decimal_from_f64(val, 3),
        }
    }

    /// Converts the current `Number` to a `Number::Fraction`.
    /// Converting a `Decimal` to a `Fraction` returns the most simplified version.
    ///
    /// Please don't convert an `Irrational` to a `Fraction`!
    pub fn to_fraction(self) -> Number {
        use Number::*;
        match self {
            Integer(i) => Fraction {
                numerator: i,
                denominator: 1,
            },
            // If the decimal number is 1.6 (represented by (16, 1)),
            // we create the fraction 16 / 10 and simplify it => 8 / 5
            Decimal { integer, decimals } => Fraction {
                numerator: integer,
                denominator: get_decimal_divisor(decimals),
            }
            .simplify(),
            Fraction { .. } => self,
            Irrational { value, .. } => {
                tracing::error!("Please don't call .to_fraction() on an Irrational!");
                // We can approximate it anyway :) pi = 22 / 7 amirite?
                Number::decimal_from_f64(value, 3).to_fraction()
            }
        }
    }

    // Target: 3 decimals
    // Currently: 12345, 5 decimals (0.12345)
    // Need to: Divide by 100

    /// Shifts the `Number` to make it have the given number of decimals.
    /// Useful for aligning different `Number::Decimal`s which each other.
    ///
    /// Note that this also can extend the number of decimals:
    ///   0.123                   0.12
    /// `(123, 3).round(2)` --> `(12, 2)`
    ///   12.3                    12.30
    /// `(123, 1).round(2)` --> `(1230, 2)`
    ///
    pub fn round(&self, new_decimal_places: u8) -> Self {
        use Number::*;
        match self {
            Decimal { integer, decimals } => Decimal {
                integer: {
                    if *decimals > new_decimal_places {
                        let new_float = *integer as f64
                            / 10i32.pow(*decimals as u32 - new_decimal_places as u32) as f64;
                        new_float.round() as i32
                    } else {
                        *integer * 10i32.pow(new_decimal_places as u32 - *decimals as u32)
                    }
                },
                decimals: new_decimal_places,
            },
            Irrational { value, .. } => Self::decimal_from_f64(*value, new_decimal_places),
            _ => *self,
        }
    }

    /// Calling value() is useful even for integers, since it lets us do things like
    /// num.value().pow(-2), which will be a float.
    pub fn value(&self) -> f64 {
        use Number::*;
        match self {
            Integer(val) => *val as f64,
            Decimal { integer, decimals } => {
                *integer as f64 / get_decimal_divisor(*decimals) as f64
            }
            Fraction {
                numerator,
                denominator,
            } => *numerator as f64 / *denominator as f64,
            Irrational { value, .. } => *value,
        }
    }

    /// If the Number is a Fraction, simplifies it (to an Integer if possible)
    pub fn simplify(self) -> Number {
        match self {
            Number::Fraction {
                numerator,
                denominator,
            } => {
                let (s_num, s_denom) = simplified_fraction(numerator, denominator);
                if s_num % s_denom == 0 {
                    Number::Integer(s_num / s_denom)
                } else {
                    Number::Fraction {
                        numerator: s_num,
                        denominator: s_denom,
                    }
                }
            }
            n => n,
        }
    }

    pub fn abs(&self) -> Number {
        use Number::*;
        match self {
            Integer(val) => Integer(val.abs()),
            Decimal { integer, decimals } => Decimal {
                integer: integer.abs(),
                decimals: *decimals,
            },
            Fraction {
                numerator,
                denominator,
            } => Fraction {
                numerator: numerator.abs(),
                denominator: denominator.abs(),
            },
            Irrational { value, symbol } => Irrational {
                value: value.abs(),
                symbol,
            },
        }
    }

    /// Get the numerator of the number (as a `Number`), if it is a fraction.
    ///
    /// Returns the Number itself if it isn't a fraction.
    pub fn numerator(&self) -> Number {
        match self {
            Number::Fraction { numerator, .. } => Number::Integer(*numerator),
            _ => *self,
        }
    }

    /// Get the denominator of the number (as a `Number`), if it is a fraction.
    ///
    /// Returns 1 if it isn't a fraction.
    pub fn denominator(&self) -> Number {
        match self {
            Number::Fraction { denominator, .. } => Number::Integer(*denominator),
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

    pub fn is_integer(&self) -> bool {
        use Number::*;
        match self {
            Integer(_) => true,
            Decimal { integer, decimals } => integer % 10i32.pow(*decimals as u32) == 0,
            Fraction {
                numerator,
                denominator,
            } => numerator % denominator == 0,
            Irrational { .. } => false,
        }
    }

    /// Inside graph strings we need actual numbers, decimals can't be output
    /// as num("1.2"), like they normally do in Display. This function accounts for that.
    ///
    /// Is (probably) only used in Graph.to_typst()
    pub fn for_graphs(&self) -> String {
        if let Number::Decimal { .. } = self {
            strip_num(format!("{self}"))
        } else {
            format!("{self}")
        }
    }
}

fn get_decimal_divisor(places: u8) -> i32 {
    10i32.pow(places as u32)
}

/// Returns the i32 that's used for `Number::Decimal` representation.
///
/// # Examples
/// ```
/// use math::float_to_int;
/// let float = 3.14;
/// assert_eq!(float_to_int(float, 3), 3_140);
/// ```
pub fn float_to_int(float: f64, places: u8) -> i32 {
    // round 1.234 to one place --> multiply by 10 --> 12.34 --> round to int --> 12 --> store
    // with decimals = 1
    (float * get_decimal_divisor(places) as f64).round() as i32
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
            Number::Decimal { integer, decimals } => {
                // The decimal value is actually an integer
                if self.is_integer() {
                    write!(f, "{}", *integer / 10i32.pow(*decimals as u32))
                } else {
                    // num() is a formatting library which outputs the decimals with commas
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

impl Zero for Number {
    fn zero() -> Self {
        Number::Integer(0)
    }
    fn is_zero(&self) -> bool {
        match self {
            Number::Integer(val) => *val == 0,
            Number::Decimal { integer, .. } => *integer == 0,
            Number::Fraction { numerator, .. } => *numerator == 0,
            Number::Irrational { value, .. } => *value == 0.0,
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

    #[test]
    fn decimal_from_f64_works_for_positive() {
        let decimal = Number::decimal_from_f64(6.76, 2);
        let long_decimal_round_up = Number::decimal_from_f64(6.7676767676767, 3);
        let long_decimal_round_down = Number::decimal_from_f64(6.7671111111111, 3);
        assert_eq!(decimal.to_string(), "num(\"6.76\")");
        assert_eq!(long_decimal_round_up.to_string(), "num(\"6.768\")");
        assert_eq!(long_decimal_round_down.to_string(), "num(\"6.767\")");
    }

    #[test]
    fn decimal_from_f64_works_for_negative() {
        let decimal = Number::decimal_from_f64(-6.76, 2);
        let long_decimal_round_up = Number::decimal_from_f64(-6.7676767676767, 3);
        let long_decimal_round_down = Number::decimal_from_f64(-6.7671111111111, 3);
        assert_eq!(decimal.to_string(), "num(\"-6.76\")");
        assert_eq!(long_decimal_round_up.to_string(), "num(\"-6.768\")"); // Or will it?
        assert_eq!(long_decimal_round_down.to_string(), "num(\"-6.767\")");
    }

    #[test]
    fn to_decimal() {
        use Number::*;
        let decimal_target = Number::decimal_from_f64(1.2, 1);
        let integer_target = Number::decimal_from_f64(1.0, 0);

        let integer = Integer(1);
        let fraction = Fraction {
            numerator: 6,
            denominator: 5,
        };
        let irrational = Irrational {
            value: 1.2,
            symbol: "test",
        };
        assert_eq!(integer.to_decimal(), integer_target);
        assert_eq!(fraction.to_decimal(), decimal_target);
        assert_eq!(irrational.to_decimal(), decimal_target);
    }

    #[test]
    fn to_fraction() {
        use Number::*;
        let fraction_target = Number::from((6, 5));
        let integer_target = Number::from((2, 1));

        let integer = Integer(2);
        let decimal = Number::decimal_from_f64(1.2, 1);
        let irrational = Irrational {
            value: 1.2,
            symbol: "test",
        };
        assert_eq!(integer.to_fraction(), integer_target);
        assert_eq!(decimal.to_fraction(), fraction_target);
        assert_eq!(irrational.to_fraction(), fraction_target);
    }

    #[test]
    fn rounding() {
        let cases = [
            (Number::decimal_from_f64(0.45, 2), 0.5),
            (Number::decimal_from_f64(1.32, 2), 1.3),
            (Number::decimal_from_f64(10.3, 1), 10.3),
            (Number::decimal_from_f64(0.03, 2), 0.0),
        ];

        for case in cases {
            assert_eq!(case.0.round(1).value(), case.1);
        }
    }
}
