use anyhow::{Result, anyhow};
use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::{HeaderName, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::time::Instant;
use tokio::{fs, process::Command};
use tracing::{debug, info};
use types::{
    errors::ApiError,
    pdf::{DocumentOptions, PDFRequest, ProblemSetSpec, QuestionSetFormattingOptions},
    problems::Problem,
};
use typst_writer::typst_file_builder::TypstFileBuilder;

/// Return type of the [`build_pdf()`] function
#[derive(Debug, Serialize)]
struct PDFResponse {
    id: i32,
    file: Vec<u8>,
}

/// Generates a proof-of-concept PDF without any attributes
pub async fn generate_example_pdf() -> Response {
    let mut sets = ProblemSetSpec::new();
    sets.problem_options.topics.push(1);
    sets.problem_options.topics.push(2);
    sets.problem_options.topics.push(3);
    sets.problem_options.topics.push(4);
    let options = DocumentOptions::default();
    let req = PDFRequest {
        sets: vec![sets.clone(), sets],
        document_options: options,
        previous_pdf: None,
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
"
                ),
            )
                .into_response()
        }
        Err(JsonRejection::JsonDataError(err)) => {
            info!("Rejected request body: {err}");

            (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        }
        Err(JsonRejection::JsonSyntaxError(err)) => {
            info!("Rejected request body: {err}");

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
async fn build_pdf(req: PDFRequest) -> Result<PDFResponse, ApiError> {
    let start = Instant::now();

    let req_for_logging = req.clone();
    let sets = req.sets;
    let document_options = req.document_options;
    info!("Building PDF with {} problem set(s)", sets.len());

    // A vec containing the sets of actual problems (With question, answer, ...)
    let mut problem_sets: Vec<Vec<Problem>> = Vec::with_capacity(sets.len());
    // The typst rendering options for each set
    let formatting_options: Vec<QuestionSetFormattingOptions> = sets
        .iter()
        .map(|set| set.formatting_options.clone())
        .collect();

    // Convert each incoming "set" (http-set) to actual problems
    for (i, set) in sets.into_iter().enumerate() {
        debug!(set_index = i, "Processing problem set");
        let problem_set =
            problems::generator::generate_problem_set(set.problem_options, document_options.lang)
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
    let writing_start = Instant::now();
    let typst_path = temp_dir_path.join("stencil.typ");
    let mut typst_file_builder =
        TypstFileBuilder::new(formatting_options, document_options).await?;
    for problem_set in problem_sets {
        typst_file_builder.parse_problem_set(problem_set)?;
    }
    let typst_as_string = typst_file_builder.build_to_string()?;
    fs::write(&typst_path, typst_as_string).await?;
    let duration = writing_start.elapsed();
    debug!("Wrote typst file in {}ms", duration.as_millis());

    // Print typst file (pretty-printed) for debugging
    #[cfg(not(feature = "docker"))]
    if std::env::args().any(|x| x == "show-output") {
        Command::new("typstyle")
            .arg(typst_path.to_str().unwrap())
            .status()
            .await?;
    }

    debug!("Compiling PDF...");
    let render_start = Instant::now();
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

    let duration = render_start.elapsed();
    debug!("Compiled PDF in {}ms", duration.as_millis());

    // Only log during production (or prod flag) to not mess up the logging stats
    let pdf_id = if cfg!(feature = "docker") || std::env::args().any(|x| x == "prod") {
        db::logging::log_pdf_and_get_id(req_for_logging, start.elapsed().as_micros() as i64).await?
    } else {
        -1
    };

    info!("PDF build complete.");
    if pdf_id >= 0 {
        info!("Logged the data with ID {pdf_id}");
    }

    let pdf_bytes = fs::read(&pdf_path).await?;

    Ok(PDFResponse {
        id: pdf_id,
        file: pdf_bytes,
    })
}

/// Converts the Result from the build_pdf function to a HTTP response
///
/// The reason this isn't simply called at the end of the build_pdf function
/// is due to easier `?` bubbling in that function. It's more ergonomic to just
/// return all errors and handle them at a higher level.
fn pdf_result_to_response(pdf_result: Result<PDFResponse, ApiError>) -> Response {
    match pdf_result {
        Ok(pdf_response) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "application/pdf".to_string()),
                (
                    header::CONTENT_DISPOSITION,
                    "inline; filename=\"stencil.pdf\"".to_string(),
                ),
                (
                    HeaderName::from_static("x-pdf-id"),
                    pdf_response.id.to_string(),
                ),
            ],
            pdf_response.file,
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}
