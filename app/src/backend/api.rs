use std::fs;

use dioxus::prelude::*;

use crate::{
    Error,
    backend::{
        PROBLEM_MAP, PROBLEM_REGISTRY, Problem, ProblemType,
        builders::{DocumentBuilder, DocumentOptions, SetBuilder, WriteSolutions},
        translations::{GENERAL_TRANSLATIONS, Translations},
    },
    frontend_types::SendableProblemSetData,
};

#[server]
pub async fn load_registry() -> Result<super::ProblemRegistry, ServerFnError> {
    Ok(PROBLEM_REGISTRY.clone())
}

#[server]
pub async fn load_translations() -> Result<Translations, ServerFnError> {
    Ok(GENERAL_TRANSLATIONS.clone())
}

#[server]
pub async fn generate_pdf(
    sets: Vec<SendableProblemSetData>,
    options: DocumentOptions,
) -> Result<Vec<u8>, ServerFnError> {
    let pdf = generate_standard_pdf(sets, options).await?;
    Ok(pdf)
}

async fn generate_standard_pdf(
    sets: Vec<SendableProblemSetData>,
    options: DocumentOptions,
) -> crate::Result<Vec<u8>> {
    let mut problem_sets: Vec<Vec<Problem>> = Vec::new();
    let courses = &PROBLEM_REGISTRY.courses;
    for set in sets {
        let mut set_builder = SetBuilder::new();
        let mut problem_types: Vec<ProblemType> = Vec::new();
        // Convert the ID strings to actual problems
        for id in set.ids {
            let topic = courses
                .iter()
                .flat_map(|course| course.chapters.iter())
                .flat_map(|chapter| chapter.topics.iter())
                .find(|topic| topic.name == id)
                .ok_or(Error::NoTopicWithTopicName { name: id.clone() })?;
            let problem_names: Vec<String> = topic
                .problems
                .iter()
                .map(|problem| topic.name.clone() + "_" + &problem.name)
                .collect();
            problem_types.append(
                &mut problem_names
                    .iter()
                    .map(|name| {
                        PROBLEM_MAP
                            .read()
                            .map_err(|_| Error::RegistryMutexIsPoisoned)?
                            .get(name)
                            .cloned()
                            .ok_or(Error::NoSuchProblemInRegistry { id: id.to_string() })
                    })
                    .collect::<crate::Result<Vec<ProblemType>>>()?,
            );
        }
        set_builder.area(problem_types).batch(
            set.starting_difficulty,
            set.ending_difficulty,
            set.n,
        );
        problem_sets.push(set_builder.build()?);
    }

    let mut document_builder = DocumentBuilder::new(options);
    document_builder.write_solutions(WriteSolutions::First);
    for problem_set in problem_sets {
        document_builder.add_problem_set(problem_set)?;
    }

    let typst_file = document_builder.build()?;
    let pdf_path = typst_file.compile()?;

    Ok(fs::read(pdf_path)?)
}
