use std::{collections::HashMap, fs};

use dioxus::prelude::*;

use crate::{
    Error,
    backend::{
        Difficulty, HasDesc, PROBLEM_MAP, PROBLEM_REGISTRY, Problem, ProblemData, ProblemType,
        TopicData,
        builders::{DocumentBuilder, DocumentOptions, SetBuilder, WriteSolutions},
        translations::{GENERAL_TRANSLATIONS, Translations},
    },
    frontend_types::{SendableProblemSetData, SetRenderingOptions},
};

#[server]
pub async fn load_registry() -> Result<super::ProblemRegistry, ServerFnError> {
    Ok(PROBLEM_REGISTRY.clone())
}

/// Finds the description and difficulty for every problem in a certain topic within the difficulty
/// parameters.
///
/// Used for the exclusion display to show all "sub-problems"
#[server]
pub async fn get_problems(
    topics: Vec<String>,
    starting_difficulty: Difficulty,
    ending_difficulty: Difficulty,
    lang: String,
) -> Result<Vec<(String, String, u8)>, ServerFnError> {
    let registry_topics: Vec<&TopicData> = PROBLEM_REGISTRY
        .courses
        .iter()
        .flat_map(|course| course.chapters.iter())
        .flat_map(|chapter| chapter.topics.iter())
        .filter(|topic| topics.contains(&topic.name))
        .collect();

    let mut problem_names_and_descs: Vec<(String, String)> = Vec::new();
    for topic in registry_topics.iter() {
        for problem in topic.problems.iter() {
            let desc = problem.get_desc(&lang)?;
            problem_names_and_descs.push((topic.name.clone() + "_" + &problem.name, desc));
        }
    }

    let mut matching_problems = Vec::new();
    for (problem_name, problem_desc) in problem_names_and_descs.iter() {
        let problem = PROBLEM_MAP
            .read()
            .map_err(|_| Error::RegistryMutexIsPoisoned)?
            .get(problem_name)
            .cloned()
            .ok_or(Error::NoSuchProblemInRegistry {
                id: problem_name.to_string(),
            })?;
        if Difficulty::enums_to_nums(starting_difficulty, ending_difficulty)
            .contains(&problem.difficulty)
        {
            matching_problems.push((
                problem_name.clone(),
                problem_desc.clone(),
                problem.difficulty,
            ));
        }
    }

    // Sort from easy to hard
    matching_problems.sort_by_key(|tuple| tuple.2);
    Ok(matching_problems)
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
    document_options: DocumentOptions,
) -> crate::Result<Vec<u8>> {
    let mut problem_sets: Vec<Vec<Problem>> = Vec::new();
    let courses = &PROBLEM_REGISTRY.courses;
    let mut set_options: Vec<SetRenderingOptions> = Vec::new();
    for set in sets {
        set_options.push(set.options);
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
                .filter(|problem| {
                    !set.exclusions
                        .contains(&(topic.name.clone() + "_" + &problem.name))
                })
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
        )?;
        problem_sets.push(set_builder.build()?);
    }

    let mut document_builder = DocumentBuilder::new(set_options, document_options);
    document_builder.write_solutions(WriteSolutions::First);
    for problem_set in problem_sets {
        document_builder.add_problem_set(problem_set)?;
    }

    let typst_file = document_builder.build()?;
    let pdf_path = typst_file.compile()?;

    Ok(fs::read(pdf_path)?)
}
