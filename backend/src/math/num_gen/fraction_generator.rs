use rand::seq::{IndexedRandom, IteratorRandom};

use crate::math;

/// How the numerator and denominator can be defined when calling
enum Kind {
    NotDefined,
    Single(i32),
    Multiple(Vec<i32>),
    Range(i32, i32),
    RangeWithExclusions(i32, i32, Vec<i32>),
}

/// The builder type returned by the fraction() function
pub struct FractionGenerator {
    num: Kind,
    denom: Kind,
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
/// let (num, denom) = stencil::math::num_gen::fraction().reducible().denom(6).random();
/// assert!(num >= 1 && num <= 5); // All numerators are allowed now
/// assert!(denom == 6);
/// ```
pub fn fraction() -> FractionGenerator {
    FractionGenerator {
        num: Kind::NotDefined,
        denom: Kind::NotDefined,
        reducible: false,
    }
}

impl FractionGenerator {
    /// Sets the denominator to a single value and matches the numerator to it
    ///
    /// If the numerator is already set, it will not be touched
    pub fn denom(&mut self, denom: i32) -> &mut Self {
        self.denom = Kind::Single(denom);
        if let Kind::NotDefined = self.num {
            self.num = Kind::Range(1, denom.abs() - 1);
        }
        self
    }

    /// Generate a random fraction from the FractionGenerator with the parameters given
    ///
    /// Non-consuming method, so the generator can be used again
    pub fn random(&self) -> (i32, i32) {
        let denom = generate_value(&self.denom);
        // If the fraction is irreducible, we can't just get any old numerator
        let num = match self.reducible {
            false => generate_irreducible_numerator(&self.num, denom),
            true => generate_value(&self.num),
        };

        (num, denom)
    }

    /// Allow the fraction to be reducible
    pub fn reducible(&mut self) -> &mut Self {
        self.reducible = true;
        self
    }
}

/// Returns a numerator which assures the resulting fraction is irreducible.
///
/// This function is a special version of generate_value(), using gcd to make sure that the resulting
/// fraction cannot be reduced; if a denom of 6 is given then the resulting fraction will have a
/// denom of 6 no matter what.
fn generate_irreducible_numerator(num: &Kind, denom: i32) -> i32 {
    let is_irreducible = |n: i32| math::utils::gcd(n, denom) == 1;
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
            .filter(|&n| is_irreducible(n))
            .choose(&mut rng)
            .unwrap(),
        Kind::Range(min, max) => (*min..=*max)
            .filter(|&n| is_irreducible(n))
            .choose(&mut rng)
            .unwrap(),
        Kind::RangeWithExclusions(min, max, exclusions) => (*min..=*max)
            .filter(|&n| is_irreducible(n))
            .filter(|i| !exclusions.contains(&i))
            .choose(&mut rng)
            .unwrap(),
    }
}

fn generate_value(kind: &Kind) -> i32 {
    let mut rng = rand::rng();

    match kind {
        Kind::NotDefined => {
            tracing::error!(
                "Called get_value() on a FractionGenerator but at least one value wasn't set"
            );
            0
        }
        Kind::Single(n) => *n,
        Kind::Multiple(vec) => *vec.choose(&mut rng).unwrap(),
        Kind::Range(min, max) => (*min..=*max).choose(&mut rng).unwrap(),
        Kind::RangeWithExclusions(min, max, exclusions) => (*min..=*max)
            .filter(|i| !exclusions.contains(&i))
            .choose(&mut rng)
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_generate() {
        for _ in 0..10 {
            let (num, denom) = fraction().denom(5).random();
            assert!(num > 0 && num < 5);
            assert_eq!(denom, 5);
        }
    }

    #[test]
    fn defaults_to_irreducible() {
        for _ in 0..10 {
            let (num, denom) = fraction().denom(6).random();
            assert!(num == 1 || num == 5);
            assert_eq!(denom, 6);
        }
    }
}

// Signature for calling this?
//
// We might be a bit nasty and do "ranges" of num/denom by making the signature of numerator() and
// denominator() into Into<Number>, then matching and seeing whether we have an integer(one number)
// or fraction (two numbers). If fraction, we set the min to the first and max to the second
//
// NO, lets just do num(int), nums(vec<int>), num_range(min, max)
//
// let range = num_gen::fraction().denom(5);  Will generate 1/5, 2/5, 3/5, 4/5
// let frac = range.random();
// let negative_frac = range.negative();
//
// let frac = num_gen::fraction().denom(6).random();  Will generate 1/6, 5/6
// let frac = num_gen::fraction().denom(6).max(2).random();  Will generate 1/6, 5/6, 7/6, 11/6
// let frac = num_gen::fraction().denom(6).reducible().random();  Will generate 1/6, 1/3 (2/6), 1/2
// (3/6), 2/3 (4/6), 5/6
