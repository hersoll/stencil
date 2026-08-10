//! The numbers module handles calculations between different types of numbers
//! (Integers, Decimals, Fractions, Irrationals) and formats them for Typst.
//!
//! The main point was to handle decimal numbers (Rust doesn't even have a round(3) method),
//! but since fractions also need to be formatted it became suitable to handle it all in one place.
//!
//! The `Number::Irrational` variant is used when their values are actually needed for calcuations,
//! otherwise you're better of just treating pi as a `Symbol` in the problem.
mod implementations;
mod operations;
use tracing::error;

use crate::{
    Number::{Decimal, Integer},
    utils::{gcd, simplified_fraction},
};

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
#[derive(Debug, Clone, Copy)]
pub enum Number {
    /// Integer values. Nuff said.
    Integer(i32),
    /// Decimals are represented internally with an `i32` for more exact calculations, and then
    /// keeping track of how many decimals the number should have.
    ///
    /// Examples of the (integer, decimals) combos are:
    /// `(123, 1)` = 12.3, `(1234, 1)` = 123.4, `(123, 3)` = 0.123
    Decimal { integer: i32, decimals: u8 },
    /// A fraction per definition has integers in the numerator and the denominator
    Fraction { numerator: i32, denominator: i32 },
    /// Irrational numbers aren't usually represented with pure numbers, but some kind of symbol.
    /// Therefore, the `Irrational` variant holds an `f64` as the approximate value and a `&str` for
    /// the in-document representation (like `"pi"` or `"sqrt(2)"`)
    Irrational { value: f64, symbol: &'static str },
}

impl Number {
    /// Instantiates a `Number::Decimal` from the given float, rounded to the provided number of decimal
    /// places.
    pub fn decimal_from_f64(num: f64, places: u8) -> Number {
        Self::Decimal {
            integer: float_to_int(num, places),
            decimals: places,
        }
    }

    /// Constructor for fractions
    pub fn fraction(numerator: impl Into<Number>, denominator: impl Into<Number>) -> Number {
        let numerator = numerator.into();
        let denominator = denominator.into();
        Self::Fraction {
            numerator: numerator.as_i32(),
            denominator: denominator.as_i32(),
        }
    }

    /// Converts any `Number` into a `Number::Decimal`.
    ///
    /// Converting a `Fraction` or `Irrational` will give the `Decimal` a maximum of 6 decimals, use
    /// `.round()` if you want less.
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
                Number::decimal_from_f64(approximation, 6)
            }
            Irrational { value: val, .. } => Number::decimal_from_f64(val, 6),
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

    /// Shifts the `Number` to make it have the given number of decimals.
    /// Useful for aligning different `Number::Decimal`s which each other or simply rounding long numbers.
    ///
    /// Note that this also can extend the number of decimals if needed (but not in the PDF):
    ///   0.123                   0.12
    /// `(123, 3).round(2)` --> `(12, 2)`
    ///   12.3                    12.30 (12.3 in the document)
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

    /// If the `Number` is a `Fraction`, simplifies it (to an `Integer` if possible)
    ///
    /// If the `Number` isn't a `Fraction`, nothing will happen to it.
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

    /// Returns true if the `Number` is a fraction and can be simplified
    pub fn can_be_simplified(&self) -> bool {
        if let Number::Fraction {
            numerator,
            denominator,
        } = self
            && gcd(*numerator, *denominator) > 1
        {
            true
        } else {
            false
        }
    }

    /// Returns the absolute value of any `Number`.
    /// Works for all enum variants (note that `Irrational` symbols aren't touched)
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

    /// Extend a fraction or integer by the specified factor.
    ///
    /// Does nothing if the Number isn't an integer or fraction.
    /// Returns 0 if the specified factor isn't an integer.
    ///
    /// ## Examples
    /// ```rust
    /// use math::Number;
    /// let integer = Number::Integer(4);
    /// assert_eq!(integer.extend(3).to_string(), "12/3");
    /// let fraction = Number::from((3, 5));
    /// assert_eq!(fraction.extend(3).to_string(), "9/15");
    /// let decimal = Number::decimal_from_f64(4.3, 1);
    /// assert_eq!(decimal.extend(3).to_string(), decimal.to_string());
    /// ```
    pub fn extend(&self, factor: impl Into<Number>) -> Number {
        let factor = factor.into().as_i32();
        match self {
            Number::Integer(i) => Number::Fraction {
                numerator: i * factor,
                denominator: factor,
            },
            Number::Fraction {
                numerator,
                denominator,
            } => Number::Fraction {
                numerator: numerator * factor,
                denominator: denominator * factor,
            },
            _ => *self,
        }
    }

    /// Deconstructs an `Integer` into the i32 contained within.
    /// Used when `if let ...` is too unergonomic.
    ///
    /// Only use when you KNOW you have a `Number::Integer`
    pub fn as_i32(self) -> i32 {
        match self {
            Number::Integer(val) => val,
            _ => {
                tracing::error!("Called into<i32> on a non-Integer Number!");
                0
            }
        }
    }

    /// Returns `true` if the **value** of the number is an integer, not if it is of the enum
    /// variant `Number::Integer`.
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

    /// Returns the underlaying integer of a Decimal number.
    ///
    /// Mostly used in solutions to show calculations
    ///
    /// ## Example
    /// ```rust
    /// use math::Number;
    /// let decimal = Number::decimal_from_f64(1.23, 2);
    /// assert_eq!(decimal.as_integer(), 123);
    /// ```
    pub fn as_integer(&self) -> Number {
        match self {
            Integer(_) => *self,
            Decimal { integer, .. } => Integer(*integer),
            _ => {
                error!("Called as_integer() on a non-decimal Number");
                Integer(0)
            }
        }
    }

    pub fn decimals(&self) -> u8 {
        match self {
            Integer(_) => 0,
            Decimal { decimals, .. } => *decimals,
            _ => {
                error!("Called decimals() on a non-compatible Number: {self}");
                0
            }
        }
    }

    /// Inside graph strings we need actual numbers, decimals can't be output
    /// as num("1.2"), like they normally do in Display. This function accounts for that.
    ///
    /// Is (probably) only used in Graph.to_typst()
    pub fn for_graphs(&self) -> String {
        let graph_string = format!("{self}");
        if let Some(stripped_string) = graph_string
            .strip_prefix("num(\"")
            .and_then(|strip| strip.strip_suffix("\")"))
        {
            stripped_string.to_string()
        } else {
            graph_string
        }
    }
}

/// It is quite common to want access to the "divisor" of `Decimal` numbers, for example when it
/// needs to be converted from its internal integer representation to its actual value.
///
/// For example, when we have `integer: 1234, decimals: 3`, the divisor that is used is 1000,
/// which results in the number 1.234.
///
/// This function essentially returns `10.pow(places)`
pub fn get_decimal_divisor(places: u8) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn number_value() {
        let cases = [
            (Number::Integer(1), 1.0),
            (
                Number::Fraction {
                    numerator: 1,
                    denominator: 5,
                },
                0.2,
            ),
            (
                Number::Fraction {
                    numerator: 1,
                    denominator: 6,
                }
                .to_decimal(),
                0.166667,
            ),
        ];
        for case in cases {
            assert_eq!(case.0.value(), case.1);
        }
    }

    #[test]
    fn simplify_fractions() {
        let simplifiable_fraction = Number::Fraction {
            numerator: 6,
            denominator: 20,
        };
        assert_eq!(simplifiable_fraction.simplify().numerator(), 3);
        assert_eq!((-simplifiable_fraction).simplify().numerator(), -3);

        let decimal_number = Number::decimal_from_f64(0.12, 2);
        assert!(matches!(decimal_number.simplify(), Number::Decimal { .. })); // Should not have changed
    }
}
