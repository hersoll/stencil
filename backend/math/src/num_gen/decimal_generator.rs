use crate::{
    Number, float_to_int,
    num_gen::{common::NumberGenerator, generate_value},
};

use super::common::NumberKind;

pub struct DecimalGenerator {
    /// Which numbers can the generator choose from?
    ///
    /// See [`NumberKind`] for more info.
    numbers: NumberKind,
    /// How many decimals will the generated number have?
    decimal_places: u8,
}

/// Typestate version where decimal places and range/choices have been set. Allows the user to call `.random()`,
/// `.and_random()`, etc. as well as `.exclude()`.
///
/// See [`DecimalGenerator`] for info on the fields
pub struct FinishedDecimalGenerator {
    numbers: NumberKind,
    exclusions: Vec<i32>,
    decimal_places: u8,
}

impl Default for DecimalGenerator {
    fn default() -> Self {
        Self {
            numbers: NumberKind::NotDefined,
            decimal_places: 3,
        }
    }
}

/// Generate a random decimal number depending on the parameters given in the builder.
///
/// # Examples
///
/// ```
/// use math::num_gen;
/// use math::num_gen::NumberGenerator;
/// // Default is three decimal places
/// let num = num_gen::decimal().range(1.234, 1.236).random();
/// assert!(num == 1.234 || num == 1.235 || num == 1.236); // Range is inclusive
///
/// let num = num_gen::decimal().with_decimals(2).range(1.2, 1.3).random();
/// assert!(num > 1.2 && num < 1.3); // Forces 2 decimal places, num can't be 1.2
///
/// let nums = num_gen::decimal().choose(&[1.2, 1.54, 1.456]);
/// let num_one = nums.random();
/// let num_two = nums.random(); // random() can be called multiple times on the same range
/// assert!(num_one == 1.2 || num_one == 1.54 || num_one == 1.456);
///
/// let nums = num_gen::decimal().with_decimals(1).range(-0.1, 0.5).exclude_multiple(&[0.1, 0.2, 0.4]);
/// let num = nums.random();
/// let num_negative = nums.negative();
/// assert!(num == -0.1 || num == 0.3 || num == 0.5);
/// assert_eq!(num_negative, -0.1);
/// ```
pub fn decimal() -> DecimalGenerator {
    DecimalGenerator::default()
}

impl DecimalGenerator {
    /// Forces the generated number to have the given amount of decimals
    ///
    /// For example, prevents the generator from producing 0.3 when decimal places is 2.
    pub fn with_decimals(mut self, places: u8) -> Self {
        self.decimal_places = places;
        self
    }

    /// Choose from the provided numbers when generating.
    pub fn choose(mut self, numbers: &[f64]) -> FinishedDecimalGenerator {
        let int_numbers: Vec<i32> = numbers
            .iter()
            .map(|num| float_to_int(*num, self.decimal_places))
            .collect();

        self.numbers = NumberKind::Multiple(int_numbers);
        FinishedDecimalGenerator {
            numbers: self.numbers,
            exclusions: Vec::new(),
            decimal_places: self.decimal_places,
        }
    }

    /// Makes the generator pick a number in the specified range. Inclusive.
    ///
    /// The reason this is `impl Into<Number>` and not `f64` is to allow for dependency on other
    /// generated numbers. For example `.range(k, 5.0)`.
    pub fn range(
        mut self,
        min: impl Into<Number>,
        max: impl Into<Number>,
    ) -> FinishedDecimalGenerator {
        let mut min = min.into().to_decimal().round(self.decimal_places);
        let mut max = max.into().to_decimal().round(self.decimal_places);
        if min > max {
            tracing::warn!("Called num_gen::decimal().range() with min and max swapped!");
            std::mem::swap(&mut min, &mut max);
        }
        if let (
            Number::Decimal {
                integer: min_num, ..
            },
            Number::Decimal {
                integer: max_num, ..
            },
        ) = (min, max)
        {
            self.numbers = NumberKind::Range(min_num, max_num, 1);
        } else {
            tracing::error!(
                "Somehow, the num_gen::decimal().range() failed to convert both min and max to Decimals!"
            );
            self.numbers = NumberKind::Single(0);
        }

        FinishedDecimalGenerator {
            numbers: self.numbers,
            exclusions: Vec::new(),
            decimal_places: self.decimal_places,
        }
    }
}

impl FinishedDecimalGenerator {
    /// Exclude a specific number from generation
    pub fn exclude(mut self, num: f64) -> Self {
        self.exclusions.push(float_to_int(num, self.decimal_places));
        self
    }

    /// Exclude several numbers from generation.
    pub fn exclude_multiple(mut self, nums: &[f64]) -> Self {
        let mut nums: Vec<i32> = nums
            .iter()
            .map(|num| float_to_int(*num, self.decimal_places))
            .collect();
        self.exclusions.append(&mut nums);
        self
    }

    /// Generate a positive value (0 inclusive).
    pub fn positive(&self) -> Number {
        self.generate_decimal_with_filters(&[|n| *n >= 0])
    }

    /// Generate a negative value (0 inclusive)
    pub fn negative(&self) -> Number {
        self.generate_decimal_with_filters(&[|n| *n <= 0])
    }

    /// Helper function, used in `.random()` and its derivatives
    fn generate_decimal_with_filters(&self, extra_filters: &[fn(&i32) -> bool]) -> Number {
        let mut filters: Vec<fn(&i32) -> bool> = vec![|n| n % 10 != 0];
        filters.extend_from_slice(extra_filters);

        let int32 = generate_value(&self.numbers, &self.exclusions, &filters);
        Number::Decimal {
            integer: int32,
            decimals: self.decimal_places,
        }
    }
}

impl NumberGenerator for FinishedDecimalGenerator {
    fn random(&self) -> Number {
        self.generate_decimal_with_filters(&[])
    }
    fn and_random(self) -> (Number, Self) {
        (self.random(), self)
    }

    #[allow(clippy::cast_sign_loss)]
    fn len(&self) -> usize {
        match &self.numbers {
            NumberKind::NotDefined => 0,
            NumberKind::Single(_) => 1,
            NumberKind::Multiple(vec) => vec.len() - self.exclusions.len(),
            NumberKind::Range(min, max, step) => {
                (*min..=*max).step_by(*step).count() - self.exclusions.len()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num_gen;

    #[test]
    fn range() {
        let decimal_range = num_gen::decimal().with_decimals(2).range(1.2, 1.4);
        for _ in 0..100 {
            let num = decimal_range.random();
            assert!(num.value() >= 1.2 && num.value() <= 1.4);
            match num {
                Number::Decimal { integer, .. } => assert!(integer % 10 != 0),
                _ => panic!("num is not a Decimal"),
            }
        }

        let decimal_range = num_gen::decimal().with_decimals(1).range(1.27, 1.34);
        for _ in 0..10 {
            let num = decimal_range.random();
            assert!(num.value() == 1.3);
        }
    }
}
