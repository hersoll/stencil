use std::fmt::Display;

use math::{
    Number, ZERO,
    functions::{Function, FunctionKind},
};

const LABEL_PADDING: f64 = 0.2;

pub struct Graph {
    pub function: Function,
    pub additions: GraphAdditions,
    /// A short name which is used to differentiate different plots
    pub name: Option<String>,
    /// Actual user-facing name, generally math-formatted
    pub label: Label,
}

pub struct Label(Option<String>);

/// Additional elements that need to be added to the graph,
/// like dots, dashed lines, labels
#[derive(Default)]
pub struct GraphAdditions {
    /// Elements where the coordinates matter, like dots and lines
    pub axis_relative: Vec<String>,
    /// Elements where the distances need to be the same no matter the graph, like labels.
    /// (We don't want the label further from the graph just because the coordinates are further
    /// apart, for example)
    pub canvas_relative: Vec<String>,
}

impl Graph {
    /// Constructor that simplifies the creation of a graph of a linear function
    pub fn linear(k: impl Into<Number>, m: impl Into<Number>) -> Graph {
        Graph {
            name: None,
            label: Label(None),
            function: Function::linear(k, m),
            additions: GraphAdditions::default(),
        }
    }

    /// Constructor that simplifies the creation of a graph of a exponential function
    pub fn exponential(c: impl Into<Number>, a: impl Into<Number>) -> Graph {
        let mut a = a.into();
        if a <= 0 {
            tracing::error!("a in an exponential function can't be negative (or 0)");
            a = Number::Integer(1)
        }
        Graph {
            name: None,
            label: Label(None),
            function: Function::exponential(c, a),
            additions: GraphAdditions::default(),
        }
    }

    pub fn label(mut self, label: impl Display) -> Self {
        self.label = Label(Some(label.to_string()));
        self
    }

    /// Must be called if and only if there are more than one graph in the same Axes.
    /// Used by additional elements (labels, dots) to know which graph to reference
    pub fn with_name(mut self, name: &str) -> Self {
        let name_string = name.to_string();
        self.name = Some(name_string);
        self
    }

    /// Add a single dot to the graph.
    ///
    /// If you need multiple dots, use dots() instead.
    pub fn dot(self, x: impl Into<Number>, y: impl Into<Number>) -> Self {
        let x = x.into();
        let y = y.into();
        self.dots(vec![(x, y)])
    }

    /// Add multiple dots to the graph.
    pub fn dots<T: Into<Number>>(mut self, coords: Vec<(T, T)>) -> Self {
        for (i, (x, y)) in coords.into_iter().enumerate() {
            let x = x.into().for_graphs();
            let y = y.into().for_graphs();
            let color = "red";
            let dot_anchor = format!("plot.add-anchor(\"dot_{i}\", ({x}, {y}))");
            let dot = format!("circle(\"graph.dot_{i}\", radius: 0.07, fill: {color})");
            self.additions.axis_relative.push(dot_anchor);
            self.additions.canvas_relative.push(dot);
        }
        self
    }

    /// Add a single dot to the graph, with dashed lines going from the x- and y-axes.
    pub fn dot_with_lines(mut self, x: impl Into<Number>, y: impl Into<Number>) -> Self {
        let x = x.into();
        let y = y.into();

        let dashed_style = "style: (stroke: (paint: black, dash: \"dashed\"))";
        let lines = format!(
            "
plot.add((({x}, 0), ({x}, {y})), {dashed_style})
plot.add(((0, {y}), ({x}, {y})), {dashed_style})"
        );

        self.additions.axis_relative.push(lines);
        self.dot(x, y)
    }

    /// Adds horizontal and vertical lines to show where one can calculate the slope.
    ///
    /// Note that you need to know where (x-wise) there are good spots to get the slope from.
    /// This method does not calculate it for you.
    ///
    /// Panics if the function isn't defined at any of the given x values!
    ///
    /// Example: y = 4x/3 + 2 => x_step should be 3 (or a multiple of it)
    fn add_dashed_slope_hints(&mut self, x_start: Number, x_step: Number) {
        let color = "black";
        let dashed_style = format!("style: (stroke: (paint: {color}, dash: \"dashed\"))");
        let x_end = x_start + x_step;
        let y_start = self.function.get_y(&x_start).unwrap();
        let y_end = self.function.get_y(&x_end).unwrap();
        // Need to use for_graphs for every printed Number variable, to make sure
        // decimal numbers are formatted correctly
        let x_0 = x_start.for_graphs();
        let x_1 = x_end.for_graphs();
        let y_0 = y_start.for_graphs();
        let y_1 = y_end.for_graphs();

        let lines = format!(
            "
plot.add((({x_0}, {y_0}), ({x_1}, {y_0})), {dashed_style})
plot.add((({x_1}, {y_0}), ({x_1}, {y_1})), {dashed_style})"
        );

        self.additions.axis_relative.push(lines);
    }

    /// The anchor suffix is used by graphs to give anchors unique names,
    /// in case there are multiple graphs with similar elements
    fn get_anchor_suffix(&self) -> String {
        match self.name {
            Some(ref name) => "-".to_string() + name,
            None => String::new(),
        }
    }

    /// Adds a label halfway across the specified x-step
    ///
    /// Used for labelling dashed slope hints. Does not need to contain "dx"
    fn add_dx_label(
        &mut self,
        label_content: &str,
        x_start: Number,
        x_step: Number,
        y_pos: Number,
    ) {
        let mut x_label_pos = x_start + (x_step / 2);
        if x_label_pos == ZERO {
            x_label_pos = x_start + (x_step / 4)
        }
        let x_label_dir =
            if self.function.get_y(&x_start) < self.function.get_y(&(x_start + x_step)) {
                "north"
            } else {
                "south"
            };

        let x_label_pos = x_label_pos.for_graphs();
        let y_pos = y_pos.for_graphs();

        let anchor_suffix = self.get_anchor_suffix();
        let anchor =
            format!("plot.add-anchor(\"dx-lbl{anchor_suffix}\", ({x_label_pos}, {y_pos}))");
        let label = format!(
            "content(\"graph.dx-lbl{anchor_suffix}\", [${label_content}$], 
            anchor: \"{x_label_dir}\", padding: {LABEL_PADDING})"
        );
        self.additions.axis_relative.push(anchor);
        self.additions.canvas_relative.push(label);
    }

    /// Adds a label halfway up the specified y-step
    ///
    /// Used for labelling dashed slope hints. Does not need to contain "dy"
    fn add_dy_label(
        &mut self,
        label_content: &str,
        y_start: Number,
        y_step: Number,
        x_pos: Number,
    ) {
        let mut y_label_pos = y_start + (y_step / 2);
        if y_label_pos == ZERO {
            y_label_pos = y_start + (y_step / 4)
        }

        let y_label_pos = y_label_pos.for_graphs();
        let x_pos = x_pos.for_graphs();

        let anchor_suffix = self.get_anchor_suffix();
        let anchor =
            format!("plot.add-anchor(\"dy-lbl{anchor_suffix}\", ({x_pos}, {y_label_pos}))");
        let label = format!(
            "content(\"graph.dy-lbl{anchor_suffix}\", [${label_content}$], 
            anchor: \"west\", padding: {LABEL_PADDING})"
        );
        self.additions.axis_relative.push(anchor);
        self.additions.canvas_relative.push(label);
    }

    /// Adds dashed lines in a graph showing how to calculate the slope,
    /// with dx and dy labels.
    pub fn with_slope_hint(
        mut self,
        x_start: impl Into<Number>,
        x_step: impl Into<Number>,
        variables: (&str, &str),
    ) -> Self {
        let x_start = x_start.into().simplify();
        let x_step = x_step.into().simplify();

        let x_var = variables.0;
        let y_var = variables.1;
        let x_end = x_start + x_step;
        let y_start = self.function.get_y(&x_start).unwrap();
        let y_end = self.function.get_y(&x_end).unwrap();
        let y_step = (y_end - y_start).simplify();

        let x_step_str = x_step.for_graphs();
        let y_step_str = y_step.for_graphs();

        self.add_dashed_slope_hints(x_start, x_step);
        self.add_dx_label(
            &format!("Delta {x_var} = {x_step_str}"),
            x_start,
            x_step,
            y_start,
        );
        self.add_dy_label(
            &format!("Delta {y_var} = {y_step_str}"),
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
        let y_start = self.function.get_y(&x_start).unwrap();
        let y_end = self.function.get_y(&x_end).unwrap();
        let y_step = y_end - y_start;
        let y_step_str = y_step.for_graphs();

        self.add_dashed_slope_hints(x_start, x_end);
        self.add_dy_label(&format!("k = {y_step_str}"), y_start, y_step, x_end);

        self
    }

    /// Formats the function to a proper typst output
    ///
    /// Even though this only concerns the function and nothing else about the graph,
    /// the method lives here due to it being typst (and therefore graph) related, not mathematical
    pub fn to_typst(&self) -> String {
        match self.function.kind {
            FunctionKind::Linear { k, m } => {
                format!("{} * float(t) + {}", k.for_graphs(), m.for_graphs())
            }
            FunctionKind::Exponential { c, a } => {
                format!("{} * calc.pow({}, t)", c.for_graphs(), a.for_graphs())
            }
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Label(Some(label)) = self {
            write!(f, ", label: [{label}]")
        } else {
            write!(f, "")
        }
    }
}
