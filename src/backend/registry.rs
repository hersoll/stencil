use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

/// A map between problem names (simple-equations-default) and ProblemTypes
pub static PROBLEM_MAP: LazyLock<RwLock<HashMap<String, super::ProblemType>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
