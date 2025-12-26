use std::collections::HashMap;

use crate::{
    Language,
    db::{self, ChapterEntry, CourseEntry, HasDesc, ProblemEntry, TopicEntry},
    errors::ApiError,
};
use axum::{
    Json,
    extract::{Path, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Only used for sending a list of courses
#[derive(Serialize, Deserialize)]
struct CourseData {
    id: i32,
    desc: String,
}

#[derive(Serialize, Deserialize)]
struct CourseHierarchy {
    name: String,
    id: String,
    desc: String,
    chapters: Vec<ChapterHierarchy>,
}
impl CourseHierarchy {
    fn from(entry: &CourseEntry, chapters: Vec<ChapterHierarchy>, lang: &Language) -> Self {
        CourseHierarchy {
            name: entry.name.clone(),
            id: entry.id.to_string(),
            desc: entry.get_desc(lang),
            chapters,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ChapterHierarchy {
    name: String,
    id: String,
    desc: String,
    topics: Vec<TopicHierarchy>,
}
impl ChapterHierarchy {
    // TopicEntry is used instead of TopicHierarchy since that is what the Db returns,
    // to avoid conversions in every function
    fn from(entry: &ChapterEntry, topics: &[TopicEntry], lang: &Language) -> Self {
        ChapterHierarchy {
            name: entry.name.clone(),
            id: entry.id.to_string(),
            desc: entry.get_desc(lang),
            topics: topics
                .iter()
                .map(|t| TopicHierarchy::from(t, &lang))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TopicHierarchy {
    name: String,
    id: String,
    desc: String,
}
impl TopicHierarchy {
    fn from(entry: &TopicEntry, lang: &Language) -> Self {
        TopicHierarchy {
            name: entry.name.clone(),
            id: entry.id.to_string(),
            desc: entry.get_desc(lang),
        }
    }
}

/// Only used for the /topic endpoint, when the problems are expected to be returned
#[derive(Serialize, Deserialize)]
struct TopicWithProblems {
    id: i32,
    desc: String,
    problems: Vec<ProblemInformation>,
}
impl TopicWithProblems {
    fn from(topic: &TopicEntry, problems: &[ProblemEntry], lang: &Language) -> Self {
        TopicWithProblems {
            id: topic.id,
            desc: topic.get_desc(&lang),
            problems: problems
                .into_iter()
                .map(|p| ProblemInformation {
                    id: p.id,
                    difficulty: p.difficulty,
                    desc: p.get_desc(&lang),
                })
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ProblemInformation {
    id: i32, // We probably don't need the name in the frontend, only id
    difficulty: i32,
    desc: String,
}

pub async fn get_translation(Path(lang_code): Path<String>) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_code)?;
    let translations = db::i18n::get_i18n_for_web(&lang).await?;

    Ok((
        StatusCode::OK,
        [
            ("Cache-Control", "public, max-age=3600"), // Cache 1 hour
            ("Content-Type", "application/json"),
        ],
        Json(json!(translations)),
    ))
}

pub async fn get_courses(Path(lang_code): Path<String>) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_code)?;
    let courses = db::get_all_course_data().await?;
    let mut course_data: HashMap<String, CourseData> = HashMap::new();
    for course in courses {
        course_data.insert(
            course.name.clone(),
            CourseData {
                id: course.id,
                desc: course.get_desc(&lang),
            },
        );
    }

    Ok((
        StatusCode::OK,
        [
            ("Cache-Control", "public, max-age=3600"), // Cache 1 hour
            ("Content-Type", "application/json"),
        ],
        Json(json!(course_data)),
    ))
}

pub async fn get_course(
    Path((lang_code, course_path)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_code)?;
    let course = parse_course_path(&course_path).await?;
    let chapters = db::get_course_chapters(course.id).await?;
    let chapter_ids: Vec<i32> = chapters.iter().map(|c| c.id).collect();
    let topics_by_chapter = db::get_topics_for_chapters(&chapter_ids).await?;
    // If a chapter has no topics, it will not be included in the topics_by_chapter.
    // The empty Vec is just a helper to have something to point to, for now.
    // TODO: Remove this eventually
    let empty_chapters = Vec::new();

    let chapter_hierarchies: Vec<ChapterHierarchy> = chapters
        .into_iter()
        .map(|chapter| {
            // For each chapter, we need to construct a separate topic list,
            // using the map between chapters and topics we got from the db.
            //
            // This is way faster than hitting the db for each chapter!
            // Important since this endpoint will be the most common hit.
            let topics = topics_by_chapter
                .get(&chapter.id)
                .unwrap_or(&empty_chapters);
            Ok(ChapterHierarchy::from(&chapter, topics, &lang))
        })
        .collect::<Result<Vec<ChapterHierarchy>, ApiError>>()?;

    let course_hierarchy = CourseHierarchy::from(&course, chapter_hierarchies, &lang);
    Ok((StatusCode::OK, Json(json!(course_hierarchy))))
}

pub async fn get_chapter(
    Path((lang_code, course_path, chapter_path)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_code)?;
    let chapter_entry = validate_chapter(&course_path, &chapter_path).await?;
    let topics = db::get_chapter_topics(chapter_entry.id).await?;
    let chapter = ChapterHierarchy::from(&chapter_entry, &topics, &lang);

    Ok((StatusCode::OK, Json(json!(chapter))))
}

pub async fn get_problems(
    Path(lang_code): Path<String>,
    payload: Result<Json<Vec<i32>>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let topic_ids = match payload {
        Ok(Json(topics)) => topics,
        Err(e) => {
            return Err(ApiError::BadRequest(e.to_string()));
        }
    };
    let lang = parse_language(&lang_code)?;
    let topics = db::get_topics_from_ids(&topic_ids).await?;
    let mut topic_vec = Vec::new();
    for topic in topics {
        let problems = db::get_topic_problems(topic.id).await?;
        let topic = TopicWithProblems::from(&topic, &problems, &lang);
        topic_vec.push(topic);
    }

    Ok((StatusCode::OK, Json(json!(topic_vec))))
}

async fn parse_course_path(course_path: &str) -> Result<CourseEntry, ApiError> {
    let course_entry = match course_path.parse::<i32>() {
        Ok(id) => db::get_course_by_id(id).await?,
        Err(_) => db::get_course_by_name(course_path).await?,
    };

    Ok(course_entry)
}

async fn validate_chapter(course_path: &str, chapter_path: &str) -> Result<ChapterEntry, ApiError> {
    let course = parse_course_path(&course_path).await?;
    let valid_chapters = db::get_course_chapters(course.id).await?;
    let chapter_entry = valid_chapters
        .into_iter()
        // chapter_path can be either an ID or a name
        .find(|c| c.name == chapter_path || &c.id.to_string() == chapter_path)
        .ok_or_else(|| {
            ApiError::BadRequest(format!(
                "There is no chapter \"{chapter_path}\" in course {course_path}"
            ))
        })?;
    Ok(chapter_entry)
}

fn parse_language(lang: &str) -> Result<Language, ApiError> {
    match lang {
        "sv" => Ok(Language::Sv),
        "en" => Ok(Language::En),
        _ => Err(ApiError::BadRequest("Invalid language".to_string())),
    }
}
