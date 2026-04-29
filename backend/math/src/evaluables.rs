use super::{Number, symbols::Symbol};

pub type Replacement<'a> = (&'static Symbol, &'a Number);
pub struct Replacements<'a>(Vec<Replacement<'a>>);

impl<'a> Replacements<'a> {
    pub fn from(vec: Vec<Replacement<'a>>) -> Self {
        Replacements(vec)
    }
    /// Returns the first [`Number`] which is in the same tuple as `symbol`.
    ///
    /// Returns `None` if the `symbol` does not appear in the [`Replacements`]
    pub fn get_replacement_for(&self, symbol: &'static Symbol) -> Option<&'a Number> {
        self.0
            .iter()
            .find(|(s, _)| **s == *symbol)
            .map(|(_, num)| *num)
    }
}

/// Prints and evalutes expressions
///
/// An [`Evaluable`] can be any kind of printable expression containing [`Symbols`](Symbol), where the `Symbols` can
/// be replaced with [`Numbers`](Number) and some kind of final result can be evaluated.
pub trait Evaluable {
    /// Prints the expression with the given [`Symbols`](Symbol) replaced with [`Numbers`](Number)
    /// and colors them.
    ///
    /// For example `f(x) = 2x - 1` => `f(3) = 2 dot colored(3) - 1`
    fn print_replacements(&self, replacements: &Replacements) -> String;

    /// Evaluates each "part" of the expression, and prints what each part becomes. Pairs nicely as
    /// a follow up to [`print_replacements()`](Self::print_replacements).
    ///
    /// A "part" can be different for different expressions. For example, for kx + m with k = 2,
    /// x = 3, m = -1, this should print `"6 - 1"`
    ///
    /// If all [`Symbols`](Symbol) are replaced, the result of the printed calculation should be equivalent to the [`Number`] returned by [`evaluate()`](Self::evaluate).
    fn print_evaluation_by_parts(&self, replacements: &Replacements) -> String;

    /// Replaces every [`Symbol`] with a [`Number`] according to the rules provided in `replacements` and calculates the final result.
    ///
    /// # Panics
    /// Should panic if the provided `Symbols` aren't equivalent to the `Symbols` in `self`.
    fn evaluate(&self, replacements: &Replacements) -> Number;
}

#[cfg(test)]
mod tests {
    use crate::symbols;

    use super::*;

    #[test]
    fn get_replacement_from_symbol() {
        let replacements = Replacements::from(vec![
            (symbols::A, &Number::Integer(1)),
            (symbols::B, &Number::Integer(2)),
            (symbols::C, &Number::Integer(3)),
        ]);

        assert!(replacements.get_replacement_for(symbols::A).is_some());
        assert!(replacements.get_replacement_for(symbols::B).is_some());
        assert!(replacements.get_replacement_for(symbols::C).is_some());
        assert!(replacements.get_replacement_for(symbols::A_CAPS).is_none());
        assert!(replacements.get_replacement_for(symbols::X).is_none());
    }
}
