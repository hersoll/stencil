use crate::math::Number;

/// Additional elements that need to be added to the plot,
/// like dots, dashed lines, labels
pub struct PlotElements {
    /// Elements where the coordinates matter,
    /// like dots and lines
    axis_relative: Vec<String>,
    /// Elements where the distances need to be the same
    /// no matter the plot, like labels.
    canvas_relative: Vec<String>,
}

pub enum Plot {
    /// k, m
    Linear(Number, Number, PlotElements),
    Polynomial(Vec<Number>, PlotElements),
    /// start, change
    Exponential(Number, Number, PlotElements),
}

impl Plot {
    pub fn linear(k: Number, m: Number) -> Plot {
        Plot::Linear(k, m, PlotElements::default())
    }

    pub fn exponential(start: Number, change: Number) -> Plot {
        Plot::Exponential(start, change, PlotElements::default())
    }

    pub fn to_typst(&self) -> String {
        match self {
            Plot::Linear(k, m, _) => format!("{} * float(t) + {}", k.for_plots(), m.for_plots()),
            Plot::Exponential(start, change, _) => format!(
                "{} * calc.pow({}, t)",
                start.for_plots(),
                change.for_plots()
            ),
            // TODO:
            Plot::Polynomial(_, _) => format!("3"),
        }
    }
}

impl Default for PlotElements {
    fn default() -> Self {
        PlotElements {
            axis_relative: Vec::new(),
            canvas_relative: Vec::new(),
        }
    }
}
