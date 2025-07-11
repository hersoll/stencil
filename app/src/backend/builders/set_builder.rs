use crate::Error;
use crate::Result;
use crate::backend::Difficulty;
use crate::backend::problems::*;
use std::cmp::Ordering;

use rand::{rngs::ThreadRng, seq::IndexedRandom};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub struct SetBuilder {
    problem_areas: Vec<Vec<ProblemType>>,
    batches: Vec<(Difficulty, u8)>,
}

//#################################
//#   PUBLIC BUILDER FUNCTIONS    #
//#################################
impl SetBuilder {
    pub fn new() -> SetBuilder {
        SetBuilder {
            problem_areas: Vec::new(),
            batches: Vec::new(),
        }
    }

    pub fn area(&mut self, area: Vec<ProblemType>) -> &mut Self {
        if !self.problem_areas.contains(&area) {
            self.problem_areas.push(area);
        }
        // else Err
        self
    }

    pub fn batch(&mut self, difficulty: Difficulty, n: u8) -> &mut Self {
        if n > 0 {
            self.batches.push((difficulty, n));
        }
        // else Err
        self
    }

    pub fn build(&self) -> Result<Vec<Problem>> {
        let mut rng = rand::rng();
        let mut problems = Vec::new();

        for (target_difficulty, n) in &self.batches {
            problems.append(&mut self.choose_problems(target_difficulty, n, &mut rng)?);
        }
        Ok(problems)
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

const DEFAULT_SCORE: u8 = 100;

#[derive(Debug, Clone, Copy)]
struct ScoredProblemType<'a> {
    problem_type: &'a ProblemType,
    score: u8,
}

impl SetBuilder {
    fn choose_problems(
        &self,
        target_difficulty: &Difficulty,
        n: &u8,
        rng: &mut ThreadRng,
    ) -> Result<Vec<Problem>> {
        let mut problems = Vec::new();
        // `ids` is not a registry of ALL chosen ids (those can be found in `problems`),
        // but rather the ids to avoid when generating a problem to encourage uniqueness.
        // The difference is that `ids` might be cleared out in `get_unique_problem_or_reset_ids()`
        let mut ids: Vec<ProblemId> = Vec::new();
        let difficulty_range = Difficulty::enum_to_nums(*target_difficulty);

        let candidates: Vec<&ProblemType> = self.get_valid_problem_types(target_difficulty)?;

        let count_per_difficulty_number = Self::get_count_per_difficulty_number(&candidates, n)?;
        let candidates_with_scores: Vec<ScoredProblemType> = candidates
            .into_iter()
            .map(|candidate| ScoredProblemType {
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
                let mut filtered_candidates =
                    Self::filter_candidates(&candidates_with_scores, &difficulty_range, i);
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

                    let chosen_index = max_indices.choose(rng).ok_or(Error::NoValidProblems)?;
                    let scored_problem_type = filtered_candidates[*chosen_index];
                    let problem = self.get_unique_problem_or_reset_ids(
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
        candidates_with_scores: &'a Vec<ScoredProblemType>,
        difficulty_range: &Vec<u8>,
        index: usize,
    ) -> Vec<ScoredProblemType<'a>> {
        candidates_with_scores
            .iter()
            .filter(|candidate| candidate.problem_type.difficulty == difficulty_range[index])
            .map(|candidate| *candidate)
            .collect()
    }

    /// Calculates how many problems should be generated from each difficulty number (0-10)
    ///
    /// This method is only intended to be called with candidates from a specific `Difficulty`
    fn get_count_per_difficulty_number(candidates: &Vec<&ProblemType>, n: &u8) -> Result<[u8; 3]> {
        let mut found_difficulty_numbers: [u8; 3] = [0; 3];

        // To make matching easier, all difficulties are mapped to 0-2
        for candidate in candidates {
            let relative_difficulty: usize = match candidate.difficulty {
                0 | 2 | 5 | 8 => 0,
                1 | 3 | 6 | 9 => 1,
                4 | 7 | 10 => 2,
                _ => {
                    return Err(Error::InvalidDifficulty {
                        difficulty: candidate.difficulty,
                    });
                }
            };
            found_difficulty_numbers[relative_difficulty] += 1;
        }

        if found_difficulty_numbers.iter().sum::<u8>() == 0 {
            return Err(Error::NoValidProblems);
        }

        let mut easier: u8 = 0;
        let mut medium: u8 = 0;
        let mut harder: u8 = 0;

        // The distribution will be different depending on which difficulties exist
        match found_difficulty_numbers {
            [_, 0, 0] => easier = *n,
            [0, _, 0] => medium = *n,
            [0, 0, _] => harder = *n,
            [_, _, 0] => {
                easier = (*n as f32 * EASY_MEDIUM_RATIO).round() as u8;
                medium = *n - easier;
            }
            [0, _, _] => {
                medium = (*n as f32 * MEDIUM_HARD_RATIO).round() as u8;
                harder = *n - medium;
            }
            [_, 0, _] => {
                easier = (*n as f32 * EASY_HARD_RATIO).round() as u8;
                harder = *n - easier;
            }

            [_, _, _] => {
                // It's intentional to have one ceil() and one round().
                // This is ensures that n = 1 works correctly and nets an easier problem
                easier = (*n as f32 * EASY_RATIO).ceil() as u8;
                medium = (*n as f32 * MEDIUM_RATIO).round() as u8;
                harder = *n - easier - medium;
            }
        }

        Ok([easier, medium, harder])
    }

    /// Returns all `ProblemType`s of the desired `target_difficulty`
    /// that haven't been called by `exclude()`
    fn get_valid_problem_types(&self, target_difficulty: &Difficulty) -> Result<Vec<&ProblemType>> {
        self.problem_areas
            .iter()
            .flat_map(|area| area.iter())
            .filter_map(
                |problem_type| match Difficulty::num_to_enum(problem_type.difficulty) {
                    Ok(difficulty) if difficulty == *target_difficulty => Some(Ok(problem_type)),
                    Ok(_) => None,
                    Err(e) => Some(Err(e)),
                },
            )
            .collect()
    }

    /// Generates a problem with a unique ID (the actual numbers that makes the problem different).
    ///
    /// If there are no more possible IDs to generate, the relevant IDs are removed from `ids` for
    /// a fresh start.
    fn get_unique_problem_or_reset_ids(
        &self,
        problem_type: &ProblemType,
        ids: &mut Vec<ProblemId>,
    ) -> Result<Problem> {
        let mut problem = (problem_type.generator)()?;
        let count = ids.iter().filter(|id| id.name == problem.id.name).count();
        if count >= problem.id.combinations {
            ids.retain(|id| id.name != problem.id.name);
        }
        while ids.contains(&problem.id) {
            problem = (problem_type.generator)()?;
        }
        ids.push(problem.id.clone());
        Ok(problem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_problem_generator() -> Result<Problem> {
        Ok(Problem::new("mock", "mock"))
    }

    fn problem_type_generator(difficulty: u8) -> ProblemType {
        ProblemType {
            difficulty,
            generator: mock_problem_generator,
        }
    }
    #[test]
    fn distributes_difficulty_numbers_when_all_numbers_present() {
        let candidates: Vec<ProblemType> = vec![
            problem_type_generator(8),
            problem_type_generator(9),
            problem_type_generator(10),
        ];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
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
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input).unwrap(),
                result
            );
        }
    }

    #[test]
    fn distributes_difficulty_numbers_when_one_number_present() {
        // Lower difficulty
        let candidates: Vec<ProblemType> = vec![problem_type_generator(5)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [1, 0, 0]),
            (2, [2, 0, 0]),
            (9, [9, 0, 0]),
            (23, [23, 0, 0]),
            (100, [100, 0, 0]),
        ];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input).unwrap(),
                result
            );
        }

        // Medium difficulty
        let candidates: Vec<ProblemType> = vec![problem_type_generator(1)];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        assert_eq!(
            SetBuilder::get_count_per_difficulty_number(&ref_candidates, &7).unwrap(),
            [0, 7, 0]
        );
        // Higher difficulty
        let candidates: Vec<ProblemType> = vec![problem_type_generator(10)];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        assert_eq!(
            SetBuilder::get_count_per_difficulty_number(&ref_candidates, &7).unwrap(),
            [0, 0, 7]
        );
    }

    #[test]
    fn distributes_difficulty_numbers_when_two_numbers_present() {
        // Lower + Medium
        let candidates: Vec<ProblemType> =
            vec![problem_type_generator(0), problem_type_generator(1)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [1, 0, 0]),
            (2, [1, 1, 0]),
            (9, [5, 4, 0]),
            (23, [14, 9, 0]),
            (100, [60, 40, 0]),
        ];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input).unwrap(),
                result
            );
        }

        // Lower + Higher
        let candidates: Vec<ProblemType> =
            vec![problem_type_generator(5), problem_type_generator(7)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [1, 0, 0]),
            (2, [1, 0, 1]),
            (9, [6, 0, 3]),
            (23, [16, 0, 7]),
            (100, [70, 0, 30]),
        ];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input).unwrap(),
                result
            );
        }

        // Medium + Higher
        let candidates: Vec<ProblemType> =
            vec![problem_type_generator(9), problem_type_generator(10)];
        let inputs_and_results = [
            (0, [0, 0, 0]),
            (1, [0, 1, 0]),
            (2, [0, 1, 1]),
            (9, [0, 5, 4]),
            (23, [0, 14, 9]),
            (100, [0, 60, 40]),
        ];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        for (input, result) in inputs_and_results {
            assert_eq!(
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input).unwrap(),
                result
            );
        }
    }
}
