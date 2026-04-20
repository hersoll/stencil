use crate::text_endpoints;
use anyhow::{Result, anyhow};
use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use problem_generator::generator::ProblemOptions;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::{fs, process::Command};
use tracing::{debug, info, instrument};
use types::{errors::ApiError, problems::Problem};
use typst_writer::typst_file_builder::{DocumentOptions, SetOptions, TypstFileBuilder};

/// Information about what to include in the problem set
///
/// Should be included in the HTTP request in the form of a Vec<ProblemSetSpec>
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProblemSetSpec {
    pub problems: ProblemOptions,
    /// Typst rendering options
    #[serde(default)]
    pub options: SetOptions,
}

impl ProblemSetSpec {
    /// Mostly used for the /pdf/example endpoint
    pub fn new() -> ProblemSetSpec {
        ProblemSetSpec {
            problems: ProblemOptions::default(),
            options: SetOptions::default(),
        }
    }
}

/// What the HTTP request is deserialized into
#[derive(Debug, Deserialize, Serialize)]
pub struct PDFRequest {
    sets: Vec<ProblemSetSpec>,
    #[serde(default)]
    document_options: DocumentOptions,
}

/// Generates a proof-of-concept PDF without any attributes
pub async fn generate_example_pdf() -> Response {
    let mut sets = ProblemSetSpec::new();
    sets.problems.topics.push(1);
    sets.problems.topics.push(2);
    sets.problems.topics.push(3);
    sets.problems.topics.push(4);
    let options = DocumentOptions::default();
    let req = PDFRequest {
        sets: vec![sets.clone(), sets],
        document_options: options,
    };

    let pdf_result = build_pdf(req).await;
    pdf_result_to_response(pdf_result)
}

pub async fn generate_pdf_from_http(payload: Result<Json<PDFRequest>, JsonRejection>) -> Response {
    match payload {
        Ok(Json(data)) => {
            // Debug tracing
            debug!("{:#?}", data.document_options);
            for (i, set) in data.sets.iter().enumerate() {
                debug!("Set {i}: {set:#?}");
            }

            let pdf_result = build_pdf(data).await;
            pdf_result_to_response(pdf_result)
        }
        Err(JsonRejection::MissingJsonContentType(rejection)) => {
            info!("Rejected request body: {rejection}");
            (
                StatusCode::BAD_REQUEST,
                format!(
                    "Rejected request: {rejection}
It seems like the request was sent without a JSON.
To generate a custom stencil, you need to attach a JSON body in the following format: 

"
                ) + &text_endpoints::get_http_schema(),
            )
                .into_response()
        }
        Err(JsonRejection::JsonDataError(err)) => {
            (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        }
        Err(JsonRejection::JsonSyntaxError(err)) => {
            (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unknown error occured in the JSON parsing.",
        )
            .into_response(),
    }
}

/// The function responsible for coordinating problem generation,
/// typst file writing and compiling
#[instrument(skip(req), fields(num_sets = req.sets.len()))]
async fn build_pdf(req: PDFRequest) -> Result<Vec<u8>, ApiError> {
    let sets = req.sets;
    let document_options = req.document_options;
    info!("Building PDF with {} problem set(s)", sets.len());

    // A vec containing the sets of actual problems (With question, answer, ...)
    let mut problem_sets: Vec<Vec<Problem>> = Vec::with_capacity(sets.len());
    // The typst rendering options for each set
    let set_options: Vec<SetOptions> = sets.iter().map(|set| set.options.clone()).collect();

    // Convert each incoming "set" (http-set) to actual problems
    let start = Instant::now();
    for (i, set) in sets.into_iter().enumerate() {
        debug!(set_index = i, "Processing problem set");
        let problem_set = problem_generator::generator::generate_problem_set(
            set.problems,
            document_options.lang.clone(),
        )
        .await?;
        problem_sets.push(problem_set);
        debug!(set_index = i, "Generated every problem for set");
    }
    let duration = start.elapsed();
    debug!("Generated problem sets in {}ms", duration.as_millis());

    #[cfg(feature = "docker")]
    let project_root = "/app";
    #[cfg(not(feature = "docker"))]
    let project_root = "./";

    // Create temp directory inside project root for .typ and .pdf files
    let temp_dir = tempfile::Builder::new()
        .prefix("temp_")
        .tempdir_in(project_root)?;
    let temp_dir_path = temp_dir.path();

    debug!("Writing typst file...");
    let start = Instant::now();
    let typst_path = temp_dir_path.join("stencil.typ");
    let mut typst_file_builder = TypstFileBuilder::new(set_options, document_options).await?;
    for problem_set in problem_sets {
        typst_file_builder.add_problem_set(problem_set)?;
    }
    let typst_as_string = typst_file_builder.build_to_string()?;
    fs::write(&typst_path, typst_as_string).await?;
    let duration = start.elapsed();
    debug!("Wrote typst file in {}ms", duration.as_millis());

    // Print typst file (pretty-printed) for debugging
    #[cfg(not(feature = "docker"))]
    if std::env::args()
        .collect::<Vec<String>>()
        .contains(&"show-output".to_string())
    {
        Command::new("typstyle")
            .arg(typst_path.to_str().unwrap())
            .status()
            .await?;
    }

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
        return Err(anyhow!("Typst compilation failed").into());
    }

    let duration = start.elapsed();
    debug!("Compiled PDF in {}ms", duration.as_millis());

    info!("PDF build complete");
    let pdf_bytes = fs::read(&pdf_path).await?;

    Ok(pdf_bytes)
}

/// Converts the Result from the build_pdf function to a HTTP response
///
/// The reason this isn't simply called at the end of the build_pdf function
/// is due to easier `?` bubbling in that function. It's more ergonomic to just
/// return all errors and handle them at a higher level.
fn pdf_result_to_response(pdf_result: Result<Vec<u8>, ApiError>) -> Response {
    match pdf_result {
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
        Err(e) => e.into_response(),
    }
}
