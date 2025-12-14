use super::common::{DbDescRow, error_context, error_context_by_name};
use super::relationships::{CourseChapters, update_relationships};
use crate::db::{self, CourseEntry};
use anyhow::{Context, Result};

impl From<DbDescRow> for CourseEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        CourseEntry { id, name, desc }
    }
}

/// Get all courses ordered by name
pub async fn get_all_courses() -> Result<Vec<CourseEntry>> {
    let pool = db::get_pool();
    let courses = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en
            FROM courses ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(courses.into_iter().map(CourseEntry::from).collect())
}

/// Get a single course by ID
pub async fn get_course_by_id(id: i32) -> Result<CourseEntry> {
    let pool = db::get_pool();
    let course = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en        
            FROM courses
            WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("get", "course", id))?;

    Ok(CourseEntry::from(course))
}

/// Get a single course by name
pub async fn get_course_by_name(name: &str) -> Result<CourseEntry> {
    let pool = db::get_pool();
    let course = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en        
            FROM courses
            WHERE name = $1"#,
        name,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("get", "course", name))?;

    Ok(CourseEntry::from(course))
}

/// Create a new course
pub async fn create_course(course: CourseEntry) -> Result<CourseEntry> {
    let pool = db::get_pool();
    let desc = course.desc;
    let created = sqlx::query_as!(
        DbDescRow,
        r#"INSERT INTO courses (name, desc_sv, desc_en) VALUES ($1, $2, $3) 
               RETURNING id, name, desc_sv, desc_en"#,
        course.name,
        desc.sv,
        desc.en,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "course", &course.name))?;

    Ok(CourseEntry::from(created))
}

/// Update an existing course
pub async fn update_course(course: CourseEntry) -> Result<CourseEntry> {
    let pool = db::get_pool();
    let desc = course.desc;
    let updated = sqlx::query_as!(
        DbDescRow,
        r#"UPDATE courses SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 
               RETURNING id, name, desc_sv, desc_en"#,
        course.name,
        desc.sv,
        desc.en,
        course.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "course", course.id))?;

    Ok(CourseEntry::from(updated))
}

/// Delete a course by ID, returns the deleted course name
pub async fn delete_course(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM courses WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "course", id))?;

    Ok(result.name)
}

/// Update the chapters associated with a course
pub async fn update_course_chapters(course_id: i32, chapter_ids: Vec<i32>) -> Result<()> {
    let pool = db::get_pool();
    update_relationships::<CourseChapters>(pool, course_id, &chapter_ids).await
}
