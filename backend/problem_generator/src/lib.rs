pub mod generator;
pub mod ma1;
pub mod ma2;
pub mod macros;
pub mod picker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, PartialOrd)]
pub enum Difficulty {
    Intro,
    #[default]
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn all() -> [Difficulty; 4] {
        [
            Difficulty::Intro,
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
        ]
    }

    /// Returns the numeric values that are associated with a certain [`Difficulty`].
    pub fn enum_to_nums(difficulty: Difficulty) -> Vec<u8> {
        match difficulty {
            Difficulty::Intro => vec![0, 1],
            Difficulty::Easy => vec![2, 3, 4],
            Difficulty::Medium => vec![5, 6, 7],
            Difficulty::Hard => vec![8, 9, 10],
        }
    }

    /// Converts a `starting_difficulty` and an `ending_difficulty` into a range of numbers,
    /// collected into a [`Vec`].
    pub fn enums_to_nums(
        starting_difficulty: Difficulty,
        ending_difficulty: Difficulty,
    ) -> Vec<u8> {
        let minimum_number = match starting_difficulty {
            Difficulty::Intro => 0,
            Difficulty::Easy => 2,
            Difficulty::Medium => 5,
            Difficulty::Hard => 8,
        };

        let maximum_number = match ending_difficulty {
            Difficulty::Intro => 1,
            Difficulty::Easy => 4,
            Difficulty::Medium => 7,
            Difficulty::Hard => 10,
        };

        (minimum_number..=maximum_number).collect()
    }
}
