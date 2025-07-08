mod macros;

pub mod document;
pub mod problems;
pub mod utils;

pub use document::typst_formatting::equation_solution;
pub use problems::Difficulty;
pub use problems::SetBuilder;
pub use utils::IntRange;
