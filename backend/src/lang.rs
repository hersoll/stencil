use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
