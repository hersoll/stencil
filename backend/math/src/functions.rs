use std::fmt::Display;

use crate::{
    Number, Term,
    evaluables::{Evaluable, Replacements},
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

    /// Returns the declaration of the function, either `f(x)` or `y`
    /// (depending on the fields `name` and `variable`, of course).
    pub fn get_declaration(&self) -> String {
        if self.show_as_function {
            format!("{f}({x})", f = self.name, x = self.variable)
        } else {
            format!("{y}", y = self.name)
        }
    }

    /// Returns the body of the function with the designated variable,
    /// i.e `3x + 1` or `-2 dot 1.07^x`
    pub fn get_function_body(&self) -> String {
        match self.kind {
            FunctionKind::Linear { k, m } => {
                let poly = (k * self.variable).and(&Term::from_num(m));
                poly.to_string()
            }
            FunctionKind::Exponential { c, a } => {
                format!("{c} dot {a}^{x}", x = self.variable)
            }
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{declaration} = {function_body}",
            declaration = self.get_declaration(),
            function_body = self.get_function_body()
        )
    }
}

impl Evaluable for Function {
    fn print_replacements(&self, replacements: &Replacements) -> String {
        // Should y be replaced with a number?
        let declaration = if let Some(y_value) = replacements.get_replacement_for(self.name) {
            format!("colored({y_value})")
        } else if self.show_as_function // Can we print f(4)?
            && let Some(x_value) = replacements.get_replacement_for(self.variable)
        {
            format!("{f}(colored({x_value}))", f = self.name)
        } else {
            self.get_declaration()
        };
        // Should x be replaced with a number?
        let body = if let Some(x_value) = replacements.get_replacement_for(self.variable) {
            let x = if *x_value < 0 {
                format!("colored(({x_value}))")
            } else {
                format!("colored({x_value})")
            };
            match self.kind {
                FunctionKind::Linear { k, m } => {
                    let m = Term::from_num(m);
                    if k == 0 {
                        format!("{m}")
                    } else if k == 1 {
                        format!("{x} {m:+}")
                    } else {
                        format!("{k} dot {x} {m:+}")
                    }
                }
                FunctionKind::Exponential { c, a } => {
                    format!("{c} dot {a}^({x})") // parentheses to make sure it's picked up?
                }
            }
        } else {
            self.get_function_body()
        };

        format!("{declaration} = {body}")
    }

    fn print_evaluation_by_parts(&self, replacements: &Replacements) -> String {
        // Should y be replaced with a number?
        let declaration = if let Some(y_value) = replacements.get_replacement_for(self.name) {
            format!("colored({y_value})")
        } else if self.show_as_function // Can we print f(4)?
            && let Some(x_value) = replacements.get_replacement_for(self.variable)
        {
            format!("{f}({x_value})", f = self.name)
        } else {
            self.get_declaration()
        };
        // Should x be replaced with a number?
        let body = if let Some(x) = replacements.get_replacement_for(self.variable) {
            match self.kind {
                FunctionKind::Linear { k, m } => {
                    let m = Term::from_num(m); // Does not print if 0
                    if k == 0 {
                        format!("{m}")
                    } else {
                        format!("{kx}{m:+}", kx = k * x)
                    }
                }
                FunctionKind::Exponential { c, a } => {
                    format!("{c} dot {ax}", ax = a.pow(*x))
                }
            }
        } else {
            self.get_function_body()
        };
        format!("{declaration} = {body}")
    }

    fn evaluate(&self, replacements: &Replacements) -> Number {
        if let Some(x) = replacements.get_replacement_for(self.variable) {
            match self.kind {
                FunctionKind::Linear { k, m } => k * x + m,
                FunctionKind::Exponential { c, a } => c * a.pow(*x),
            }
        } else {
            panic!(
                "Did not provide the correct variable name for evaluate() in the replacements for function with {:#?}, {:#?}",
                self.name, self.variable
            );
        }
    }
}

#[cfg(test)]
mod linear_tests;
