pub use crate::picker;
use crate::picker::CountPerDifficultyCategoryNumber;
use anyhow::{Context, Result, anyhow};
use math::Number;
use rand::{rngs::ThreadRng, seq::IndexedRandom};
pub use registry::RegistryError;
use registry::get_problem_data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use std::{cmp::Ordering, collections::HashSet};
use tracing::error;
use types::difficulty::{AbsoluteDifficulty, DifficultyCategory, RelativeDifficulty};
pub use types::problems::Problem;
use types::{errors::ApiError, lang::Language};

const DEFAULT_SCORE: u8 = 100;
const DEFAULT_STARTING_DIFFICULTY: DifficultyCategory = DifficultyCategory::Intro;
const DEFAULT_ENDING_DIFFICULTY: DifficultyCategory = DifficultyCategory::Hard;
const DEFAULT_PROBLEM_COUNT: u8 = 10;

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

/// Defines which problems are available to choose from and which
/// criteria the problem_picker functions can use to
/// choose which problems to generate
///
/// # Lifetime
/// The [`Language`] reference comes from a higher order function and will live during the entire
/// lifetime of the [`ProblemPool`]
#[derive(Debug)]
pub struct ProblemPool<'pool> {
    pub problem_candidates: Vec<ProblemCandidate>,
    pub lang: &'pool Language,
    pub starting_difficulty: DifficultyCategory,
    pub ending_difficulty: DifficultyCategory,
    pub n: u8,
}

/// Problem that is a candidate for selection.
/// Data grouping for easier ergonomics when choosing problems.
///
/// The [`picker`] module will filter down the candidate list throughout the module,
/// until problems are generated.
#[derive(Debug, Clone, Eq)]
pub struct ProblemCandidate {
    pub id: i32,
    pub absolute_difficulty: AbsoluteDifficulty,
    pub relative_difficulty: RelativeDifficulty,

    /// The "score" is what the module uses to determine which problem is chosen.
    /// When a problem is generated, that problem's score is lowered.
    ///
    /// This assures a variety of problems (if there is variety, that is)
    pub score: u8,

    /// Identifiers of already generated problems.
    ///
    /// Stored with the [`ProblemCandidate`] to make the `identifiers` persist between generations.
    pub generated_identifiers: HashSet<Vec<i32>>,
}

impl PartialEq for ProblemCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Generates problems and distributes them across the desired difficulties.
///
/// As the "main" function of this module, this function takes the input [`ProblemOptions`] and
/// finds out which [`Difficulties`](DifficultyCategory) to choose and how many problems from each, and then
/// generates those problems and returns them.
pub async fn generate_problem_set(
    options: ProblemOptions,
    lang: &Language,
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

    // Construct an initial list of candidates from the problem ids
    let problem_candidates: Vec<ProblemCandidate> = problems
        .into_iter()
        .map(|problem| ProblemCandidate {
            id: problem.id,
            absolute_difficulty: problem.absolute_difficulty,
            relative_difficulty: problem.relative_difficulty,
            score: DEFAULT_SCORE.max(options.n),
            generated_identifiers: HashSet::new(),
        })
        .collect();

    let mut problem_pool = ProblemPool {
        problem_candidates,
        lang,
        starting_difficulty: options.starting_difficulty,
        ending_difficulty: options.ending_difficulty,
        n: options.n,
    };
    picker::filter_pool_by_difficulty(&mut problem_pool)?;
    let distribution_by_difficulty_num = picker::distribute_problems(&problem_pool)?;

    let mut rng = rand::rng();
    let problem_set =
        generate_problems(&mut problem_pool, distribution_by_difficulty_num, &mut rng)?;
    Ok(problem_set)
}

fn generate_problems(
    problem_pool: &mut ProblemPool,
    distribution_by_difficulty_num: CountPerDifficultyCategoryNumber,
    rng: &mut ThreadRng,
) -> Result<Vec<Problem>> {
    // The actual generated problems
    let mut problems = Vec::new();

    // Take all candidates and sort them into difficulty categories.
    // This will speed up the problem generation significantly
    let mut problem_indices_by_difficulty: [Vec<usize>; 11] = Default::default();
    for (i, candidate) in problem_pool.problem_candidates.iter().enumerate() {
        let difficulty = candidate.absolute_difficulty.number as usize;
        //Check bounds
        if difficulty < problem_indices_by_difficulty.len() {
            problem_indices_by_difficulty[difficulty].push(i);
        }
    }

    // This loop goes through each difficulty number, and finds the max score of that
    // difficulty number. If several problems has the max score, one is chosen at random.
    // It generates a problem of that type, and lowers that problem's score.
    // Indices are tracked to be able to change the relevant score in `candidates_with_scores`
    for (difficulty, &count) in distribution_by_difficulty_num.0.iter().enumerate() {
        let indices = &problem_indices_by_difficulty[difficulty];
        // Skip difficulties that don't have problems
        if indices.is_empty() || count == 0 {
            continue;
        }

        for _ in 0..count {
            // Find the max score among the indices
            let mut max_score = 0u8;
            let mut best_indices = Vec::new();

            for &idx in indices {
                let candidate = &problem_pool.problem_candidates[idx];
                match candidate.score.cmp(&max_score) {
                    Ordering::Greater => {
                        max_score = candidate.score;
                        best_indices.clear();
                        best_indices.push(idx);
                    }
                    Ordering::Equal => {
                        best_indices.push(idx);
                    }
                    _ => {}
                }
            }

            let chosen_index = *best_indices
                .choose(rng)
                .context("No valid problems within the max_indices")?;

            let chosen_candidate = &mut problem_pool.problem_candidates[chosen_index];
            let problem = get_unique_problem(chosen_candidate, *problem_pool.lang)?;
            // Lower the score
            chosen_candidate.score = chosen_candidate.score.saturating_sub(1);
            problems.push(problem);
        }
    }
    Ok(problems)
}

/// Generates a problem with a unique ID (the actual numbers that makes the problem different).
///
/// If there are no more possible IDs to generate, reset (which might repeat problems)
fn get_unique_problem(candidate: &mut ProblemCandidate, lang: Language) -> Result<Problem> {
    let generator = get_generator_function(candidate.id)?;
    let mut problem = (generator)(candidate.id, lang)?;

    // Reset if all combinations are exhausted
    if candidate.generated_identifiers.len() >= problem.combinations {
        candidate.generated_identifiers.clear();
    }

    let mut problem_identifiers_as_i32 = extract_identifiers(&problem.identifiers);

    // If we generate a non-unique problem, retry until we get a unique problem.
    // We should always be able to do this, since the earlier `if` statement makes
    // sure there are less finished problems than there are combinations.
    //
    // Therefore, if we are unable to generate a unique problem, something has gone wrong,
    // most likely when defining `identifiers` and `combinations` in the `Problem`. Fix it!!!
    let mut tries = 0u16;
    while candidate
        .generated_identifiers
        .contains(&problem_identifiers_as_i32)
    {
        problem = (generator)(candidate.id, lang)?;
        problem_identifiers_as_i32 = extract_identifiers(&problem.identifiers);
        tries += 1;

        // 65_535 tries until we call it a day
        if tries == u16::MAX {
            let error_msg = format!(
                "Stuck while generating problem {}! 
Check the identifiers and combinations in the Problem definition",
                candidate.id
            );
            tracing::error!(error_msg);
            return Err(anyhow!(error_msg));
        }
    }
    candidate
        .generated_identifiers
        .insert(problem_identifiers_as_i32);
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
