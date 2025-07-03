//#################################
//#           IMPORTS             #
//#################################
pub mod ma1;

//#################################
//#          FLATTENING           #
//#################################
pub use ma1::*;

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

#[derive(Debug, Default, PartialEq)]
pub struct Problem {
    question: String,
    answer: String,
    solution: String,
}

impl Problem {
    pub fn new(question: impl ToString, answer: impl ToString) -> Problem {
        Problem {
            question: question.to_string(),
            answer: answer.to_string(),
            solution: String::new(),
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
    pub fn build(&mut self) -> Vec<Problem> {
        let mut problems = Vec::new();

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
                    Ok(problem) => (problem.generator)(),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        Problem::new("ERROR", "ERROR")
                    }
                };
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
                solution: String::new()
            }
        )
    }
}
