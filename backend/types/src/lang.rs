use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Language {
    Sv,
    En,
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "sv" => Language::Sv,
            "en" => Language::En,
            _ => {
                error!("Recieved unknown String {value} when converting to Language.");
                Language::Sv
            }
        }
    }
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
        write!(f, "{}", self.to_str())
    }
}
