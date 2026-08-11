use std::fmt::Display;

use rand::seq::IndexedRandom;

#[derive(Debug, Copy, Clone)]
pub enum InequalitySign {
    Greater,
    Less,
    Geq,
    Leq,
}

impl InequalitySign {
    /// Returns a new InequalitySign based on `self`
    ///
    /// Does not mutate `self`!
    pub fn swapped(&self) -> InequalitySign {
        use InequalitySign::*;
        match self {
            Greater => Less,
            Less => Greater,
            Geq => Leq,
            Leq => Geq,
        }
    }
    pub fn random() -> InequalitySign {
        let mut rng = rand::rng();
        *[
            InequalitySign::Greater,
            InequalitySign::Less,
            InequalitySign::Geq,
            InequalitySign::Leq,
        ]
        .choose(&mut rng)
        .unwrap_or(&InequalitySign::Less) // Should never be reached
    }

    /// Returns either > or <
    pub fn strict() -> InequalitySign {
        let mut rng = rand::rng();
        *[InequalitySign::Greater, InequalitySign::Less]
            .choose(&mut rng)
            .unwrap_or(&InequalitySign::Less) // Should never be reached
    }

    /// Returns either >= or <=
    pub fn non_strict() -> InequalitySign {
        let mut rng = rand::rng();
        *[InequalitySign::Geq, InequalitySign::Leq]
            .choose(&mut rng)
            .unwrap_or(&InequalitySign::Leq) // Should never be reached
    }

    pub fn random_less() -> InequalitySign {
        let mut rng = rand::rng();
        *[InequalitySign::Less, InequalitySign::Leq]
            .choose(&mut rng)
            .unwrap_or(&InequalitySign::Less) // Should never be reached
    }

    pub fn random_greater() -> InequalitySign {
        let mut rng = rand::rng();
        *[InequalitySign::Greater, InequalitySign::Geq]
            .choose(&mut rng)
            .unwrap_or(&InequalitySign::Greater) // Should never be reached
    }

    pub fn is_less(&self) -> bool {
        use InequalitySign::*;
        matches!(self, Less | Leq)
    }

    pub fn is_greater(&self) -> bool {
        use InequalitySign::*;
        matches!(self, Greater | Geq)
    }
}

impl Display for InequalitySign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InequalitySign::*;
        write!(
            f,
            "{}",
            match self {
                Greater => ">",
                Less => "<",
                Geq => ">=",
                Leq => "<=",
            }
        )
    }
}
