use super::{DbDescRow, error_context, error_context_by_name};
use crate::{DescriptionTranslations, ForceReadPrivateData, HasDesc};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Representation of data about a course from the DB
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
    pub chapter_ids: Vec<i32>,
}
impl HasDesc for CourseEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for CourseEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        CourseEntry {
            id,
            name,
            desc,
            chapter_ids: Vec::new(),
        }
    }
}

/// Returns every course that is marked as public in the DB.
///
/// Used by the user-facing API (force_private: false) and
/// the editor for listing the courses (force_private: true)
pub async fn get_all_course_data(
    force_read_private_data: ForceReadPrivateData,
) -> Result<Vec<CourseEntry>> {
    let ForceReadPrivateData(dev_mode) = force_read_private_data;
    // In prod we only want the public rows,
    // in dev we want all
    let production_mode =
        !dev_mode && (cfg!(feature = "docker") || std::env::args().any(|x| x == "prod"));

    let pool = crate::get_pool();
    let courses = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en
            FROM courses 
            WHERE (NOT $1::bool OR public)
            ORDER BY name"#,
        production_mode
    )
    .fetch_all(pool)
    .await?;

    Ok(courses.into_iter().map(CourseEntry::from).collect())
}

// pub async fn get_all_course_data() -> Result<Vec<CourseEntry>> {
//     let pool = crate::get_pool();
//     let courses = sqlx::query_as!(
//         DbDescRow,
//         r#"SELECT id, name, desc_sv, desc_en
//             FROM courses ORDER BY name"#,
//     )
//     .fetch_all(pool)
//     .await?;
//
//     Ok(courses.into_iter().map(CourseEntry::from).collect())
// }

pub async fn get_course_by_id(id: i32) -> Result<CourseEntry> {
    let pool = crate::get_pool();
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

pub async fn get_courses_from_ids(course_ids: &[i32]) -> Result<Vec<CourseEntry>> {
    let pool = crate::get_pool();
    let courses = sqlx::query_as!(
        DbDescRow,
        r#"SELECT c.id, c.name, c.desc_sv, c.desc_en
        FROM courses c
        JOIN UNNEST($1::int[]) WITH ORDINALITY AS u(id, ord) ON c.id = u.id
        ORDER BY u.ord"#,
        course_ids
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get courses with ids {course_ids:?}"))?;

    Ok(courses.into_iter().map(CourseEntry::from).collect())
}

pub async fn get_course_by_name(name: &str) -> Result<CourseEntry> {
    let pool = crate::get_pool();
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

pub async fn get_courses_from_chapter(chapter_id: &i32) -> Result<Vec<CourseEntry>> {
    let pool = crate::get_pool();
    let courses = sqlx::query_as!(
        DbDescRow,
        r#"SELECT co.id, co.name, co.desc_sv, co.desc_en
        FROM courses co
        JOIN course_chapters coch ON co.id = coch.course_id
        WHERE coch.chapter_id = $1
        ORDER BY coch.order_index, co.name"#,
        chapter_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get courses from chapter {}", chapter_id))?;

    Ok(courses.into_iter().map(CourseEntry::from).collect())
}

pub async fn create_course_from_entry(course: &CourseEntry) -> Result<i32> {
    let pool = crate::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO courses (name, desc_sv, desc_en) VALUES ($1, $2, $3) 
               RETURNING id"#,
        course.name,
        course.desc.sv,
        course.desc.en,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "course", &course.name))?;

    Ok(created.id)
}

pub async fn update_course_from_entry(course: CourseEntry) -> Result<String> {
    let pool = crate::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE courses SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 
               RETURNING name"#,
        course.name,
        course.desc.sv,
        course.desc.en,
        course.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "course", course.id))?;

    Ok(updated.name)
}

pub async fn delete_course_with_id(id: i32) -> Result<String> {
    let pool = crate::get_pool();
    let _result = sqlx::query!(r#"DELETE FROM course_chapters WHERE course_id = $1"#, id)
        .execute(pool)
        .await
        .with_context(|| error_context("delete", "course", id))?;
    let result = sqlx::query!(r#"DELETE FROM courses WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "course", id))?;

    Ok(result.name)
}
