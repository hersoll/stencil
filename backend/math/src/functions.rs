mod exponential;
mod linear;
mod quadratic;
pub use exponential::*;
pub use linear::*;
pub use quadratic::*;
use tracing::error;

use std::fmt::Display;

use crate::{
    Number, Replacement, Term,
    evaluables::{Evaluable, Replacements},
    symbols::{self, Symbol},
    utils,
};

#[derive(Debug, Copy, Clone)]
pub struct Function {
    /// The output [`Symbol`], generally the "y variable"
    pub name: &'static Symbol,
    /// The input [`Symbol`], generally the "x variable"
    pub variable: &'static Symbol,
    /// Which kind of function is it?
    pub kind: FunctionKind,
    /// Determines whether to print the function as `y(x) = kx + m` (`true`) or `y = kx + m` (`false`)
    show_as_function: bool,
    /// Will it print with a `&` for Typst alignment?
    aligned: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum FunctionKind {
    Linear(LinearFunction),
    Exponential(ExponentialFunction),
    Quadratic(QuadraticFunction),
}

impl Default for FunctionKind {
    fn default() -> Self {
        FunctionKind::Linear(LinearFunction {
            k: Number::Integer(1),
            m: Number::Integer(0),
        })
    }
}

impl Default for Function {
    fn default() -> Self {
        Self {
            name: symbols::Y,
            variable: symbols::X,
            kind: FunctionKind::Linear(LinearFunction {
                k: Number::Integer(1),
                m: Number::Integer(0),
            }),
            show_as_function: false,
            aligned: false,
        }
    }
}
impl Function {
    /// Ergonomic constructor
    pub fn linear(k: impl Into<Number>, m: impl Into<Number>) -> Function {
        let k = k.into();
        let m = m.into();

        Function {
            kind: FunctionKind::Linear(LinearFunction { k, m }),
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
            kind: FunctionKind::Exponential(ExponentialFunction { c, a }),
            ..Default::default()
        }
    }

    /// Returns a quadratic with the specified symmetry line and distance to zero points
    pub fn quadratic_from_sym_dist(
        symmetry: impl Into<Number> + Copy,
        distance: impl Into<Number> + Copy,
    ) -> Self {
        let kind = if let Some(quad) = QuadraticFunction::from_symmetry_distance(symmetry, distance)
        {
            FunctionKind::Quadratic(quad)
        } else {
            FunctionKind::default()
        };
        Function {
            kind,
            ..Default::default()
        }
    }

    /// Returns a quadratic with the specified symmetry line and distance to zero points,
    /// with factor k
    pub fn quadratic_from_sym_dist_k(
        symmetry: impl Into<Number>,
        distance: impl Into<Number>,
        k: impl Into<Number>,
    ) -> Self {
        let kind = if let Some(quad) =
            QuadraticFunction::from_symmetry_distance_k(symmetry, distance, k)
        {
            FunctionKind::Quadratic(quad)
        } else {
            FunctionKind::default()
        };
        Function {
            kind,
            ..Default::default()
        }
    }

    /// Returns a quadratic that goes through the specified points
    ///
    /// Will return the default linear if at least two x:es are identical
    pub fn quadratic_from_points(
        (x1, y1): (impl Into<Number>, impl Into<Number>),
        (x2, y2): (impl Into<Number>, impl Into<Number>),
        (x3, y3): (impl Into<Number>, impl Into<Number>),
    ) -> Self {
        let kind = if let Some(quad) = QuadraticFunction::from_points((x1, y1), (x2, y2), (x3, y3))
        {
            FunctionKind::Quadratic(quad)
        } else {
            FunctionKind::default()
        };
        Function {
            kind,
            ..Default::default()
        }
    }

    /// Returns a quadratic in the form of `ax^2 + bx +c`.
    ///
    /// If `a == 0`, returns a linear function instead
    pub fn quadratic_from_abc(
        a: impl Into<Number> + Copy,
        b: impl Into<Number> + Copy,
        c: impl Into<Number> + Copy,
    ) -> Self {
        let kind = if let Some(quad) = QuadraticFunction::from_abc(a, b, c) {
            FunctionKind::Quadratic(quad)
        } else {
            FunctionKind::Linear(LinearFunction {
                k: b.into(),
                m: c.into(),
            })
        };
        Function {
            kind,
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

    /// Adds alignment characters (`&`) by the equality sign
    pub fn aligned(mut self) -> Self {
        self.aligned = true;
        self
    }

    /// Finds the x-value(s) of the function, given a certain y-value.
    ///
    /// Returns an empty [`Vec`] if the y-value is outside the domain of the function
    pub fn get_x(&self, y: &Number) -> Vec<Number> {
        match &self.kind {
            FunctionKind::Linear(LinearFunction { k, m }) => {
                // Don't use this when k = 0, please
                if *k == 0 { vec![] } else { vec![(*y - m) / k] }
            }
            FunctionKind::Exponential(ExponentialFunction { c, a }) => {
                // No solution if y and c have opposite signs, since a positive value can't become
                // negative through exponentiation
                if *y * c < 0 {
                    vec![]
                } else {
                    // y = c a^x => x = lg(y/c) / lg(a)
                    vec![((*y / c).value().log2() / a.value().log2()).into()]
                }
            }
            FunctionKind::Quadratic(quad) => quad.get_x(y),
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
        match &self.kind {
            FunctionKind::Linear(LinearFunction { k, m }) => Some(k * x + m),
            FunctionKind::Exponential(ExponentialFunction { c, a }) => {
                Some(c * a.value().powf(x.value()))
            }
            FunctionKind::Quadratic(quad) => Some(quad.get_y(x)),
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
            FunctionKind::Linear(LinearFunction { k, m }) => {
                let poly = (k * self.variable).and(&Term::from_num(m));
                poly.to_string()
            }
            FunctionKind::Exponential(ExponentialFunction { c, a }) => {
                format!("{c} dot {a}^{x}", x = self.variable)
            }
            FunctionKind::Quadratic(QuadraticFunction { a, b, c }) => {
                let poly = (a * self.variable * self.variable)
                    .and(&(b * self.variable))
                    .and(&Term::from_num(c));
                poly.to_string()
            }
        }
    }

    fn get_equality_sign(&self) -> &'static str {
        if self.aligned { "&=" } else { "=" }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{declaration} {equality_sign} {function_body}",
            declaration = self.get_declaration(),
            equality_sign = self.get_equality_sign(),
            function_body = self.get_function_body()
        )
    }
}

impl Evaluable for Function {
    fn print_replacements(&self, replacement_vec: &[Replacement]) -> String {
        let replacements = Replacements::from_array(replacement_vec);
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
            let x = format!("colored({})", utils::parenthesize(x_value));
            match &self.kind {
                FunctionKind::Linear(LinearFunction { k, m }) => {
                    let m = Term::from_num(*m);
                    if *k == 0 {
                        format!("{m}")
                    } else if *k == 1 {
                        format!("{x} {m:+}")
                    } else {
                        format!("{k} dot {x} {m:+}")
                    }
                }
                FunctionKind::Exponential(ExponentialFunction { c, a }) => {
                    format!("{c} dot {a}^({x})")
                }
                FunctionKind::Quadratic(func) => func
                    .as_poly(self.variable)
                    .print_replacements(replacement_vec),
            }
        } else {
            self.get_function_body()
        };

        let equality_sign = self.get_equality_sign();
        format!("{declaration} {equality_sign} {body}")
    }

    fn print_evaluation_by_parts(&self, replacement_vec: &[Replacement]) -> String {
        let replacements = Replacements::from_array(replacement_vec);

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
            match &self.kind {
                FunctionKind::Linear(LinearFunction { k, m }) => {
                    let m = Term::from_num(*m); // Does not print if 0
                    if *k == 0 {
                        format!("{m}")
                    } else {
                        format!("{kx}{m:+}", kx = k * x)
                    }
                }
                FunctionKind::Exponential(ExponentialFunction { c, a }) => {
                    format!("{c} dot {ax}", ax = a.pow(*x))
                }
                FunctionKind::Quadratic(func) => func
                    .as_poly(self.variable)
                    .print_evaluation_by_parts(replacement_vec),
            }
        } else {
            self.get_function_body()
        };
        let equality_sign = self.get_equality_sign();
        format!("{declaration} {equality_sign} {body}")
    }

    fn evaluate(&self, replacements: &[Replacement]) -> Number {
        let replacements = Replacements::from_array(replacements);

        if let Some(x) = replacements.get_replacement_for(self.variable) {
            if let Some(y) = self.get_y(x) {
                y
            } else {
                error!("Tried to evaluate() a functions with an invalid x value");
                Number::Integer(-1000)
            }
        } else {
            panic!(
                "Did not provide the correct variable name for evaluate() in the replacements for function with {:#?}, {:#?}",
                self.name, self.variable
            );
        }
    }
}
