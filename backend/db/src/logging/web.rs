use anyhow::Result;
use types::lang::Language;

pub async fn log_language(lang: Language) -> Result<()> {
    let pool = crate::get_pool();
    sqlx::query!(r#"INSERT INTO logs_lang (lang) VALUES ($1)"#, lang.to_str())
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn log_course(course_id: i32) -> Result<()> {
    let pool = crate::get_pool();
    sqlx::query!(
        r#"INSERT INTO logs_course (course_id) VALUES ($1)"#,
        course_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
