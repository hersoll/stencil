use crate::{drawing::FontSize, graphing::graphs::Graph};
use anyhow::{Result, anyhow};
use math::{Number, ZERO};
use std::fmt::Write;

pub enum GridType {
    Both,
    Major,
    None,
}

pub struct Axes {
    font_size: FontSize,

    // Coordinates for the viewport
    x_min: Number,
    x_max: Number,
    y_min: Option<Number>,
    y_max: Option<Number>,

    /// The amount of padding to add the y-axis after auto-fitting the range. Having some padding
    /// makes the graph look a bit more aesthetically pleasing, instead of the graph hitting the
    /// corner EVERY time which can look sterile
    padding: Number,

    // Distance between each tick
    x_tick: Option<Number>,
    y_tick: Option<Number>,
    x_minor_tick: Option<Number>,
    y_minor_tick: Option<Number>,
    has_minor_tick: bool,

    /// Where should grid lines be drawn?
    grid: GridType,

    /// Can the graph start at a non-zero value and "break" to the x-axis?
    can_break: bool,

    /// The actual graphs (plots) to be drawn in the coordinate system
    graphs: Vec<Graph>,
}

impl Default for Axes {
    fn default() -> Self {
        Axes {
            font_size: FontSize::Em(0.75),
            x_min: Number::Integer(-1),
            x_max: Number::Integer(1),
            y_min: None,
            y_max: None,
            x_tick: None,
            y_tick: None,
            has_minor_tick: false,
            x_minor_tick: None,
            y_minor_tick: None,
            grid: GridType::Both,
            can_break: false,
            graphs: Vec::new(),
            padding: ZERO,
        }
    }
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

    pub fn new_solution() -> Self {
        let mut axes = Axes::default();
        axes.font_size = FontSize::Em(1.0);
        axes
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

    pub fn x_tick(&mut self, distance: impl Into<Number>) -> &mut Self {
        self.x_tick = Some(distance.into());
        self
    }

    pub fn y_tick(&mut self, distance: impl Into<Number>) -> &mut Self {
        self.y_tick = Some(distance.into());
        self
    }

    pub fn with_minor_tick(&mut self) -> &mut Self {
        self.has_minor_tick = true;
        self
    }

    pub fn minor_x_tick(&mut self, distance: impl Into<Number>) -> &mut Self {
        self.x_minor_tick = Some(distance.into());
        self
    }

    pub fn minor_y_tick(&mut self, distance: impl Into<Number>) -> &mut Self {
        self.y_minor_tick = Some(distance.into());
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
        self.set_ticks();
        //self.auto_fit_range();

        // If we haven't passed any graphs, assume we want it empty and add an invisible line
        if self.graphs.len() == 0 {
            self.graphs.push(Graph::linear(0, self.y_max.unwrap() + 10));
        }

        let mut out = String::with_capacity(256);
        writeln!(
            out,
            "#block(height: 5cm)[#set text(size: {})\n#cetz.canvas({{",
            self.font_size
        )?;
        writeln!(out, "import cetz.draw: *")?;
        writeln!(out, "plot.plot(")?;
        writeln!(out, "axis-style: \"school-book\",")?;
        writeln!(out, "name: \"graph\",")?;
        writeln!(out, "size: (4, 4),")?;
        writeln!(out, "x-min: {},", self.x_min.for_graphs())?;
        writeln!(out, "x-max: {},", self.x_max.for_graphs())?;
        // This will always be Some() due to set_y_range().
        writeln!(out, "y-min: {},", self.y_min.unwrap().for_graphs())?;
        writeln!(out, "y-max: {},", self.y_max.unwrap().for_graphs())?;
        writeln!(out, "x-grid: \"{}\",", self.grid.to_typst())?;
        writeln!(out, "y-grid: \"{}\",", self.grid.to_typst())?;
        // This will always be Some() due to set_ticks().
        writeln!(out, "x-tick-step: {},", self.x_tick.unwrap().for_graphs())?;
        writeln!(out, "y-tick-step: {},", self.y_tick.unwrap().for_graphs())?;
        if let Some(x_minor_tick) = self.x_minor_tick {
            writeln!(out, "x-minor-tick-step: {},", x_minor_tick.for_graphs())?;
        }
        if let Some(y_minor_tick) = self.y_minor_tick {
            writeln!(out, "y-minor-tick-step: {},", y_minor_tick.for_graphs())?;
        }

        writeln!(out, "{{")?;
        for graph in self.graphs.iter() {
            writeln!(
                out,
                "plot.add(domain: ({}, {}), t => {})",
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
                "Tried to call auto_y_range() without adding a graph first. Please specify y_min and y_max if you want the graph to be empty",
            ));
        }

        let mut min = Number::Integer(i32::MAX);
        let mut max = Number::Integer(i32::MIN);
        for graph in self.graphs.iter() {
            // Check the endpoints of the graph
            for val in [self.x_min, self.x_max].iter() {
                if let Some(extreme) = graph.function.get_y(&val) {
                    if extreme < min {
                        min = extreme;
                    }
                    if extreme > max {
                        max = extreme;
                    }
                }
            }
            // Here we can also check for other extremes, like the extremum of a quadratic function
            // or amplitude of a sine wave
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

    // NOTE: Currently not in use, decided that the axes shouldn't be changed if explicitly set.
    // Make sure the graph looks OK when you create a problem

    // /// Makes the ratio between the x-axis and y-axis closer to 1 (including ticks)
    // /// to make the graph look good
    // fn auto_fit_range(&mut self) {
    //     let max_ratio = Number::Integer(2);
    //
    //     // Distance between grid lines
    //     // x_tick and y_tick will always be Some() here
    //     let (x_dist, y_dist) = match self.grid {
    //         GridType::Both => (
    //             self.x_minor_tick.unwrap_or(self.x_tick.unwrap()),
    //             self.y_minor_tick.unwrap_or(self.y_tick.unwrap()),
    //         ),
    //         _ => (self.x_tick.unwrap(), self.y_tick.unwrap()),
    //     };
    //
    //     // Alternate between increasing to the right and to the left
    //     let mut go_right = true;
    //     while ((self.y_max.unwrap() - &self.y_min.unwrap()) / &y_dist)
    //         / &((self.x_max - &self.x_min) / &x_dist)
    //         >= max_ratio
    //     {
    //         match go_right {
    //             true => self.x_max = self.x_max + 1,
    //             false => self.x_min = self.x_min - 1,
    //         }
    //         go_right = !go_right;
    //     }
    //
    //     let mut go_up = true;
    //     while ((self.x_max - &self.x_min) / &x_dist)
    //         / &((self.y_max.unwrap() - &self.y_min.unwrap()) / &y_dist)
    //         >= max_ratio
    //     {
    //         match go_up {
    //             true => self.y_max = Some(self.y_max.unwrap() + 1),
    //             false => self.y_min = Some(self.y_min.unwrap() - 1),
    //         }
    //         go_up = !go_up;
    //     }
    // }

    // Don't change ticks if explicitly set
    fn set_ticks(&mut self) {
        match (self.x_tick, self.y_tick) {
            (None, None) => self.auto_set_ticks(),
            (_, _) => (),
        }
    }

    /// Adjust the tick distance of the axes
    ///
    /// Not all ticks are created equal. When ticking a graph, we want to do jumps of:
    /// - 1
    /// - 5
    /// - Powers of 10 (100, 1000, 0.1, 0.01, ...)
    /// - 5 * powers of 10
    fn auto_set_ticks(&mut self) {
        // We need to keep track of whether the tick starts with a 1 or 5 to know what to multiply
        // with (5 if it's a one, 2 if it's a five)
        enum StartingNumber {
            One,
            Five,
        }

        let mut x_tick = Number::Integer(1);
        let mut y_tick = Number::Integer(1);

        for (tick, min, max) in [
            (&mut x_tick, self.x_min, self.x_max),
            (&mut y_tick, self.y_min.unwrap(), self.y_max.unwrap()),
        ] {
            let mut starting_number = StartingNumber::One;
            // Distance of 11 = 12 ticks
            while (max - &min) / &*tick > Number::Integer(11) {
                match starting_number {
                    StartingNumber::One => {
                        *tick *= 5;
                        starting_number = StartingNumber::Five;
                    }
                    StartingNumber::Five => {
                        *tick *= 2;
                        starting_number = StartingNumber::One;
                    }
                }
            }

            // Distance of 1 = 2 ticks
            while (max - &min) / &*tick < Number::Integer(1) {
                match starting_number {
                    StartingNumber::One => {
                        *tick /= 2;
                        starting_number = StartingNumber::Five;
                    }
                    StartingNumber::Five => {
                        *tick /= 5;
                        starting_number = StartingNumber::One;
                    }
                }
            }
        }

        if self.has_minor_tick {
            if x_tick != Number::Integer(1) {
                self.x_minor_tick = Some(x_tick / 5);
                self.grid = GridType::Both;
            }

            if y_tick != Number::Integer(1) {
                self.y_minor_tick = Some(y_tick / 5);
                self.grid = GridType::Both;
            }
        }

        self.x_tick = Some(x_tick);
        self.y_tick = Some(y_tick);
    }
}
