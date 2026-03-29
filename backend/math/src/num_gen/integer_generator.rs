use super::common::{NumberKind, generate_value};

pub struct IntegerGenerator {
    pub numbers: NumberKind,
    pub exclusions: Vec<i32>,
}

/// Generate a random integer depending on the parameters given in the builder.
///
/// # Examples
///
/// ```
/// let num = stencil::math::num_gen::integer().range(4, 9).random();
/// assert!(num >= 4 && num <= 9); // Range is inclusive
/// ```
/// ```
/// let nums = stencil::math::num_gen::integer().numbers(&[3, 5, 7]);
/// let num_one = nums.random();
/// let num_two = nums.random(); // random() can be called multiple times on the same range
/// assert!(num_one == 3 || num_one == 5 || num_one == 7);
/// ```
/// ```
/// let nums = stencil::math::num_gen::integer().range(-3, 2).exclude_multiple(&[-2, -1, 0, 1]);
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

    pub fn range(mut self, min: i32, max: i32) -> Self {
        self.numbers = NumberKind::Range(min, max);
        self
    }

    pub fn exclude(mut self, num: i32) -> Self {
        self.exclusions.push(num);
        self
    }

    pub fn exclude_multiple(mut self, nums: &[i32]) -> Self {
        let mut nums = Vec::from(nums);
        self.exclusions.append(&mut nums);
        self
    }

    pub fn random(&self) -> i32 {
        generate_value(&self.numbers, &self.exclusions, &[] as &[fn(&i32) -> bool])
    }

    pub fn positive(&self) -> i32 {
        generate_value(&self.numbers, &self.exclusions, &[|n| *n >= 0])
    }

    pub fn negative(&self) -> i32 {
        generate_value(&self.numbers, &self.exclusions, &[|n| *n <= 0])
    }

    pub fn len(&self) -> usize {
        match &self.numbers {
            NumberKind::NotDefined => 0,
            NumberKind::Single(_) => 1,
            NumberKind::Multiple(vec) => vec.len() - self.exclusions.len(),
            NumberKind::Range(min, max) => (max - min) as usize - self.exclusions.len(),
        }
    }
}
