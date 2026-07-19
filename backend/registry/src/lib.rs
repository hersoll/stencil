//! The `registry` module is responsible for holding static data which is retrieved during startup.
//!
//! This data is mostly accessed during problem generation, and since data about problems
//! won't change during runtime (in production), this is done to minimize DB hits (100 problems
//! would cause AT LEAST 100 DB hits).
mod split_strings;
use split_strings::*;

use anyhow::{Context, Result};
use db::{self, PrefixEntry, ProblemEntry};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use types::format_strings::{Answer, Question, Solution};
use types::lang::Language;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("problem id not found: {id}")]
    ProblemNotFound { id: i32 },
    #[error("prefix id not found: {id}")]
    PrefixNotFound { id: i32 },
    #[error("Mutex {registry} is poisoned")]
    RegistryMutexIsPoisoned { registry: String },
}

/// A map between a `Problem's` id in the DB and a [`SplitProblemEntry`] struct.
///
/// The `ProblemEntry` contains all the information about the problem, like questions for each
/// language, its difficulty, etc.
pub static PROBLEM_DATA: LazyLock<RwLock<HashMap<i32, SplitProblemEntry>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// A map between a `Prefix's` id in the DB and a [`PrefixEntry`] struct.
pub static PREFIX_DATA: LazyLock<RwLock<HashMap<i32, PrefixEntry>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Loads data about every problem in the database to the `PROBLEM_DATA` hashmap.
///
/// Accessed during startup
pub async fn load_problem_data() -> Result<()> {
    let problems = db::get_all_problem_data().await?;
    for problem in problems {
        PROBLEM_DATA
            .write()
            .ok()
            .context("Failed to write to PROBLEM_DATA")?
            .insert(problem.id, problem.try_into()?);
    }
    Ok(())
}

/// Loads every prefix in the database to the PREFIX_DATA hashmap
///
/// Accessed during startup
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

/// Get information about prefixes through their `id`
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

/// Read information about a problem from the registry
///
/// Has shortcut methods for when you only need a specific field:
/// [`get_question()`], [`get_answer()`], [`get_solution()`]
pub fn get_problem_data(id: i32) -> Result<ProblemEntry> {
    match PROBLEM_DATA
        .write()
        .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
            registry: "PROBLEM_DATA".to_string(),
        })?
        .get_mut(&id)
        .ok_or(RegistryError::ProblemNotFound { id })?
    {
        SplitProblemEntry::Single(entry) => Ok(entry.clone()),
        SplitProblemEntry::Multiple {
            entry,
            latest_index,
            split_texts,
        } => {
            *latest_index = (*latest_index + 1) % split_texts.iter().len();
            let mut new_entry = entry.clone();
            new_entry.translations = split_texts[*latest_index].clone();
            Ok(new_entry)
        }
    }
}

/// Returns an owned Question from the registry
///
/// The reason we clone and return it owned is that problem functions always return an
/// owned struct anyway, so this must be cloned somewhere.
pub fn get_question(id: i32, lang: Language) -> Result<Question> {
    Ok(get_problem_data(id)?.get_question(lang).clone())
}

/// Returns an owned Answer from the registry
///
/// The reason we clone and return it owned is that problem functions always return an
/// owned struct anyway, so this must be cloned somewhere.
pub fn get_answer(id: i32, lang: Language) -> Result<Answer> {
    Ok(get_problem_data(id)?.get_answer(lang).clone())
}

/// Returns an owned Solution from the registry
///
/// The reason we clone and return it owned is that problem functions always return an
/// owned struct anyway, so this must be cloned somewhere.
pub fn get_solution(id: i32, lang: Language) -> Result<Solution> {
    Ok(get_problem_data(id)?.get_solution(lang).clone())
}
