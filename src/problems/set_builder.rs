use std::cmp::Ordering;

use super::*;
use rand::{rngs::ThreadRng, seq::IndexedRandom};

#[derive(Debug, Default)]
pub struct SetBuilder {
    problem_areas: Vec<&'static [&'static ProblemType]>,
    exclusions: Vec<ProblemType>,
    batches: Vec<(Difficulty, u8)>,
}

//#################################
//#   PUBLIC BUILDER FUNCTIONS    #
//#################################
impl SetBuilder {
    pub fn new() -> SetBuilder {
        SetBuilder::default()
    }

    pub fn area<T: ProblemArea>(&mut self, _area_struct: T) -> &mut Self {
        let area = T::get_problem_types();
        if !self.problem_areas.contains(&area) {
            self.problem_areas.push(&area);
        }
        self
    }

    pub fn batch(&mut self, difficulty: Difficulty, n: u8) -> &mut Self {
        self.batches.push((difficulty, n));
        self
    }

    pub fn exclude(&mut self, problem_id: &ProblemType) -> &mut Self {
        if !self.exclusions.contains(problem_id) {
            self.exclusions.push(*problem_id);
        }
        self
    }

    pub fn build(&self) -> Vec<Problem> {
        let mut rng = rand::rng();
        let mut problems = Vec::new();

        for (target_difficulty, n) in &self.batches {
            problems.append(&mut self.choose_problems(target_difficulty, n, &mut rng));
        }
        problems
    }
}

//#################################
//# PROBLEM SELECTION ALGORITHMS  #
//#################################

impl SetBuilder {
    fn choose_problems(
        &self,
        target_difficulty: &Difficulty,
        n: &u8,
        rng: &mut ThreadRng,
    ) -> Vec<Problem> {
        let mut problems = Vec::new();
        // `ids` is not a registry of ALL chosen ids (those can be found in `problems`),
        // but rather the ids to avoid when generating a problem to encourage uniqueness.
        // The difference is that `ids` might be cleared out in `get_unique_problem_or_reset_ids()`
        let mut ids: Vec<ProblemId> = Vec::new();
        let difficulty_range = Difficulty::enum_to_nums(*target_difficulty);

        let candidates: Vec<&ProblemType> = self.get_valid_problem_types(target_difficulty);
        let count_per_difficulty_number = Self::get_count_per_difficulty_number(&candidates, n);
        let candidates_with_scores: Vec<(&ProblemType, u8)> = candidates
            .into_iter()
            .map(|candidate| (candidate, 100))
            .collect();

        // This loop goes through each difficulty number, and finds the max score of that
        // difficulty number. If several problems has the max score, one is chosen at random.
        // It generates a problem of that type, and lowers that problem's score.
        for (i, count) in count_per_difficulty_number.iter().enumerate() {
            if *count > 0 {
                let mut filtered_candidates =
                    Self::filter_candidates(&candidates_with_scores, &difficulty_range, i);
                for _ in 0..*count {
                    let mut max_score: u8 = 0;
                    let mut max_indices: Vec<usize> = Vec::new();

                    for (i, (_, score)) in filtered_candidates.iter().enumerate() {
                        match score.partial_cmp(&max_score) {
                            Some(Ordering::Greater) => {
                                max_score = *score;
                                max_indices.clear();
                                max_indices.push(i);
                            }
                            Some(Ordering::Equal) => {
                                max_indices.push(i);
                            }
                            _ => {}
                        }
                    }

                    let chosen_index = max_indices.choose(rng).unwrap();
                    let (problem_type, _) = filtered_candidates[*chosen_index];
                    let problem = Self::get_unique_problem_or_reset_ids(problem_type, &mut ids);
                    filtered_candidates[*chosen_index].1 -= 1;
                    problems.push(problem);
                }
            }
        }
        problems
    }

    fn filter_candidates<'a>(
        candidates_with_scores: &Vec<(&'a ProblemType, u8)>,
        difficulty_range: &Vec<u8>,
        index: usize,
    ) -> Vec<(&'a ProblemType, u8)> {
        candidates_with_scores
            .iter()
            .filter(|(candidate, _)| candidate.difficulty == difficulty_range[index])
            .map(|(candidate, score)| (*candidate, *score))
            .collect()
    }

    fn get_count_per_difficulty_number(candidates: &Vec<&ProblemType>, n: &u8) -> [u8; 3] {
        let mut found_difficulty_numbers: [u8; 3] = [0; 3];

        // To make matching easier, all difficulties are mapped to 0-2
        for candidate in candidates {
            let relative_difficulty: usize = match candidate.difficulty {
                0 | 2 | 5 | 8 => 0,
                1 | 3 | 6 | 9 => 1,
                4 | 7 | 10 => 2,
                _ => panic!("Recieved a difficulty larger than 10!"),
            };
            found_difficulty_numbers[relative_difficulty] += 1;
        }

        if found_difficulty_numbers.iter().sum::<u8>() == 0 {
            panic!("get_count_per_difficulty_number recieved a candidates vec with no problems");
        }

        let mut easier: u8 = 0;
        let mut medium: u8 = 0;
        let mut harder: u8 = 0;

        // The distribution will be different depending on which difficulties exist
        match found_difficulty_numbers {
            [_, 0, 0] => easier = *n,
            [0, _, 0] => medium = *n,
            [0, 0, _] => harder = *n,
            [0, _, _] => {
                medium = (*n as f32 * 0.60).round() as u8;
                harder = *n - medium;
            }
            [_, 0, _] => {
                easier = (*n as f32 * 0.70).round() as u8;
                harder = *n - easier;
            }
            [_, _, 0] => {
                easier = (*n as f32 * 0.60).round() as u8;
                medium = *n - easier;
            }
            [_, _, _] => {
                easier = (*n as f32 * 0.40).ceil() as u8;
                medium = (*n as f32 * 0.30).round() as u8;
                harder = *n - easier - medium;
            }
        }

        [easier, medium, harder]
    }

    fn get_valid_problem_types(&self, target_difficulty: &Difficulty) -> Vec<&ProblemType> {
        self.problem_areas
            .iter()
            .flat_map(|area| area.iter())
            .filter(|problem_type| {
                Difficulty::num_to_enum(problem_type.difficulty) == *target_difficulty
                    && !self.exclusions.contains(problem_type)
            })
            .map(|problem_type| *problem_type)
            .collect()
    }

    fn get_unique_problem_or_reset_ids(
        problem_type: &ProblemType,
        ids: &mut Vec<ProblemId>,
    ) -> Problem {
        let mut problem;
        loop {
            // This loop will both:
            // - Generate a problem until it is unique
            // - Start fresh if all IDs are taken
            problem = (problem_type.generator)();

            let count = ids.iter().filter(|id| id.name == problem.id.name).count();
            if count >= problem.id.combinations {
                ids.retain(|id| id.name != problem.id.name);
            }

            if !ids.contains(&problem.id) {
                ids.push(problem.id.clone());
                return problem;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_problem_generator() -> Problem {
        Problem::new("mock", "mock")
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
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input),
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
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input),
                result
            );
        }

        // Medium difficulty
        let candidates: Vec<ProblemType> = vec![problem_type_generator(1)];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        assert_eq!(
            SetBuilder::get_count_per_difficulty_number(&ref_candidates, &7),
            [0, 7, 0]
        );
        // Higher difficulty
        let candidates: Vec<ProblemType> = vec![problem_type_generator(10)];
        let ref_candidates: Vec<&ProblemType> = candidates.iter().map(|p_type| p_type).collect();
        assert_eq!(
            SetBuilder::get_count_per_difficulty_number(&ref_candidates, &7),
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
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input),
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
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input),
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
                SetBuilder::get_count_per_difficulty_number(&ref_candidates, &input),
                result
            );
        }
    }
}
