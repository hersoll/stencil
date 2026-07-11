use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use types::errors::ApiError;

use crate::api::stats::DurationPath;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PDFAttribute {
    Title,
    Subtitle,
    NameField,
    FontSize,
    AnswerColumns,
    Lang,
    WriteSolutions,
    Color,
    PaperSize,
    XMargin,
    YMargin,
    ParSpacing,
    MaxPrefixGroup,
    PageBreakBeforeAnswers,
}

/// To keep the router clean, we have one endpoint which is dynamic over attribute and duration.
pub async fn get_pdf_attribute(
    Path((attribute, duration_path)): Path<(PDFAttribute, DurationPath)>,
) -> Result<impl IntoResponse, ApiError> {
    use PDFAttribute::*;
    use db::logging::stats;
    let duration = duration_path.as_duration();
    let count = match attribute {
        Title => stats::get_title_count(duration).await?,
        Subtitle => stats::get_subtitle_count(duration).await?,
        NameField => stats::get_name_field_count(duration).await?,
        FontSize => stats::get_font_size_count(duration).await?,
        AnswerColumns => stats::get_answer_column_count(duration).await?,
        Lang => stats::get_lang_count(duration).await?,
        WriteSolutions => stats::get_write_solutions_count(duration).await?,
        Color => stats::get_color_count(duration).await?,
        PaperSize => stats::get_paper_size_count(duration).await?,
        XMargin => stats::get_x_margin_count(duration).await?,
        YMargin => stats::get_y_margin_count(duration).await?,
        ParSpacing => stats::get_par_spacing_count(duration).await?,
        MaxPrefixGroup => stats::get_max_prefix_group_count(duration).await?,
        PageBreakBeforeAnswers => stats::get_page_break_before_answer_count(duration).await?,
    };
    Ok((StatusCode::OK, Json(json!(count))))
}
