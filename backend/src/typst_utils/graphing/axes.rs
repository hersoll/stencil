use crate::{
    math::{Number, ZERO, functions::Function},
    typst_utils::graphing::graphs::Graph,
};
use anyhow::{Result, anyhow};
use std::fmt::Write;

pub struct Axes {
    x_min: Number,
    x_max: Number,
    y_min: Option<Number>,
    y_max: Option<Number>,
    x_tick: Number,
    y_tick: Number,
    grid: GridType,
    /// Can the graph start at a non-zero value and "break" to the x-axis?
    can_break: bool,
    graphs: Vec<Graph>,
    padding: Number,
}

impl Default for Axes {
    fn default() -> Self {
        Axes {
            x_min: Number::Integer(-1),
            x_max: Number::Integer(1),
            y_min: None,
            y_max: None,
            x_tick: Number::Integer(1),
            y_tick: Number::Integer(1),
            grid: GridType::Major,
            can_break: false,
            graphs: Vec::new(),
            padding: ZERO,
        }
    }
}

pub enum GridType {
    Both,
    Major,
    None,
}

impl GridType {
    fn to_typst(&self) -> &str {
        match self {
            GridType::Both => "both",
            GridType::Major => "major",
            GridType::None => "none",
        }
    }
}

impl Axes {
    pub fn new() -> Self {
        Axes::default()
    }

    pub fn x_range(&mut self, min: impl Into<Number>, max: impl Into<Number>) -> &mut Self {
        self.x_min = min.into();
        self.x_max = max.into();
        self
    }

    pub fn y_range(&mut self, min: impl Into<Number>, max: impl Into<Number>) -> &mut Self {
        self.y_min = Some(min.into());
        self.y_max = Some(max.into());
        self
    }

    pub fn grid(&mut self, grid: GridType) -> &mut Self {
        self.grid = grid;
        self
    }

    pub fn padding(&mut self, padding: impl Into<Number>) -> &mut Self {
        self.padding = padding.into();
        self
    }

    pub fn add_break(&mut self) -> &mut Self {
        self.can_break = true;
        self
    }

    pub fn add_graph(&mut self, graph: Graph) -> &mut Self {
        self.graphs.push(graph);
        self
    }

    pub fn build_string(&mut self) -> Result<String> {
        // Make sure the y_range is set if not manually set
        self.set_y_range()?;

        let mut out = String::with_capacity(256);
        writeln!(out, "#block(height: 5cm)[#cetz.canvas({{")?;
        writeln!(out, "import cetz.draw: *")?;
        writeln!(out, "graph.graph(")?;
        writeln!(out, "axis-style: \"school-book\",")?;
        writeln!(out, "name: \"graph\",")?;
        writeln!(out, "size: (4, 4),")?;
        writeln!(out, "x-min: {},", self.x_min.for_graphs())?;
        writeln!(out, "x-max: {},", self.x_max.for_graphs())?;
        writeln!(out, "y-min: {},", self.y_min.unwrap().for_graphs())?;
        writeln!(out, "y-max: {},", self.y_max.unwrap().for_graphs())?;
        writeln!(out, "x-grid: \"{}\",", self.grid.to_typst())?;
        writeln!(out, "y-grid: \"{}\",", self.grid.to_typst())?;
        writeln!(out, "x-tick-step: {},", self.x_tick.for_graphs())?;
        writeln!(out, "y-tick-step: {},", self.y_tick.for_graphs())?;
        writeln!(out, "{{")?;
        for graph in self.graphs.iter() {
            writeln!(
                out,
                "graph.add(domain: ({}, {}), t => {})",
                self.x_min.for_graphs(),
                self.x_max.for_graphs(),
                graph.to_typst()
            )?;
            for add in graph.additions.axis_relative.iter() {
                writeln!(out, "{add}")?;
            }
        }
        writeln!(out, "}})")?;
        // Need to loop through the graphs again to get all the canvas-relative additions
        for graph in self.graphs.iter() {
            for add in graph.additions.canvas_relative.iter() {
                writeln!(out, "{add}")?;
            }
        }
        writeln!(out, "}})]")?;
        Ok(out)
    }

    /// Makes absolutely sure that y_min and y_max are Some() values,
    /// by auto-adjusting the y range if it hasn't been set manually
    fn set_y_range(&mut self) -> Result<()> {
        // Don't touch the range if it has already been set...
        match (self.y_min, self.y_max) {
            (Some(_), Some(_)) => (),
            (_, _) => self.auto_y_range()?,
        }
        // ...except to make sure the y-axis looks good
        if !self.can_break {
            self.move_y_range_to_axis();
        }

        Ok(())
    }

    /// Finds the highest and lowest y-values among the graphs
    /// and sets y_min and y_max acccordingly
    fn auto_y_range(&mut self) -> Result<()> {
        // If you want to draw an empty graph, specify y_min and y_max yourself
        if self.graphs.len() == 0 {
            return Err(anyhow!(
                "Tried to call auto_y_range() without adding a graph first.",
            ));
        }

        let mut min = Number::Integer(i32::MAX);
        let mut max = Number::Integer(i32::MIN);
        for graph in self.graphs.iter() {
            for val in [self.x_min, self.x_max].iter() {
                let extreme = match graph.function {
                    Function::Linear(k, m) => k * val + &m,
                    Function::Exponential(c, a) => c * &a.value().powf(val.value()).into(),
                };

                if extreme < min {
                    min = extreme;
                }
                if extreme > max {
                    max = extreme;
                }
            }
        }

        self.y_min = Some(min - &self.padding);
        self.y_max = Some(max + &self.padding);

        Ok(())
    }

    /// Moves the y_range to make sure the graph goes to 0.
    /// Not used if the graph has can_break: true.
    fn move_y_range_to_axis(&mut self) {
        let min = self.y_min.unwrap();
        let max = self.y_max.unwrap();
        let zero = Number::Integer(0);
        if min < zero && max < zero {
            self.y_max = Some(zero)
        };

        if min > zero && max > zero {
            self.y_min = Some(zero)
        };
    }
}
