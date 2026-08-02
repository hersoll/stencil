use rand::seq::IteratorRandom;

use crate::Number;

#[derive(Debug, Clone)]
pub enum NumberKind {
    NotDefined,
    Single(i32),
    Multiple(Vec<i32>),
    Range(i32, i32),
}

pub trait NumberGenerator {
    /// Generate a random number from the previously configured parameters.
    fn random(&self) -> Number;
    /// Generate a random value, and also pass on the `NumberGenerator` object.
    fn and_random(self) -> (Number, Self);

    /// Returns the number of choices the generator has to choose from.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn generate_value<F>(kind: &NumberKind, exclusions: &[i32], filters: &[F]) -> i32
where
    F: Fn(&i32) -> bool,
{
    let mut rng = rand::rng();

    let apply_filters = |n: &i32| -> bool {
        !exclusions.contains(n) && (filters.is_empty() || filters.iter().all(|f| f(n)))
    };

    match kind {
        NumberKind::NotDefined => {
            tracing::error!("Called get_value() on a generator with Kind::NotDefined");
            0
        }
        NumberKind::Single(n) => *n,
        NumberKind::Multiple(vec) => *vec
            .iter()
            .filter(|&&n| apply_filters(&n))
            .choose(&mut rng)
            .unwrap(),
        NumberKind::Range(min, max) => (*min..=*max)
            .filter(apply_filters)
            .choose(&mut rng)
            .unwrap(),
    }
}
