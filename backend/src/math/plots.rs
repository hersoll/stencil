use anyhow::{Result, anyhow};
use std::{fmt::Write, i32};

use crate::math::Number;

pub struct Plot {
    x_min: Number,
    x_max: Number,
    y_min: Number,
    y_max: Number,
    x_tick: Number,
    y_tick: Number,
    grid: GridType,
    /// Can the graph start at a non-zero value and "break" to the x-axis?
    can_break: bool,
    plots: Vec<PlotType>,
}

impl Default for Plot {
    fn default() -> Self {
        Plot {
            x_min: Number::Integer(-1),
            x_max: Number::Integer(1),
            y_min: -Number::Integer(-1),
            y_max: Number::Integer(1),
            x_tick: Number::Integer(1),
            y_tick: Number::Integer(1),
            grid: GridType::Major,
            can_break: false,
            plots: Vec::new(),
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
            PlotType::Linear(k, m) => format!("{} * float(t) + {}", k.for_plots(), m.for_plots()),
            PlotType::Exponential(start, change) => format!(
                "{} * calc.pow({}, t)",
                start.for_plots(),
                change.for_plots()
            ),
            // TODO:
            PlotType::Polynomial(_) => format!("3"),
        }
    }
}

impl Plot {
    pub fn new() -> Self {
        Plot::default()
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

    pub fn grid(&mut self, grid: GridType) -> &mut Self {
        self.grid = grid;
        self
    }

    pub fn add_break(&mut self) -> &mut Self {
        self.can_break = true;
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
        writeln!(out, "#block(height: 5cm)[#cetz.canvas({{")?;
        writeln!(out, "plot.plot(")?;
        writeln!(out, "axis-style: \"school-book\",")?;
        writeln!(out, "size: (4, 4),")?;
        writeln!(out, "x-min: {},", self.x_min.for_plots())?;
        writeln!(out, "x-max: {},", self.x_max.for_plots())?;
        //writeln!(out, "y-min: {},", self.y_min.for_plots())?;
        //writeln!(out, "y-max: {},", self.y_max.for_plots())?;
        writeln!(out, "x-grid: \"{}\",", self.grid.to_typst())?;
        writeln!(out, "y-grid: \"{}\",", self.grid.to_typst())?;
        writeln!(out, "x-tick-step: {},", self.x_tick.for_plots())?;
        writeln!(out, "y-tick-step: {},", self.y_tick.for_plots())?;
        writeln!(out, "{{")?;
        for plot in self.plots.iter() {
            writeln!(
                out,
                "plot.add(domain: ({}, {}), t => {})",
                self.x_min.for_plots(),
                self.x_max.for_plots(),
                plot.to_typst()
            )?;
        }
        writeln!(out, "}})}})]")?;
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
