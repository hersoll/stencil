use crate::{Number, ZERO};

/// The Function enum contains information about which kind of function it is (duh), but also numbers
/// that are specific to that kind of plot. This makes ergonomics easier when matching over the
/// kinds since you can do Function::Linear(k, m) => ...
pub enum Function {
    Linear { k: Number, m: Number },
    Exponential { c: Number, a: Number },
}

impl Function {
    /// Ergonomic constructor
    pub fn linear(k: impl Into<Number>, m: impl Into<Number>) -> Function {
        let k = k.into();
        let m = m.into();

        Function::Linear { k, m }
    }

    /// Ergonomic constructor
    pub fn exponential(c: impl Into<Number>, a: impl Into<Number>) -> Function {
        let c = c.into();
        let mut a = a.into();
        if a <= ZERO {
            tracing::error!("a in an exponential function can't be negative (or 0)");
            a = Number::Integer(1)
        }
        Function::Exponential { c, a }
    }
    /// Finds the x-value(s) of the function, given a certain y-value.
    ///
    /// Returns None if the y-value is outside the domain of the function
    pub fn get_x(&self, y: &Number) -> Option<Vec<Number>> {
        match self {
            Function::Linear { k, m } => {
                // Don't use this when k = 0, please
                if *k == ZERO {
                    None
                } else {
                    Some(vec![(y - m) / k])
                }
            }
            Function::Exponential { c, a } => {
                // No solution if y and c have opposite signs, since a positive value can't become
                // negative through exponentiation
                if y * c < ZERO {
                    None
                } else {
                    // y = c a^x => x = lg(y/c) / lg(a)
                    Some(vec![((y / c).value().log2() / a.value().log2()).into()])
                }
            }
        }
    }

    /// Finds the y-value of the function, given a certain x-value.
    ///
    /// Returns None if the function isn't defined for that x-value, for example x = 0 and f(x) =
    /// 1/x
    pub fn get_y(&self, x: &Number) -> Option<Number> {
        match self {
            Function::Linear { k, m } => Some(k * x + m),
            Function::Exponential { c, a } => Some(c * a.value().powf(x.value())),
        }
    }
}

// ===================================================
// TESTS
// ===================================================

#[cfg(test)]
mod linear_function_tests {
    use super::*;

    mod get_y {
        use super::*;

        #[test]
        fn integers() {
            let x = Number::Integer(-1);
            let cases = [
                (Function::linear(1, 2), 1),
                (Function::linear(0, 2), 2),
                (Function::linear(-3, -5), -2),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_y(&x).unwrap(), expected);
            }
        }

        #[test]
        fn decimals() {
            let x = Number::Decimal(2500); // 2.5

            let cases = [
                (Function::linear(1.1, 1.1), Number::Decimal(3850)),
                (Function::linear(0, -3.7), Number::Decimal(-3700)),
                (Function::linear(1.234, 5), Number::Decimal(8085)),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_y(&x).unwrap(), expected);
            }
        }

        #[test]
        fn fractions() {
            let x = Number::Fraction(2, 5);

            let cases = [
                (Function::linear((3, 5), 1), Number::Fraction(31, 25)),
                (Function::linear(0, (2, 3)), Number::Fraction(2, 3)),
                (Function::linear((-4, 3), (1, 6)), Number::Fraction(-11, 30)),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_y(&x).unwrap(), expected);
            }
        }
    }

    mod get_x {
        use super::*;

        #[test]
        fn integers() {
            let y = Number::Integer(4);

            let cases = [
                (Function::linear(3, 2), Some(vec![Number::Fraction(2, 3)])),
                (Function::linear(0, 2), None),
                (Function::linear(-1, -5), Some(vec![Number::Integer(-9)])),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_x(&y), expected);
            }
        }

        #[test]
        fn decimals() {
            let y = Number::Decimal(2500); // 2.5

            let cases = [
                (
                    Function::linear(1.1, 1.1),
                    Some(vec![Number::Decimal(1273)]),
                ),
                (Function::linear(0, -3.7), None),
                (
                    Function::linear(1.234, 5),
                    Some(vec![Number::Decimal(-2026)]),
                ),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_x(&y), expected);
            }
        }

        #[test]
        fn fractions() {
            let y = Number::Fraction(2, 5);

            let cases = [
                (Function::linear((3, 5), 1), Some(vec![Number::Integer(-1)])),
                (Function::linear(0, (2, 3)), None),
                (
                    Function::linear((-4, 3), (1, 6)),
                    Some(vec![Number::Fraction(-21, 120)]),
                ),
                // Test another fraction representation
                (
                    Function::linear((-4, 3), (1, 6)),
                    Some(vec![Number::Fraction(7, -40)]),
                ),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_x(&y), expected);
            }
        }
    }
}
