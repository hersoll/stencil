use crate::math;

pub fn simplified_fraction(numerator: i32, denominator: i32) -> (i32, i32) {
    let mut new_numerator = numerator;
    let mut new_denominator = denominator;
    if new_numerator < 0 && new_denominator < 0 {
        // abs() since one negative fucks it up
        new_numerator = new_numerator.abs();
        new_denominator = new_denominator.abs();
    }

    let gcd = math::utils::gcd(new_numerator, new_denominator);
    (new_numerator / gcd, new_denominator / gcd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cant_be_simplified() {
        assert_eq!(simplified_fraction(9, 11), (9, 11));
        assert_eq!(simplified_fraction(11, 9), (11, 9));
        assert_eq!(simplified_fraction(1, 4), (1, 4));
    }

    #[test]
    fn simplifies_when_denominator_is_larger() {
        assert_eq!(simplified_fraction(6, 12), (1, 2));
    }

    #[test]
    fn simplifies_when_numerator_is_larger() {
        assert_eq!(simplified_fraction(12, 6), (2, 1));
    }

    #[test]
    fn simplifies_when_one_is_negative() {
        assert_eq!(simplified_fraction(-4, 8), (-1, 2));
        assert_eq!(simplified_fraction(18, -6), (3, -1));
    }

    #[test]
    fn simplifies_when_both_are_negative() {
        assert_eq!(simplified_fraction(-4, -8), (1, 2));
        assert_eq!(simplified_fraction(-18, -6), (3, 1));
    }
}
