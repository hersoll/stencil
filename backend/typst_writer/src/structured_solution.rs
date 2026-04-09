use std::fmt::Display;

#[derive(Clone)]
struct SolutionPart {
    expression: String,
    step: Option<String>,
}

/// Usage:
///
///```rust
///let sol = StructuredSolution::new();
///sol.add_line(polynomial).with_step(formatting::subtract_term(t1))
///sol.add_aligned(lhs, rhs)
///```
pub struct StructuredSolution {
    parts: Vec<SolutionPart>,
}

impl StructuredSolution {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    /// Adds a step instruction to the latest line
    pub fn with_step(&mut self, step: impl Display) {
        match self.parts.last_mut() {
            Some(part) => part.step = Some(step.to_string()),
            None => tracing::error!(
                "Tried to call StructuredSolution.with_step() on an empty Solution!"
            ),
        }
    }

    /// The most generic public version of adding an Expression. Used for things that aren't
    /// equations, like simplifying expressions or function calls.
    ///
    /// Note that this class of methods return &mut Self to be able to chain .add_line().with_step()
    pub fn add_line(&mut self, line: String) -> &mut Self {
        self.add_expression(line);
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

    /// Helper method to avoid having to do self.parts.push(.....) in every public method
    fn add_expression(&mut self, expr: String) {
        self.parts.push(SolutionPart {
            expression: expr,
            step: None,
        });
    }
}

impl Display for StructuredSolution {
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
