use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use crate::{
    db, 
    problems::problem_picker::ProblemGenerator, 
    shared::{
        ParsedPrefixData,
        ParsedProblemData,
        PrefixData, 
        ProblemData
    }
};
use anyhow::{Context, Result};

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("problem id not found: {id}")]
    ProblemNotFound { id: String },
    #[error("prefix id not found: {id}")]
    PrefixNotFound { id: i32 },
    #[error("Mutex {registry} is poisoned")]
    RegistryMutexIsPoisoned { registry: String },
}

/// A map between problem names (simple_equations_default) and their functions
///
/// This HashMap is written to in the problem! macro (during startup)
pub static PROBLEM_MAP: LazyLock<RwLock<HashMap<String, ProblemGenerator>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub static PROBLEM_DATA: LazyLock<RwLock<HashMap<String, ProblemData>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub static PREFIX_DATA: LazyLock<RwLock<HashMap<i32, PrefixData>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Loads data about every problem in the database to the PROBLEM_DATA hashmap
/// to lessen database hits during runtime
pub async fn load_problem_data() -> Result<()> {
    let problems = db::problems::get_all_problem_data().await?;
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
    let prefixes = db::problems::get_all_prefix_data().await?;
    for prefix in prefixes {
        PREFIX_DATA
            .write()
            .ok()
            .context("Failed to write to PREFIX_DATA")?
            .insert(prefix.id, prefix);
    }
    Ok(())
}

/// Gets problem data from the database and returns it with only the relevant language
pub fn get_parsed_problem(full_name: &str, lang: &str) -> Result<ParsedProblemData> {
    let problem = PROBLEM_DATA
        .read()
        .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
            registry: "PROBLEM_DATA".to_string(),
        })?
        .get(full_name)
        .cloned()
        .ok_or(RegistryError::ProblemNotFound {
            id: full_name.to_string(),
        })?;
    Ok(problem.parse(lang))
}

/// Gets prefix data from the database and returns it with only the relevant language
pub fn get_parsed_prefix(id: i32, lang: &str) -> Result<ParsedPrefixData> {
    let prefix = PREFIX_DATA
        .read()
        .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
            registry: "PREFIX_DATA".to_string(),
        })?
        .get(&id)
        .cloned()
        .ok_or(RegistryError::PrefixNotFound { id })?;
    Ok(prefix.parse(lang))
}

/// Used in problems with dynamic text questions, for example:
/// "Use the function {f} to solve..."
///
/// TODO: Find a more appropriate module for this function
pub fn replace_placeholders(template: &str, values: &HashMap<&str, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}
