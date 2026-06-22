use crate::picker;
use anyhow::{Result, anyhow};
use math::Number;
pub use registry::RegistryError;
use registry::get_problem_data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use tracing::error;
use types::difficulty::{AbsoluteDifficulty, DifficultyCategory};
pub use types::problems::Problem;
use types::{errors::ApiError, lang::Language};

const DEFAULT_STARTING_DIFFICULTY: DifficultyCategory = DifficultyCategory::Intro;
const DEFAULT_ENDING_DIFFICULTY: DifficultyCategory = DifficultyCategory::Hard;
const DEFAULT_PROBLEM_COUNT: u8 = 20;

pub type ProblemGenerator = fn(i32, Language) -> Result<Problem>;

/// A map between problem names with `{module}_{problem}` syntax
/// (e.g. `simple_equations_default`) and their functions.
///
/// This HashMap is written to in the problem! macro (during startup, before `main`)
pub static PROBLEM_NAME_TO_FUNCTION_MAP: LazyLock<RwLock<HashMap<String, ProblemGenerator>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Information about what to include in the problem set
///
/// Should be included in the HTTP request in the form of a Vec<ProblemSetSpec>
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProblemOptions {
    /// Topics to draw problems from
    pub topics: Vec<i32>,
    /// Which problems to exclude from the generator
    #[serde(default)]
    pub exclusions: Vec<i32>,
    pub starting_difficulty: DifficultyCategory,
    pub ending_difficulty: DifficultyCategory,
    /// Number of problems
    pub n: u8,
}

impl Default for ProblemOptions {
    /// Mostly used for the /pdf/example endpoint
    fn default() -> ProblemOptions {
        ProblemOptions {
            topics: Vec::new(),
            exclusions: Vec::new(),
            starting_difficulty: DEFAULT_STARTING_DIFFICULTY,
            ending_difficulty: DEFAULT_ENDING_DIFFICULTY,
            n: DEFAULT_PROBLEM_COUNT,
        }
    }
}

/// Generates problems and distributes them across the desired difficulties.
///
/// As the "main" function of this module, this function takes the input [`ProblemOptions`] and
/// finds out which [`Difficulties`](DifficultyCategory) to choose and how many problems from each, and then
/// generates those problems and returns them.
pub async fn generate_problem_set(
    options: ProblemOptions,
    lang: Language,
) -> Result<Vec<Problem>, ApiError> {
    // Is of type {id: i32, absolute_difficulty, relative_difficulty}
    let problems = db::get_valid_problems_from_pdf_request(
        options.topics,
        options.exclusions,
        DifficultyCategory::categories_to_absolute_difficulties(
            &options.starting_difficulty,
            &options.ending_difficulty,
        ),
    )
    .await?;

    let problem_ids = picker::select_problems(
        options.n,
        &problems,
        AbsoluteDifficulty::from_num(DifficultyCategory::to_minimum_difficulty_num(
            &options.starting_difficulty,
        )),
        AbsoluteDifficulty::from_num(DifficultyCategory::to_maximum_difficulty_num(
            &options.ending_difficulty,
        )),
    )?;
    tracing::debug!("Problem order:\n{problem_ids:?}");

    let problem_set = generate_problems(&problem_ids, lang)?;
    Ok(problem_set)
}

fn generate_problems(problem_ids: &[i32], lang: Language) -> Result<Vec<Problem>> {
    // The actual generated problems
    let mut problems = Vec::new();
    let mut generated_identifiers_per_problem: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();

    for problem_id in problem_ids {
        problems.push(get_unique_problem(
            *problem_id,
            generated_identifiers_per_problem
                .entry(*problem_id)
                .or_default(),
            lang,
        )?);
    }
    Ok(problems)
}

/// Generates a problem with a unique ID (the actual numbers that makes the problem different).
///
/// If there are no more possible IDs to generate, reset (which might repeat problems)
fn get_unique_problem(
    problem_id: i32,
    generated_identifiers: &mut Vec<Vec<i32>>,
    lang: Language,
) -> Result<Problem> {
    let generator = get_generator_function(problem_id)?;
    let mut problem = (generator)(problem_id, lang)?;

    // Reset if all combinations are exhausted
    if generated_identifiers.len() >= problem.combinations {
        generated_identifiers.clear();
    }

    let mut problem_identifiers_as_i32 = extract_identifiers(&problem.identifiers);

    // If we generate a non-unique problem, retry until we get a unique problem.
    // We should always be able to do this, since the earlier `if` statement makes
    // sure there are less finished problems than there are combinations.
    //
    // Therefore, if we are unable to generate a unique problem, something has gone wrong,
    // most likely when defining `identifiers` and `combinations` in the `Problem`. Fix it!!!
    let mut tries = 0u16;
    while generated_identifiers.contains(&problem_identifiers_as_i32) {
        problem = (generator)(problem_id, lang)?;
        problem_identifiers_as_i32 = extract_identifiers(&problem.identifiers);
        tries += 1;

        // 65_535 tries until we call it a day
        if tries == u16::MAX {
            let error_msg = format!(
                "Stuck while generating problem {}! 
Check the identifiers and combinations in the Problem definition",
                problem_id
            );
            tracing::error!(error_msg);
            return Err(anyhow!(error_msg));
        }
    }
    generated_identifiers.push(problem_identifiers_as_i32);
    Ok(problem)
}

/// Converts identifiers into `i32`s.
///
/// While the identifiers are [`Numbers`](Number), they can't be hashed due to the `f64` variant.
/// It's easier to store them as `i32`s.
fn extract_identifiers(identifiers: &[Number]) -> Vec<i32> {
    identifiers
        .iter()
        .map(|num| match num {
            Number::Integer(i) => *i,
            Number::Decimal { integer, .. } => *integer,
            Number::Fraction { numerator, .. } => *numerator,
            Number::Irrational { .. } => 0i32,
        })
        .collect()
}

/// Given a problem_id, returns a pointer to the function that generates that problem.
fn get_generator_function(id: i32) -> Result<ProblemGenerator> {
    let generator = {
        let problem_data = get_problem_data(id)?;
        let problem_name = problem_data.module + "_" + &problem_data.name;

        let lock = PROBLEM_NAME_TO_FUNCTION_MAP
            .read()
            .expect("Mutex is poisoned");
        lock.get(&problem_name).copied().ok_or_else(|| {
            error!("Failed to retrieve problem {problem_name}");
            RegistryError::ProblemNotFound { id }
        })?
    }; // Lock is dropped here

    Ok(generator)
}
