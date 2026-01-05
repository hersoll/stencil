use crate::math::{Number, ZERO, functions::Function};

const LABEL_PADDING: f64 = 0.2;

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

    fn add_dashed_slope_hints(&mut self, x_start: Number, x_step: Number) {
        let color = "black";
        let dashed_style = format!("style: (stroke: (paint: {color}, dash: \"dashed\"))");
        let x_end = x_start + &x_step;
        let y_start = self.function.get_y(&x_start);
        let y_end = self.function.get_y(&x_end);
        // Need to use for_plots for every printed Number variable, to make sure
        // decimal numbers are formatted correctly
        let x_0 = x_start.for_plots();
        let x_1 = x_end.for_plots();
        let y_0 = y_start.for_plots();
        let y_1 = y_end.for_plots();

        let lines = format!(
            "
plot.add((({x_0}, {y_0}), ({x_1}, {y_0})), {dashed_style})
plot.add((({x_1}, {y_0}), ({x_1}, {y_1})), {dashed_style})"
        );

        self.additions.axis_relative.push(lines);
    }

    fn get_anchor_suffix(&self) -> String {
        match self.name {
            Some(ref name) => "-".to_string() + name,
            None => String::new(),
        }
    }

    fn add_dx_label(
        &mut self,
        label_content: String,
        x_start: Number,
        x_step: Number,
        y_pos: Number,
    ) {
        let mut x_label_pos = x_start + &(x_step / 2);
        if x_label_pos == ZERO {
            x_label_pos = x_start + &(x_step / 4)
        }
        let x_label_dir =
            if self.function.get_y(&x_start) < self.function.get_y(&(x_start + &x_step)) {
                "north"
            } else {
                "south"
            };

        let x_label_pos = x_label_pos.for_plots();
        let y_pos = y_pos.for_plots();

        let anchor_suffix = self.get_anchor_suffix();
        let anchor =
            format!("plot.add-anchor(\"dx-lbl{anchor_suffix}\", ({x_label_pos}, {y_pos}))");
        let label = format!(
            "content(\"plot.dx-lbl{anchor_suffix}\", [${label_content}$], 
            anchor: \"{x_label_dir}\", padding: {LABEL_PADDING})"
        );
        self.additions.axis_relative.push(anchor);
        self.additions.canvas_relative.push(label);
    }

    fn add_dy_label(
        &mut self,
        label_content: String,
        y_start: Number,
        y_step: Number,
        x_pos: Number,
    ) {
        let mut y_label_pos = y_start + &(y_step / 2);
        if y_label_pos == ZERO {
            y_label_pos = y_start + &(y_step / 4)
        }

        let y_label_pos = y_label_pos.for_plots();
        let x_pos = x_pos.for_plots();

        let anchor_suffix = self.get_anchor_suffix();
        let anchor =
            format!("plot.add-anchor(\"dy-lbl{anchor_suffix}\", ({x_pos}, {y_label_pos}))");
        let label = format!(
            "content(\"plot.dy-lbl{anchor_suffix}\", [${label_content}$], 
            anchor: \"west\", padding: {LABEL_PADDING})"
        );
        self.additions.axis_relative.push(anchor);
        self.additions.canvas_relative.push(label);
    }

    /// Adds dashed lines in a plot showing how to calculate the slope,
    /// with dx and dy labels.
    pub fn with_slope_hint(
        mut self,
        x_start: impl Into<Number>,
        x_step: impl Into<Number>,
        variables: (&str, &str),
    ) -> Self {
        let x_start = x_start.into();
        let x_step = x_step.into();

        let x_var = variables.0;
        let y_var = variables.1;
        let x_end = x_start + &x_step;
        let y_start = self.function.get_y(&x_start);
        let y_end = self.function.get_y(&x_end);
        let y_step = y_end - &y_start;

        let x_step_str = x_step.for_plots();
        let y_step_str = y_step.for_plots();

        self.add_dashed_slope_hints(x_start, x_step);
        self.add_dx_label(
            format!("Delta {x_var} = {x_step_str}"),
            x_start,
            x_step,
            y_start,
        );
        self.add_dy_label(
            format!("Delta {y_var} = {y_step_str}"),
            y_start,
            y_step,
            x_end,
        );

        self
    }

    /// Like with_slope_hint(), but used for functions where the slope (k) and start (m) are whole numbers.
    /// Assumes the x_start is 0, x_step is 1 and only labels the dy (with a "k")
    pub fn with_simple_slope_hint(mut self) -> Self {
        let x_start = Number::Integer(0);
        let x_end = Number::Integer(1);
        let y_start = self.function.get_y(&x_start);
        let y_end = self.function.get_y(&x_end);
        let y_step = y_end - &y_start;
        let y_step_str = y_step.for_plots();

        self.add_dashed_slope_hints(x_start, x_end);
        self.add_dy_label(format!("k = {y_step_str}"), y_start, y_step, x_end);

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
