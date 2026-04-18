use anyhow::Result;
use math::Number;
use std::{
    cmp::{max, min},
    fmt::Write,
};

use crate::drawing::FontSize;

/// The length of the line, including arrow
const LINE_LENGTH: f32 = 5.0;
/// The length of the part that ticks can be placed on
const TICK_SPACE: f32 = 4.5;
/// The height of the tick from the line.
///
/// Note that this is just the deviation from the line, so the total tick will be twice this height
/// since it deviates up and down.
const TICK_HEIGHT: f32 = 0.1;
const MAJOR_TICK_HEIGHT: f32 = 0.15;
const TICK_LABEL_POS: f32 = -0.3;

struct Tick {
    label: String,
    position: f32,
}

struct Arc {
    start: Number,
    end: Number,
    label: String,
}

pub struct NumberLine {
    /// The starting number
    min: Number,
    /// The maximum number that needs to exist on the line
    ///
    /// NOTE: This is not _necessarily_ the same as the endpoint.
    /// The code makes sure to have one tick that is _at least_ this much. If you want to make sure
    /// this value is ticked, make sure `tick_step` aligns.
    max: Number,
    tick_step: Number,
    font_size: FontSize,

    arc: Option<Arc>,
}

impl Default for NumberLine {
    fn default() -> Self {
        Self {
            min: Number::Integer(0),
            max: Number::Integer(5),
            tick_step: Number::Integer(1),
            font_size: FontSize::Em(1.0),
            arc: None,
        }
    }
}

impl NumberLine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_ends(first: impl Into<Number>, second: impl Into<Number>) -> Self {
        let first = first.into();
        let second = second.into();
        let min = min(first, second);
        let max = max(first, second);
        Self {
            min,
            max,
            tick_step: Number::Integer(1),
            font_size: FontSize::Em(1.0),
            arc: None,
        }
    }

    pub fn with_ends(mut self, first: impl Into<Number>, second: impl Into<Number>) -> Self {
        let first = first.into();
        let second = second.into();
        let min = min(first, second);
        let max = max(first, second);
        self.min = min;
        self.max = max;
        self
    }

    pub fn with_arc(
        mut self,
        start: impl Into<Number>,
        end: impl Into<Number>,
        label: impl Into<String>,
    ) -> Self {
        let arc = Arc {
            start: start.into(),
            end: end.into(),
            label: label.into(),
        };
        self.arc = Some(arc);
        self
    }

    pub fn build_string(&mut self) -> Result<String> {
        let mut out = String::with_capacity(256);
        let ticks = self.get_ticks();
        writeln!(
            out,
            "#block[#set text(size: {})\n#cetz.canvas({{",
            self.font_size
        )?;
        writeln!(out, "import cetz.draw: *")?;
        writeln!(
            out,
            "line((0, 0), ({LINE_LENGTH}, 0), mark: (end: \">\"), fill: black)"
        )?;
        for tick in ticks {
            let label = tick.label;
            let pos = tick.position;
            let tick_height = if label == "0" {
                MAJOR_TICK_HEIGHT
            } else {
                TICK_HEIGHT
            };
            writeln!(out, "line(({pos}, {tick_height}), ({pos}, -{tick_height}))")?;
            writeln!(out, "content(({pos}, {TICK_LABEL_POS}), str({label}))")?;
        }

        if let Some(arc) = &self.arc {
            let start =
                ((arc.start - self.min) / (self.max - self.min)).value() as f32 * TICK_SPACE;
            let end = ((arc.end - self.min) / (self.max - self.min)).value() as f32 * TICK_SPACE;
            let middle = (start + end) / 2.0;

            writeln!(
                out,
                "bezier-through(({start}, 0), ({middle}, 0.25), ({end}, 0), mark: (end: \">\", fill: purple), stroke: purple)"
            )?;
            writeln!(
                out,
                "content(({middle}, 0.5), text(fill: purple)[${}$])",
                arc.label
            )?;
        }
        writeln!(out, "}})]")?;
        Ok(out)
    }

    fn get_ticks(&mut self) -> Vec<Tick> {
        let total_distance = self.max - self.min;
        // If min = 1, max = 7.5, tick_step = 2.5 => should be 4 ticks: 1, 3.5, 6, 8.5
        // total_distance = 6.5, t_d / tick_step = 2.6, so ceil() it and add 1.
        let amount_of_ticks = (total_distance / self.tick_step).value().ceil() as i32 + 1;

        // Need to adjust font size in case there are too many ticks:
        if amount_of_ticks >= 15 {
            self.font_size = FontSize::Em(0.9);
        }

        (0..amount_of_ticks)
            .map(|i| {
                let label_num = self.min + self.tick_step * i;
                let position: f32 = i as f32 / (amount_of_ticks - 1) as f32 * TICK_SPACE;
                Tick {
                    label: label_num.to_string(),
                    position,
                }
            })
            .collect()
    }
}
