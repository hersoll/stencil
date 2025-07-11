use std::fs;

use dioxus::prelude::*;

use crate::backend::{
    PROBLEM_REGISTRY, ProblemType,
    builders::{DocumentBuilder, DocumentOptions, WriteSolutions},
};

#[server]
pub async fn load_registry() -> Result<super::ProblemRegistry, ServerFnError> {
    let json = std::fs::read_to_string("registry.json")?;
    let parsed: crate::backend::ProblemRegistry = serde_json::from_str(&json)?;
    Ok(parsed)
}

#[server]
pub async fn generate_pdf() -> Result<Vec<u8>, ServerFnError> {
    // Take the sets, parse each Vec of Strings into a Vec of ProblemTypes
    // Turn them into batches
    // Create sets
    // Run them through the document builder with the documentoptions
    let pdf = generate_standard_pdf().await;
    Ok(pdf)
}

async fn generate_standard_pdf() -> Vec<u8> {
    let registry = PROBLEM_REGISTRY.lock().unwrap();
    // println!("{:#?}", registry);
    // println!(
    //     "{:#?}",
    //     registry.get("standard_equations_mult_only").unwrap()
    // );
    let ids = vec![
        "standard_equations_mult_only",
        "standard_equations_add_sub_only",
        "standard_equations_default_positive",
    ];
    let problem_types: Vec<ProblemType> = ids
        .iter()
        .map(|id| registry.get(*id).unwrap().clone())
        .collect();
    let problems = crate::backend::builders::SetBuilder::new()
        .area(problem_types)
        .batch(crate::backend::Difficulty::Intro, 5)
        .batch(crate::backend::Difficulty::Easy, 10)
        .build();

    let typst_file = DocumentBuilder::new()
        .heading("Equations")
        .write_solutions(WriteSolutions::First)
        .add_problem_set(problems)
        .build()
        .unwrap();

    let pdf_path = typst_file.compile().unwrap();

    fs::read(pdf_path).expect("Failed to read generated PDF")
}
