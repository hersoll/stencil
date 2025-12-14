use crate::{
    Language,
    db::{self, ChapterEntry, CourseEntry, HasDesc, ProblemEntry, TopicEntry},
    errors::ApiError,
};
use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
struct TopicHierarchyWithProblems {
    name: String,
    id: String,
    desc: String,
    problems: Vec<ProblemInformation>,
}
impl TopicHierarchyWithProblems {
    fn from(entry: &TopicEntry, problems: &[ProblemEntry], lang: &Language) -> Self {
        TopicHierarchyWithProblems {
            name: entry.name.clone(),
            id: entry.id.to_string(),
            desc: entry.get_desc(lang),
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

// lang_c instead of lang_code to shorten line :)
pub async fn get_topic(
    Path((lang_c, course_path, chapter_path, topic_path)): Path<(String, String, String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_c)?;
    let topic_entry = validate_topic(&course_path, &chapter_path, &topic_path).await?;
    let problems = db::get_topic_problems(topic_entry.id).await?;
    let topic = TopicHierarchyWithProblems::from(&topic_entry, &problems, &lang);

    Ok((StatusCode::OK, Json(json!(topic))))
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

async fn validate_topic(
    course_path: &str,
    chapter_path: &str,
    topic_path: &str,
) -> Result<TopicEntry, ApiError> {
    let chapter_entry = validate_chapter(course_path, chapter_path).await?;
    let valid_topics = db::get_chapter_topics(chapter_entry.id).await?;
    let topic_entry = valid_topics
        .into_iter()
        // topic_path can be either an ID or a name
        .find(|t| t.name == topic_path || &t.id.to_string() == topic_path)
        .ok_or_else(|| {
            ApiError::BadRequest(format!(
                "There is no topic \"{topic_path}\" in chapter {chapter_path}"
            ))
        })?;

    Ok(topic_entry)
}

fn parse_language(lang: &str) -> Result<Language, ApiError> {
    match lang {
        "sv" => Ok(Language::Sv),
        "en" => Ok(Language::En),
        _ => Err(ApiError::BadRequest("Invalid language".to_string())),
    }
}
