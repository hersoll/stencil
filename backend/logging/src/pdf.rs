use anyhow::Result;
use db::logging::{PDFRow, PgInterval, SetRow};
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

    let set_rows: Vec<SetRow> = request
        .sets
        .into_iter()
        .enumerate()
        .map(|(i, set)| SetRow {
            topics: set.problem_options.topics,
            exclusions: set.problem_options.exclusions,
            columns: set.formatting_options.question_columns as i16,
            starting_difficulty: set
                .problem_options
                .starting_difficulty
                .to_minimum_difficulty_num() as i16,
            ending_difficulty: set
                .problem_options
                .ending_difficulty
                .to_maximum_difficulty_num() as i16,
            problem_count: set.problem_options.n as i16,
            has_heading: set.formatting_options.heading.is_some(),
            order_index: i as i32,
            pagebreak_after: set.formatting_options.pagebreak_after,
            problem_spacing: set.formatting_options.spacing.map(|spacing| spacing as i16),
        })
        .collect();

    db::logging::register_pdf(&db_row, &set_rows).await
}
