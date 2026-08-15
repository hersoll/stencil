use anyhow::Result;
pub use sqlx::postgres::types::PgInterval;
use types::pdf::PDFRequest;

type MicroSeconds = i64;
type SmallInt = i16;
type Integer = i32;

#[derive(Debug)]
struct PDFRow {
    // Request data
    pub has_title: bool,
    pub has_subtitle: bool,
    pub has_name_field: bool,
    pub font_size: SmallInt,
    pub answer_columns: SmallInt,
    pub lang: String,
    pub write_solutions: String,
    pub color: bool,
    pub paper_size: String,
    pub x_margin: SmallInt,
    pub y_margin: SmallInt,
    pub par_spacing: Option<SmallInt>,
    pub max_prefix_group: SmallInt,
    pub page_break_before_answers: bool,
    // Metadata in request
    pub previous_pdf: Option<Integer>,
    // Metadata determined by backend
    pub time_taken: PgInterval,
}

#[derive(Debug)]
struct SetRow {
    pub topics: Vec<Integer>,
    pub exclusions: Vec<Integer>,
    // Represented as min and max within their respective category
    pub starting_difficulty: SmallInt,
    pub ending_difficulty: SmallInt,
    pub problem_count: SmallInt,
    pub columns: SmallInt,
    pub has_heading: bool,
    pub problem_spacing: Option<SmallInt>,
    pub pagebreak_after: bool,
    pub order_index: Integer,
}

/// Convert the [`PDFRequest`] into a database-compatible struct and send it for logging
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
        font_size: d_o.font_size as SmallInt,
        answer_columns: d_o.answer_columns as SmallInt,
        lang: d_o.lang.to_string(),
        write_solutions: d_o.show_solutions.to_string(),
        color: d_o.color,
        paper_size: d_o.paper_size.to_string(),
        x_margin: d_o.x_margin as SmallInt,
        y_margin: d_o.y_margin as SmallInt,
        par_spacing: d_o.par_spacing.map(|v| v as SmallInt),
        max_prefix_group: d_o.max_prefix_group as SmallInt,
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
            columns: set.formatting_options.question_columns as SmallInt,
            starting_difficulty: set
                .problem_options
                .starting_difficulty
                .to_minimum_difficulty_num() as SmallInt,
            ending_difficulty: set
                .problem_options
                .ending_difficulty
                .to_maximum_difficulty_num() as SmallInt,
            problem_count: set.problem_options.n as SmallInt,
            has_heading: set.formatting_options.heading.is_some(),
            order_index: i as Integer,
            pagebreak_after: set.formatting_options.pagebreak_after,
            problem_spacing: set
                .formatting_options
                .spacing
                .map(|spacing| spacing as SmallInt),
        })
        .collect();

    log_pdf(&db_row, &set_rows).await
}

async fn log_pdf(row: &PDFRow, set_rows: &[SetRow]) -> Result<i32> {
    let pool = crate::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO logs_pdf (
            has_title,
            has_subtitle,
            has_name_field,
            font_size,
            answer_columns,
            lang,
            write_solutions,
            color,
            paper_size,
            x_margin,
            y_margin,
            par_spacing,
            max_prefix_group,
            page_break_before_answers,
            previous_pdf,
            time_taken
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) 
        RETURNING id
        "#,
        row.has_title,
        row.has_subtitle,
        row.has_name_field,
        row.font_size,
        row.answer_columns,
        row.lang,
        row.write_solutions,
        row.color,
        row.paper_size,
        row.x_margin,
        row.y_margin,
        row.par_spacing,
        row.max_prefix_group,
        row.page_break_before_answers,
        row.previous_pdf,
        row.time_taken
    )
    .fetch_one(pool)
    .await?;

    for set in set_rows {
        log_set(set, created.id).await?;
    }

    Ok(created.id)
}

async fn log_set(set: &SetRow, pdf_id: i32) -> Result<()> {
    let pool = crate::get_pool();
    sqlx::query!(
        r#"INSERT INTO logs_problem_set (
            topics,
            exclusions,
            starting_difficulty,
            ending_difficulty,
            problem_count,
            columns,
            has_heading,
            problem_spacing,
            pagebreak_after,
            order_index,
            pdf_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
        "#,
        &set.topics,
        &set.exclusions,
        set.starting_difficulty,
        set.ending_difficulty,
        set.problem_count,
        set.columns,
        set.has_heading,
        set.problem_spacing,
        set.pagebreak_after,
        set.order_index,
        pdf_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
