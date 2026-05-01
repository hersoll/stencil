use std::fmt::Display;

/// Internal representation of a line in the [`SolutionWithSteps`]
#[derive(Clone)]
struct SolutionPart {
    expression: String,
    step: Option<String>,
}

/// A structured way to print the parts of solutions that require multiple mathematical steps,
/// usually where an original expression or equation is manipulated over the course of the steps.
/// Internally, each line is represented by an `expression` ([`String`]) and a `step` ([`Option<String>`]).
/// Math notation (`$`) is automatically applied to each line.
///
/// In the finished document, it will look something like this:
/// (expression 0) 3x + 1 = 19 | -1 (step 0)
/// (expression 1)     3x = 18 | /3 (step 1)
/// (expression 2)      x = 6  |    (empty step 2)
///
/// # Usage:
///```rust
/// use typst_writer::formatting::{self, SolutionWithSteps};
///
/// let mut sol = SolutionWithSteps::new();
/// sol.add_line(String::from("Hello,")).with_step(formatting::subtract_number(1));
/// sol.add_line(String::from("world!"));
/// let str = sol.to_string();
///```
#[derive(Default)]
pub struct SolutionWithSteps {
    parts: Vec<SolutionPart>,
}

impl SolutionWithSteps {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a step instruction to the latest line
    ///
    /// Overrides any previously set step for the same line!
    pub fn with_step(&mut self, step: impl Display) -> &mut Self {
        match self.parts.last_mut() {
            Some(part) => part.step = Some(step.to_string()),
            None => tracing::error!(
                "Tried to call StructuredSolution.with_step() on an empty Solution!"
            ),
        }
        self
    }

    /// The most generic public version of adding an `expression`. Used for things that don't require
    /// equality signs, like an expression.
    ///
    /// Generally, [`SolutionWithSteps::add_equation()`] and [`SolutionWithSteps::add_aligned()`] should be used whenever a `=` is included in
    /// the expression, since those eliminate a lot `format!()` usage in calls.
    ///
    /// Note that this class of methods return `&mut Self` to be able to chain `.add_line().with_step()`
    pub fn add_line(&mut self, line: impl Display) -> &mut Self {
        self.add_expression(format!("{line}"));
        self
    }

    pub fn add_equation(&mut self, lhs: impl Display, rhs: impl Display) -> &mut Self {
        let equation_string = format!("{lhs} = {rhs}");
        self.add_expression(equation_string);
        self
    }

    pub fn add_aligned(&mut self, lhs: impl Display, rhs: impl Display) -> &mut Self {
        let equation_string = format!("{lhs} &= {rhs}");
        self.add_expression(equation_string);
        self
    }

    /// Helper method to avoid having to do `self.parts.push(...)` in every public method
    fn add_expression(&mut self, expr: String) {
        self.parts.push(SolutionPart {
            expression: expr,
            step: None,
        });
    }
}

impl Display for SolutionWithSteps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (expressions, steps): (Vec<String>, Vec<String>) = self
            .parts
            .iter()
            .map(|part| {
                let expr = format!("$ {} $", part.expression.trim_matches('$').trim());
                let step = format!(
                    "${}$",
                    part.step
                        .as_deref()
                        .unwrap_or_default()
                        .trim_matches('$')
                        .trim()
                );
                (expr, step)
            })
            .unzip();

        let combined_expressions = expressions.join(", ");
        let combined_steps = steps.join(", ");
        write!(
            f,
            "#v(-0.5em)\n#equation-solution(({combined_expressions}),({combined_steps}),)"
        )
    }
}
