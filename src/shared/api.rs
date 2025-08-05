use crate::shared::{
    self, ChapterData, CourseData, Difficulty, ParsedChapterData, ParsedCourseData,
    ParsedProblemData, ParsedTopicData, ProblemData, TopicData,
};
use dioxus::prelude::{server_fn::codec::Json, *};
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
pub async fn set_course(course: CourseData) -> Result<CourseData, ServerFnError> {
    let result: CourseData;
    if course.id == 0 {
        result = crate::backend::db::ProblemDatabase::create_course(course).await?;
    } else {
        result = crate::backend::db::ProblemDatabase::update_course(course).await?;
    }
    Ok(result)
}

#[server]
pub async fn delete_course(id: i32) -> Result<String, ServerFnError> {
    let name = crate::backend::db::ProblemDatabase::delete_course(id).await?;
    Ok(name)
}

#[server]
pub async fn load_courses(lang: String) -> Result<Vec<ParsedCourseData>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_all_courses().await?;
    let courses = data.into_iter().map(|course| course.parse(&lang)).collect();
    Ok(courses)
}

#[server]
pub async fn load_all_course_data() -> Result<Vec<CourseData>, ServerFnError> {
    let courses = crate::backend::db::ProblemDatabase::get_all_course_data().await?;
    Ok(courses)
}

#[server]
pub async fn load_course(id: i32, lang: String) -> Result<ParsedCourseData, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_course(id).await?;
    let course = data.parse(&lang);
    Ok(course)
}

#[server]
pub async fn load_course_name(id: i32) -> Result<String, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_course(id).await?;
    Ok(data.name)
}

#[server]
pub async fn load_course_desc(id: i32, lang: String) -> Result<String, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_course(id).await?;
    let course = data.parse(&lang);
    Ok(course.desc)
}

//###############################
//#          CHAPTERS           #
//###############################
#[server]
pub async fn set_chapter(chapter: ChapterData) -> Result<ChapterData, ServerFnError> {
    let result: ChapterData;
    if chapter.id == 0 {
        result = crate::backend::db::ProblemDatabase::create_chapter(chapter).await?;
    } else {
        result = crate::backend::db::ProblemDatabase::update_chapter(chapter).await?;
    }
    Ok(result)
}

#[server]
pub async fn delete_chapter(id: i32) -> Result<String, ServerFnError> {
    let name = crate::backend::db::ProblemDatabase::delete_chapter(id).await?;
    Ok(name)
}

#[server]
pub async fn load_course_chapters(course_id: i32) -> Result<Vec<ChapterData>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_course_chapters(course_id).await?;
    Ok(data)
}

#[server(input = Json, output = Json)]
pub async fn set_course_chapters(
    course_id: i32,
    chapters: Vec<ChapterData>,
) -> Result<(), ServerFnError> {
    let ids: Vec<i32> = chapters.iter().map(|ch| ch.id).collect();
    let data = crate::backend::db::ProblemDatabase::update_course_chapters(course_id, ids).await?;
    Ok(data)
}

#[server]
pub async fn load_course_chapter_ids(course_id: i32) -> Result<Vec<i32>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_course_chapters(course_id).await?;
    Ok(data.iter().map(|chapter| chapter.id).collect())
}

#[server]
pub async fn load_all_chapter_ids() -> Result<Vec<i32>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_all_chapter_data().await?;
    let ids = data.into_iter().map(|chapter| chapter.id).collect();
    Ok(ids)
}

#[server]
pub async fn load_all_chapter_data() -> Result<Vec<ChapterData>, ServerFnError> {
    let chapters = crate::backend::db::ProblemDatabase::get_all_chapter_data().await?;
    Ok(chapters)
}

#[server]
pub async fn load_parsed_chapter(
    id: i32,
    lang: String,
) -> Result<ParsedChapterData, ServerFnError> {
    let chapter = crate::backend::db::ProblemDatabase::get_chapter(id).await?;
    Ok(chapter.parse(&lang))
}

#[server(input = Json, output = Json)]
pub async fn load_chapters_by_id(ids: Vec<i32>) -> Result<Vec<ChapterData>, ServerFnError> {
    let chapters = crate::backend::db::ProblemDatabase::get_chapters(&ids).await?;
    Ok(chapters)
}

#[server]
pub async fn load_chapter_name(id: i32) -> Result<String, ServerFnError> {
    let chapter = crate::backend::db::ProblemDatabase::get_chapter(id).await?;
    Ok(chapter.name)
}

#[server]
pub async fn load_chapter_desc(id: i32, lang: String) -> Result<String, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_chapter(id).await?;
    let chapter = data.parse(&lang);
    Ok(chapter.desc)
}

/// It is important that the descs are sent back in the same order as the ids came in
#[server]
pub async fn load_chapter_descs(ids: Vec<i32>, lang: String) -> Result<Vec<String>, ServerFnError> {
    let chapters = crate::backend::db::ProblemDatabase::get_chapters(&ids).await?;
    let descs: Result<Vec<_>, _> = ids
        .iter()
        .map(|&id| {
            chapters
                .iter()
                .find(|chapter| chapter.id == id)
                .ok_or(ServerFnError::ServerError(format!(
                    "Chapter not found with id {id}"
                )))
                .map(|chapter| {
                    if lang == String::from("sv") {
                        chapter.desc_sv.clone()
                    } else {
                        chapter.desc_en.clone()
                    }
                })
        })
        .collect();
    descs
}

//###############################
//#          TOPICS             #
//###############################
#[server]
pub async fn delete_topic(id: i32) -> Result<String, ServerFnError> {
    let name = crate::backend::db::ProblemDatabase::delete_topic(id).await?;
    Ok(name)
}

#[server(input = Json, output = Json)]
pub async fn set_chapter_topics(
    chapter_id: i32,
    topics: Vec<TopicData>,
) -> Result<(), ServerFnError> {
    let ids: Vec<i32> = topics.iter().map(|to| to.id).collect();
    let data = crate::backend::db::ProblemDatabase::update_chapter_topics(chapter_id, ids).await?;
    Ok(data)
}

#[server]
pub async fn load_chapter_topics(chapter_id: i32) -> Result<Vec<TopicData>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_chapter_topics(chapter_id).await?;
    Ok(data)
}

#[server]
pub async fn load_chapter_topic_ids(chapter_id: i32) -> Result<Vec<i32>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_chapter_topics(chapter_id).await?;
    Ok(data.iter().map(|topic| topic.id).collect())
}

#[server]
pub async fn load_all_topic_data() -> Result<Vec<TopicData>, ServerFnError> {
    let topics = crate::backend::db::ProblemDatabase::get_all_topic_data().await?;
    Ok(topics)
}

#[server]
pub async fn load_all_topic_ids() -> Result<Vec<i32>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_all_topic_data().await?;
    let ids = data.into_iter().map(|chapter| chapter.id).collect();
    Ok(ids)
}

#[server(input = Json, output = Json)]
pub async fn load_topics_by_id(ids: Vec<i32>) -> Result<Vec<TopicData>, ServerFnError> {
    let topics = crate::backend::db::ProblemDatabase::get_topics(&ids).await?;
    Ok(topics)
}

#[server]
pub async fn load_topic(id: i32, lang: String) -> Result<ParsedTopicData, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic(id).await?;
    let topic = data.parse(&lang);
    Ok(topic)
}

#[server]
pub async fn load_topic_by_id(
    topic_id: i32,
    lang: String,
) -> Result<ParsedTopicData, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic(topic_id).await?;
    let topic = data.parse(&lang);
    Ok(topic)
}

#[server]
pub async fn load_topic_name(id: i32) -> Result<String, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic(id).await?;
    Ok(data.name)
}

#[server]
pub async fn load_topic_desc(id: i32, lang: String) -> Result<String, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic(id).await?;
    let topic = data.parse(&lang);
    Ok(topic.desc)
}

/// It is important that the descs are sent back in the same order as the ids came in
#[server]
pub async fn load_topic_descs(ids: Vec<i32>, lang: String) -> Result<Vec<String>, ServerFnError> {
    let topics = crate::backend::db::ProblemDatabase::get_topics(&ids).await?;
    let descs: Result<Vec<_>, _> = ids
        .iter()
        .map(|&id| {
            topics
                .iter()
                .find(|chapter| chapter.id == id)
                .ok_or(ServerFnError::ServerError(format!(
                    "Chapter not found with id {id}"
                )))
                .map(|topic| {
                    if lang == String::from("sv") {
                        topic.desc_sv.clone()
                    } else {
                        topic.desc_en.clone()
                    }
                })
        })
        .collect();
    descs
}

//###############################
//#          PROBLEMS           #
//###############################
#[server]
pub async fn delete_problem(id: i32) -> Result<String, ServerFnError> {
    let name = crate::backend::db::ProblemDatabase::delete_problem(id).await?;
    Ok(name)
}
#[server]
pub async fn load_topic_problems(
    topic_id: i32,
    lang: String,
) -> Result<Vec<ParsedProblemData>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic_problems(topic_id).await?;
    let problems = data
        .into_iter()
        .map(|problem| problem.parse(&lang))
        .collect();
    Ok(problems)
}

#[server]
pub async fn load_topic_problem_ids(topic_id: i32) -> Result<Vec<i32>, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_topic_problems(topic_id).await?;
    let ids = data.into_iter().map(|problem| problem.id).collect();
    Ok(ids)
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
) -> Result<ParsedProblemData, ServerFnError> {
    let data = crate::backend::db::ProblemDatabase::get_problem(problem_id).await?;
    let problem = data.parse(&lang);
    Ok(problem)
}

#[server]
pub async fn load_valid_problems(
    topic_ids: Vec<i32>,
    starting_difficulty: Difficulty,
    ending_difficulty: Difficulty,
    lang: String,
) -> Result<Vec<ParsedProblemData>, ServerFnError> {
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
    )
    .await?;
    let problems = data
        .into_iter()
        .map(|problem| problem.parse(&lang))
        .collect();
    Ok(problems)
}

#[server]
pub async fn generate_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    options: shared::DocumentOptions,
) -> Result<Vec<u8>, ServerFnError> {
    let pdf = crate::backend::create_pdf(sets, options).await?;
    Ok(pdf)
}
