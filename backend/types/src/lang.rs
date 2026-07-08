use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Language {
    Sv,
    En,
}

impl Language {
    pub fn to_str(&self) -> &str {
        match self {
            Language::Sv => "sv",
            Language::En => "en",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Sv => write!(f, "sv"),
            Language::En => write!(f, "en"),
        }
    }
}
