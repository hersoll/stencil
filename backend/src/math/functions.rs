use crate::math::{Number, ZERO};

/// The Function enum contains information about which kind of function it is (duh), but also numbers
/// that are specific to that kind of plot. This makes ergonomics easier when matching over the
/// kinds since you can do Function::Linear(k, m) => ... and then use k and m by those names.
pub enum Function {
    /// k, m
    Linear(Number, Number),
    /// start, change
    Exponential(Number, Number),
}

impl Function {
    pub fn get_x(&self, y: &Number) -> Option<Number> {
        match self {
            Function::Linear(k, m) => {
                // Don't use this when k = 0, please
                if *k == ZERO { None } else { Some((y - m) / k) }
            }
            Function::Exponential(c, a) => {
                // No solution if y and c have opposite signs,
                // since a positive value can't become negative through exponentiation
                if y * c < ZERO {
                    None
                } else {
                    // y = c a^x => x = lg(y/c) / lg(a)
                    Some(((y / &c).value().log2() / a.value().log2()).into())
                }
            }
        }
    }

    pub fn get_y(&self, x: &Number) -> Number {
        match self {
            Function::Linear(k, m) => k * x + m,
            Function::Exponential(c, a) => c * a.value().powf(x.value()),
        }
    }
}

#[cfg(test)]
mod linear_function_tests {
    use super::*;
    use crate::typst_utils::graphing::Graph;

    mod get_y {
        use super::*;

        #[test]
        fn integers() {
            let x = Number::Integer(-1);
            let cases = [
                (Graph::linear(1, 2), Number::Integer(1)),
                (Graph::linear(0, 2), Number::Integer(2)),
                (Graph::linear(-3, -5), Number::Integer(-2)),
            ];

            for (graph, expected) in cases {
                assert_eq!(graph.function.get_y(&x), expected);
            }
        }

        #[test]
        fn decimals() {
            let x = Number::Decimal(2500); // 2.5

            let cases = [
                (Graph::linear(1.1, 1.1), Number::Decimal(3850)),
                (Graph::linear(0, -3.7), Number::Decimal(-3700)),
                (Graph::linear(1.234, 5), Number::Decimal(8085)),
            ];

            for (graph, expected) in cases {
                assert_eq!(graph.function.get_y(&x), expected);
            }
        }

        #[test]
        fn fractions() {
            let x = Number::Fraction(2, 5);

            let cases = [
                (Graph::linear((3, 5), 1), Number::Fraction(31, 25)),
                (Graph::linear(0, (2, 3)), Number::Fraction(2, 3)),
                (Graph::linear((-4, 3), (1, 6)), Number::Fraction(-11, 30)),
            ];

            for (graph, expected) in cases {
                assert_eq!(graph.function.get_y(&x), expected);
            }
        }
    }

    mod get_x {
        use super::*;

        #[test]
        fn integers() {
            let y = Number::Integer(4);

            let cases = [
                (Graph::linear(3, 2), Some(Number::Fraction(2, 3))),
                (Graph::linear(0, 2), None),
                (Graph::linear(-1, -5), Some(Number::Integer(-9))),
            ];

            for (graph, expected) in cases {
                assert_eq!(graph.function.get_x(&y), expected);
            }
        }

        #[test]
        fn decimals() {
            let y = Number::Decimal(2500); // 2.5

            let cases = [
                (Graph::linear(1.1, 1.1), Some(Number::Decimal(1273))),
                (Graph::linear(0, -3.7), None),
                (Graph::linear(1.234, 5), Some(Number::Decimal(-2026))),
            ];

            for (graph, expected) in cases {
                assert_eq!(graph.function.get_x(&y), expected);
            }
        }

        #[test]
        fn fractions() {
            let y = Number::Fraction(2, 5);

            let cases = [
                (Graph::linear((3, 5), 1), Some(Number::Integer(-1))),
                (Graph::linear(0, (2, 3)), None),
                (
                    Graph::linear((-4, 3), (1, 6)),
                    Some(Number::Fraction(-21, 120)),
                ),
                // Test another fraction representation
                (
                    Graph::linear((-4, 3), (1, 6)),
                    Some(Number::Fraction(7, -40)),
                ),
            ];

            for (graph, expected) in cases {
                assert_eq!(graph.function.get_x(&y), expected);
            }
        }
    }
}
