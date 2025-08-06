use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use crate::Error;
use crate::Result;
use crate::shared::{ParsedProblemData, ProblemData};

/// A map between problem names (simple-equations-default) and ProblemTypes
pub static PROBLEM_MAP: LazyLock<RwLock<HashMap<String, super::ProblemType>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub static PROBLEM_DATA: LazyLock<RwLock<HashMap<String, ProblemData>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn load_problem_data() -> Result<()> {
    let problems = crate::backend::db::ProblemDatabase::get_all_problem_data().await?;
    for problem in problems {
        PROBLEM_DATA
            .write()
            .map_err(|_| Error::RegistryMutexIsPoisoned)?
            .insert(problem.module.clone() + "_" + &problem.name, problem);
    }
    Ok(())
}

pub fn get_parsed_problem(full_name: &str, lang: &str) -> Result<ParsedProblemData> {
    let problem = PROBLEM_DATA
        .read()
        .map_err(|_| Error::RegistryMutexIsPoisoned)?
        .get(full_name)
        .cloned()
        .ok_or(Error::NoSuchProblemInRegistry {
            id: full_name.to_string(),
        })?;
    Ok(problem.parse(lang))
}

pub fn replace_placeholders(template: &str, values: &HashMap<&str, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}
