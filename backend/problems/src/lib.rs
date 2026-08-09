use math::Number;
use rand::Rng;

pub mod generator;
pub mod ma1;
pub mod ma2;
pub mod macros;
pub mod picker;
pub mod split_strings;

/// Shuffles the order of two numbers. Returns true if switched.
///
/// Utility function for problems to "make two problems" at once, instead of
/// having separate problems for A + B and B + A
fn shuffle_numbers(num_1: &mut Number, num_2: &mut Number) -> bool {
    let mut rng = rand::rng();
    if rng.random::<f32>() > 0.5 {
        std::mem::swap(&mut *num_1, &mut *num_2);
        true
    } else {
        false
    }
}
