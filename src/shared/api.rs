use crate::shared::{
    self, ChapterData, ChapterInfo, CourseData, CourseInfo, Difficulty, ProblemData, ProblemInfo,
    TopicData, TopicInfo,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[server]
pub async fn load_translations(lang: String) -> Result<HashMap<String, String>, ServerFnError> {
    let data = crate::backend::db::I18nDatabase::get_i18n(&lang).await?;
    Ok(data)
}

//###############################
//#          COURSES            #
//###############################
#[server]
pub async fn load_courses(lang: String) -> Result<Vec<CourseInfo>, ServerFnError> {
    let courses = crate::backend::db::ProblemDatabase::get_all_courses(&lang).await?;
    Ok(courses)
}

#[server]
pub async fn load_all_course_data() -> Result<Vec<CourseData>, ServerFnError> {
    let courses = crate::backend::db::ProblemDatabase::get_all_course_data().await?;
    Ok(courses)
}

#[server]
pub async fn load_course(id: i32, lang: String) -> Result<CourseInfo, ServerFnError> {
    let course = crate::backend::db::ProblemDatabase::get_course(id, &lang).await?;
    Ok(course)
}

#[server]
pub async fn load_course_name(id: i32) -> Result<String, ServerFnError> {
    let course = crate::backend::db::ProblemDatabase::get_course(id, "sv").await?;
    Ok(course.name)
}

#[server]
pub async fn load_course_desc(id: i32, lang: String) -> Result<String, ServerFnError> {
    let course = crate::backend::db::ProblemDatabase::get_course(id, &lang).await?;
    Ok(course.desc)
}

//###############################
//#          CHAPTERS           #
//###############################
#[server]
pub async fn load_course_chapters(course_id: i32, lang: String) -> Result<Vec<i32>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_course_chapters(course_id, &lang).await?;
    Ok(data.iter().map(|chapter| chapter.id).collect())
}

#[server]
pub async fn load_all_chapter_data() -> Result<Vec<ChapterData>, ServerFnError> {
    let chapters = crate::backend::db::ProblemDatabase::get_all_chapter_data().await?;
    Ok(chapters)
}

#[server]
pub async fn load_chapter(id: i32, lang: String) -> Result<ChapterInfo, ServerFnError> {
    let chapter = crate::backend::db::ProblemDatabase::get_chapter(id, &lang).await?;
    Ok(chapter)
}

#[server]
pub async fn load_chapter_name(id: i32) -> Result<String, ServerFnError> {
    let chapter = crate::backend::db::ProblemDatabase::get_chapter(id, "sv").await?;
    Ok(chapter.name)
}

#[server]
pub async fn load_chapter_desc(id: i32, lang: String) -> Result<String, ServerFnError> {
    let chapter = crate::backend::db::ProblemDatabase::get_chapter(id, &lang).await?;
    Ok(chapter.desc)
}

/// It is important that the descs are sent back in the same order as the ids came in
#[server]
pub async fn load_chapter_descs(ids: Vec<i32>, lang: String) -> Result<Vec<String>, ServerFnError> {
    let chapters = crate::backend::db::ProblemDatabase::get_chapters(&ids, &lang).await?;
    let descs: Result<Vec<_>, _> = ids
        .iter()
        .map(|&id| {
            chapters
                .iter()
                .find(|chapter| chapter.id == id)
                .ok_or(ServerFnError::ServerError(format!(
                    "Chapter not found with id {id}"
                )))
                .map(|chapter| chapter.desc.clone())
        })
        .collect();
    descs
}

//###############################
//#          TOPICS             #
//###############################
#[server]
pub async fn load_chapter_topics(chapter_id: i32, lang: String) -> Result<Vec<i32>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_chapter_topics(chapter_id, &lang).await?;
    Ok(data.iter().map(|topic| topic.id).collect())
}

#[server]
pub async fn load_all_topic_data() -> Result<Vec<TopicData>, ServerFnError> {
    let topics = crate::backend::db::ProblemDatabase::get_all_topic_data().await?;
    Ok(topics)
}

#[server]
pub async fn load_topic(id: i32, lang: String) -> Result<TopicInfo, ServerFnError> {
    let topic = crate::backend::db::ProblemDatabase::get_topic(id, &lang).await?;
    Ok(topic)
}

#[server]
pub async fn load_topic_by_id(topic_id: i32, lang: String) -> Result<TopicInfo, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic(topic_id, &lang).await?;
    Ok(data)
}

#[server]
pub async fn load_topic_name(id: i32) -> Result<String, ServerFnError> {
    let topic = crate::backend::db::ProblemDatabase::get_topic(id, "sv").await?;
    Ok(topic.name)
}

#[server]
pub async fn load_topic_desc(id: i32, lang: String) -> Result<String, ServerFnError> {
    let topic = crate::backend::db::ProblemDatabase::get_topic(id, &lang).await?;
    Ok(topic.desc)
}

/// It is important that the descs are sent back in the same order as the ids came in
#[server]
pub async fn load_topic_descs(ids: Vec<i32>, lang: String) -> Result<Vec<String>, ServerFnError> {
    let topics = crate::backend::db::ProblemDatabase::get_topics(&ids, &lang).await?;
    let descs: Result<Vec<_>, _> = ids
        .iter()
        .map(|&id| {
            topics
                .iter()
                .find(|chapter| chapter.id == id)
                .ok_or(ServerFnError::ServerError(format!(
                    "Chapter not found with id {id}"
                )))
                .map(|chapter| chapter.desc.clone())
        })
        .collect();
    descs
}

//###############################
//#          PROBLEMS           #
//###############################

#[server]
pub async fn load_topic_problems(
    topic_id: i32,
    lang: String,
) -> Result<Vec<ProblemInfo>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic_problems(topic_id, &lang).await?;
    Ok(data)
}

#[server]
pub async fn load_all_problem_data() -> Result<Vec<ProblemData>, ServerFnError> {
    let problems = crate::backend::db::ProblemDatabase::get_all_problem_data().await?;
    Ok(problems)
}

#[server]
pub async fn load_problem_by_id(
    problem_id: i32,
    lang: String,
) -> Result<ProblemInfo, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_problem(problem_id, &lang).await?;
    Ok(data)
}

#[server]
pub async fn load_valid_problems(
    topic_ids: Vec<i32>,
    starting_difficulty: Difficulty,
    ending_difficulty: Difficulty,
    lang: String,
) -> Result<Vec<ProblemInfo>, ServerFnError> {
    let starting_difficulty_num = *Difficulty::enum_to_nums(starting_difficulty)
        .iter()
        .min()
        .unwrap() as i32;
    let ending_difficulty_num = *Difficulty::enum_to_nums(ending_difficulty)
        .iter()
        .max()
        .unwrap() as i32;
    let data = crate::backend::db::ProblemDatabase::get_topic_problems_in_difficulty_range(
        topic_ids,
        starting_difficulty_num,
        ending_difficulty_num,
        &lang,
    )
    .await?;
    Ok(data)
}

#[server]
pub async fn generate_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    options: shared::DocumentOptions,
) -> Result<Vec<u8>, ServerFnError> {
    let pdf = crate::backend::create_pdf(sets, options).await?;
    Ok(pdf)
}
