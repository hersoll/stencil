use crate::{Number, float_to_int};

use super::common::{NumberKind, generate_value};

/// The starting type of a type-state situation.
///
/// Makes sure the user calls .with_decimals() before doing anything else
pub struct DecimalGeneratorPrimitive;

pub struct DecimalGenerator {
    numbers: NumberKind,
    exclusions: Vec<i32>,
    decimal_places: u8,
}

// TODO:
// - Rewrite the examples
// - Do we WANT to force with_places? How about when doing .numbers()?
// - Rename numbers() to something better. from_...()?
// - Do I want to change DECIMAL_FACTOR...? Sometimes longer decimals are wanted. And now we can
// control decimal places during generation. May need rounding function in that case, since answers
// will likely max out at 3 decimals a LOT of the time.

/// Generate a random decimal number depending on the parameters given in the builder.
///
/// # Examples
///
/// ```
/// let num = math::num_gen::integer().range(4, 9).random();
/// assert!(num >= 4 && num <= 9); // Range is inclusive
/// ```
/// ```
/// let nums = math::num_gen::integer().numbers(&[3, 5, 7]);
/// let num_one = nums.random();
/// let num_two = nums.random(); // random() can be called multiple times on the same range
/// assert!(num_one == 3 || num_one == 5 || num_one == 7);
/// ```
/// ```
/// let nums = math::num_gen::integer().range(-3, 2).exclude_multiple(&[-2, -1, 0, 1]);
/// let num = nums.random();
/// let num_positive = nums.positive();
/// assert!(num == 2 || num == -3);
/// assert_eq!(num_positive, 2);
/// ```
pub fn decimal() -> DecimalGeneratorPrimitive {
    DecimalGeneratorPrimitive {}
}

impl DecimalGeneratorPrimitive {
    /// Makes sure that the generated `Number` has **at most** `decimals` number of decimals.
    ///
    /// Note that `.with_decimals(2)` can still generate 1.1, since it's equal to 1.10. It can't
    /// generate 1.111, however.
    pub fn with_places(self, places: u8) -> DecimalGenerator {
        DecimalGenerator {
            numbers: NumberKind::NotDefined,
            exclusions: Vec::new(),
            decimal_places: places,
        }
    }
}

impl DecimalGenerator {
    pub fn numbers(mut self, numbers: &[f64]) -> Self {
        let int_numbers: Vec<i32> = numbers.iter().map(|num| float_to_int(*num)).collect();

        self.numbers = NumberKind::Multiple(int_numbers);
        self
    }

    // The reason this accepts impl Into<Number> while actually just passing on an i32, is that we
    // will sometimes call this method with a Number that depends on a previous Number, like:
    // `range(k, 1.34)`
    // We need to accomodate for this.
    pub fn range(mut self, min: impl Into<Number>, max: impl Into<Number>) -> Self {
        let mut min = min.into().to_decimal();
        let mut max = max.into().to_decimal();
        if min > max {
            tracing::error!("Called num_gen::decimal().range() with min and max swapped!");
            std::mem::swap(&mut min, &mut max);
        }
        if let (Number::Decimal(min_num), Number::Decimal(max_num)) = (min, max) {
            self.numbers = NumberKind::Range(min_num, max_num);
        } else {
            tracing::error!(
                "Somehow, the num_gen::decimal().range() failed to convert both min and max to Decimals!"
            )
        }

        self
    }

    pub fn exclude(mut self, num: impl Into<Number>) -> Self {
        let num = num.into().to_decimal();
        if let Number::Decimal(decimal_integer) = num {
            self.exclusions.push(decimal_integer);
        } else {
            tracing::error!("Failed to convert {num} to a Decimal")
        }
        self
    }

    pub fn exclude_multiple(mut self, nums: &[f64]) -> Self {
        let mut nums: Vec<i32> = nums.iter().map(|num| float_to_int(*num)).collect();
        self.exclusions.append(&mut nums);
        self
    }

    pub fn random(&self) -> Number {
        let int32 = generate_value(
            &self.numbers,
            &self.exclusions,
            &[|integer| has_at_most_places(*integer, self.decimal_places.into())],
        );
        Number::Decimal(int32)
    }

    pub fn and_random(self) -> (Number, Self) {
        (self.random(), self)
    }

    pub fn positive(&self) -> Number {
        let int32 = generate_value(&self.numbers, &self.exclusions, &[|n| *n >= 0]);
        Number::Decimal(int32)
    }

    pub fn negative(&self) -> Number {
        let int32 = generate_value(&self.numbers, &self.exclusions, &[|n| *n <= 0]);
        Number::Decimal(int32)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match &self.numbers {
            NumberKind::NotDefined => 0,
            NumberKind::Single(_) => 1,
            NumberKind::Multiple(vec) => vec.len() - self.exclusions.len(),
            NumberKind::Range(min, max) => (1 + max - min) as usize - self.exclusions.len(),
        }
    }
}

// I get 1230
// Check: Is it divisible by 1? Yes -> Has at MOST 3
// Check: Is it divisible by 10? Yes -> Has at MOST 2
// Check: Is it divisible by 100? No -> Does NOT have 1 decimal

/// Checks whether the integer representation of a `Decimal` has the given amount of decimal places
fn has_at_most_places(num: i32, places: u32) -> bool {
    let divisor = crate::DECIMAL_FACTOR / 10i32.pow(places);
    num % divisor == 0
}

#[cfg(test)]
mod tests {
    use crate::num_gen;

    use super::*;

    #[test]
    fn helper_function_works() {
        assert!(has_at_most_places(1234, 3));
        assert!(!has_at_most_places(1234, 2));
        assert!(has_at_most_places(1230, 3));
        assert!(has_at_most_places(1230, 2));
        assert!(!has_at_most_places(1230, 1));
        assert!(has_at_most_places(1200, 3));
        assert!(has_at_most_places(1200, 2));
        assert!(has_at_most_places(1200, 1));
        assert!(!has_at_most_places(1200, 0));
        assert!(has_at_most_places(1000, 3));
        assert!(has_at_most_places(1000, 2));
        assert!(has_at_most_places(1000, 1));
        assert!(has_at_most_places(1000, 0));
    }

    #[test]
    fn range() {
        let decimal_range = num_gen::decimal().with_places(2).range(1.2, 1.4);
        for _ in 0..100 {
            let num = decimal_range.random();
            assert!(num.value() >= 1.2 && num.value() <= 1.4);
            match num {
                Number::Decimal(d) => assert!(has_at_most_places(d, 2)),
                _ => panic!("num is not a Decimal"),
            }
        }

        let decimal_range = num_gen::decimal().with_places(1).range(1.27, 1.34);
        for _ in 0..10 {
            let num = decimal_range.random();
            assert!(num.value() == 1.3);
        }
    }
}
