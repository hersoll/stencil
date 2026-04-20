use db::{self, PrefixEntry, ProblemEntry};

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("problem id not found: {name}")]
    ProblemNotFound { name: String },
    #[error("prefix id not found: {id}")]
    PrefixNotFound { id: i32 },
    #[error("Mutex {registry} is poisoned")]
    RegistryMutexIsPoisoned { registry: String },
}

pub static PROBLEM_DATA: LazyLock<RwLock<HashMap<String, ProblemEntry>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub static PREFIX_DATA: LazyLock<RwLock<HashMap<i32, PrefixEntry>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Loads data about every problem in the database to the PROBLEM_DATA hashmap
/// to lessen database hits during runtime
pub async fn load_problem_data() -> Result<()> {
    let problems = db::get_all_problem_data().await?;
    for problem in problems {
        PROBLEM_DATA
            .write()
            .ok()
            .context("Failed to write to PROBLEM_DATA")?
            .insert(problem.module.clone() + "_" + &problem.name, problem);
    }
    Ok(())
}

/// Loads every prefix in the database to the PREFIX_DATA hashmap
/// to lessen database hits during runtime
pub async fn load_prefix_data() -> Result<()> {
    let prefixes = db::get_all_prefix_data().await?;
    for prefix in prefixes {
        PREFIX_DATA
            .write()
            .ok()
            .context("Failed to write to PREFIX_DATA")?
            .insert(prefix.id, prefix);
    }
    Ok(())
}

pub fn get_problem_data(full_name: &str) -> Result<ProblemEntry> {
    let problem = PROBLEM_DATA
        .read()
        .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
            registry: "PROBLEM_DATA".to_string(),
        })?
        .get(full_name)
        .cloned()
        .ok_or(RegistryError::ProblemNotFound {
            name: full_name.to_string(),
        })?;
    Ok(problem)
}

pub fn get_prefix_data(id: i32) -> Result<PrefixEntry> {
    let prefix = PREFIX_DATA
        .read()
        .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
            registry: "PREFIX_DATA".to_string(),
        })?
        .get(&id)
        .cloned()
        .ok_or(RegistryError::PrefixNotFound { id })?;
    Ok(prefix)
}

/// Used in problems with dynamic text questions, for example:
/// "Use the function {f} to solve..."
pub fn replace_placeholders(template: &str, values: &[(&str, String)]) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}
