//#################################
//#           IMPORTS             #
//#################################
pub mod ma1;
pub mod range;

//#################################
//#          FLATTENING           #
//#################################
pub use ma1::*;
pub use range::*;

//#################################
//#   PROBLEM ENUMS AND STRUCTS   #
//#################################
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Difficulty {
    Intro,
    #[default]
    Easy,
    Medium,
    Hard,
}

type ProblemId = (String, Vec<i32>, usize);

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub question: String,
    pub answer: String,
    pub solution: String,
    pub id: ProblemId,
}

impl Problem {
    pub fn new(question: impl ToString, answer: impl ToString) -> Problem {
        Problem {
            question: question.to_string(),
            answer: answer.to_string(),
            solution: String::new(),
            id: (String::new(), Vec::new(), 0),
        }
    }
    pub fn question(&self) -> &String {
        &self.question
    }
    pub fn answer(&self) -> &String {
        &self.answer
    }
    pub fn solution(&self) -> &String {
        &self.solution
    }

    pub fn process(self) -> ProcessedProblem {
        ProcessedProblem {
            question: self.question,
            answer: self.answer,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProcessedProblem {
    pub question: String,
    pub answer: String,
}

pub trait ProblemArea {
    fn get_problem_types() -> &'static [&'static ProblemType];
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct ProblemType {
    pub difficulty: Difficulty,
    pub weight: u8,
    pub generator: fn() -> Problem,
}

impl PartialEq for ProblemType {
    fn eq(&self, other: &Self) -> bool {
        self.generator as usize == other.generator as usize
    }
}

//#################################
//#            BUILDER            #
//#################################
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

            let mut rng = rand::rng();
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
                            let (ref name, _, ref max_count) = problem.id;

                            let count = ids.iter().filter(|(n, _, _)| n == name).count();
                            if count >= *max_count {
                                ids.retain(|(n, _, _)| n != name);
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

//#################################
//#             TESTS             #
//#################################

#[cfg(test)]
mod tests {
    use super::*;

    // PROBLEM STRUCT
    #[test]
    fn problem_initialisation() {
        assert_eq!(
            Problem::new("question", "answer"),
            Problem {
                question: String::from("question"),
                answer: String::from("answer"),
                solution: String::new(),
                id: (String::new(), Vec::new(), 0),
            }
        )
    }
}
