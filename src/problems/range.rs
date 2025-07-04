use rand::seq::IndexedRandom;

pub struct IntRange {
    min: i32,
    max: i32,
    exclude: Vec<i32>,
}

impl IntRange {
    pub fn with_zero(min: i32, max: i32) -> IntRange {
        Self::new(min, max, Vec::new())
    }

    pub fn without_zero(min: i32, max: i32) -> IntRange {
        Self::new(min, max, vec![0])
    }

    pub fn without_ones(min: i32, max: i32) -> IntRange {
        Self::new(min, max, vec![1, -1])
    }

    pub fn without_ones_and_zero(min: i32, max: i32) -> IntRange {
        Self::new(min, max, vec![-1, 0, 1])
    }

    fn new(min: i32, max: i32, wanted_exclusions: Vec<i32>) -> IntRange {
        assert!(min <= max, "min must be smaller than max!");
        IntRange {
            min,
            max,
            exclude: Self::get_exclusions_in_range(wanted_exclusions, &min, &max),
        }
    }

    /// Makes sure the integers in `exclude` are part of the range
    ///
    /// This is important since the number of exclusions is part of the `len` calculation. If there
    /// are unnecessary exclusions, `len` gets the wrong number.
    fn get_exclusions_in_range(wanted_exclusions: Vec<i32>, min: &i32, max: &i32) -> Vec<i32> {
        wanted_exclusions
            .into_iter()
            .filter(|num| min <= num && max >= num)
            .collect()
    }

    pub fn len(&self) -> usize {
        (self.max + 1 - self.min - self.exclude.len() as i32) as usize
    }

    pub fn values(&self) -> Vec<i32> {
        (self.min..=self.max)
            .filter(|num| !self.exclude.contains(num))
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
