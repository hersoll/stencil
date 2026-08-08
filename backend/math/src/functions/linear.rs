use crate::Number;

#[derive(Debug, Copy, Clone)]
pub struct LinearFunction {
    pub k: Number,
    pub m: Number,
}

#[cfg(test)]
mod tests {
    use super::*;
    mod display {
        use crate::{functions::Function, symbols};

        #[test]
        fn defaults() {
            let cases = [
                (Function::linear(3, 5), "y = 3x+5"),
                (Function::linear(3, 5).with_name(symbols::T), "t = 3x+5"),
                (Function::linear(3, 5).with_variable(symbols::T), "y = 3t+5"),
                (Function::linear(-3, -5), "y = -3x-5"),
            ];

            for (function, printing) in cases {
                assert_eq!(function.to_string(), printing);
            }
        }

        #[test]
        fn corner_cases() {
            let cases = [
                (Function::linear(0, 1), "y = 1"),
                (Function::linear(1, 0), "y = x"),
                (Function::linear(-1, 0), "y = -x"),
            ];

            for (function, printing) in cases {
                assert_eq!(function.to_string(), printing);
            }
        }

        #[test]
        fn with_function_notation() {
            let cases = [
                (
                    Function::linear(3, 5).with_function_notation(),
                    "y(x) = 3x+5",
                ),
                (
                    Function::linear(3, 5)
                        .with_name(symbols::T)
                        .with_function_notation(),
                    "t(x) = 3x+5",
                ),
                (
                    Function::linear(3, 5)
                        .with_variable(symbols::T)
                        .with_function_notation(),
                    "y(t) = 3t+5",
                ),
                (
                    Function::linear(3, 5)
                        .with_name(symbols::F)
                        .with_variable(symbols::T)
                        .with_function_notation(),
                    "f(t) = 3t+5",
                ),
            ];

            for (function, printing) in cases {
                assert_eq!(function.to_string(), printing);
            }
        }
    }

    mod evaluable {
        use super::*;

        mod print_replacements {
            use crate::{Evaluable, functions::Function, symbols};

            use super::*;
            #[test]
            fn replace_x() {
                let func = Function::linear(4, -1)
                    .with_name(symbols::F)
                    .with_variable(symbols::X)
                    .with_function_notation();
                assert_eq!(
                    func.print_replacements(&[(symbols::X, &Number::Integer(2))]),
                    "f(colored(2)) = 4 dot colored(2) -1"
                );

                let func = Function::linear(0, 1)
                    .with_name(symbols::F)
                    .with_variable(symbols::X)
                    .with_function_notation();
                assert_eq!(
                    func.print_replacements(&[(symbols::X, &Number::Integer(2))]),
                    "f(colored(2)) = 1"
                );
            }

            #[test]
            fn replace_y() {
                let func = Function::linear(4, -1)
                    .with_name(symbols::Y)
                    .with_variable(symbols::X)
                    .with_function_notation();
                assert_eq!(
                    func.print_replacements(&[(symbols::Y, &Number::Integer(2))]),
                    "colored(2) = 4x-1"
                );

                let func = Function::linear(0, 1)
                    .with_name(symbols::Y)
                    .with_variable(symbols::X)
                    .with_function_notation();
                assert_eq!(
                    func.print_replacements(&[(symbols::Y, &Number::Integer(2))]),
                    "colored(2) = 1"
                );
            }

            #[test]
            fn no_valid_replacement() {
                let func = Function::linear(4, -1)
                    .with_name(symbols::Y)
                    .with_variable(symbols::X)
                    .without_function_notation();
                assert_eq!(
                    func.print_replacements(&[(symbols::A, &Number::Integer(2))]),
                    "y = 4x-1"
                );
            }
        }

        mod print_evaluation_by_parts {
            use crate::{Evaluable, functions::Function, symbols};

            use super::*;
            #[test]
            fn it_works() {
                let replacement = [(symbols::X, &Number::Integer(2))];
                let cases = [
                    (Function::linear(3, 1), "y = 6+1"),
                    (Function::linear(1, -1), "y = 2-1"),
                    (Function::linear(4, 0), "y = 8"),
                    (Function::linear(0, 0), "y = 0"),
                ];

                for (func, str) in cases {
                    assert_eq!(func.print_evaluation_by_parts(&replacement), str);
                }
            }
        }
    }

    mod get_y {
        use crate::functions::Function;

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
            let x = Number::decimal_from_f64(2.5, 1);

            let cases = [
                (
                    Function::linear(1.1, 1.1),
                    Number::decimal_from_f64(3.850, 2),
                ),
                (Function::linear(0, -3.7), Number::decimal_from_f64(-3.7, 1)),
                (
                    Function::linear(1.234, 5),
                    Number::decimal_from_f64(8.085, 3),
                ),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_y(&x).unwrap(), expected);
            }
        }

        #[test]
        fn fractions() {
            let x = Number::from((2, 5));

            let cases = [
                (Function::linear((3, 5), 1), Number::from((31, 25))),
                (Function::linear(0, (2, 3)), Number::from((2, 3))),
                (Function::linear((-4, 3), (1, 6)), Number::from((-11, 30))),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_y(&x).unwrap(), expected);
            }
        }
    }

    mod get_x {
        use crate::functions::Function;

        use super::*;

        #[test]
        fn integers() {
            let y = Number::Integer(4);

            let cases = [
                (Function::linear(3, 2), vec![Number::from((2, 3))]),
                (Function::linear(0, 2), vec![]),
                (Function::linear(-1, -5), vec![Number::Integer(-9)]),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_x(&y), expected);
            }
        }

        #[test]
        fn decimals() {
            let y = Number::decimal_from_f64(2.5, 1);

            let cases = [
                (
                    Function::linear(1.1, 1.1),
                    vec![Number::decimal_from_f64(1.273, 3)],
                ),
                (Function::linear(0, -3.7), vec![]),
                (
                    Function::linear(1.234, 5),
                    vec![Number::decimal_from_f64(-2.026, 3)],
                ),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_x(&y), expected);
            }
        }

        #[test]
        fn fractions() {
            let y = Number::from((2, 5));

            let cases = [
                (Function::linear((3, 5), 1), vec![Number::Integer(-1)]),
                (Function::linear(0, (2, 3)), vec![]),
                (
                    Function::linear((-4, 3), (1, 6)),
                    vec![Number::from((-21, 120))],
                ),
                // Test another fraction representation
                (
                    Function::linear((-4, 3), (1, 6)),
                    vec![Number::from((7, -40))],
                ),
            ];

            for (f, expected) in cases {
                assert_eq!(f.get_x(&y), expected);
            }
        }
    }
}
