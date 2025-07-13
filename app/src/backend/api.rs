use std::fs;

use dioxus::prelude::*;

use crate::{
    Error,
    backend::{
        PROBLEM_REGISTRY, ProblemType,
        builders::{DocumentBuilder, WriteSolutions},
        translations::{REGISTRY_TRANSLATIONS},
    },
};

#[server]
pub async fn load_registry() -> Result<super::ProblemRegistry, ServerFnError> {
    Ok(REGISTRY_TRANSLATIONS.clone())
}

#[server]
pub async fn generate_pdf() -> Result<Vec<u8>, ServerFnError> {
    let pdf = generate_standard_pdf().await?;
    Ok(pdf)
}

async fn generate_standard_pdf() -> crate::Result<Vec<u8>> {
    let registry = PROBLEM_REGISTRY
        .lock()
        .map_err(|_| Error::RegistryMutexIsPoisoned)?;
    let ids: Vec<&str> = vec![
        "standard_equations_mult_only",
        "standard_equations_add_sub_only",
        "standard_equations_default_positive",
        "f_x_without_notation_y",
        "f_x_without_notation_x",
    ];
    let problem_types: Vec<ProblemType> = ids
        .iter()
        .map(|id| {
            registry
                .get(*id)
                .cloned()
                .ok_or(Error::NoSuchProblemInRegistry { id: id.to_string() })
        })
        .collect::<crate::Result<Vec<ProblemType>>>()?;

    let problems = crate::backend::builders::SetBuilder::new()
        .area(problem_types)
        .lang("en")
        .batch(crate::backend::Difficulty::Intro, 5)
        .batch(crate::backend::Difficulty::Easy, 10)
        .build()?;

    let typst_file = DocumentBuilder::new()
        .heading("Equations")
        .lang("en")
        .write_solutions(WriteSolutions::First)
        .add_problem_set(problems)?
        .build()?;

    let pdf_path = typst_file.compile()?;

    Ok(fs::read(pdf_path)?)
}
