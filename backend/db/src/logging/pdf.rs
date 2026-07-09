use anyhow::Result;
pub use sqlx::postgres::types::PgInterval;

type SmallInt = i16;
type Integer = i32;

#[derive(Debug)]
pub struct PDFRow {
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
pub struct SetRow {
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

pub async fn register_pdf(row: &PDFRow, set_rows: &[SetRow]) -> Result<i32> {
    let pool = crate::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO stats_pdf (
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
    time_taken) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) 
               RETURNING id"#,
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
        register_set(set, created.id).await?;
    }

    Ok(created.id)
}

async fn register_set(set: &SetRow, pdf_id: i32) -> Result<()> {
    let pool = crate::get_pool();
    sqlx::query!(
        r#"INSERT INTO stats_sets (
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
