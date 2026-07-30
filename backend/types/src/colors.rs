use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::error;

const ALLOWED_HEX_CHARS: &str = "0123456789abcdefABCDEF";

static SOLUTION_TEXT_COLORED: &str = "#8A6AA8";
static SOLUTION_TEXT_BW: &str = "#949494";

static SOLUTION_FILL_COLORED: &str = "#F7F6E9";
static SOLUTION_FILL_BW: &str = "#F9F9F9";
static SOLUTION_BORDER_COLORED: &str = "#B79DCF";
static SOLUTION_BORDER_BW: &str = "#949494";

static PRIMARY_COLOR_COLORED: &str = "#B79DCF";
static PRIMARY_COLOR_BW: &str = "#222222";
static SECONDARY_COLOR_COLORED: &str = "#B79DCF";
static SECONDARY_COLOR_BW: &str = "#222222";
static TERTIARY_COLOR_COLORED: &str = "#B79DCF";
static TERTIARY_COLOR_BW: &str = "#222222";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct HexColor(String);

impl Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl From<String> for HexColor {
    fn from(s: String) -> Self {
        // Strip eventual # so every string is the same
        let s = s.strip_prefix("#").unwrap_or(&s);

        let s = if s.len() > 6 { &s[..6] } else { s };

        if s.chars().all(|c| ALLOWED_HEX_CHARS.contains(c)) {
            Self(s.to_string())
        } else {
            error!("Got invalid hex color: #{s}");
            Self("000000".to_string())
        }
    }
}

impl From<&str> for HexColor {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<HexColor> for String {
    fn from(color: HexColor) -> Self {
        color.to_string()
    }
}

impl HexColor {
    pub fn default_solution_text(colored: bool) -> Self {
        if colored {
            Self::from(SOLUTION_TEXT_COLORED)
        } else {
            Self::from(SOLUTION_TEXT_BW)
        }
    }
    pub fn default_solution_fill(colored: bool) -> Self {
        if colored {
            Self::from(SOLUTION_FILL_COLORED)
        } else {
            Self::from(SOLUTION_FILL_BW)
        }
    }

    pub fn default_solution_border(colored: bool) -> Self {
        if colored {
            Self::from(SOLUTION_BORDER_COLORED)
        } else {
            Self::from(SOLUTION_BORDER_BW)
        }
    }

    pub fn default_primary(colored: bool) -> Self {
        if colored {
            Self::from(PRIMARY_COLOR_COLORED)
        } else {
            Self::from(PRIMARY_COLOR_BW)
        }
    }

    pub fn default_secondary(colored: bool) -> Self {
        if colored {
            Self::from(SECONDARY_COLOR_COLORED)
        } else {
            Self::from(SECONDARY_COLOR_BW)
        }
    }

    pub fn default_tertiary(colored: bool) -> Self {
        if colored {
            Self::from(TERTIARY_COLOR_COLORED)
        } else {
            Self::from(TERTIARY_COLOR_BW)
        }
    }

    pub fn white() -> Self {
        Self::from("ffffff")
    }
}
