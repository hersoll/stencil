use std::collections::HashMap;
use dioxus::prelude::*;
use crate::shared::{self, ChapterInfo, CourseInfo, Difficulty, ProblemInfo, TopicInfo};

#[server]
pub async fn load_translations(lang: String) -> Result<HashMap<String, String>, ServerFnError> {
    let data = crate::backend::db::I18nDatabase::get_i18n(&lang).await?;
    Ok(data)
}

#[server]
pub async fn load_courses(lang: String) -> Result<Vec<CourseInfo>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_all_courses(&lang).await?;
    Ok(data)
}

#[server]
pub async fn load_course_chapters(course_id: i32, lang: String) -> Result<Vec<ChapterInfo>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_course_chapters(course_id, &lang).await?;
    Ok(data)
}

#[server]
pub async fn load_chapter_topics(chapter_id: i32, lang: String) -> Result<Vec<TopicInfo>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_chapter_topics(chapter_id, &lang).await?;
    Ok(data)
}

#[server]
pub async fn load_topic_problems(topic_id: i32, lang: String) -> Result<Vec<ProblemInfo>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic_problems(topic_id, &lang).await?;
    Ok(data)
}

#[server]
pub async fn load_valid_problems(topic_ids: Vec<i32>, starting_difficulty: Difficulty, ending_difficulty: Difficulty, lang: String) -> Result<Vec<ProblemInfo>, ServerFnError> {
    let starting_difficulty_num = *Difficulty::enum_to_nums(starting_difficulty).iter().min().unwrap() as i32;
    let ending_difficulty_num = *Difficulty::enum_to_nums(ending_difficulty).iter().max().unwrap() as i32;
    let data = crate::backend::db::ProblemDatabase::get_valid_problems(topic_ids, starting_difficulty_num, ending_difficulty_num, lang).await?;
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
