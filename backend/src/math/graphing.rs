use anyhow::Result;
use std::fmt::Write;

pub struct Graph {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    x_tick: i32,
    y_tick: i32,
    x_grid: GridType,
    y_grid: GridType,
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
        }
    }
}

enum GridType {
    Both,
    Major,
    None,
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn render(&self) -> Result<String> {
        let mut out = String::with_capacity(256);
        write!(out, "#block(height: 5cm)[#cetz.canvas({{")?;
        write!(out, "plot.plot(")?;
        write!(out, "axis-style: \"school-book\",")?;
        write!(out, "size: (4, 4),")?;
        write!(out, "x-min: {},", self.x_min)?;
        write!(out, "x-max: {},", self.x_max)?;
        write!(out, "y-min: {},", self.y_min)?;
        write!(out, "y-max: {},", self.y_max)?;
        write!(out, "x-tick-step: {},", self.x_tick)?;
        write!(out, "y-tick-step: {},", self.y_tick)?;
        write!(out, "{{ plot.add(domain: (-2, 2), x => 2 * x + 1) }},")?;
        write!(out, ")}})]")?;
        Ok(out)
    }
}
