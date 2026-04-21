use crate::Number;

/// Converts a Number from **number of percentages** (not decimal form!)
/// to a change factor.
///
/// # Examples:
/// ```rust
/// use math::Number;
/// use math::utils::to_change_factor;
/// let increase = Number::Integer(10);
/// let change_factor = Number::decimal_from_f64(1.1);
/// assert_eq!(to_change_factor(increase), change_factor);
/// let decrease = Number::Integer(-13);
/// let change_factor = Number::decimal_from_f64(0.87);
/// assert_eq!(to_change_factor(decrease), change_factor);
/// ```
/// 10 to 1,10 and -13 to 0,87.
pub fn to_change_factor(number: Number) -> Number {
    let total_percentage = 100 + number;
    (total_percentage / 100).to_decimal()
}

#[cfg(test)]
mod change_factor_tests {
    use super::*;

    fn check_integer_change_factors(cases: &[(i32, f64)]) {
        for pair in cases {
            let integer = Number::Integer(pair.0);
            assert_eq!(to_change_factor(integer), Number::decimal_from_f64(pair.1));
        }
    }

    fn check_fractional_change_factors(cases: &[((i32, i32), f64)]) {
        for pair in cases {
            let fraction = Number::Fraction(pair.0.0, pair.0.1);
            assert_eq!(to_change_factor(fraction), Number::decimal_from_f64(pair.1));
        }
    }

    fn check_decimal_change_factors(cases: &[(f64, f64)]) {
        for pair in cases {
            let decimal = Number::decimal_from_f64(pair.0);
            assert_eq!(to_change_factor(decimal), Number::decimal_from_f64(pair.1));
        }
    }

    #[test]
    fn converts_from_positive_integers() {
        check_integer_change_factors(&[(3, 1.03), (10, 1.1), (100, 2.0)]);
    }

    #[test]
    fn converts_from_negative_integers() {
        check_integer_change_factors(&[(-3, 0.97), (-10, 0.9), (-100, 0.0)]);
    }

    #[test]
    fn converts_from_zero() {
        check_integer_change_factors(&[(0, 1.0)]);
    }

    #[test]
    fn converts_from_positive_fractions() {
        // Note = 1 is 1%, so 1/100 is 0.01%
        check_fractional_change_factors(&[((1, 10), 1.001), ((30, 4), 1.075), ((4, 8), 1.005)]);
    }

    #[test]
    fn converts_from_negative_fractions() {
        check_fractional_change_factors(&[((-1, 10), 0.999), ((-30, 4), 0.925), ((-4, 8), 0.995)]);
    }

    #[test]
    fn converts_from_positive_decimals() {
        check_decimal_change_factors(&[
            (1.2, 1.012),
            (10.5, 1.105),
            (100.2, 2.002),
            (1.2345, 1.012), // Max three decimals stored
        ]);
    }

    #[test]
    fn converts_from_negative_decimals() {
        check_decimal_change_factors(&[
            (-1.2, 0.988),
            (-10.5, 0.895),
            (-99.9, 0.001),
            (-1.2345, 0.988),
        ]);
    }
}
