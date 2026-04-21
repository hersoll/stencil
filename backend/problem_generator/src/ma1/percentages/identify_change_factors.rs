use anyhow::Result;
use macros::problem;
use math::{Number, num_gen};
use types::{lang::Language, problems::Problem};

/// Which change factor is equivalent to an increase of 10%?
/// Difficulty: 0
#[problem]
fn integer_increase_to_factor(name: String, lang: &Language) -> Result<Problem> {
    let increase_range = num_gen::integer().range(2, 99);
    let increase = increase_range.random();
    let total_percentage = 100 + increase;
    let factor = total_percentage / 100;
    todo!();
}
