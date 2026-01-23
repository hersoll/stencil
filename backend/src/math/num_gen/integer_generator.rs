use super::common::{Kind, generate_value};

pub struct IntegerGenerator {
    numbers: Kind,
    exclusions: Vec<i32>,
}

pub fn integer() -> IntegerGenerator {
    IntegerGenerator {
        numbers: Kind::NotDefined,
        exclusions: Vec::new(),
    }
}

impl IntegerGenerator {
    pub fn numbers(mut self, numbers: &[i32]) -> Self {
        self.numbers = Kind::Multiple(Vec::from(numbers));
        self
    }

    pub fn range(mut self, min: i32, max: i32) -> Self {
        self.numbers = Kind::Range(min, max);
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

    pub fn random(&mut self) -> i32 {
        generate_value(&self.numbers, &self.exclusions)
    }

    pub fn len(&self) -> usize {
        match &self.numbers {
            Kind::NotDefined => 0,
            Kind::Single(_) => 1,
            Kind::Multiple(vec) => vec.len() - self.exclusions.len(),
            Kind::Range(min, max) => (max - min) as usize - self.exclusions.len(),
        }
    }
}
