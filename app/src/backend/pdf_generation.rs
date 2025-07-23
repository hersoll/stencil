use crate::{
    Error,
    backend::{self, Database},
    shared,
};
use std::fs;

pub async fn create_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    document_options: shared::DocumentOptions,
    db: Database,
) -> crate::Result<Vec<u8>> {
    let mut problem_sets: Vec<Vec<backend::Problem>> = Vec::new();
    let courses = db.get_all_courses(&document_options.lang).await?;
    let mut set_options: Vec<shared::SetRenderingOptions> = Vec::new();
    for set in sets {
        set_options.push(set.options);
        let mut set_builder = backend::SetBuilder::new();
        set_builder.lang(&document_options.lang);
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
