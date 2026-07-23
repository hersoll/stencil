use axum::{
    Json,
    extract::{Path, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::api::parse_language;
use db::{
    self, ChapterEntry, CourseEntry, ForceReadPrivateData, HasDesc, ProblemEntry, TopicEntry,
    logging,
};
use types::{errors::ApiError, lang::Language};

/// Relevant data which is sent when a user requests a list of courses for the home page
///
/// See [`get_course_list()`].
#[derive(Serialize, Deserialize)]
struct HTTPCourseData {
    id: i32,
    name: String,
    desc: String,
}
impl HTTPCourseData {
    fn from_course_entry(course: CourseEntry, lang: Language) -> Self {
        Self {
            id: course.id,
            desc: course.get_desc_for_lang(lang),
            name: course.name,
        }
    }
}

/// The relevant data when the end user requests topics.
///
/// Not sent on its own, but as part of [`ChapterWithTopics`].
#[derive(Serialize, Deserialize, Clone)]
struct HTTPTopicData {
    id: i32,
    desc: String,
}
impl HTTPTopicData {
    fn from_topic_entry(topic: &TopicEntry, lang: Language) -> Self {
        Self {
            id: topic.id,
            desc: topic.get_desc_for_lang(lang),
        }
    }
}

/// The relevant data about a chapter, including the topics it contains
#[derive(Serialize, Deserialize)]
struct ChapterWithTopics {
    id: i32,
    desc: String,
    topics: Vec<HTTPTopicData>,
}
impl ChapterWithTopics {
    fn from_chapter_and_topics(
        chapter: &ChapterEntry,
        topics: &[HTTPTopicData],
        lang: Language,
    ) -> Self {
        Self {
            id: chapter.id,
            desc: chapter.get_desc_for_lang(lang),
            topics: topics.to_vec(),
        }
    }
}
/// The relevant data when the end user requests problems.
///
/// Not sent on its own, but as part of [`TopicWithProblems`].
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct HTTPProblemData {
    id: i32,
    absolute_difficulty: i32,
    desc: String,
}
impl HTTPProblemData {
    fn from_problem_and_topic_id(problem: &ProblemEntry, topic_id: i32, lang: Language) -> Self {
        Self {
            id: problem.id,
            absolute_difficulty: problem
                .topic_data
                .iter()
                .find(|topic| topic.topic_id == topic_id)
                .expect("During HTTPProblemData::from_problem_and_topic_id, a problem was encountered without the appropriate topic.")
                .absolute_difficulty
                .number as i32,
            desc: problem.get_desc_for_lang(lang),
        }
    }
}

/// Struct which maps a topic ID with a list of problems
///
/// When the end user edit a sets, they need to see every problem associated with each topic.
/// This struct tells the frontend which problems are connected to each topic.
///
/// The struct also contains the descs for ease of use in the frontend. Otherwise there would have
/// to be a lot of mapping and passing things around.
#[derive(Serialize, Deserialize)]
struct TopicWithProblems {
    id: i32,
    desc: String,
    problems: Vec<HTTPProblemData>,
}

/// Returns a `Vec` with data about every course in the database
///
/// This is most likely the "first" API hit of the frontend when accessed through the home page.
/// It's used to list all the courses on the home page.
///
/// While courses are semantically grouped together in certain ways, we return a `Vec` with data
/// and let the frontend handle the structuring in the UI.
pub async fn get_course_list(Path(lang_code): Path<String>) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_code)?;
    let courses: Vec<HTTPCourseData> = db::get_all_course_data(ForceReadPrivateData(false))
        .await?
        .into_iter()
        .map(|course| HTTPCourseData::from_course_entry(course, lang))
        .collect();

    Ok((
        StatusCode::OK,
        [
            ("Cache-Control", "public, max-age=3600"), // Cache 1 hour
            ("Content-Type", "application/json"),
        ],
        Json(json!(courses)),
    ))
}

/// Given a course (either by ID or by name), returns all chapters associated with that course and all topics within those chapters.
///
/// This is the big endpoint for the frontend, which is called whenever the topics load.
/// The returned data is nested, so that each chapter contains the relevant topic data within them
/// ([`ChapterWithTopics`]).
pub async fn get_chapters_and_topics_for_course(
    Path((lang_code, course_path)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_code)?;
    let course = parse_course_path(&course_path).await?;

    // Only log during production (or prod flag) to not mess up the stats
    if cfg!(feature = "docker") || std::env::args().any(|x| x == "prod") {
        logging::log_course(course.id).await?;
    }

    let chapters = db::get_course_chapters(&course.id, ForceReadPrivateData(false)).await?;
    let chapter_ids: Vec<i32> = chapters.iter().map(|c| c.id).collect();
    let topics_by_chapter = db::get_topics_for_chapters(&chapter_ids).await?;

    let chapters: Vec<ChapterWithTopics> = chapters
        .into_iter()
        .map(|chapter| {
            // For each chapter, we need to construct a separate topic list,
            // using the map between chapters and topics we got from the db.
            //
            // This is faster than hitting the db for each chapter!
            // Important since this endpoint will be the most common hit.
            let topics: Vec<HTTPTopicData> = topics_by_chapter
                .get(&chapter.id)
                .expect("Don't include empty chapters in the DB please!")
                .iter()
                .map(|topic_entry| HTTPTopicData::from_topic_entry(topic_entry, lang))
                .collect();
            Ok(ChapterWithTopics::from_chapter_and_topics(
                &chapter, &topics, lang,
            ))
        })
        .collect::<Result<Vec<ChapterWithTopics>, ApiError>>()?;

    Ok((
        StatusCode::OK,
        [
            // Cache 10 minutes to prevent it from re-fetching during a single session
            ("Cache-Control", "public, max-age=600"),
            ("Content-Type", "application/json"),
        ],
        Json(json!(chapters)),
    ))
}

/// Given a list of topic IDs, returns data about every problem associated with each topic
///
/// Used when problems are listed for exclusion when editing sets in the frontend
pub async fn get_problems_for_topics(
    Path(lang_code): Path<String>,
    payload: Result<Json<Vec<i32>>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let topic_ids = match payload {
        Ok(Json(topics)) => topics,
        Err(e) => {
            info!("{}", e.to_string());
            return Err(ApiError::BadRequest(e.to_string()));
        }
    };
    let lang = parse_language(&lang_code)?;
    let topics = db::get_topics_from_ids(&topic_ids).await?;
    let mut topics_with_problems = Vec::new();
    for topic in topics {
        let problems = db::get_topic_problems(&topic.id, ForceReadPrivateData(false))
            .await?
            .into_iter()
            .map(|problem| HTTPProblemData::from_problem_and_topic_id(&problem, topic.id, lang))
            .collect();
        let topic_with_problems = TopicWithProblems {
            id: topic.id,
            desc: topic.get_desc_for_lang(lang),
            problems,
        };
        topics_with_problems.push(topic_with_problems);
    }

    Ok((StatusCode::OK, Json(json!(topics_with_problems))))
}

/// Tries to parse either a course ID or a course name from a `&str` and finds that [`CourseEntry`].
async fn parse_course_path(course_path: &str) -> Result<CourseEntry, ApiError> {
    let course_entry = match course_path.parse::<i32>() {
        Ok(id) => db::get_course_by_id(id).await?,
        Err(_) => db::get_course_by_name(course_path).await?,
    };

    Ok(course_entry)
}
