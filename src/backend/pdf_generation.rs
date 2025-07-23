use crate::{
    backend::{self, db::ProblemDatabase}, shared, Error
};
use std::fs;

pub async fn create_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    document_options: shared::DocumentOptions,
) -> crate::Result<Vec<u8>> {
    let mut problem_sets: Vec<Vec<backend::Problem>> = Vec::new();
    let mut set_options: Vec<shared::SetRenderingOptions> = Vec::new();
    for set in sets {
        set_options.push(set.options);
        let mut set_builder = backend::SetBuilder::new();
        set_builder.lang(&document_options.lang);
        let problem_names = ProblemDatabase::get_problem_names_for_pdf(set.topics, set.exclusions).await?;
        let problem_types: Vec<backend::ProblemType> = problem_names.iter()
            .map(|name| {
                        backend::PROBLEM_MAP
                            .read()
                            .map_err(|_| Error::RegistryMutexIsPoisoned)?
                            .get(name)
                            .cloned()
                            .ok_or(Error::NoSuchProblemInRegistry { id: name.to_string() })
            }).collect::<crate::Result<Vec<backend::ProblemType>>>()?;
        set_builder.area(problem_types).batch(
            set.starting_difficulty,
            set.ending_difficulty,
            set.n,
        )?;
        problem_sets.push(set_builder.build()?);
    }

    let mut document_builder = backend::DocumentBuilder::new(set_options, document_options).await?;
    for problem_set in problem_sets {
        document_builder.add_problem_set(problem_set)?;
    }

    let typst_file = document_builder.build()?;
    let pdf_path = typst_file.compile()?;

    Ok(fs::read(pdf_path)?)
}
