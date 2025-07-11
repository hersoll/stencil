// server_app/src/registry.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::backend::ProblemType;

pub static PROBLEM_REGISTRY: Lazy<Mutex<HashMap<String, super::ProblemType>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

