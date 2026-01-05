use crate::math::{Number, ZERO, functions::Function};

pub struct Plot {
    pub name: Option<String>,
    pub function: Function,
    pub additions: PlotAdditions,
}

/// Additional elements that need to be added to the plot,
/// like dots, dashed lines, labels
pub struct PlotAdditions {
    /// Elements where the coordinates matter, like dots and lines
    pub axis_relative: Vec<String>,
    /// Elements where the distances need to be the same no matter the plot, like labels.
    /// (We don't want the label further from the graph just because the coordinates are further
    /// apart, for example)
    pub canvas_relative: Vec<String>,
}

impl Plot {
    pub fn linear(k: impl Into<Number>, m: impl Into<Number>) -> Plot {
        let k = k.into();
        let m = m.into();
        Plot {
            name: None,
            function: Function::Linear(k, m),
            additions: PlotAdditions::default(),
        }
    }

    pub fn exponential(c: impl Into<Number>, a: impl Into<Number>) -> Plot {
        let c = c.into();
        let mut a = a.into();
        if a <= ZERO {
            tracing::error!("a in an exponential function can't be negative (or 0)");
            a = Number::Integer(1)
        }
        Plot {
            name: None,
            function: Function::Exponential(c, a),
            additions: PlotAdditions::default(),
        }
    }

    /// Must be called if and only if there are more than one Plot in the same Axes.
    /// Used by additional elements (labels, dots) to know which plot to reference
    pub fn with_name(mut self, name: &str) -> Self {
        let name_string = name.to_string();
        self.name = Some(name_string);
        self
    }

    /// Adds dashed lines in a linear plot showing how to calculate the slope
    /// Automatically calculates the dy depending on k and m
    pub fn with_slope_hint(
        mut self,
        x_start: impl Into<Number>,
        x_step: impl Into<Number>,
        variables: (&str, &str),
    ) -> Self {
        let x_start = x_start.into();
        let x_step = x_step.into();

        let color = "black";
        let label_padding = "0.2";
        let dashed_style = format!("style: (stroke: (paint: {color}, dash: \"dashed\"))");

        let x_var = variables.0;
        let y_var = variables.1;
        let x_end = x_start + &x_step;
        let y_start = self.function.get_y(&x_start);
        let y_end = self.function.get_y(&x_end);
        let y_step = y_end - &y_start;
        let x_label_dir = if y_step > ZERO { "north" } else { "south" };

        let mut x_label_pos = x_start + &(x_step / 2);
        let mut y_label_pos = y_start + &(y_step / 2);

        // Prevent the labels being smack on the axes
        if x_label_pos == ZERO {
            x_label_pos = x_start + &(x_step / 4)
        }
        if y_label_pos == ZERO {
            y_label_pos = y_start + &(y_step * 3 / 4)
        }

        // Used to differentiate between multiple plots in the same Axes
        let anchor_suffix = match self.name {
            Some(ref name) => "-".to_string() + &name,
            None => String::new(),
        };

        let lines = format!(
            "
plot.add((({x_start}, {y_start}), ({x_end}, {y_start})), {dashed_style})
plot.add((({x_end}, {y_start}), ({x_end}, {y_end})), {dashed_style})
plot.add-anchor(\"dx-lbl{anchor_suffix}\", ({x_label_pos}, {y_start}))
plot.add-anchor(\"dy-lbl{anchor_suffix}\", ({x_end}, {y_label_pos}))"
        );

        let labels = format!(
                    "
  content(\"plot.dx-lbl{anchor_suffix}\", [$Delta {x_var} = {x_step}$], anchor: \"{x_label_dir}\", padding: {label_padding})
  content(\"plot.dy-lbl{anchor_suffix}\", [$Delta {y_var} = {y_step}$], anchor: \"west\", padding: {label_padding})
"
                );

        self.additions.axis_relative.push(lines);
        self.additions.canvas_relative.push(labels);

        self
    }

    pub fn to_typst(&self) -> String {
        match self.function {
            Function::Linear(k, m) => format!("{} * float(t) + {}", k.for_plots(), m.for_plots()),
            Function::Exponential(start, change) => format!(
                "{} * calc.pow({}, t)",
                start.for_plots(),
                change.for_plots()
            ),
        }
    }
}

impl Default for PlotAdditions {
    fn default() -> Self {
        PlotAdditions {
            axis_relative: Vec::new(),
            canvas_relative: Vec::new(),
        }
    }
}
