use crate::Number;

pub fn gcd(first: impl Into<Number>, second: impl Into<Number>) -> i32 {
    let first = first.into();
    let second = second.into();
    if let (Number::Integer(first), Number::Integer(second)) = (first, second) {
        if first.abs() == second.abs() {
            return first.abs();
        }
        let mut a: i32;
        let mut b: i32;
        if first.abs() > second.abs() {
            a = first.abs();
            b = second.abs();
        } else {
            a = second.abs();
            b = first.abs();
        }

        while b > 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }
        a
    } else {
        tracing::error!("gcd can only be used with integers");
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_of_two_positive_numbers() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
        assert_eq!(gcd(1, 4), 1);
        assert_eq!(gcd(100, 10), 10);
    }
    #[test]
    fn gcd_with_one_negative_number() {
        assert_eq!(gcd(48, -18), 6);
        assert_eq!(gcd(-48, 18), 6);
    }
    #[test]
    fn gcd_with_two_negative_numbers() {
        assert_eq!(gcd(-48, -18), 6);
    }
    #[test]
    fn gcd_with_zero() {
        assert_eq!(gcd(0, 18), 18);
        assert_eq!(gcd(0, -18), 18);
        assert_eq!(gcd(0, 0), 0);
    }
}
