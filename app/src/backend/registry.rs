use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

/// A map between problem names (simple-equations-default) and ProblemTypes
pub static PROBLEM_MAP: LazyLock<RwLock<HashMap<String, super::ProblemType>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// Keeps track of course structure
pub static PROBLEM_REGISTRY: Lazy<crate::shared::ProblemRegistry> = Lazy::new(|| {
    let json = std::fs::read_to_string("registry.json").expect("Failed to read registry.json");
    let parsed: crate::shared::ProblemRegistry =
        serde_json::from_str(&json).expect("Failed to parse registry JSON");
    parsed
});
