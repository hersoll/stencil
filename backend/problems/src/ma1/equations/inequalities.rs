use anyhow::Result;
use macros::problem;
use math::{Number, symbols::inequality_sign::InequalitySign};
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters},
};

/// test
/// Absolute difficulty:
/// Relative difficulty:
#[problem]
fn proof_of_concept(id: i32, _lang: Language) -> Result<Problem> {
    let sign = InequalitySign::random();

    Ok(Problem::from(ProblemParameters {
        id,
        question: format!("$x {sign} 3$"),
        answer: Number::Integer(10),
        solution: Number::Integer(0),
        identifiers: 3,
        combinations: 1,
    }))
}
