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
//
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

//#################################
//#            CONFIG             #
//#################################

#[derive(Debug)]
pub struct Config<E> {
    exclusions: Vec<E>,
    sets: Vec<(Difficulty, u8)>,
}

impl<E> Default for Config<E> {
    fn default() -> Self {
        Self {
            exclusions: Vec::new(),
            sets: Vec::new(),
        }
    }
}

impl<E: PartialEq> Config<E> {
    pub fn exclusions(&self) -> &Vec<E> {
        &self.exclusions
    }

    pub fn exclude(&mut self, problem_id: E) -> &mut Self {
        if !self.exclusions.contains(&problem_id) {
            self.exclusions.push(problem_id);
        }
        self
    }

    pub fn add(&mut self, difficulty: Difficulty, n: u8) -> &mut Self {
        self.sets.push((difficulty, n));
        self
    }

    pub fn sets(&self) -> &Vec<(Difficulty, u8)> {
        &self.sets
    }
}

//#################################
//#            BUILDER            #
//#################################
use rand::seq::IndexedRandom;

pub trait ProblemType {}

pub struct SetBuilder {
    problem_types: Vec<Box<dyn ProblemType>>,
    exclusions: Vec<Box<dyn ProblemType>>,
    sets: Vec<(Difficulty, u8)>,
}

pub trait ProblemBuilder {
    type ProblemId: Copy + PartialEq + Eq;
    fn new() -> Self;
    fn config(&mut self) -> &mut Config<Self::ProblemId>;
    fn read_config(&self) -> &Config<Self::ProblemId>;
    fn problem_registry(&mut self) -> Vec<(Self::ProblemId, fn(&Self) -> Problem, u8, Difficulty)>;
    fn add(&mut self, difficulty: Difficulty, n: u8) -> &mut Self {
        self.config().add(difficulty, n);
        self
    }
    fn exclude(&mut self, problem_id: Self::ProblemId) -> &mut Self {
        self.config().exclude(problem_id);
        self
    }
    fn build(&mut self) -> Vec<Problem> {
        let mut problems = Vec::new();
        let problem_registry = self.problem_registry();
        let config = self.read_config();

        for (target_difficulty, n) in &config.sets {
            let candidates: Vec<_> = problem_registry
                .iter()
                .filter(|(id, _, _, diff)| {
                    *target_difficulty == *diff && !config.exclusions.contains(id)
                })
                .collect();

            let mut rng = rand::rng();
            for _ in 0..*n {
                let chosen = candidates.choose_weighted(&mut rng, |(_, _, weight, _)| *weight);
                let problem = match chosen {
                    Ok((_, func, _, _)) => func(self),
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
    fn problem_type_initialisation() {
        assert_eq!(
            Problem::new("question", "answer"),
            Problem {
                question: String::from("question"),
                answer: String::from("answer"),
                solution: String::new()
            }
        )
    }

    // CONFIG
    #[derive(Debug, PartialEq, Eq)]
    enum TestEnum {
        Problem1,
        Problem2,
    }

    #[test]
    fn config_default() {
        let config: Config<TestEnum> = Config::default();
        assert!(config.sets.len() == 0);
        assert!(config.exclusions.len() == 0);
    }

    #[test]
    fn config_getters() {
        let mut config: Config<TestEnum> = Config::default();
        config.exclude(TestEnum::Problem1);
        assert_eq!(config.sets(), config.sets());
        assert_eq!(*config.exclusions(), config.exclusions);
    }

    #[test]
    fn config_exclude() {
        let mut config: Config<TestEnum> = Config::default();
        config.exclude(TestEnum::Problem1);
        assert_eq!(config.exclusions, vec![TestEnum::Problem1]);
        config.exclude(TestEnum::Problem2);
        assert_eq!(
            config.exclusions,
            vec![TestEnum::Problem1, TestEnum::Problem2]
        );
        config.exclude(TestEnum::Problem2);
        assert_eq!(
            config.exclusions,
            vec![TestEnum::Problem1, TestEnum::Problem2]
        );
    }

    #[test]
    fn config_add() {
        let mut config: Config<TestEnum> = Config::default();
        config.add(Difficulty::Intro, 2);
        assert_eq!(config.sets, vec![(Difficulty::Intro, 2)]);
        config.add(Difficulty::Hard, 3);
        assert_eq!(
            config.sets,
            vec![(Difficulty::Intro, 2), (Difficulty::Hard, 3)]
        );
    }
}
