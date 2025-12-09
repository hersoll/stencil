use crate::{
    db,
    pdf_generation::ProblemSetSpec,
    problems::{Difficulty, Problem, ProblemGenerator, ProblemId},
    RegistryError,
};
use anyhow::{anyhow, Context, Result};
use std::cmp::Ordering;

use rand::{rngs::ThreadRng, seq::IndexedRandom};

/// Keeps track of which difficulty and generator function is
/// associated with each problem.
///
/// Used throughout the problem_picker functions to group the data together
#[derive(Debug, Clone, Eq)]
pub struct ProblemDescriptor {
    pub name: String,
    pub difficulty: u8,
    pub generator: ProblemGenerator,
}

impl PartialEq for ProblemDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// Defines which criteria the problem_picker functions can use to
/// choose which problems to generate
#[derive(Debug)]
pub struct ProblemPool {
    problems: Vec<ProblemDescriptor>,
    lang: String,
    starting_difficulty: Difficulty,
    ending_difficulty: Difficulty,
    n: u8,
}

/// The entry point of the module.
///
/// Given the SetInformation, finds the appropriate problems
/// and distributes them appropriately across the difficulties given
pub async fn generate_problems_for_set(set: ProblemSetSpec, lang: String) -> Result<Vec<Problem>> {
    // Look at the topics (and exclusions) from the http,
    // get all matching problem names from the db
    let problem_names = db::problems::get_problem_names_for_pdf(set.topics, set.exclusions).await?;

    let problems: Vec<ProblemDescriptor> = problem_names
        .iter()
        .map(|name| {
            let generator = get_generator_function(name)?;
            let difficulty = get_problem_difficulty(name)?;
            Ok(ProblemDescriptor {
                name: name.to_string(),
                difficulty,
                generator,
            })
        })
        .collect::<Result<Vec<ProblemDescriptor>>>()?;

    let problem_pool = ProblemPool {
        problems,
        lang,
        starting_difficulty: set.starting_difficulty,
        ending_difficulty: set.ending_difficulty,
        n: set.n,
    };
    let mut rng = rand::rng();
    let problem_set = choose_problems(problem_pool, &mut rng)?;

    Ok(problem_set)
}

/// Given a complete problem name (module_problem),
/// returns a pointer to the function that generates that problem.
fn get_generator_function(name: &String) -> Result<ProblemGenerator> {
    let generator = {
        let lock = crate::PROBLEM_MAP.read().expect("Mutex is poisoned");
        lock.get(name)
            .copied()
            .ok_or(RegistryError::ProblemNotFound {
                id: name.to_string(),
            })?
    }; // Lock is dropped here

    Ok(generator)
}

/// Given a complete problem name (module_problem),
/// returns the difficulty of that problem.
///
/// Used after retrieving the problem names from
/// PROBLEM_MAP matching a HTTP request.
fn get_problem_difficulty(name: &String) -> Result<u8> {
    let difficulty = {
        let lock = crate::PROBLEM_DATA.read().expect("Mutex is poisoned");
        lock.get(name)
            .ok_or(RegistryError::ProblemNotFound {
                id: name.to_string(),
            })?
            .difficulty
    }; // Lock is dropped here

    // Validate the difficulty
    match difficulty {
        0..=10 => Ok(difficulty as u8),
        _ => Err(anyhow!(
            "Difficulty {difficulty} from problem {name} outside range."
        )),
    }
}

//#################################
//# PROBLEM SELECTION ALGORITHMS  #
//#################################

// Ratios when choosing a difficulty level (0-10) within a Difficulty
const EASY_MEDIUM_RATIO: f32 = 0.60;
const MEDIUM_HARD_RATIO: f32 = 0.60;
const EASY_HARD_RATIO: f32 = 0.70;
const EASY_RATIO: f32 = 0.40;
const MEDIUM_RATIO: f32 = 0.30;

// TODO: Doc
const DEFAULT_SCORE: u8 = 100;

// TODO: Doc + rename?
#[derive(Debug, Clone, Copy)]
struct ScoredProblemDescriptor<'a> {
    problem_type: &'a ProblemDescriptor,
    score: u8,
}

// TODO: Refactor this shit
fn choose_problems(problem_pool: ProblemPool, rng: &mut ThreadRng) -> Result<Vec<Problem>> {
    let mut problems = Vec::new();
    let mut ids: Vec<ProblemId> = Vec::new();
    let difficulty_range = Difficulty::enums_to_nums(
        problem_pool.starting_difficulty,
        problem_pool.ending_difficulty,
    );

    // Find all problems which match the desired difficulty and give them a score for assured
    // spread of problems during selection
    let candidates: Vec<&ProblemDescriptor> =
        get_valid_problem_types(&problem_pool, &difficulty_range)?;
    let count_per_difficulty = get_count_per_difficulty(&candidates, &problem_pool.n)?;
    let mut count_per_difficulty_number = [0; 11];
    let intro_counts =
        get_count_per_difficulty_number(&candidates, &count_per_difficulty[0], &Difficulty::Intro)?;
    let easy_counts =
        get_count_per_difficulty_number(&candidates, &count_per_difficulty[1], &Difficulty::Easy)?;
    let medium_counts = get_count_per_difficulty_number(
        &candidates,
        &count_per_difficulty[2],
        &Difficulty::Medium,
    )?;
    let hard_counts =
        get_count_per_difficulty_number(&candidates, &count_per_difficulty[3], &Difficulty::Hard)?;
    count_per_difficulty_number[0..=1].copy_from_slice(&intro_counts[0..=1]);
    count_per_difficulty_number[2..=4].copy_from_slice(&easy_counts);
    count_per_difficulty_number[5..=7].copy_from_slice(&medium_counts);
    count_per_difficulty_number[8..=10].copy_from_slice(&hard_counts);
    let candidates_with_scores: Vec<ScoredProblemDescriptor> = candidates
        .into_iter()
        .map(|candidate| ScoredProblemDescriptor {
            problem_type: candidate,
            score: DEFAULT_SCORE.max(count_per_difficulty_number.iter().sum()),
        })
        .collect();

    // This loop goes through each difficulty number, and finds the max score of that
    // difficulty number. If several problems has the max score, one is chosen at random.
    // It generates a problem of that type, and lowers that problem's score.
    // Indices are tracked to be able to change the relevant score in `candidates_with_scores`
    for (i, count) in count_per_difficulty_number.iter().enumerate() {
        if *count > 0 {
            let mut filtered_candidates = filter_candidates(&candidates_with_scores, i as u8);
            for _ in 0..*count {
                let mut max_score: u8 = 0;
                let mut max_indices: Vec<usize> = Vec::new();

                for (i, candidate) in filtered_candidates.iter().enumerate() {
                    match candidate.score.partial_cmp(&max_score) {
                        Some(Ordering::Greater) => {
                            max_score = candidate.score;
                            max_indices.clear();
                            max_indices.push(i);
                        }
                        Some(Ordering::Equal) => {
                            max_indices.push(i);
                        }
                        _ => {}
                    }
                }

                let chosen_index = max_indices
                    .choose(rng)
                    .context("No valid problems within the max_indices")?;
                let scored_problem_type = filtered_candidates[*chosen_index];
                let problem = get_unique_problem_or_reset_ids(
                    &problem_pool,
                    scored_problem_type.problem_type,
                    &mut ids,
                )?;
                filtered_candidates[*chosen_index].score -= 1;
                problems.push(problem);
            }
        }
    }
    Ok(problems)
}

fn filter_candidates<'a>(
    candidates_with_scores: &'a Vec<ScoredProblemDescriptor>,
    difficulty: u8,
) -> Vec<ScoredProblemDescriptor<'a>> {
    candidates_with_scores
        .iter()
        .filter(|candidate| candidate.problem_type.difficulty == difficulty)
        .map(|candidate| *candidate)
        .collect()
}

fn get_count_per_difficulty(candidates: &[&ProblemDescriptor], n: &u8) -> Result<[u8; 4]> {
    if candidates.len() == 0 {
        return Err(anyhow!("get_count_per_difficulty() called with empty Vec"));
    }
    // Intro is its own variable due to get_problem_ratios not counting it
    let found_intro = candidates
        .iter()
        .find(|candidate| [0, 1].contains(&candidate.difficulty))
        .is_some();
    let mut found_difficulties: [bool; 3] = [false; 3];
    found_difficulties[0] = candidates
        .iter()
        .find(|candidate| [2, 3, 4].contains(&candidate.difficulty))
        .is_some();
    found_difficulties[1] = candidates
        .iter()
        .find(|candidate| [5, 6, 7].contains(&candidate.difficulty))
        .is_some();
    found_difficulties[2] = candidates
        .iter()
        .find(|candidate| [8, 9, 10].contains(&candidate.difficulty))
        .is_some();

    let intro: u8 = if found_difficulties.iter().filter(|found| **found).count() == 0 {
        *n
    } else if found_intro {
        5.min((0.2 * (*n as f32)).ceil() as u8)
    } else {
        0
    };
    let remaining = *n - intro;
    let [easy, medium, hard] = get_problem_ratios(found_difficulties, remaining);

    Ok([intro, easy, medium, hard])
}

/// Calculates how many problems should be generated from each difficulty number (2-4 or 5-7)
///
/// This method is only intended to be called with candidates from a specific `Difficulty`
fn get_count_per_difficulty_number(
    candidates: &[&ProblemDescriptor],
    n: &u8,
    difficulty: &Difficulty,
) -> Result<[u8; 3]> {
    if candidates.len() == 0 {
        return Err(anyhow!(
            "get_count_per_difficulty_number() called with empty Vec"
        ));
    }
    if *n == 0 {
        return Ok([0, 0, 0]);
    }

    let (easy_num, medium_num, hard_num) = match *difficulty {
        Difficulty::Intro => (0, 1, 99),
        Difficulty::Easy => (2, 3, 4),
        Difficulty::Medium => (5, 6, 7),
        Difficulty::Hard => (8, 9, 10),
    };

    let mut found_difficulty_numbers: [bool; 3] = [false; 3];

    found_difficulty_numbers[0] = candidates
        .iter()
        .find(|candidate| candidate.difficulty == easy_num)
        .is_some();
    found_difficulty_numbers[1] = candidates
        .iter()
        .find(|candidate| candidate.difficulty == medium_num)
        .is_some();
    found_difficulty_numbers[2] = candidates
        .iter()
        .find(|candidate| candidate.difficulty == hard_num)
        .is_some();

    Ok(get_problem_ratios(found_difficulty_numbers, *n))
}

fn get_problem_ratios(found: [bool; 3], n: u8) -> [u8; 3] {
    let mut easier: u8 = 0;
    let mut medium: u8 = 0;
    let mut harder: u8 = 0;

    // The distribution will be different depending on which difficulties exist
    match found {
        [_, false, false] => easier = n,
        [false, _, false] => medium = n,
        [false, false, _] => harder = n,
        [_, _, false] => {
            easier = (n as f32 * EASY_MEDIUM_RATIO).round() as u8;
            medium = n - easier;
        }
        [false, _, _] => {
            medium = (n as f32 * MEDIUM_HARD_RATIO).round() as u8;
            harder = n - medium;
        }
        [_, false, _] => {
            easier = (n as f32 * EASY_HARD_RATIO).round() as u8;
            harder = n - easier;
        }

        [_, _, _] => {
            // It's intentional to have one ceil() and one round().
            // This ensures that n = 1 works correctly and nets an easier problem
            easier = (n as f32 * EASY_RATIO).ceil() as u8;
            medium = (n as f32 * MEDIUM_RATIO).round() as u8;
            harder = n - easier - medium;
        }
    }
    [easier, medium, harder]
}

/// Returns all `ProblemDescriptor`s of the desired difficulties
/// TODO: Check the lifetimes
fn get_valid_problem_types<'a>(
    problem_pool: &'a ProblemPool,
    difficulties: &[u8],
) -> Result<Vec<&'a ProblemDescriptor>> {
    let valid_problem_types: Vec<&ProblemDescriptor> = problem_pool
        .problems
        .iter()
        .filter(|problem_type| difficulties.contains(&problem_type.difficulty))
        .collect();
    if valid_problem_types.len() > 0 {
        Ok(valid_problem_types)
    } else {
        Err(anyhow!("No valid problem types of the desired difficulty"))
    }
}

/// Generates a problem with a unique ID (the actual numbers that makes the problem different).
///
/// If there are no more possible IDs to generate, the relevant IDs are removed from `ids` for
/// a fresh start.
/// NOTE: The `combinations` field might be smaller than the actual number of combinations.
///       This occurs because in some problems, some numbers depend on the value of other
///       numbers. This will mess up the range calculation. It's not considered an
///       issue, and will only result in a "bug" (possibility of repeated problem)
///       if several (>20) problems are generated from the same **function**.
fn get_unique_problem_or_reset_ids(
    problem_pool: &ProblemPool,
    problem_type: &ProblemDescriptor,
    ids: &mut Vec<ProblemId>,
) -> Result<Problem> {
    let mut problem = (problem_type.generator)(problem_type.name.clone(), &problem_pool.lang)?;
    let mut current_id = ProblemId {
        name: problem_type.name.clone(),
        identifiers: problem.identifiers.clone(),
    };

    let count = ids.iter().filter(|id| id.name == problem_type.name).count();

    while ids.contains(&current_id) {
        problem = (problem_type.generator)(problem_type.name.clone(), &problem_pool.lang)?;
        current_id.identifiers = problem.identifiers.clone();

        if count >= problem.combinations {
            ids.retain(|id| id.name != problem_type.name);
        }
    }
    ids.push(current_id.clone());
    Ok(problem)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_problem_generator(_: String, _: &str) -> Result<Problem> {
        Ok(Problem {
            question: "mock".to_string(),
            answer: "mock".to_string(),
            solution: "mock".to_string(),
            id: "mock".to_string(),
            identifiers: vec![],
            combinations: 1,
        })
    }

    fn problem_type_generator(difficulty: u8) -> ProblemDescriptor {
        ProblemDescriptor {
            name: "test".to_string(),
            difficulty,
            generator: mock_problem_generator,
        }
    }
    #[test]
    fn distributes_difficulty_numbers_when_all_numbers_present() {
        let candidates: Vec<ProblemDescriptor> = vec![
            problem_type_generator(8),
            problem_type_generator(9),
            problem_type_generator(10),
        ];
        let ref_candidates: Vec<&ProblemDescriptor> =
            candidates.iter().map(|p_type| p_type).collect();
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [1, 0, 0]),
            (2, [1, 1, 0]),
            (3, [2, 1, 0]),
            (4, [2, 1, 1]),
            (9, [4, 3, 2]),
            (23, [10, 7, 6]),
            (100, [40, 30, 30]),
        ];
        for (input, result) in inputs_and_results {
            assert_eq!(
                get_count_per_difficulty_number(&ref_candidates, &input, &Difficulty::Hard)
                    .unwrap(),
                result
            );
        }
    }

    #[test]
    fn distributes_difficulty_numbers_when_one_number_present() {
        // Lower difficulty
        let candidates: Vec<ProblemDescriptor> = vec![problem_type_generator(5)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [1, 0, 0]),
            (2, [2, 0, 0]),
            (9, [9, 0, 0]),
            (23, [23, 0, 0]),
            (100, [100, 0, 0]),
        ];
        let ref_candidates: Vec<&ProblemDescriptor> =
            candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                get_count_per_difficulty_number(&ref_candidates, &input, &Difficulty::Medium)
                    .unwrap(),
                result
            );
        }

        // Medium difficulty
        let candidates: Vec<ProblemDescriptor> = vec![problem_type_generator(1)];
        let ref_candidates: Vec<&ProblemDescriptor> =
            candidates.iter().map(|p_type| p_type).collect();
        assert_eq!(
            get_count_per_difficulty_number(&ref_candidates, &7, &Difficulty::Intro).unwrap(),
            [0, 7, 0]
        );
        // Higher difficulty
        let candidates: Vec<ProblemDescriptor> = vec![problem_type_generator(10)];
        let ref_candidates: Vec<&ProblemDescriptor> =
            candidates.iter().map(|p_type| p_type).collect();
        assert_eq!(
            get_count_per_difficulty_number(&ref_candidates, &7, &Difficulty::Hard).unwrap(),
            [0, 0, 7]
        );
    }

    #[test]
    fn distributes_difficulty_numbers_when_two_numbers_present() {
        // Lower + Medium
        let candidates: Vec<ProblemDescriptor> =
            vec![problem_type_generator(0), problem_type_generator(1)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [1, 0, 0]),
            (2, [1, 1, 0]),
            (9, [5, 4, 0]),
            (23, [14, 9, 0]),
            (100, [60, 40, 0]),
        ];
        let ref_candidates: Vec<&ProblemDescriptor> =
            candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                get_count_per_difficulty_number(&ref_candidates, &input, &Difficulty::Intro)
                    .unwrap(),
                result
            );
        }

        // Lower + Higher
        let candidates: Vec<ProblemDescriptor> =
            vec![problem_type_generator(5), problem_type_generator(7)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [1, 0, 0]),
            (2, [1, 0, 1]),
            (9, [6, 0, 3]),
            (23, [16, 0, 7]),
            (100, [70, 0, 30]),
        ];
        let ref_candidates: Vec<&ProblemDescriptor> =
            candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                get_count_per_difficulty_number(&ref_candidates, &input, &Difficulty::Medium)
                    .unwrap(),
                result
            );
        }

        // Medium + Higher
        let candidates: Vec<ProblemDescriptor> =
            vec![problem_type_generator(9), problem_type_generator(10)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [0, 1, 0]),
            (2, [0, 1, 1]),
            (9, [0, 5, 4]),
            (23, [0, 14, 9]),
            (100, [0, 60, 40]),
        ];
        let ref_candidates: Vec<&ProblemDescriptor> =
            candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                get_count_per_difficulty_number(&ref_candidates, &input, &Difficulty::Hard)
                    .unwrap(),
                result
            );
        }
    }
}
