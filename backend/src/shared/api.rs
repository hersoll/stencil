use crate::shared::{
    self, ChapterData, CourseData, Difficulty, ParsedChapterData, ParsedCourseData,
    ParsedProblemData, ParsedTopicData, PrefixData, ProblemData, TopicData,
};
use std::collections::HashMap;

#[server]
pub async fn load_translations(lang: String) -> Result<HashMap<String, String>> {
    let data = crate::backend::db::I18nDatabase::get_i18n(&lang).await?;
    Ok(data)
}

//###############################
//#          COURSES            #
//###############################
#[server]
pub async fn set_course(course: CourseData) -> Result<CourseData> {
    let result: CourseData;
    if course.id == 0 {
        result = crate::backend::db::ProblemDatabase::create_course(course).await?;
    } else {
        result = crate::backend::db::ProblemDatabase::update_course(course).await?;
    }
    Ok(result)
}

#[server]
pub async fn delete_course(id: i32) -> Result<String> {
    let name = crate::backend::db::ProblemDatabase::delete_course(id).await?;
    Ok(name)
}

#[server]
pub async fn load_courses(lang: String) -> Result<Vec<ParsedCourseData>> {
    let data = crate::backend::db::ProblemDatabase::get_all_courses().await?;
    let courses = data.into_iter().map(|course| course.parse(&lang)).collect();
    Ok(courses)
}

#[server]
pub async fn load_all_course_data() -> Result<Vec<CourseData>> {
    let courses = crate::backend::db::ProblemDatabase::get_all_course_data().await?;
    Ok(courses)
}

#[server]
pub async fn load_course(id: i32, lang: String) -> Result<ParsedCourseData> {
    let data = crate::backend::db::ProblemDatabase::get_course(id).await?;
    let course = data.parse(&lang);
    Ok(course)
}

#[server]
pub async fn load_course_name(id: i32) -> Result<String> {
    let data = crate::backend::db::ProblemDatabase::get_course(id).await?;
    Ok(data.name)
}

#[server]
pub async fn load_course_desc(id: i32, lang: String) -> Result<String> {
    let data = crate::backend::db::ProblemDatabase::get_course(id).await?;
    let course = data.parse(&lang);
    Ok(course.desc)
}

//###############################
//#          CHAPTERS           #
//###############################
#[server]
pub async fn set_chapter(chapter: ChapterData) -> Result<ChapterData> {
    let result: ChapterData;
    if chapter.id == 0 {
        result = crate::backend::db::ProblemDatabase::create_chapter(chapter).await?;
    } else {
        result = crate::backend::db::ProblemDatabase::update_chapter(chapter).await?;
    }
    Ok(result)
}

#[server]
pub async fn delete_chapter(id: i32) -> Result<String> {
    let name = crate::backend::db::ProblemDatabase::delete_chapter(id).await?;
    Ok(name)
}

#[server]
pub async fn load_course_chapters(course_id: i32) -> Result<Vec<ChapterData>> {
    let data = crate::backend::db::ProblemDatabase::get_course_chapters(course_id).await?;
    Ok(data)
}

#[server(input = Json, output = Json)]
pub async fn set_course_chapters(course_id: i32, chapters: Vec<ChapterData>) -> Result<()> {
    let ids: Vec<i32> = chapters.iter().map(|ch| ch.id).collect();
    let data = crate::backend::db::ProblemDatabase::update_course_chapters(course_id, ids).await?;
    Ok(data)
}

#[server]
pub async fn load_course_chapter_ids(course_id: i32) -> Result<Vec<i32>> {
    let data = crate::backend::db::ProblemDatabase::get_course_chapters(course_id).await?;
    Ok(data.iter().map(|chapter| chapter.id).collect())
}

#[server]
pub async fn load_all_chapter_ids() -> Result<Vec<i32>> {
    let data = crate::backend::db::ProblemDatabase::get_all_chapter_data().await?;
    let ids = data.into_iter().map(|chapter| chapter.id).collect();
    Ok(ids)
}

#[server]
pub async fn load_all_chapter_data() -> Result<Vec<ChapterData>> {
    let chapters = crate::backend::db::ProblemDatabase::get_all_chapter_data().await?;
    Ok(chapters)
}

#[server]
pub async fn load_parsed_chapter(id: i32, lang: String) -> Result<ParsedChapterData> {
    let chapter = crate::backend::db::ProblemDatabase::get_chapter(id).await?;
    Ok(chapter.parse(&lang))
}

#[server(input = Json, output = Json)]
pub async fn load_chapters_by_id(ids: Vec<i32>) -> Result<Vec<ChapterData>> {
    let chapters = crate::backend::db::ProblemDatabase::get_chapters(&ids).await?;
    Ok(chapters)
}

#[server]
pub async fn load_chapter_name(id: i32) -> Result<String> {
    let chapter = crate::backend::db::ProblemDatabase::get_chapter(id).await?;
    Ok(chapter.name)
}

#[server]
pub async fn load_chapter_desc(id: i32, lang: String) -> Result<String> {
    let data = crate::backend::db::ProblemDatabase::get_chapter(id).await?;
    let chapter = data.parse(&lang);
    Ok(chapter.desc)
}

/// It is important that the descs are sent back in the same order as the ids came in
#[server]
pub async fn load_chapter_descs(ids: Vec<i32>, lang: String) -> Result<Vec<String>> {
    let chapters = crate::backend::db::ProblemDatabase::get_chapters(&ids).await?;
    let descs: Result<Vec<_>> = ids
        .iter()
        .map(|&id| {
            let chapter = chapters
                .iter()
                .find(|chapter| chapter.id == id)
                .or_internal_server_error(format!("Chapter not found with id {}", id))?;

            Ok(if lang == "sv" {
                chapter.desc_sv.clone()
            } else {
                chapter.desc_en.clone()
            })
        })
        .collect();
    descs
}

//###############################
//#          TOPICS             #
//###############################
#[server]
pub async fn set_topic(topic: TopicData) -> Result<TopicData> {
    let result: TopicData;
    if topic.id == 0 {
        result = crate::backend::db::ProblemDatabase::create_topic(topic).await?;
    } else {
        result = crate::backend::db::ProblemDatabase::update_topic(topic).await?;
    }
    Ok(result)
}
#[server]
pub async fn delete_topic(id: i32) -> Result<String> {
    let name = crate::backend::db::ProblemDatabase::delete_topic(id).await?;
    Ok(name)
}

#[server(input = Json, output = Json)]
pub async fn set_chapter_topics(chapter_id: i32, topics: Vec<TopicData>) -> Result<()> {
    let ids: Vec<i32> = topics.iter().map(|to| to.id).collect();
    let data = crate::backend::db::ProblemDatabase::update_chapter_topics(chapter_id, ids).await?;
    Ok(data)
}

#[server]
pub async fn load_chapter_topics(chapter_id: i32) -> Result<Vec<TopicData>> {
    let data = crate::backend::db::ProblemDatabase::get_chapter_topics(chapter_id).await?;
    Ok(data)
}

#[server]
pub async fn load_chapter_topic_ids(chapter_id: i32) -> Result<Vec<i32>> {
    let data = crate::backend::db::ProblemDatabase::get_chapter_topics(chapter_id).await?;
    Ok(data.iter().map(|topic| topic.id).collect())
}

#[server]
pub async fn load_all_topic_data() -> Result<Vec<TopicData>> {
    let topics = crate::backend::db::ProblemDatabase::get_all_topic_data().await?;
    Ok(topics)
}

#[server]
pub async fn load_all_topic_ids() -> Result<Vec<i32>> {
    let data = crate::backend::db::ProblemDatabase::get_all_topic_data().await?;
    let ids = data.into_iter().map(|chapter| chapter.id).collect();
    Ok(ids)
}

#[server(input = Json, output = Json)]
pub async fn load_topics_by_id(ids: Vec<i32>) -> Result<Vec<TopicData>> {
    let topics = crate::backend::db::ProblemDatabase::get_topics(&ids).await?;
    Ok(topics)
}

#[server]
pub async fn load_topic(id: i32, lang: String) -> Result<ParsedTopicData> {
    let data = crate::backend::db::ProblemDatabase::get_topic(id).await?;
    let topic = data.parse(&lang);
    Ok(topic)
}

#[server]
pub async fn load_topic_by_id(topic_id: i32, lang: String) -> Result<ParsedTopicData> {
    let data = crate::backend::db::ProblemDatabase::get_topic(topic_id).await?;
    let topic = data.parse(&lang);
    Ok(topic)
}

#[server]
pub async fn load_topic_name(id: i32) -> Result<String> {
    let data = crate::backend::db::ProblemDatabase::get_topic(id).await?;
    Ok(data.name)
}

#[server]
pub async fn load_topic_desc(id: i32, lang: String) -> Result<String> {
    let data = crate::backend::db::ProblemDatabase::get_topic(id).await?;
    let topic = data.parse(&lang);
    Ok(topic.desc)
}

/// It is important that the descs are sent back in the same order as the ids came in
#[server]
pub async fn load_topic_descs(ids: Vec<i32>, lang: String) -> Result<Vec<String>> {
    let topics = crate::backend::db::ProblemDatabase::get_topics(&ids).await?;
    let descs: Result<Vec<_>> = ids
        .iter()
        .map(|&id| {
            let topic = topics
                .iter()
                .find(|chapter| chapter.id == id)
                .or_internal_server_error(format!("No chapter found with id {id}"))?;

            Ok(if lang == String::from("sv") {
                topic.desc_sv.clone()
            } else {
                topic.desc_en.clone()
            })
        })
        .collect();
    descs
}

//###############################
//#          PROBLEMS           #
//###############################
#[server]
pub async fn set_problem(problem: ProblemData) -> Result<i32> {
    let result = if problem.id == 0 {
        crate::backend::db::ProblemDatabase::create_problem(problem).await?
    } else {
        crate::backend::db::ProblemDatabase::update_problem(problem).await?
    };
    Ok(result)
}
#[server]
pub async fn delete_problem(id: i32) -> Result<String> {
    let name = crate::backend::db::ProblemDatabase::delete_problem(id).await?;
    Ok(name)
}
#[server]
pub async fn load_topic_problems(topic_id: i32) -> Result<Vec<ProblemData>> {
    let data = crate::backend::db::ProblemDatabase::get_topic_problems(topic_id).await?;
    Ok(data)
}

#[server(input = Json, output = Json)]
pub async fn set_topic_problems(topic_id: i32, problems: Vec<ProblemData>) -> Result<()> {
    let ids: Vec<i32> = problems.iter().map(|pr| pr.id).collect();
    let data = crate::backend::db::ProblemDatabase::update_topic_problems(topic_id, ids).await?;
    Ok(data)
}

#[server]
pub async fn load_topic_problem_ids(topic_id: i32) -> Result<Vec<i32>> {
    let data = crate::backend::db::ProblemDatabase::get_topic_problems(topic_id).await?;
    let ids = data.into_iter().map(|problem| problem.id).collect();
    Ok(ids)
}

#[server]
pub async fn load_all_problem_data() -> Result<Vec<ProblemData>> {
    let problems = crate::backend::db::ProblemDatabase::get_all_problem_data().await?;
    Ok(problems)
}

#[server]
pub async fn load_all_problem_ids() -> Result<Vec<i32>> {
    let data = crate::backend::db::ProblemDatabase::get_all_problem_data().await?;
    let ids = data.into_iter().map(|problem| problem.id).collect();
    Ok(ids)
}

#[server]
pub async fn load_problem_by_id(problem_id: i32, lang: String) -> Result<ParsedProblemData> {
    let data = crate::backend::db::ProblemDatabase::get_problem(problem_id).await?;
    let problem = data.parse(&lang);
    Ok(problem)
}

#[server(input = Json, output = Json)]
pub async fn load_problems_by_id(ids: Vec<i32>) -> Result<Vec<ProblemData>> {
    let problems = crate::backend::db::ProblemDatabase::get_problems(&ids).await?;
    Ok(problems)
}

#[server]
pub async fn load_valid_problems(
    topic_ids: Vec<i32>,
    starting_difficulty: Difficulty,
    ending_difficulty: Difficulty,
    lang: String,
) -> Result<Vec<ParsedProblemData>> {
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
pub async fn load_all_prefix_data() -> Result<Vec<PrefixData>> {
    let prefixes = crate::backend::db::ProblemDatabase::get_all_prefix_data().await?;
    Ok(prefixes)
}

#[server]
pub async fn set_prefix(prefix: PrefixData) -> Result<i32> {
    let result = if prefix.id == 0 {
        crate::backend::db::ProblemDatabase::create_prefix(prefix).await?
    } else {
        crate::backend::db::ProblemDatabase::update_prefix(prefix).await?
    };
    Ok(result)
}
// PDF

#[server]
pub async fn generate_pdf(
    sets: Vec<shared::SendableProblemSetData>,
    options: shared::DocumentOptions,
) -> Result<Vec<u8>> {
    let pdf = crate::backend::create_pdf(sets, options).await?;
    Ok(pdf)
}
