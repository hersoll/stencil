use crate::{
    DatabaseRow, DescriptionTranslations, HasDesc, ID, Name, PublicFlag, error_context,
    error_context_by_name,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representation of data about a course from the DB, the way it's sent to the user
///
/// Note that this struct does not have an `is_new` field, unlike `ChapterEntry` and the rest,
/// since we don't care about new courses :)
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseEntry {
    pub id: ID,
    pub name: Name,
    pub desc: DescriptionTranslations,
    pub chapter_ids: Vec<ID>,
}

/// The same data as [`CourseEntry`], except it includes information about whether the entry is
/// public or not.
///
/// This is needed so the editor can edit the `public` value.
#[derive(Debug, Serialize, Deserialize)]
pub struct CourseEntryForEditor {
    #[serde(flatten)]
    pub entry: CourseEntry,
    pub public: PublicFlag,
}

impl HasDesc for CourseEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DatabaseRow> for CourseEntry {
    fn from(row: DatabaseRow) -> Self {
        CourseEntry {
            id: row.id,
            desc: row.as_desc_translations(),
            chapter_ids: Vec::new(),
            name: row.name, // This comes last since name is not Copy
        }
    }
}
impl From<DatabaseRow> for CourseEntryForEditor {
    fn from(row: DatabaseRow) -> Self {
        CourseEntryForEditor {
            public: row.public,
            entry: CourseEntry::from(row),
        }
    }
}

/// Returns every course that is marked as public in the DB.
///
/// Used by the user-facing API for the course dropdown
pub async fn get_public_courses() -> Result<Vec<CourseEntry>> {
    let production_mode = crate::production_mode();
    let pool = crate::get_pool();
    let courses = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT id, name, desc_sv, desc_en, public, false as "is_new!"
            FROM courses 
            WHERE (NOT $1::bool OR public)
            ORDER BY name"#,
        production_mode
    )
    .fetch_all(pool)
    .await?;

    Ok(courses.into_iter().map(CourseEntry::from).collect())
}

/// Returns the data about *every* course in the DB.
///
/// Used by calls from the editor, since we want to see both public and private rows
pub async fn get_all_courses() -> Result<Vec<CourseEntryForEditor>> {
    let pool = crate::get_pool();
    let courses = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT id, name, desc_sv, desc_en, public, false as "is_new!"
            FROM courses ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(courses
        .into_iter()
        .map(CourseEntryForEditor::from)
        .collect())
}

/// Retrieve a Course row using its name.
///
/// Used by the user in the AddSetView, when they have selected a course and want to see the chapters and topics (and
/// description of the course)
pub async fn get_course_by_name(name: &str) -> Result<CourseEntry> {
    let pool = crate::get_pool();
    let course = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT id, name, desc_sv, desc_en, public, false as "is_new!"
            FROM courses
            WHERE name = $1"#,
        name,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("get", "course", name))?;

    Ok(CourseEntry::from(course))
}

/// Retrieve all of the courses that contain a certain chapter.
///
/// Used by the editor, to fill the list of related courses when a chapter is selected for editing
pub async fn get_courses_from_chapter(chapter_id: &i32) -> Result<Vec<CourseEntryForEditor>> {
    let pool = crate::get_pool();
    let courses = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT co.id, co.name, co.desc_sv, co.desc_en, co.public, false as "is_new!"
        FROM courses co
        JOIN course_chapters coch ON co.id = coch.course_id
        WHERE coch.chapter_id = $1
        ORDER BY coch.order_index, co.name"#,
        chapter_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get courses from chapter {}", chapter_id))?;

    Ok(courses
        .into_iter()
        .map(CourseEntryForEditor::from)
        .collect())
}

/// Fetches every course related to each chapter, collected as a HashMap.
///
/// Used when loading the initial chapter list in the editor
pub async fn get_courses_for_chapters(chapter_ids: &[ID]) -> Result<HashMap<ID, Vec<ID>>> {
    let pool = crate::get_pool();
    let rows = sqlx::query!(
        r#"SELECT co.id, coch.chapter_id
        FROM courses co
        JOIN course_chapters coch ON co.id = coch.course_id
        WHERE coch.chapter_id = ANY($1)
        ORDER BY coch.chapter_id, coch.order_index, co.name"#,
        chapter_ids
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get courses for chapters {:?}", chapter_ids))?;

    let mut map: HashMap<ID, Vec<ID>> = HashMap::new();
    for row in rows {
        let chapter_id = row.chapter_id;
        map.entry(chapter_id).or_default().push(row.id);
    }
    Ok(map)
}

pub async fn create_course_from_entry(course: &CourseEntryForEditor) -> Result<i32> {
    let pool = crate::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO courses (name, desc_sv, desc_en, public) VALUES ($1, $2, $3, $4) 
               RETURNING id"#,
        course.entry.name,
        course.entry.desc.sv,
        course.entry.desc.en,
        course.public
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "course", &course.entry.name))?;

    Ok(created.id)
}

pub async fn update_course_from_entry(course: CourseEntryForEditor) -> Result<String> {
    let pool = crate::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE courses SET name = $2, desc_sv = $3, desc_en = $4, public = $5 WHERE id = $1
               RETURNING name"#,
        course.entry.id,
        course.entry.name,
        course.entry.desc.sv,
        course.entry.desc.en,
        course.public,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "course", course.entry.id))?;

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

/// Makes every course public
pub async fn publish_all_courses() -> Result<u64> {
    let pool = crate::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE courses 
        SET public = true
        WHERE public = false"#,
    )
    .execute(pool)
    .await
    .with_context(|| "Failed to publish courses")?;

    Ok(updated.rows_affected())
}
