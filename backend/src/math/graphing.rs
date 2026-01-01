use anyhow::{anyhow, Result};
use std::{fmt::Write, i32};

pub struct Graph {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    x_tick: i32,
    y_tick: i32,
    x_grid: GridType,
    y_grid: GridType,
    /// Can the graph start at a non-zero value and "break" to the x-axis?
    can_break: bool,
    plots: Vec<PlotType>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            x_min: -1,
            x_max: 1,
            y_min: -1,
            y_max: 1,
            x_tick: 1,
            y_tick: 1,
            x_grid: GridType::Major,
            y_grid: GridType::Major,
            can_break: false,
            plots: Vec::new(),
        }
    }
}

enum GridType {
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

// TODO: Replace with Number
pub enum PlotType {
    /// k, m
    Linear(i32, i32),
    Polynomial(Vec<i32>),
    /// start, change
    Exponential(i32, i32),
}

impl PlotType {
    fn to_typst(&self) -> String {
        match self {
            PlotType::Linear(k, m) => format!("{} * x + {}", k, m),
            PlotType::Exponential(start, change) => format!("{} * calc.pow({}, x)", start, change),
            // TODO:
            PlotType::Polynomial(_) => format!("3"),
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn x_range(&mut self, min: i32, max: i32) -> &mut Self {
        self.x_min = min;
        self.x_max = max;
        self
    }

    pub fn auto_y_range(&mut self) -> Result<&mut Self> {
        if self.plots.len() == 0 {
            return Err(anyhow!(
                "Tried to call auto_y_range() without adding a plot first",
            ));
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;
        //TODO: handle polynomials properly
        for plot in self.plots.iter() {
            for val in [self.x_min, self.x_max] {
                let extreme = match plot {
                    PlotType::Linear(k, m) => k * val + m,
                    PlotType::Exponential(c, a) => c * a.pow(val as u32),
                    PlotType::Polynomial(_) => 0,
                };

                if extreme < min {
                    min = extreme;
                }
                if extreme > max {
                    max = extreme;
                }
            }
        }

        self.y_min = min;
        self.y_max = max;

        Ok(self)
    }

    pub fn y_range(&mut self, min: i32, max: i32) -> &mut Self {
        self.y_min = min;
        self.y_max = max;
        self
    }

    pub fn add_plot(&mut self, plot: PlotType) -> &mut Self {
        self.plots.push(plot);
        self
    }

    pub fn render(&mut self) -> Result<String> {
        if !self.can_break {
            self.move_y_range_to_axis();
        }
        let mut out = String::with_capacity(256);
        write!(out, "#block(height: 5cm)[#cetz.canvas({{\n")?;
        write!(out, "plot.plot(")?;
        write!(out, "axis-style: \"school-book\",")?;
        write!(out, "size: (4, 4),")?;
        write!(out, "x-min: {},", self.x_min)?;
        write!(out, "x-max: {},", self.x_max)?;
        write!(out, "y-min: {},", self.y_min)?;
        write!(out, "y-max: {},", self.y_max)?;
        write!(out, "x-grid: \"{}\",", self.x_grid.to_typst())?;
        write!(out, "y-grid: \"{}\",", self.y_grid.to_typst())?;
        write!(out, "x-tick-step: {},", self.x_tick)?;
        write!(out, "y-tick-step: {},\n", self.y_tick)?;
        for plot in self.plots.iter() {
            write!(
                out,
                "plot.add(domain: ({}, {}), x => {}),",
                self.x_min,
                self.x_max,
                plot.to_typst()
            )?;
        }
        write!(out, ")}})]")?;
        Ok(out)
    }

    /// Moves the y_range to make sure the graph goes to 0.
    /// Not used if the graph has can_break: true.
    fn move_y_range_to_axis(&mut self) {
        if self.y_min < 0 && self.y_max < 0 {
            self.y_max = 0
        };

        if self.y_min > 0 && self.y_max > 0 {
            self.y_min = 0
        };
    }
}
