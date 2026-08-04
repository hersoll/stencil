pub mod answers;
pub mod questions;
pub mod solutions;
pub use answers::Answer;
pub use questions::Question;
pub use solutions::Solution;

use math::{
    Number,
    num_gen::{FinishedDecimalGenerator, FractionGenerator, IntegerGenerator, NumberGenerator},
};

pub type ID = i32;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Identifiers(Vec<i32>);

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Combinations(usize);

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub id: ID,
    pub question: Question,
    pub answer: Answer,
    pub solution: Solution,
    pub identifiers: Identifiers,
    pub combinations: Combinations,
}

/// Generic version of Problem
///
/// Improves ergonomics during problem generation
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProblemParameters<Q, A, S, I, C>
where
    Q: Into<Question>,
    A: Into<Answer>,
    S: Into<Solution>,
    I: Into<Identifiers>,
    C: Into<Combinations>,
{
    pub id: ID,
    pub question: Q,
    pub answer: A,
    pub solution: S,
    pub identifiers: I,
    pub combinations: C,
}

impl<Q, A, S, I, C> From<ProblemParameters<Q, A, S, I, C>> for Problem
where
    Q: Into<Question>,
    A: Into<Answer>,
    S: Into<Solution>,
    I: Into<Identifiers>,
    C: Into<Combinations>,
{
    fn from(p: ProblemParameters<Q, A, S, I, C>) -> Self {
        Problem {
            id: p.id,
            question: p.question.into(),
            answer: p.answer.into(),
            solution: p.solution.into(),
            identifiers: p.identifiers.into(),
            combinations: p.combinations.into(),
        }
    }
}

impl From<i32> for Identifiers {
    fn from(num: i32) -> Self {
        Self(vec![num])
    }
}

impl From<Number> for Identifiers {
    fn from(num: Number) -> Self {
        Self(vec![num.as_integer().as_i32()])
    }
}

impl From<(Number, Number)> for Identifiers {
    fn from(nums: (Number, Number)) -> Self {
        Self(vec![
            nums.0.as_integer().as_i32(),
            nums.1.as_integer().as_i32(),
        ])
    }
}

impl From<(Number, Number, Number)> for Identifiers {
    fn from(nums: (Number, Number, Number)) -> Self {
        Self(vec![
            nums.0.as_integer().as_i32(),
            nums.1.as_integer().as_i32(),
            nums.2.as_integer().as_i32(),
        ])
    }
}

impl From<Vec<Number>> for Identifiers {
    fn from(vec: Vec<Number>) -> Self {
        Self(vec.iter().map(|n| n.as_integer().as_i32()).collect())
    }
}

impl From<usize> for Combinations {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<IntegerGenerator> for Combinations {
    fn from(generator: IntegerGenerator) -> Self {
        Self(generator.len())
    }
}

impl From<FinishedDecimalGenerator> for Combinations {
    fn from(generator: FinishedDecimalGenerator) -> Self {
        Self(generator.len())
    }
}

impl From<FractionGenerator> for Combinations {
    fn from(generator: FractionGenerator) -> Self {
        Self(generator.len())
    }
}

impl From<Vec<usize>> for Combinations {
    fn from(vec: Vec<usize>) -> Self {
        let product = vec.iter().product();
        Self(product)
    }
}

impl Identifiers {
    pub fn vec(&self) -> &Vec<i32> {
        &self.0
    }
}

impl Combinations {
    pub fn size(&self) -> usize {
        self.0
    }
}
