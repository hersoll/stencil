mod number_line;

pub use number_line::NumberLine;

use std::fmt::Display;

pub(crate) enum FontSize {
    Points(f32),
    Em(f32),
}

impl Display for FontSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontSize::Points(pt) => write!(f, "{pt}pt")?,
            FontSize::Em(em) => write!(f, "{em}em")?,
        };
        Ok(())
    }
}
