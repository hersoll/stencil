use crate::get_decimal_divisor;

use super::Number;

impl Number {
    pub fn pow(&self, exponent: impl Into<Number>) -> Number {
        use Number::*;
        let exponent = exponent.into();
        match (self, exponent) {
            // Integer ^ Integer
            (Integer(base), Integer(exp)) => {
                if exp >= 0 {
                    Integer(base.pow(exp.cast_unsigned()))
                } else {
                    // e.g. 2^-3 = 1/8
                    Fraction {
                        numerator: 1,
                        denominator: base.pow((-exp).cast_unsigned()),
                    }
                }
            }

            // Fraction ^ Integer
            (
                Fraction {
                    numerator,
                    denominator,
                },
                Integer(exp),
            ) => {
                if exp >= 0 {
                    let e = exp.cast_unsigned();
                    Fraction {
                        numerator: numerator.pow(e),
                        denominator: denominator.pow(e),
                    }
                } else {
                    // (a/b)^-n = b^n / a^n
                    let e = (-exp).cast_unsigned();
                    Fraction {
                        numerator: denominator.pow(e),
                        denominator: numerator.pow(e),
                    }
                }
            }

            // Everything else: fall back to f64
            (l_val, r_val) => Number::from(l_val.value().powf(r_val.value())),
        }
    }

    /// Returns the square root, generally as a decimal number
    pub fn sqrt(&self) -> Number {
        Number::from(self.value().sqrt())
    }
}

impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(val) => Self::Integer(-val),
            Self::Decimal { integer, decimals } => Self::Decimal {
                integer: -integer,
                decimals,
            },
            Self::Fraction {
                numerator,
                denominator,
            } => Self::Fraction {
                numerator: -numerator,
                denominator,
            },
            // Yes, we keep the same symbol even though it's negated.
            // This is merely intended for getting a numerical value in calculations
            Self::Irrational { value, symbol } => Self::Irrational {
                value: -value,
                symbol,
            },
        }
    }
}

impl std::ops::Add<&Number> for Number {
    type Output = Number;
    fn add(self, rhs: &Number) -> Self::Output {
        use Number::*;
        match (self, rhs) {
            // Two integers: just add them
            (Integer(l), Integer(r)) => Integer(l + r),
            // Integer + Fraction: Turn the integer into (int*denom)/denom, then add
            (
                Integer(l),
                Fraction {
                    numerator,
                    denominator,
                },
            ) => Fraction {
                numerator: numerator + l * denominator,
                denominator: *denominator,
            },
            // Fraction + Integer: Same as above
            (
                Fraction {
                    numerator,
                    denominator,
                },
                Integer(r),
            ) => Fraction {
                numerator: numerator + r * denominator,
                denominator,
            },
            // Two fractions: Make a common denominator by multiplying them
            (
                Fraction {
                    numerator: l_num,
                    denominator: l_denom,
                },
                Fraction {
                    numerator: r_num,
                    denominator: r_denom,
                },
            ) => Fraction {
                numerator: l_num * r_denom + r_num * l_denom,
                denominator: l_denom * r_denom,
            },
            // Decimal + Integer: Make the integer larger to align with the decimal_places
            (Decimal { integer, decimals }, Integer(int)) => Decimal {
                integer: integer + int * get_decimal_divisor(decimals),
                decimals,
            },
            // Integer + Decimals: same as above
            (Integer(int), Decimal { integer, decimals }) => Decimal {
                integer: *integer + int * get_decimal_divisor(*decimals),
                decimals: *decimals,
            },
            // Two decimals: Make them have the same number of decimals, then add
            (
                Decimal {
                    decimals: l_decimals,
                    ..
                },
                Decimal {
                    decimals: r_decimals,
                    ..
                },
            ) => {
                let number_of_decimals = l_decimals.max(*r_decimals);
                let sum = if let (Decimal { integer: l, .. }, Decimal { integer: r, .. }) = (
                    self.round(number_of_decimals),
                    rhs.round(number_of_decimals),
                ) {
                    l + r
                } else {
                    tracing::error!("Somehow, we got to this branch in Add<Number>");
                    0
                };

                Decimal {
                    integer: sum,
                    decimals: number_of_decimals,
                }
            }

            (l_val, r_val) => Number::from(l_val.value() + r_val.value()),
        }
    }
}

impl std::ops::Add<i32> for Number {
    type Output = Number;
    fn add(self, rhs: i32) -> Self::Output {
        self + Number::Integer(rhs)
    }
}

impl std::ops::Add<Number> for i32 {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        rhs + Number::Integer(self)
    }
}

impl std::ops::Add<&Number> for &Number {
    type Output = Number;
    fn add(self, rhs: &Number) -> Self::Output {
        *self + rhs
    }
}

impl std::ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<&Number> for Number {
    type Output = Number;
    fn sub(self, rhs: &Number) -> Self::Output {
        use Number::*;
        match (self, rhs) {
            // Two integers: just subtract
            (Integer(l), Integer(r)) => Integer(l - r),
            // Integer - Fraction: Turn the integer into (int*denom)/denom, then subtract
            (
                Integer(l),
                Fraction {
                    numerator,
                    denominator,
                },
            ) => Fraction {
                numerator: l * denominator - numerator,
                denominator: *denominator,
            },
            // Fraction - Integer: same as above
            (
                Fraction {
                    numerator,
                    denominator,
                },
                Integer(r),
            ) => Fraction {
                numerator: numerator - r * denominator,
                denominator,
            },
            // Two fractions: Make a common denominator by multiplying them
            (
                Fraction {
                    numerator: l_num,
                    denominator: l_denom,
                },
                Fraction {
                    numerator: r_num,
                    denominator: r_denom,
                },
            ) => Fraction {
                numerator: l_num * r_denom - r_num * l_denom,
                denominator: l_denom * r_denom,
            },
            // Decimal - Integer: Make the integer larger to align with the decimal_places
            (Decimal { integer, decimals }, Integer(int)) => Decimal {
                integer: integer - int * get_decimal_divisor(decimals),
                decimals,
            },
            // Integer - Decimals: same as above
            (Integer(int), Decimal { integer, decimals }) => Decimal {
                integer: int * get_decimal_divisor(*decimals) - *integer,
                decimals: *decimals,
            },
            // Two decimals: Make them have the same number of decimals, then subtract
            (
                Decimal {
                    decimals: l_decimals,
                    ..
                },
                Decimal {
                    decimals: r_decimals,
                    ..
                },
            ) => {
                let number_of_decimals = l_decimals.max(*r_decimals);
                let diff = if let (Decimal { integer: l, .. }, Decimal { integer: r, .. }) = (
                    self.round(number_of_decimals),
                    rhs.round(number_of_decimals),
                ) {
                    l - r
                } else {
                    tracing::error!("Somehow, we got to this branch in Sub<Number>");
                    0
                };

                Decimal {
                    integer: diff,
                    decimals: number_of_decimals,
                }
            }
            (l_val, r_val) => Number::from(l_val.value() - r_val.value()),
        }
    }
}

impl std::ops::Sub<&Number> for &Number {
    type Output = Number;
    fn sub(self, rhs: &Number) -> Self::Output {
        *self - rhs
    }
}

impl std::ops::Sub<Number> for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Sub<i32> for Number {
    type Output = Number;
    fn sub(self, rhs: i32) -> Self::Output {
        self - Number::Integer(rhs)
    }
}

impl std::ops::Sub<Number> for i32 {
    type Output = Number;
    fn sub(self, rhs: Number) -> Self::Output {
        Number::Integer(self) - rhs
    }
}

impl std::ops::SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<&Number> for Number {
    type Output = Number;
    fn mul(self, rhs: &Number) -> Self::Output {
        use Number::*;
        match (self, rhs) {
            // Two integers: just multiply
            (Integer(l), Integer(r)) => Integer(l * r),
            // Integer * Fraction: multiply numerators
            (
                Integer(l),
                Fraction {
                    numerator,
                    denominator,
                },
            ) => Fraction {
                numerator: l * numerator,
                denominator: *denominator,
            },
            // Fraction * Integer: multiply numerators
            (
                Fraction {
                    numerator,
                    denominator,
                },
                Integer(r),
            ) => Fraction {
                numerator: numerator * r,
                denominator,
            },
            // Fraction * Fraction: multiply like a king
            (
                Fraction {
                    numerator: l_num,
                    denominator: l_denom,
                },
                Fraction {
                    numerator: r_num,
                    denominator: r_denom,
                },
            ) => Fraction {
                numerator: l_num * r_num,
                denominator: l_denom * r_denom,
            },
            // Integer * Decimal: just multiply the values
            (Integer(int), Decimal { integer, decimals }) => Decimal {
                integer: int * integer,
                decimals: *decimals,
            },

            // Two decimals: 23.34 * 45.321 --> Multiply them as integers (2334 * 45321), answer
            // will have 2 + 3 = 5 decimals
            (Decimal { integer, decimals }, Integer(int)) => Decimal {
                integer: int * integer,
                decimals,
            },
            (
                Decimal {
                    integer: l_int,
                    decimals: l_decimals,
                },
                Decimal {
                    integer: r_int,
                    decimals: r_decimals,
                },
            ) => Decimal {
                integer: l_int * r_int,
                decimals: l_decimals + r_decimals,
            },
            (l_val, r_val) => Number::from(l_val.value() * r_val.value()),
        }
    }
}

impl std::ops::Mul<&Number> for &Number {
    type Output = Number;
    fn mul(self, rhs: &Number) -> Self::Output {
        *self * rhs
    }
}

impl std::ops::Mul<Number> for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        &self * &rhs
    }
}

impl std::ops::Mul<i32> for Number {
    type Output = Number;
    fn mul(self, rhs: i32) -> Self::Output {
        self * Number::Integer(rhs)
    }
}

impl std::ops::Mul<Number> for i32 {
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        Number::Integer(self) * rhs
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
        self * Number::from(rhs)
    }
}

impl std::ops::MulAssign<Number> for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<i32> for Number {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<Number> for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            // Integer / Integer => Just make a fraction
            (Number::Integer(numerator), Number::Integer(denominator)) => Number::Fraction {
                numerator,
                denominator,
            }
            .simplify(),
            // Integer / Fraction => Invert the fraction, multiply
            (
                Number::Integer(l),
                Number::Fraction {
                    numerator,
                    denominator,
                },
            ) => Number::Fraction {
                numerator: l * denominator,
                denominator: numerator,
            },
            // Fraction / Integer => Extend the denominator by integer
            (
                Number::Fraction {
                    numerator,
                    denominator,
                },
                Number::Integer(r),
            ) => Number::Fraction {
                numerator,
                denominator: denominator * r,
            },
            // Fraction / Fraction => Invert right fraction, multiply
            (
                Number::Fraction {
                    numerator: l_num,
                    denominator: l_denom,
                },
                Number::Fraction {
                    numerator: r_num,
                    denominator: r_denom,
                },
            ) => Number::Fraction {
                numerator: l_num * r_denom,
                denominator: l_denom * r_num,
            },
            (l_val, r_val) => Number::from(l_val.value() / r_val.value()),
        }
    }
}

impl std::ops::Div<&Number> for &Number {
    type Output = Number;
    fn div(self, rhs: &Number) -> Self::Output {
        *self / *rhs
    }
}

impl std::ops::Div<&Number> for Number {
    type Output = Number;
    fn div(self, rhs: &Number) -> Self::Output {
        self / *rhs
    }
}

impl std::ops::Div<i32> for Number {
    type Output = Number;
    fn div(self, rhs: i32) -> Self::Output {
        self / Number::Integer(rhs)
    }
}

impl std::ops::Div<Number> for i32 {
    type Output = Number;
    fn div(self, rhs: Number) -> Self::Output {
        Number::Integer(self) / rhs
    }
}

impl std::ops::DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<i32> for Number {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem<Number> for Number {
    type Output = Self;
    fn rem(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l % r),
            (_, _) => {
                tracing::error!(
                    "Number % Number is currently only implemented in the case where both are Integers."
                );
                Number::Integer(1)
            }
        }
    }
}

impl std::ops::Rem<i32> for Number {
    type Output = Self;
    fn rem(self, rhs: i32) -> Self::Output {
        let rhs = Number::Integer(rhs);
        self % rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PI;

    #[test]
    fn addition() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let decimal_2: Number = 1.8.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((integer + integer).to_string(), "6");
        assert_eq!((integer + decimal).to_string(), "num(\"4.2\")");
        assert_eq!((decimal + integer).to_string(), "num(\"4.2\")");
        assert_eq!((integer + fraction).to_string(), "15/4");
        assert_eq!((fraction + integer).to_string(), "15/4");
        assert_eq!((decimal + fraction).to_string(), "num(\"1.95\")");
        assert_eq!((fraction + decimal).to_string(), "num(\"1.95\")");
        assert_eq!((PI + integer).to_string(), "num(\"6.142\")");
        assert_eq!((decimal + decimal_2).to_string(), "3");
    }

    #[test]
    fn subtraction() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((integer - integer).to_string(), "0");
        assert_eq!((integer - decimal).to_string(), "num(\"1.8\")");
        assert_eq!((decimal - integer).to_string(), "num(\"-1.8\")");
        assert_eq!((integer - fraction).to_string(), "9/4");
        assert_eq!((fraction - integer).to_string(), "-9/4");
        assert_eq!((decimal - fraction).to_string(), "num(\"0.45\")");
        assert_eq!((fraction - decimal).to_string(), "num(\"-0.45\")");
        assert_eq!((PI - integer).to_string(), "num(\"0.142\")");
    }

    #[test]
    fn multiplication() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((integer * integer).to_string(), "9");
        assert_eq!((integer * decimal).to_string(), "num(\"3.6\")");
        assert_eq!((decimal * integer).to_string(), "num(\"3.6\")");
        assert_eq!((integer * fraction).to_string(), "9/4");
        assert_eq!((fraction * integer).to_string(), "9/4");
        assert_eq!((decimal * fraction).to_string(), "num(\"0.9\")");
        assert_eq!((fraction * decimal).to_string(), "num(\"0.9\")");
        assert_eq!((PI * integer).to_string(), "num(\"9.425\")");
    }

    #[test]
    fn multiply_will_round() {
        let two_point_five = Number::decimal_from_f64(2.5, 1);
        let two_point_fifty_four = Number::decimal_from_f64(2.54, 2);
        let two = Number::Integer(2);

        assert_eq!((two * two_point_five).to_string(), "5");
        assert_eq!((two_point_five * two).to_string(), "5");
        assert_eq!(
            (two_point_five * two_point_fifty_four).to_string(),
            "num(\"6.35\")"
        );
        assert_eq!((two * two_point_fifty_four).to_string(), "num(\"5.08\")");
    }

    #[test]
    fn division() {
        let integer = Number::Integer(3);
        let decimal = Number::decimal_from_f64(1.2, 1);
        let decimal_2 = Number::decimal_from_f64(0.6, 1);
        let fraction = Number::from((3, 4));

        assert_eq!((integer / integer).to_string(), "1");
        assert_eq!((integer / decimal).to_string(), "num(\"2.5\")");
        assert_eq!((decimal / integer).to_string(), "num(\"0.4\")");
        assert_eq!((decimal / decimal_2).to_string(), "2");
        assert_eq!((decimal_2 / decimal).to_string(), "num(\"0.5\")");
        assert_eq!((integer / fraction).simplify().to_string(), "4");
        assert_eq!((fraction / integer).simplify().to_string(), "1/4");
        assert_eq!((decimal / fraction).to_string(), "num(\"1.6\")");
        assert_eq!((fraction / decimal).to_string(), "num(\"0.625\")");
        assert_eq!((PI / integer).to_string(), "num(\"1.047\")");
    }

    #[test]
    fn roots_are_calculated() {
        let integer = Number::Integer(3);
        let decimal = Number::decimal_from_f64(1.2, 1);
        let fraction = Number::from((3, 4));

        assert_eq!(integer.sqrt().to_string(), "num(\"1.732\")");
        assert_eq!(decimal.sqrt().to_string(), "num(\"1.095\")");
        assert_eq!(fraction.sqrt().to_string(), "num(\"0.866\")");
    }

    #[test]
    fn integer_roots_are_displayed_as_integers() {
        let square = Number::Integer(16);
        assert_eq!(square.sqrt().to_string(), "4");
    }
}
