use super::*;
use rand::{rngs::ThreadRng, seq::IndexedRandom};

#[derive(Debug, Default)]
pub struct SetBuilder {
    problem_areas: Vec<&'static [&'static ProblemType]>,
    exclusions: Vec<ProblemType>,
    batches: Vec<(Difficulty, u8)>,
}

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
        let mut added_problem_types: Vec<&ProblemType> = Vec::new();
        // NOTE: IDEA! The weight of the problem types should change
        //       depending on if it was chosen or not.
        //       Also, if candidates.len() >= n, just take one from each candidate (set weight of
        //       chosen to 0?)
        let candidates: Vec<&ProblemType> = self.get_valid_problem_types(target_difficulty);
        for _ in 0..*n {
            let problem_type = candidates
                .choose_weighted(rng, |problem_type| problem_type.weight)
                .unwrap();
            let problem = Self::get_unique_problem_or_reset_ids(problem_type, &mut ids);
            ids.push(problem.id.clone());
            problems.push(problem);
        }
        problems
    }

    fn get_valid_problem_types(&self, target_difficulty: &Difficulty) -> Vec<&ProblemType> {
        self.problem_areas
            .iter()
            .flat_map(|area| area.iter())
            .filter(|problem_type| {
                Difficulty::to_enum(problem_type.difficulty) == *target_difficulty
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
                return problem;
            }
        }
    }
}
