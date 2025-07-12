use crate::{Error, Result};
use rand::seq::IndexedRandom;

/// Range of integers where both limits are inclusive.
///
/// Has the ability to exclude specific values from the range, and these exclusions are accounted
/// for when using `len()`, `values()`, etc.
/// There is currently no support for excluding custom numbers. Use `without_zero()`,
/// `without_ones()` and `without_ones_and_zero()` for exclusions. If your range happens to not
/// include a number, calling a method that excludes it is still valid and will not impact the
/// output or performance.
///
/// # Examples
///
/// ```
/// use app::backend::IntRange;
///
/// let range = IntRange::with_zero(-2,3).unwrap();
/// assert_eq!(range.values(), vec![-2,-1,0,1,2,3]);
///
/// let range = IntRange::without_zero(-2,3).unwrap();
/// assert_eq!(range.values(), vec![-2,-1,1,2,3]);
///
/// let range = IntRange::without_ones_and_zero(-2,3).unwrap();
/// assert_eq!(range.values(), vec![-2,2,3]);
///
/// // with_zero or without_zero will not impact ranges that don't include 0
/// let range = IntRange::with_zero(5,7).unwrap();
/// assert_eq!(range.values(), vec![5,6,7]);
///
/// let range = IntRange::without_ones_and_zero(5,7).unwrap();
/// assert_eq!(range.values(), vec![5,6,7]);
/// ```
pub struct IntRange {
    min: i32,
    max: i32,
    exclude: Vec<i32>,
}

impl IntRange {
    pub fn with_zero(min: i32, max: i32) -> Result<IntRange> {
        Self::new(min, max, Vec::new())
    }

    pub fn without_zero(min: i32, max: i32) -> Result<IntRange> {
        Self::new(min, max, vec![0])
    }

    pub fn without_ones(min: i32, max: i32) -> Result<IntRange> {
        Self::new(min, max, vec![1, -1])
    }

    pub fn without_ones_and_zero(min: i32, max: i32) -> Result<IntRange> {
        Self::new(min, max, vec![-1, 0, 1])
    }

    fn new(min: i32, max: i32, wanted_exclusions: Vec<i32>) -> Result<IntRange> {
        if min > max {
            return Err(Error::InvalidIntRange { min, max });
        }
        Ok(IntRange {
            min,
            max,
            exclude: Self::get_exclusions_in_range(wanted_exclusions, &min, &max),
        })
    }

    /// Passes on the integers in `exclude` that are part of the range
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
        // The +1 is necessary, since a min of 8 and a max of 8 should represent a len of 1
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

    /// Returns a random value alongside the range in a tuple
    ///
    /// Useful for when you want to generate a random value immediately, but also need the range
    /// for later.
    pub fn and_random(self) -> (i32, Self) {
        let random = self.random();
        (random, self)
    }
}
