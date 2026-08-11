mod evaluables;
pub mod functions;
pub mod num_gen;
mod numbers;
mod polynomials;
pub mod symbols;
mod terms;
pub mod utils;
mod variables;

use std::fmt::Display;

pub use evaluables::*;
pub use numbers::*;
pub use polynomials::Polynomial;
pub use terms::Term;
pub use variables::PolynomialVariable;
pub use variables::VariableList;

/// Allows for every type with Display to immediately be printed as Typst math output: $ num $
///
/// This is mostly useful for individual elements which need to math formatted on their own. When
/// you do something like `format!("${num_1} dot {num_2}$")` you still need the `format` to parse
/// the parameters correctly, so the equivalent function call would be
/// `format!("{num_1} dot {num_2}").as_math()`
pub trait MathDisplay: Display {
    fn as_math(&self) -> String {
        format!("${self}$")
    }

    fn as_block_math(&self) -> String {
        format!("#block($ {self} $)")
    }
}

impl<T: Display> MathDisplay for T {}

/// Trait for numeric and algebraic types which has some kind of numeric value up front
///
/// Useful for knowing when to add parentheses around something, for example,
/// we might want to print 3 + 2x but also 3 + (-2x)
pub trait HasCoef {
    fn coef(&self) -> Number;
}

impl HasCoef for i32 {
    fn coef(&self) -> Number {
        Number::from(*self)
    }
}

impl HasCoef for f64 {
    fn coef(&self) -> Number {
        Number::from(*self)
    }
}
