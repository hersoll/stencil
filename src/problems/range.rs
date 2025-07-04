use rand::seq::IndexedRandom;

pub struct IntRange {
    min: i32,
    max: i32,
    include_zero: bool,
}

impl IntRange {
    pub fn with_zero(min: i32, max: i32) -> IntRange {
        assert!(min <= max, "min must be smaller than max!");
        IntRange {
            min,
            max,
            include_zero: true,
        }
    }

    pub fn without_zero(min: i32, max: i32) -> IntRange {
        assert!(min <= max, "min must be smaller than max!");
        IntRange {
            min,
            max,
            include_zero: false,
        }
    }

    pub fn len(&self) -> usize {
        if !self.include_zero && self.min < 0 && self.max > 0 {
            (self.max - self.min) as usize
        } else {
            (self.max + 1 - self.min) as usize
        }
    }

    pub fn values(&self) -> Vec<i32> {
        (self.min..=self.max)
            .filter(|&num| self.include_zero || num != 0)
            .collect()
    }

    pub fn random(&self) -> i32 {
        assert!(
            self.len() > 0,
            "Trying to access a random number from an empty range."
        );
        let values = self.values();
        let mut rng = rand::rng();
        *values.choose(&mut rng).unwrap()
    }
}
