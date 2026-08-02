use crate::{Number, num_gen::common::NumberGenerator};

use super::common::{NumberKind, generate_value};

#[derive(Debug, Clone)]
pub struct IntegerGenerator {
    pub numbers: NumberKind,
    pub exclusions: Vec<i32>,
}

/// Generate a random integer depending on the parameters given in the builder.
///
/// # Examples
///
/// ```
/// use math::num_gen::NumberGenerator;
/// let num = math::num_gen::integer().range(4, 9).random();
/// assert!(num >= 4 && num <= 9); // Range is inclusive
///
/// let nums = math::num_gen::integer().numbers(&[3, 5, 7]);
/// let num_one = nums.random();
/// let num_two = nums.random(); // random() can be called multiple times on the same range
/// assert!(num_one == 3 || num_one == 5 || num_one == 7);
///
/// let nums = math::num_gen::integer().range(-3, 2).exclude_multiple(&[-2, -1, 0, 1]);
/// let num = nums.random();
/// let num_positive = nums.positive();
/// assert!(num == 2 || num == -3);
/// assert_eq!(num_positive, 2);
/// ```
pub fn integer() -> IntegerGenerator {
    IntegerGenerator {
        numbers: NumberKind::NotDefined,
        exclusions: Vec::new(),
    }
}

impl IntegerGenerator {
    pub fn number(mut self, number: i32) -> Self {
        self.numbers = NumberKind::Single(number);
        self
    }

    pub fn numbers(mut self, numbers: &[i32]) -> Self {
        self.numbers = NumberKind::Multiple(Vec::from(numbers));
        self
    }

    /// Alias for range_step(min, max, 1)
    pub fn range(self, min: impl Into<Number>, max: impl Into<Number>) -> Self {
        self.range_step(min, max, 1)
    }

    // The reason this accepts impl Into<Number> while actually just passing on an i32, is that we
    // will sometimes call this method with a Number that depends on a previous Number, like:
    // `range(k, 10)`
    // We need to accomodate for this.
    pub fn range_step(
        mut self,
        min: impl Into<Number>,
        max: impl Into<Number>,
        step: impl Into<Number>,
    ) -> Self {
        let min = min.into();
        let max = max.into();
        let step = step.into();
        if min > max {
            tracing::error!("Called num_gen::integer().range() with min and max swapped!");
        }
        if let (
            Number::Integer(min_number),
            Number::Integer(max_number),
            Number::Integer(step_num),
        ) = (min, max, step)
        {
            self.numbers = NumberKind::Range(min_number, max_number, step_num as usize);
        } else {
            tracing::error!(
                "Don't call num_gen::integer().range() with non-integers, ya dum dum! Min: {min}, Max: {max}"
            );
            self.numbers = NumberKind::Range(1, 1, 1);
        }
        self
    }

    pub fn exclude(mut self, num: impl Into<Number>) -> Self {
        let num = num.into();
        if let Number::Integer(integer_num) = num {
            self.exclusions.push(integer_num);
        } else {
            tracing::error!(
                "Don't call num_gen::integer().exclude() with a non-integer, ya dum dum! Passed value: {num}"
            )
        }
        self
    }

    pub fn exclude_multiple(mut self, nums: &[i32]) -> Self {
        let mut nums = Vec::from(nums);
        self.exclusions.append(&mut nums);
        self
    }

    pub fn positive(&self) -> Number {
        let int32 = generate_value(&self.numbers, &self.exclusions, &[|n: &i32| *n >= 0]);
        Number::Integer(int32)
    }

    pub fn negative(&self) -> Number {
        let int32 = generate_value(&self.numbers, &self.exclusions, &[|n: &i32| *n <= 0]);
        Number::Integer(int32)
    }
}

impl NumberGenerator for IntegerGenerator {
    fn random(&self) -> Number {
        let int32 = generate_value(&self.numbers, &self.exclusions, &[] as &[fn(&i32) -> bool]);
        Number::Integer(int32)
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
