use anyhow::{Result, anyhow};
use std::{fmt::Write, i32};

use crate::math::Number;

pub struct Graph {
    x_min: Number,
    x_max: Number,
    y_min: Number,
    y_max: Number,
    x_tick: Number,
    y_tick: Number,
    x_grid: GridType,
    y_grid: GridType,
    /// Can the graph start at a non-zero value and "break" to the x-axis?
    can_break: bool,
    plots: Vec<PlotType>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            x_min: Number::Integer(-1),
            x_max: Number::Integer(1),
            y_min: -Number::Integer(-1),
            y_max: Number::Integer(1),
            x_tick: Number::Integer(1),
            y_tick: Number::Integer(1),
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

pub enum PlotType {
    /// k, m
    Linear(Number, Number),
    Polynomial(Vec<Number>),
    /// start, change
    Exponential(Number, Number),
}

impl PlotType {
    fn to_typst(&self) -> String {
        match self {
            PlotType::Linear(k, m) => format!("{} * float(x) + {}", k.for_plots(), m.for_plots()),
            PlotType::Exponential(start, change) => format!(
                "{} * calc.pow({}, x)",
                start.for_plots(),
                change.for_plots()
            ),
            // TODO:
            PlotType::Polynomial(_) => format!("3"),
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn x_range<T: Into<Number>, U: Into<Number>>(&mut self, min: T, max: U) -> &mut Self {
        self.x_min = min.into();
        self.x_max = max.into();
        self
    }

    pub fn auto_y_range(&mut self) -> Result<&mut Self> {
        if self.plots.len() == 0 {
            return Err(anyhow!(
                "Tried to call auto_y_range() without adding a plot first",
            ));
        }

        let mut min = Number::Integer(i32::MAX);
        let mut max = Number::Integer(i32::MIN);
        //TODO: handle polynomials properly
        for plot in self.plots.iter() {
            for val in [self.x_min, self.x_max].iter() {
                let extreme = match plot {
                    PlotType::Linear(k, m) => k * &val + m,
                    PlotType::Exponential(c, a) => c * &a.value().powf(val.value()).into(),
                    PlotType::Polynomial(_) => Number::Integer(0),
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

    pub fn y_range<T: Into<Number>, U: Into<Number>>(&mut self, min: T, max: U) -> &mut Self {
        self.y_min = min.into();
        self.y_max = max.into();
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
        write!(out, "x-min: {},", self.x_min.for_plots())?;
        write!(out, "x-max: {},", self.x_max.for_plots())?;
        write!(out, "y-min: {},", self.y_min.for_plots())?;
        write!(out, "y-max: {},", self.y_max.for_plots())?;
        write!(out, "x-grid: \"{}\",", self.x_grid.to_typst())?;
        write!(out, "y-grid: \"{}\",", self.y_grid.to_typst())?;
        write!(out, "x-tick-step: {},", self.x_tick.for_plots())?;
        write!(out, "y-tick-step: {},\n", self.y_tick.for_plots())?;
        for plot in self.plots.iter() {
            write!(
                out,
                "plot.add(domain: ({}, {}), x => {}),",
                self.x_min.for_plots(),
                self.x_max.for_plots(),
                plot.to_typst()
            )?;
        }
        write!(out, ")}})]")?;
        Ok(out)
    }

    /// Moves the y_range to make sure the graph goes to 0.
    /// Not used if the graph has can_break: true.
    fn move_y_range_to_axis(&mut self) {
        let zero = Number::Integer(0);
        if self.y_min < zero && self.y_max < zero {
            tracing::debug!("Moving y_max");
            self.y_max = zero
        };

        if self.y_min > zero && self.y_max > zero {
            tracing::debug!("Moving y_min");
            self.y_min = zero
        };
    }
}
