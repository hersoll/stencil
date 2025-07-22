use std::fs;
use crate::shared::types::HasDesc;

use dioxus::prelude::*;

use crate::{Error, backend, shared};

#[server]
pub async fn load_registry() -> Result<shared::ProblemRegistry, ServerFnError> {
    Ok(backend::PROBLEM_REGISTRY.clone())
}

/// Finds the description and difficulty for every problem in a certain topic within the difficulty
/// parameters.
///
/// Used for the exclusion display to show all "sub-problems"
#[server]
pub async fn get_problems(
    topics: Vec<String>,
    starting_difficulty: shared::Difficulty,
    ending_difficulty: shared::Difficulty,
    lang: String,
) -> Result<Vec<(String, String, u8)>, ServerFnError> {
    let registry_topics: Vec<&shared::TopicData> = backend::PROBLEM_REGISTRY
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
        let problem = backend::PROBLEM_MAP
            .read()
            .map_err(|_| Error::RegistryMutexIsPoisoned)?
            .get(problem_name)
            .cloned()
            .ok_or(Error::NoSuchProblemInRegistry {
                id: problem_name.to_string(),
            })?;
        if shared::Difficulty::enums_to_nums(starting_difficulty, ending_difficulty)
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
pub async fn load_translations() -> Result<backend::Translations, ServerFnError> {
    Ok(backend::translations::GENERAL_TRANSLATIONS.clone())
}

#[server]
pub async fn generate_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    options: shared::DocumentOptions,
) -> Result<Vec<u8>, ServerFnError> {
    let pdf = create_pdf(sets, options).await?;
    Ok(pdf)
}

async fn create_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    document_options: shared::DocumentOptions,
) -> crate::Result<Vec<u8>> {
    let mut problem_sets: Vec<Vec<backend::Problem>> = Vec::new();
    let courses = &backend::PROBLEM_REGISTRY.courses;
    let mut set_options: Vec<shared::SetRenderingOptions> = Vec::new();
    for set in sets {
        set_options.push(set.options);
        let mut set_builder = backend::SetBuilder::new();
        let mut problem_types: Vec<backend::ProblemType> = Vec::new();
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
                        backend::PROBLEM_MAP
                            .read()
                            .map_err(|_| Error::RegistryMutexIsPoisoned)?
                            .get(name)
                            .cloned()
                            .ok_or(Error::NoSuchProblemInRegistry { id: id.to_string() })
                    })
                    .collect::<crate::Result<Vec<backend::ProblemType>>>()?,
            );
        }
        set_builder.area(problem_types).batch(
            set.starting_difficulty,
            set.ending_difficulty,
            set.n,
        )?;
        problem_sets.push(set_builder.build()?);
    }

    let mut document_builder = backend::DocumentBuilder::new(set_options, document_options);
    for problem_set in problem_sets {
        document_builder.add_problem_set(problem_set)?;
    }

    let typst_file = document_builder.build()?;
    let pdf_path = typst_file.compile()?;

    Ok(fs::read(pdf_path)?)
}
