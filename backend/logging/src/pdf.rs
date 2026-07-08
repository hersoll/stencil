use anyhow::Result;
use db::logging::{PDFRow, PgInterval};
use types::pdf::PDFRequest;

type MicroSeconds = i64;

pub async fn log_pdf_and_get_id(request: PDFRequest, time_taken: MicroSeconds) -> Result<i32> {
    // Note that this is still represented in seconds in the DB (with a resolution of milliseconds)
    let time_taken_interval = PgInterval {
        months: 0,
        days: 0,
        microseconds: time_taken,
    };

    // Just aliasing :)
    let d_o = request.document_options;
    let db_row = PDFRow {
        has_title: d_o.title.is_some(),
        has_subtitle: d_o.subtitle.is_some(),
        has_name_field: d_o.name_field,
        font_size: d_o.font_size as i16,
        answer_columns: d_o.answer_columns as i16,
        lang: d_o.lang.to_string(),
        write_solutions: d_o.write_solutions.to_string(),
        color: d_o.color,
        paper_size: d_o.paper_size.to_string(),
        x_margin: d_o.x_margin as i16,
        y_margin: d_o.y_margin as i16,
        par_spacing: d_o.par_spacing.map(|v| v as i16),
        max_prefix_group: d_o.max_prefix_group as i16,
        page_break_before_answers: d_o.page_break_before_answers,
        previous_pdf: request.previous_pdf,
        time_taken: time_taken_interval,
    };

    db::logging::register_pdf(&db_row).await
}
