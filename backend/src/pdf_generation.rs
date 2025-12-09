use crate::{
    problems::problem_picker,
    problems::Difficulty,
    problems::Problem,
    typst_utils::typst_file_builder::{DocumentOptions, SetOptions, TypstFileBuilder},
};
use anyhow::{anyhow, Result};
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tempfile::Builder;
use tokio::{fs, process::Command};
use tracing::{debug, info, instrument};

/// Information about what to include in the problem set
///
/// Should be included in the HTTP request in the form of a Vec<SetInformation>
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemSetSpec {
    /// Topics to draw problems from
    pub topics: Vec<i32>,
    /// Which problems to exclude from the generations
    #[serde(default)]
    pub exclusions: Vec<i32>,
    pub starting_difficulty: Difficulty,
    pub ending_difficulty: Difficulty,
    /// Number of problems
    pub n: u8,
    /// Typst rendering options
    pub options: SetOptions,
}

// TODO: Do we need this? Or will this reside in the frontend?
impl ProblemSetSpec {
    pub fn new() -> ProblemSetSpec {
        ProblemSetSpec {
            topics: Vec::new(),
            exclusions: Vec::new(),
            starting_difficulty: Difficulty::Intro,
            ending_difficulty: Difficulty::Hard,
            n: 10,
            options: SetOptions {
                question_columns: 2,
                title: String::new(),
                spacing: None,
            },
        }
    }
}

/// ONLY to be used while mocking is required.
/// Make build_pdf the endpoint after that!
pub async fn send_pdf() -> Response {
    let mut sets = ProblemSetSpec::new();
    sets.n = 50;
    sets.topics.push(1);
    let options = DocumentOptions::default();
    match build_pdf_from_http(vec![sets], options).await {
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
    sets: Vec<ProblemSetSpec>,
    document_options: DocumentOptions,
) -> Result<Vec<u8>> {
    info!("Building PDF with {} problem set(s)", sets.len());

    // A vec containing the sets of actual problems (With question, answer, ...)
    let mut problem_sets: Vec<Vec<Problem>> = Vec::with_capacity(sets.len());
    // The typst rendering options for each set
    let set_options: Vec<SetOptions> = sets.iter().map(|set| set.options.clone()).collect();

    // Convert each incoming "set" (http-set) to actual problems
    let start = Instant::now();
    for (i, set) in sets.into_iter().enumerate() {
        debug!(set_index = i, "Processing problem set");
        let problem_set =
            problem_picker::generate_problems_for_set(set, document_options.lang.clone()).await?;
        problem_sets.push(problem_set);
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
    let typst_path = temp_dir_path.join("stencil.typ");
    let mut document_builder = TypstFileBuilder::new(set_options, document_options).await?;
    for problem_set in problem_sets {
        document_builder.add_problem_set(problem_set)?;
    }
    let typst_as_string = document_builder.build_to_string()?;
    fs::write(&typst_path, typst_as_string).await?;
    let duration = start.elapsed();
    debug!("Wrote typst file in {}ms", duration.as_millis());

    debug!("Compiling PDF...");
    let start = Instant::now();
    let pdf_path = temp_dir_path.join("stencil.pdf");
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
