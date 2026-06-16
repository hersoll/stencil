use serde::{Deserialize, Serialize};

/// Designates how hard a problem is relative to the other problems in that topic
///
/// Relative difficulties aren't bounded - they should always increase when a problem is slightly
/// harder than the previously one. This ensures smooth onboarding during practice.
///
/// This means that relative difficulties and absolute difficulties won't match. A topic can have
/// ten problems with absolute difficulty 3, but these problems could have relative difficulties of
/// 15 - 24 if there are fourteen easier problems in that topic.
#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct RelativeDifficulty {
    pub number: u8,
}
impl std::fmt::Debug for RelativeDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

/// Designates how hard a problem is in the course
///
/// While we want to be able to differentiate between problems and slightly harder problems (see
/// [`RelativeDifficulty`]), we also want a good representation of where a certain problem lies in
/// the course, difficulty-wise. This ensures that when topics are mixed, problems are grouped
/// together appropriately.
///
/// Note that absolute difficulties **are** bounded (1-10). See [`DifficultyCategory`] for their
/// meanings.
#[derive(Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct AbsoluteDifficulty {
    pub number: u8,
}

impl std::fmt::Debug for AbsoluteDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

/// User-facing enum for categorizing difficulties
///
/// While the program uses [`RelativeDifficulty`] and [`AbsoluteDifficulty`] to sort and select
/// problems for the stencil, these numbers aren't shown to the user (yet...). Instead, the HTTP
/// request will contain a category (`"intro", "easy", "medium", "hard"`).
///
/// This enum maps the categories to the [`AbsoluteDifficulty`] numbers (note that
/// [`RelativeDifficulty`] does not correlate to specific categories in that way):
///
/// Intro: 1, 2, 3
/// Easy: 4, 5
/// Medium: 6, 7
/// Hard: 8, 9, 10
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, PartialOrd)]
pub enum DifficultyCategory {
    Intro,
    #[default]
    Easy,
    Medium,
    Hard,
}

/// Required for DB serializing
impl From<i32> for RelativeDifficulty {
    fn from(value: i32) -> Self {
        let u8_value: u8 = value.try_into().unwrap_or_default();
        RelativeDifficulty::from_num(u8_value)
    }
}
/// Required for DB serializing
impl From<i32> for AbsoluteDifficulty {
    fn from(value: i32) -> Self {
        let u8_value: u8 = value.try_into().unwrap_or_default();
        AbsoluteDifficulty::from_num(u8_value)
    }
}

impl RelativeDifficulty {
    /// Constructor method
    pub fn from_num(number: u8) -> Self {
        if number < 1 {
            tracing::error!("RelativeDifficulty recieved a number outside its bounds: {number}");
            return Self { number: 1 };
        }
        Self { number }
    }
}

impl AbsoluteDifficulty {
    /// Constructor method
    pub fn from_num(number: u8) -> Self {
        if !(1..=10).contains(&number) {
            tracing::error!("AbsoluteDifficulty recieved a number outside its bounds: {number}");
            return Self { number: 1 };
        }
        Self { number }
    }
}

impl DifficultyCategory {
    /// Returns every [`DifficultyCategory`] for iteration purposes
    pub fn get_all_categories() -> [DifficultyCategory; 4] {
        [
            DifficultyCategory::Intro,
            DifficultyCategory::Easy,
            DifficultyCategory::Medium,
            DifficultyCategory::Hard,
        ]
    }

    /// Returns the [`AbsoluteDifficulty`] values that are associated with a certain [`DifficultyCategory`].
    pub fn to_absolute_difficulties(&self) -> Vec<AbsoluteDifficulty> {
        let numbers = match self {
            DifficultyCategory::Intro => vec![1, 2, 3],
            DifficultyCategory::Easy => vec![4, 5],
            DifficultyCategory::Medium => vec![6, 7],
            DifficultyCategory::Hard => vec![8, 9, 10],
        };
        numbers
            .into_iter()
            .map(AbsoluteDifficulty::from_num)
            .collect()
    }

    pub fn to_minimum_difficulty_num(difficulty_category: &DifficultyCategory) -> u8 {
        match difficulty_category {
            DifficultyCategory::Intro => 1,
            DifficultyCategory::Easy => 4,
            DifficultyCategory::Medium => 6,
            DifficultyCategory::Hard => 8,
        }
    }

    pub fn to_maximum_difficulty_num(difficulty_category: &DifficultyCategory) -> u8 {
        match difficulty_category {
            DifficultyCategory::Intro => 3,
            DifficultyCategory::Easy => 5,
            DifficultyCategory::Medium => 7,
            DifficultyCategory::Hard => 10,
        }
    }
    /// Returns every [`AbsoluteDifficulty`] in the span of a `starting_difficulty` and an `ending_difficulty`.
    pub fn categories_to_absolute_difficulties(
        starting_difficulty: &DifficultyCategory,
        ending_difficulty: &DifficultyCategory,
    ) -> Vec<AbsoluteDifficulty> {
        let minimum_number = Self::to_minimum_difficulty_num(starting_difficulty);
        let maximum_number = Self::to_maximum_difficulty_num(ending_difficulty);

        (minimum_number..=maximum_number)
            .map(AbsoluteDifficulty::from_num)
            .collect()
    }
}
