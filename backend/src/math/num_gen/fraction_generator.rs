use super::common::{Kind, generate_value};
use crate::math;
use rand::seq::IteratorRandom;

/// The builder type returned by the fraction() function
pub struct FractionGenerator {
    num: Kind,
    denom: Kind,
    num_exclusions: Vec<i32>,
    denom_exclusions: Vec<i32>,
    min_value: i32,
    max_value: i32,
    reducible: bool,
}

/// Generate fractions by specifying attributes about the numerator and denominator
///
/// Note: Fractions are returned as tuples (num, denom) instead of Number::Fraction for easier access to the pieces. It can easily be converted to a Number with .into()
///
/// # Examples
///
/// ```
/// let (num, denom) = stencil::math::num_gen::fraction().denom(7).random();
/// assert!(num >= 1 && num <= 6); // Generates a fraction between 1/7 and 6/7
/// assert!(denom == 7);
/// ```
///
/// Fractions are irreducible by default - if a certain denominator is given the generator is sure
/// not to accidentally generate a fraction which can be reduced to another denominator:
/// ```
/// let (num, denom) = stencil::math::num_gen::fraction().denom(6).random();
/// assert!(num == 1 || num == 5); // Only 1/6 and 5/6 are irreducible
/// assert!(denom == 6);
/// ```
///
/// ```
/// let (num, denom) = stencil::math::num_gen::fraction().denom(6).min(2).max(4).random();
/// assert!(num == 13 || num == 17 || num == 19 || num == 23);
/// assert!(denom == 6);
/// ```
///
/// ```
/// let (num, denom) = stencil::math::num_gen::fraction().reducible().denom(6).random();
/// assert!(num >= 0 && num <= 6); // All numerators are allowed now, even 0
/// assert!(denom == 6);
/// ```
/// ```
/// let (num, denom) = stencil::math::num_gen::fraction().reducible().denom(6).max(2).random();
/// assert!(num >= 0 && num <= 12);
/// assert!(denom == 6);
/// ```
///
/// The numerator can of course be set independently:
/// ```
/// let (num, denom) = stencil::math::num_gen::fraction().num(5).denom(6).random();
/// assert_eq!(num, 5);
/// assert_eq!(denom, 6);
/// ```
///
/// As shown above, if the numerator isn't set it will be auto adjusted depending on the
/// denominator. The denominator is always expected to be set though. If it isn't, the method will emit a tracing::error and set the denominator to 0:
/// ```
/// let (num, denom) = stencil::math::num_gen::fraction().num(5).random();
/// assert_eq!(num, 5);
/// assert_eq!(denom, 0);
/// ```
pub fn fraction() -> FractionGenerator {
    FractionGenerator {
        num: Kind::NotDefined,
        denom: Kind::NotDefined,
        num_exclusions: Vec::new(),
        denom_exclusions: Vec::new(),
        min_value: 0,
        max_value: 1,
        reducible: false,
    }
}

impl FractionGenerator {
    /// Sets the numerator to a specific number
    pub fn num(mut self, num: i32) -> Self {
        self.num = Kind::Single(num);
        self
    }

    /// Sets the numerator to one of several numbers
    pub fn nums(mut self, nums: &[i32]) -> Self {
        self.num = Kind::Multiple(Vec::from(nums));
        self
    }

    /// Sets the numerator to a number within a range
    pub fn num_range(mut self, min: i32, max: i32) -> Self {
        self.num = Kind::Range(min, max);
        self
    }

    /// Sets the denominator to a specific number
    pub fn denom(mut self, denom: i32) -> Self {
        self.denom = Kind::Single(denom);
        self
    }

    /// Sets the denominator to one of several numbers
    pub fn denoms(mut self, denoms: &[i32]) -> Self {
        self.denom = Kind::Multiple(Vec::from(denoms));
        self
    }

    /// Sets the denominator to a number within a range
    pub fn denom_range(mut self, min: i32, max: i32) -> Self {
        self.denom = Kind::Range(min, max);
        self
    }

    pub fn min(mut self, min: i32) -> Self {
        self.min_value = min;
        self
    }

    pub fn max(mut self, max: i32) -> Self {
        self.max_value = max;
        self
    }

    /// Allow the fraction to be reducible
    pub fn reducible(mut self) -> Self {
        self.reducible = true;
        self
    }

    pub fn exclude_num(mut self, num: i32) -> Self {
        self.num_exclusions.push(num);
        self
    }

    pub fn exclude_nums(mut self, nums: &[i32]) -> Self {
        let mut nums = Vec::from(nums);
        self.num_exclusions.append(&mut nums);
        self
    }

    pub fn exclude_denom(mut self, denom: i32) -> Self {
        self.denom_exclusions.push(denom);
        self
    }

    pub fn exclude_denoms(mut self, denoms: &[i32]) -> Self {
        let mut denoms = Vec::from(denoms);
        self.denom_exclusions.append(&mut denoms);
        self
    }

    /// Generate a random fraction from the FractionGenerator with the parameters given
    ///
    /// Non-consuming method, so the generator can be used again
    pub fn random(&mut self) -> (i32, i32) {
        let denom = generate_value(&self.denom, &self.denom_exclusions);
        self.auto_set_num(&denom);

        // If the fraction is irreducible, we can't just get any old numerator
        let num = match self.reducible {
            false => generate_irreducible_numerator(&self.num, denom, &self.num_exclusions),
            true => generate_value(&self.num, &self.num_exclusions),
        };

        (num, denom)
    }

    /// With a given denominator, sets the numerator to make sure the fraction is between min_value and max_value
    fn auto_set_num(&mut self, denom: &i32) {
        if let Kind::NotDefined = self.num {
            self.num = Kind::Range(
                self.min_value * denom.abs() + 1,
                self.max_value * denom.abs() - 1,
            );
        }
    }
}

/// Returns a numerator which assures the resulting fraction is irreducible.
///
/// This function is a special version of generate_value(), using gcd to make sure that the resulting
/// fraction cannot be reduced; if a denom of 6 is given then the resulting fraction will have a
/// denom of 6 no matter what.
fn generate_irreducible_numerator(num: &Kind, denom: i32, exclusions: &[i32]) -> i32 {
    let is_irreducible = |n: i32| math::utils::gcd(n, denom) == 1;
    let is_integer = |n: i32| n % denom == 0;
    let mut rng = rand::rng();

    match num {
        Kind::NotDefined => {
            tracing::error!(
                "Called get_irreducible_numerator() on a FractionGenerator but numerator wasn't set"
            );
            0
        }
        Kind::Single(n) => *n,
        Kind::Multiple(vec) => vec
            .iter()
            .copied()
            .filter(|&n| is_irreducible(n) && !is_integer(n) && !exclusions.contains(&n))
            .choose(&mut rng)
            .unwrap(),
        Kind::Range(min, max) => (*min..=*max)
            .filter(|&n| is_irreducible(n) && !is_integer(n) && !exclusions.contains(&n))
            .choose(&mut rng)
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_generate() {
        let mut frac = fraction().denom(5);
        for _ in 0..10 {
            let (num, denom) = frac.random();
            assert!(num > 0 && num < 5);
            assert_eq!(denom, 5);
        }
    }

    #[test]
    fn defaults_to_irreducible() {
        let mut frac = fraction().denom(6);
        for _ in 0..10 {
            let (num, denom) = frac.random();
            assert!(num == 1 || num == 5);
            assert_eq!(denom, 6);
        }
    }

    #[test]
    fn reducible_works() {
        let mut found_two = false;
        let mut frac = fraction().denom(4).reducible();
        for _ in 0..100 {
            let (num, _) = frac.random();
            if num == 2 {
                found_two = true;
                break;
            }
        }
        assert!(found_two);
    }

    #[test]
    fn exclude_num() {
        let mut frac = fraction().denom(3).exclude_num(2);
        for _ in 0..10 {
            let (num, _) = frac.random();
            assert_eq!(num, 1);
        }
    }

    #[test]
    fn exclude_nums() {
        let mut frac = fraction().denom(4).exclude_nums(&[2, 3]);
        for _ in 0..10 {
            let (num, _) = frac.random();
            assert_eq!(num, 1);
        }
    }

    #[test]
    fn exclude_denom() {
        let mut frac = fraction().denom_range(3, 5).exclude_denom(4);
        for _ in 0..10 {
            let (_, denom) = frac.random();
            assert!(denom == 3 || denom == 5);
        }
    }

    #[test]
    fn exclude_denoms() {
        let mut frac = fraction().denom_range(5, 7).exclude_denoms(&[5, 6]);
        for _ in 0..10 {
            let (_, denom) = frac.random();
            assert_eq!(denom, 7);
        }
    }

    #[test]
    fn denom_not_set() {
        let (num, denom) = fraction().num(2).random();
        assert_eq!(num, 2);
        assert_eq!(denom, 0);
    }

    #[test]
    fn min_max_works() {
        let mut frac = fraction().denom(7).min(2).max(5);
        for _ in 0..100 {
            let (num, denom) = frac.random();
            assert!(num >= 15 && num <= 34 && num % 7 > 0);
            assert_eq!(denom, 7);
        }

        let mut frac = fraction().denom(3).min(-2).max(-1);
        for _ in 0..10 {
            let (num, denom) = frac.random();
            assert!(num == -4 || num == -5);
            assert_eq!(denom, 3);
        }
    }
}
