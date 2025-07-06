use super::*;
use rand::seq::IndexedRandom;

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
        let mut problems = Vec::new();
        let mut ids: Vec<ProblemId> = Vec::new();
        let mut rng = rand::rng();

        // NOTE: IDEA! The weight of the problem types should change
        //       depending on if it was chosen or not.
        //       Also, if candidates.len() >= n, just take one from each candidate (set weight of
        //       chosen to 0?)
        for (target_difficulty, n) in &self.batches {
            let candidates: Vec<&ProblemType> = self
                .problem_areas
                .iter()
                .flat_map(|area| area.iter())
                .filter(|problem_type| {
                    problem_type.difficulty == *target_difficulty
                        && !self.exclusions.contains(problem_type)
                })
                .map(|problem_type| *problem_type)
                .collect();

            for _ in 0..*n {
                let chosen =
                    candidates.choose_weighted(&mut rng, |problem_type| problem_type.weight);
                let problem = match chosen {
                    Ok(problem_type) => {
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
                                break;
                            }
                        }

                        problem
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        Problem::new("ERROR", "ERROR")
                    }
                };
                ids.push(problem.id.clone());
                problems.push(problem);
            }
        }
        problems
    }
}
