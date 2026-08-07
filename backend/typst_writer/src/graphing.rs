mod axes;
mod graphs;

use std::fmt::Display;

pub use axes::*;
pub use graphs::*;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    West,
    South,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;

        write!(
            f,
            "{}",
            match self {
                North => "north",
                East => "east",
                South => "south",
                West => "west",
                NorthEast => "north-east",
                NorthWest => "north-west",
                SouthEast => "south-east",
                SouthWest => "south-west",
            }
        )
    }
}
