mod document_builder;
mod set_builder;

pub use document_builder::*;
pub use set_builder::*;

use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::{
    Error, Problem, ProblemType,
    db::ProblemDatabase,
    shared::{self, DocumentOptions, ProblemSetData, SendableProblemSetData},
};
use std::fs;

/// ONLY to be used while mocking is required.
/// Make build_pdf the endpoint after that!
pub async fn send_pdf() -> Response {
    let mut sets = ProblemSetData::new(1);
    sets.n = 50;
    sets.topics.push(1);
    let sendable_sets: SendableProblemSetData = sets.into();
    let options = DocumentOptions::default();
    match build_pdf(vec![sendable_sets], options).await {
        Ok(pdf_bytes) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "application/pdf"),
                (
                    header::CONTENT_DISPOSITION,
                    "inline; filename=\"stencil.pdf\"",
                ),
            ],
            pdf_bytes,
        )
            .into_response(),
        Err(e) => {
            println!("{e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn build_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    document_options: shared::DocumentOptions,
) -> crate::Result<Vec<u8>> {
    let mut problem_sets: Vec<Vec<Problem>> = Vec::new();
    let mut set_options: Vec<shared::SetRenderingOptions> = Vec::new();
    for set in sets {
        set_options.push(set.options);
        let mut set_builder = SetBuilder::new();
        set_builder.lang(&document_options.lang);
        let problem_names =
            ProblemDatabase::get_problem_names_for_pdf(set.topics, set.exclusions).await?;
        let problem_types: crate::Result<Vec<ProblemType>> = problem_names
            .iter()
            .map(|name| {
                let generator = crate::PROBLEM_MAP
                    .read()
                    .expect("Mutex is poisoned")
                    .get(name)
                    .cloned()
                    .ok_or(Error::NoSuchProblemInRegistry {
                        id: name.to_string(),
                    })?;
                let problem = crate::PROBLEM_DATA
                    .read()
                    .expect("Mutex is poisoned")
                    .get(name)
                    .cloned()
                    .ok_or(Error::NoSuchProblemInRegistry {
                        id: name.to_string(),
                    })?;
                Ok(ProblemType {
                    name: name.clone(),
                    difficulty: problem.difficulty as u8,
                    generator,
                })
            })
            .collect();
        let problem_types = problem_types?;
        set_builder.area(problem_types).batch(
            set.starting_difficulty,
            set.ending_difficulty,
            set.n,
        )?;
        problem_sets.push(set_builder.build()?);
    }

    let mut document_builder = DocumentBuilder::new(set_options, document_options).await?;
    for problem_set in problem_sets {
        document_builder.add_problem_set(problem_set)?;
    }

    let typst_file = document_builder.build()?;
    let pdf_path = typst_file.compile()?;

    Ok(fs::read(pdf_path)?)
}
