use anyhow::Result;

use crate::{
    get_pool,
    logging::stats::{AggregationDuration, ValueCount},
};

/// Returns the topics that have been part of the most reloaded (unchanged) PDFs
///
/// YES, the query is massive. Sorry.
pub async fn most_reloaded_topics(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = get_pool();
    let tops = sqlx::query_as!(
        ValueCount,
        r#"
            WITH copies AS (
                SELECT p.id, p.previous_pdf, p.has_title, p.has_subtitle, p.has_name_field, 
                    p.font_size, p.answer_columns, p.lang, p.write_solutions, p.color, p.paper_size, 
                    p.x_margin, p.y_margin, p.par_spacing, p.max_prefix_group, p.page_break_before_answers, 
                    ps.topics, ps.exclusions, ps.problem_count, ps.columns, ps.has_heading, ps.problem_spacing, 
                    ps.pagebreak_after, ps.order_index, ps.starting_difficulty, ps.ending_difficulty 
                FROM logs_pdf p 
                JOIN logs_problem_set ps
                ON p.id = ps.pdf_id

-- Condition below means it is a copy

                WHERE p.previous_pdf IS NOT null
                AND p.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval),

            originals AS (
                SELECT p.id, p.has_title, p.has_subtitle, p.has_name_field, 
                    p.font_size, p.answer_columns, p.lang, p.write_solutions, p.color, p.paper_size, 
                    p.x_margin, p.y_margin, p.par_spacing, p.max_prefix_group, p.page_break_before_answers, 
                    ps.topics, ps.exclusions, ps.problem_count, ps.columns, ps.has_heading, ps.problem_spacing, 
                    ps.pagebreak_after, ps.order_index, ps.starting_difficulty, ps.ending_difficulty 
                FROM logs_pdf p 
                JOIN logs_problem_set ps 
                ON p.id = ps.pdf_id

-- Condition below means it is an original of a copied PDF

                WHERE EXISTS( 
                    SELECT 1 
                    FROM logs_pdf p2 
                    WHERE p2.previous_pdf = p.id 
                )
                AND p.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
                ORDER BY (id, order_index) ASC
            )

            SELECT t.desc_sv as "value!", COUNT(*) AS "count!" 
            FROM (
-- Find every row where the copy and original is identical, and count them by topic
                SELECT UNNEST(topics) as topic_id FROM(
                    SELECT o.id, o.topics, 
                    ROW(c.has_title, c.has_subtitle, c.has_name_field, c.font_size, c.answer_columns, c.lang, c.write_solutions,
                    c.color, c.paper_size, c.x_margin, c.y_margin, c.par_spacing, c.max_prefix_group, c.page_break_before_answers,
                    c.topics, c.exclusions, c.problem_count, c.columns, c.has_heading, c.problem_spacing, c.pagebreak_after, c.order_index,
                    c.starting_difficulty, c.ending_difficulty
                    )
                    IS NOT DISTINCT FROM 
                    ROW(o.has_title, o.has_subtitle, o.has_name_field, o.font_size, o.answer_columns, o.lang, o.write_solutions,
                    o.color, o.paper_size, o.x_margin, o.y_margin, o.par_spacing, o.max_prefix_group, o.page_break_before_answers,
                    o.topics, o.exclusions, o.problem_count, o.columns, o.has_heading, o.problem_spacing, o.pagebreak_after, o.order_index,
                    o.starting_difficulty, o.ending_difficulty) 
                    AS is_identical
                    FROM copies c JOIN originals o ON c.previous_pdf = o.id) sub
                WHERE is_identical
            ) counts JOIN topics t ON t.id = counts.topic_id
            GROUP BY topic_id, t.desc_sv 
            ORDER BY "count!" DESC
            LIMIT 10;
        "#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(tops)
}

/// Returns the fields that have been changed the most often between reloads of a PDF
///
/// YES, the query is massive. Sorry.
pub async fn most_changed_fields(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = get_pool();
    let tops = sqlx::query_as!(
        ValueCount,
        r#"
            WITH copies AS (
                SELECT p.id, p.previous_pdf, p.has_title, p.has_subtitle, p.has_name_field, 
                    p.font_size, p.answer_columns, p.lang, p.write_solutions, p.color, p.paper_size, 
                    p.x_margin, p.y_margin, p.par_spacing, p.max_prefix_group, p.page_break_before_answers, 
                    ps.topics, ps.exclusions, ps.problem_count, ps.columns, ps.has_heading, ps.problem_spacing, 
                    ps.pagebreak_after, ps.order_index, ps.starting_difficulty, ps.ending_difficulty 
                FROM logs_pdf p 
                JOIN logs_problem_set ps
                ON p.id = ps.pdf_id
                WHERE p.previous_pdf IS NOT null
                AND p.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
            ),

            originals AS (
                SELECT p.id, p.has_title, p.has_subtitle, p.has_name_field, 
                    p.font_size, p.answer_columns, p.lang, p.write_solutions, p.color, p.paper_size, 
                    p.x_margin, p.y_margin, p.par_spacing, p.max_prefix_group, p.page_break_before_answers, 
                    ps.topics, ps.exclusions, ps.problem_count, ps.columns, ps.has_heading, ps.problem_spacing, 
                    ps.pagebreak_after, ps.order_index, ps.starting_difficulty, ps.ending_difficulty 
                FROM logs_pdf p 
                JOIN logs_problem_set ps 
                ON p.id = ps.pdf_id
                WHERE EXISTS( 
                    SELECT 1 
                    FROM logs_pdf p2 
                    WHERE p2.previous_pdf = p.id 
                )
                AND p.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
                ORDER BY (id, order_index) ASC
            ),
            counts AS (
                SELECT 
    -- COUNT DISTINCT with regards to c.id since one pdf can have multiple rows due to being joined with the problem_set table 
                    COUNT(DISTINCT CASE WHEN c.has_title IS DISTINCT FROM o.has_title THEN c.id END) AS "Title (on/off)!", 
                    COUNT(DISTINCT CASE WHEN c.has_subtitle IS DISTINCT FROM o.has_subtitle THEN c.id END) AS "Subtitle (on/off)!", 
                    COUNT(DISTINCT CASE WHEN c.has_name_field IS DISTINCT FROM o.has_name_field THEN c.id END) AS "Name field!", 
                    COUNT(DISTINCT CASE WHEN c.font_size IS DISTINCT FROM o.font_size THEN c.id END) AS "Font size!", 
                    COUNT(DISTINCT CASE WHEN c.answer_columns IS DISTINCT FROM o.answer_columns THEN c.id END) AS "Answer columns!", 
                    COUNT(DISTINCT CASE WHEN c.lang IS DISTINCT FROM o.lang THEN c.id END) AS "Language!", 
                    COUNT(DISTINCT CASE WHEN c.write_solutions IS DISTINCT FROM o.write_solutions THEN c.id END) AS "Which solutions!", 
                    COUNT(DISTINCT CASE WHEN c.color IS DISTINCT FROM o.color THEN c.id END) AS "Color!", 
                    COUNT(DISTINCT CASE WHEN c.paper_size IS DISTINCT FROM o.paper_size THEN c.id END) AS "Paper size!", 
                    COUNT(DISTINCT CASE WHEN c.x_margin IS DISTINCT FROM o.x_margin THEN c.id END) AS "Margin (x)!", 
                    COUNT(DISTINCT CASE WHEN c.y_margin IS DISTINCT FROM o.y_margin THEN c.id END) AS "Margin (y)!", 
                    COUNT(DISTINCT CASE WHEN c.par_spacing IS DISTINCT FROM o.par_spacing THEN c.id END) AS "Spacing between sets!", 
                    COUNT(DISTINCT CASE WHEN c.max_prefix_group IS DISTINCT FROM o.max_prefix_group THEN c.id END) AS "Prefix group length!", 
                    COUNT(DISTINCT CASE WHEN c.page_break_before_answers IS DISTINCT FROM o.page_break_before_answers THEN c.id END) AS "Page break before answers!", 
    -- DISTINCT and c.id ends here since we start looking at problem set attributes 
                    COUNT(CASE WHEN c.topics IS DISTINCT FROM o.topics THEN 1 END) AS "Topics!", 
                    COUNT(CASE WHEN c.exclusions IS DISTINCT FROM o.exclusions THEN 1 END) AS "Exclusions!", 
                    COUNT(CASE WHEN c.problem_count IS DISTINCT FROM o.problem_count THEN 1 END) AS "Problem count!", 
                    COUNT(CASE WHEN c.columns IS DISTINCT FROM o.columns THEN 1 END) AS "Question columns!", 
                    COUNT(CASE WHEN c.has_heading IS DISTINCT FROM o.has_heading THEN 1 END) AS "Problem set heading (on/off)!", 
                    COUNT(CASE WHEN c.problem_spacing IS DISTINCT FROM o.problem_spacing THEN 1 END) AS "Spacing between problems!", 
                    COUNT(CASE WHEN c.pagebreak_after IS DISTINCT FROM o.pagebreak_after THEN 1 END) AS "Pagebreak after set!", 
                    COUNT(CASE WHEN c.starting_difficulty IS DISTINCT FROM o.starting_difficulty THEN 1 END) AS "Starting difficulty!", 
                    COUNT(CASE WHEN c.ending_difficulty IS DISTINCT FROM o.ending_difficulty THEN 1 END) AS "Ending difficulty!" 
                FROM copies c 
                JOIN originals o 
                ON c.previous_pdf = o.id
            )

            SELECT rtrim(key, '!') AS "value!", value::int AS "count!"
            FROM counts,
            LATERAL jsonb_each(to_jsonb(counts))
            ORDER BY "count!" DESC
            LIMIT 10
        "#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(tops)
}
