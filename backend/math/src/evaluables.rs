use super::{Number, symbols::Symbol};

type Replacement<'a> = (&'static Symbol, &'a Number);

/// Prints and evalutes expressions
///
/// An [`Evaluable`] can be any kind of printable expression containing [`Symbols`](Symbol), where the `Symbols` can
/// be replaced with [`Numbers`](Number) and some kind of final result can be evaluated.
pub trait Evaluable {
    /// Prints the expression with the given [`Symbols`](Symbol) replaced with [`Numbers`](Number)
    /// and colors them.
    ///
    /// For example `f(x) = 2x - 1` => `f(3) = 2 dot colored(3) - 1`
    fn print_replacements(&self, replacements: &[Replacement]) -> String;

    /// Evaluates each "part" of the expression, and prints what each part becomes. Pairs nicely as
    /// a follow up to [`print_replacements()`](Self::print_replacements).
    ///
    /// A "part" can be different for different expressions. For example, for kx + m with k = 2,
    /// x = 3, m = -1, this should print `"6 - 1"`
    ///
    /// The result of the printed calculation should be equivalent to the [`Number`] returned by [`evaluate()`](Self::evaluate).
    fn print_evaluation_by_parts(&self, replacements: &[Replacement]) -> String;

    /// Replaces every [`Symbol`] with a [`Number`] according to the rules provided in `replacements` and calculates the final result.
    ///
    /// # Panics
    /// Should panic if the provided `Symbols` aren't equivalent to the `Symbols` in `self`.
    fn evaluate(&self, replacements: &[Replacement]) -> Number;
}
