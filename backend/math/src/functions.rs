use std::fmt::Display;

use crate::{
    Number,
    symbols::{self, Symbol},
};

pub struct Function {
    /// The output [`Symbol`], generally the "y variable"
    pub name: &'static Symbol,
    /// The input [`Symbol`], generally the "x variable"
    pub variable: &'static Symbol,
    /// Determines whether to print the function as `y(x) = kx + m` (`true`) or `y = kx + m` (`false`)
    show_as_function: bool,
    /// Which kind of function is it?
    pub kind: FunctionKind,
}

/// The FunctionKind enum contains information about which kind of function it is (duh), but also numbers
/// that are specific to that kind of function. This makes ergonomics easier when matching over the
/// kinds since you can do FunctionKind::Linear {k, m} => ...
pub enum FunctionKind {
    Linear { k: Number, m: Number },
    Exponential { c: Number, a: Number },
}

impl Default for Function {
    fn default() -> Self {
        Self {
            name: symbols::Y,
            variable: symbols::X,
            show_as_function: false,
            kind: FunctionKind::Linear {
                k: Number::Integer(1),
                m: Number::Integer(0),
            },
        }
    }
}
impl Function {
    /// Ergonomic constructor
    pub fn linear(k: impl Into<Number>, m: impl Into<Number>) -> Function {
        let k = k.into();
        let m = m.into();

        Function {
            kind: FunctionKind::Linear { k, m },
            ..Default::default()
        }
    }

    /// Ergonomic constructor
    pub fn exponential(c: impl Into<Number>, a: impl Into<Number>) -> Function {
        let c = c.into();
        let mut a = a.into();
        if a <= 0 {
            tracing::error!("a in an exponential function can't be negative (or 0)");
            a = Number::Integer(1)
        }
        Function {
            kind: FunctionKind::Exponential { c, a },
            ..Default::default()
        }
    }
    /// Set the `name` of the [`Function`].
    pub fn with_name(mut self, name: &'static Symbol) -> Self {
        self.name = name;
        self
    }

    /// Set the `variable` of the [`Function`].
    pub fn with_variable(mut self, variable: &'static Symbol) -> Self {
        self.variable = variable;
        self
    }

    /// Makes sure the function prints as `y(x) = ...` rather than `y = ...`
    pub fn with_function_notation(mut self) -> Self {
        self.show_as_function = true;
        self
    }

    /// Makes sure the function prints as `y = ...` rather than `y(x) = ...`
    pub fn without_function_notation(mut self) -> Self {
        self.show_as_function = false;
        self
    }

    /// Finds the x-value(s) of the function, given a certain y-value.
    ///
    /// Returns an empty [`Vec`] if the y-value is outside the domain of the function
    pub fn get_x(&self, y: &Number) -> Vec<Number> {
        match self.kind {
            FunctionKind::Linear { k, m } => {
                // Don't use this when k = 0, please
                if k == 0 { vec![] } else { vec![(*y - m) / k] }
            }
            FunctionKind::Exponential { c, a } => {
                // No solution if y and c have opposite signs, since a positive value can't become
                // negative through exponentiation
                if *y * c < 0 {
                    vec![]
                } else {
                    // y = c a^x => x = lg(y/c) / lg(a)
                    vec![((*y / c).value().log2() / a.value().log2()).into()]
                }
            }
        }
    }

    /// Finds the y-value of the function, given a certain x-value.
    ///
    /// Returns None if the function isn't defined for that x-value, for example x = 0 and f(x) =
    /// 1/x
    ///
    /// Note that this method and [`get_x()`](Self::get_x) are not intended for pretty printing
    /// their results, but rather to be used in [`Graphs`](typst_writer::graphing::Graph) to get coordinates.
    /// As such, the Number might not be formatted properly for printing. Use
    /// [`Evaluable`](super::evaluables::Evaluable) for pretty printing!
    pub fn get_y(&self, x: &Number) -> Option<Number> {
        match self.kind {
            FunctionKind::Linear { k, m } => Some(k * x + m),
            FunctionKind::Exponential { c, a } => Some(c * a.value().powf(x.value())),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let function_body = match self.kind {
            FunctionKind::Linear { k, m } => {
                format!("{kx}{m:+}", kx = k * self.variable)
            }
            FunctionKind::Exponential { c, a } => {
                format!("{c} dot {a}^{x}", x = self.variable)
            }
        };
        if self.show_as_function {
            write!(
                f,
                "{name}({variable}) = {function_body}",
                name = self.name,
                variable = self.variable
            )?;
        } else {
            write!(f, "{name} = {function_body}", name = self.name)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod linear_tests;
