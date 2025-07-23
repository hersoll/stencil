use std::{collections::HashMap, fs};

use dioxus::prelude::*;
use sqlx::Postgres;

use crate::{backend::{self, Database}, shared::{self, CourseInfo}, Error};

#[server]
pub async fn load_translations(lang: String) -> Result<HashMap<String, String>, ServerFnError> {
    let pool = extract::<sqlx::Pool<Postgres>>().await?;
    let db = Database::new(pool);
    let data = db.get_i18n(lang)?;
    Ok(data)
}

#[server]
pub async fn load_courses(lang: String) -> Result<Vec<CourseInfo>, ServerFnError> {
    let pool = extract::<sqlx::Pool<Postgres>>().await?;
    let db = Database::new(pool);
    let data = db.get_courses(&lang)?;
    Ok(data)
}

#[server]
pub async fn load_course_chapters(course_id: i32, lang: String) -> Result<HashMap<String, String>, ServerFnError> {
    let pool = extract::<sqlx::Pool<Postgres>>().await?;
    let db = Database::new(pool);
    let data = db.get_course_chapters(course_id, &lang)?;
    Ok(data)
}


#[server]
pub async fn generate_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    options: shared::DocumentOptions,
) -> Result<Vec<u8>, ServerFnError> {
    let pool = extract::<sqlx::Pool<Postgres>>().await?;
    let db = Database::new(pool);
    let pdf = backend::create_pdf(sets, options, db).await?;
    Ok(pdf)
}
