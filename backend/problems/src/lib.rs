use rand::Rng;

pub mod basic;
pub mod generator;
pub mod ma1;
pub mod ma2;
pub mod macros;
pub mod picker;
pub mod split_strings;

/// Shuffles the order of two items. Returns true if switched.
///
/// Utility function for problems to "make two problems" at once, instead of
/// having separate problems for A + B and B + A
fn shuffle<T>(item_1: &mut T, item_2: &mut T) -> bool {
    let mut rng = rand::rng();
    if rng.random::<f32>() > 0.5 {
        std::mem::swap(&mut *item_1, &mut *item_2);
        true
    } else {
        false
    }
}
