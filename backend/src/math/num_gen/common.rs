use rand::seq::IteratorRandom;

pub enum Kind {
    NotDefined,
    Single(i32),
    Multiple(Vec<i32>),
    Range(i32, i32),
}

pub fn generate_value(kind: &Kind, exclusions: &[i32]) -> i32 {
    let mut rng = rand::rng();

    match kind {
        Kind::NotDefined => {
            tracing::error!("Called get_value() on a generator with Kind::NotDefined");
            0
        }
        Kind::Single(n) => *n,
        Kind::Multiple(vec) => *vec
            .iter()
            .filter(|n| !exclusions.contains(&n))
            .choose(&mut rng)
            .unwrap(),
        Kind::Range(min, max) => (*min..=*max)
            .filter(|n| !exclusions.contains(&n))
            .choose(&mut rng)
            .unwrap(),
    }
}
