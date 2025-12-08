use crate::ProblemGenerator;
use crate::RegistryError;
use crate::builders::DocumentBuilder;
use crate::builders::SetBuilder;
use crate::{
    Problem, ProblemArea, db,
    shared::{self, DocumentOptions, ProblemSetData, SendableProblemSetData},
};
use anyhow::{Result, anyhow};
use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use std::time::Instant;
use tempfile::Builder;
use tokio::fs;
use tokio::process::Command;
use tracing::debug;
use tracing::info;
use tracing::instrument;

/// ONLY to be used while mocking is required.
/// Make build_pdf the endpoint after that!
pub async fn send_pdf() -> Response {
    let mut sets = ProblemSetData::new(1);
    sets.n = 50;
    sets.topics.push(1);
    let sendable_sets: SendableProblemSetData = sets.into();
    let options = DocumentOptions::default();
    match build_pdf_from_http(vec![sendable_sets], options).await {
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

#[instrument(skip(sets, document_options), fields(num_sets = sets.len()))]
pub async fn build_pdf_from_http(
    sets: Vec<shared::SendableProblemSetData>,
    document_options: shared::DocumentOptions,
) -> Result<Vec<u8>> {
    info!("Building PDF with {} problem set(s)", sets.len());

    // A vec containing the sets of actual problems (With question, answer, ...)
    let mut problem_sets: Vec<Vec<Problem>> = Vec::with_capacity(sets.len());
    // The rendering options for each set
    // NOTE: (Should not be in shared)
    let mut set_options: Vec<shared::SetRenderingOptions> = Vec::with_capacity(sets.len());

    // Convert each incoming "set" (http-set) to actual problems
    let start = Instant::now();
    for (i, set) in sets.into_iter().enumerate() {
        debug!(set_index = i, "Processing problem set");
        // Store the options from the http set in the options vec
        set_options.push(set.options.clone());
        problem_sets = generate_problems_for_set(problem_sets, set, &document_options.lang).await?;
        debug!(set_index = i, "Generated every problem for set");
    }
    let duration = start.elapsed();
    debug!("Generated problem sets in {}ms", duration.as_millis());

    #[cfg(feature = "docker")]
    let project_root = "/app";
    #[cfg(not(feature = "docker"))]
    let project_root = "./";

    // Create temp directory inside project root
    let temp_dir = Builder::new().prefix("temp_").tempdir_in(project_root)?;
    let temp_dir_path = temp_dir.path();

    debug!("Writing typst file...");
    let start = Instant::now();
    let mut document_builder = DocumentBuilder::new(set_options, document_options).await?;
    for problem_set in problem_sets {
        document_builder.add_problem_set(problem_set)?;
    }
    let typst_as_string = document_builder.build_to_string()?;

    let typst_path = temp_dir_path.join("stencil.typ");
    let pdf_path = temp_dir_path.join("stencil.pdf");
    fs::write(&typst_path, typst_as_string).await?;
    let duration = start.elapsed();
    debug!("Wrote typst file in {}ms", duration.as_millis());

    debug!("Compiling PDF...");
    let start = Instant::now();
    let status = Command::new("typst")
        .args([
            "compile",
            "--root",
            project_root,
            typst_path.to_str().unwrap(),
            pdf_path.to_str().unwrap(),
        ])
        .status()
        .await?;

    if !status.success() {
        return Err(anyhow!("Typst compilation failed"));
    }

    let duration = start.elapsed();
    debug!("Compiled PDF in {}ms", duration.as_millis());

    info!("PDF build complete");
    let pdf_bytes = fs::read(&pdf_path).await?;

    Ok(pdf_bytes)
}

/// Given a complete problem name (module_problem),
/// returns a pointer to the function that generates that problem.
fn get_generator_function(name: &String) -> Result<ProblemGenerator> {
    let generator = {
        let lock = crate::PROBLEM_MAP.read().expect("Mutex is poisoned");
        lock.get(name)
            .copied()
            .ok_or(RegistryError::ProblemNotFound {
                id: name.to_string(),
            })?
    }; // Lock is dropped here

    Ok(generator)
}

/// Given a complete problem name (module_problem),
/// returns the difficulty of that problem.
///
/// Used after retrieving the problem names from
/// PROBLEM_MAP matching a HTTP request.
fn get_problem_difficulty(name: &String) -> Result<u8> {
    let difficulty = {
        let lock = crate::PROBLEM_DATA.read().expect("Mutex is poisoned");
        lock.get(name)
            .ok_or(RegistryError::ProblemNotFound {
                id: name.to_string(),
            })?
            .difficulty
    }; // Lock is dropped here

    match difficulty {
        0..=10 => Ok(difficulty as u8),
        _ => Err(anyhow!(
            "Difficulty {difficulty} from problem {name} outside range."
        )),
    }
}

async fn generate_problems_for_set(
    mut problem_sets: Vec<Vec<Problem>>,
    set: shared::SendableProblemSetData,
    lang: &String,
) -> Result<Vec<Vec<Problem>>> {
    // Look at the topics (and exclusions) from the http,
    // get all matching problem names from the db
    let problem_names = db::problems::get_problem_names_for_pdf(set.topics, set.exclusions).await?;

    let problem_areas: Vec<ProblemArea> = problem_names
        .iter()
        .map(|name| {
            let generator = get_generator_function(name)?;
            let difficulty = get_problem_difficulty(name)?;
            Ok(ProblemArea {
                name: name.to_string(),
                difficulty,
                generator,
            })
        })
        .collect::<Result<Vec<ProblemArea>>>()?;

    // Pass all the data to the SetBuilder
    problem_sets.push(
        SetBuilder::new()
            .number_of_problems(set.n)
            .difficulties(set.starting_difficulty, set.ending_difficulty)
            .problem_areas(problem_areas)
            .lang(lang)
            // Generates every problem HERE
            .build()?,
    );
    Ok(problem_sets)
}
