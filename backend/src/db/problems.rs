use crate::{
    db,
    shared::{ChapterData, CourseData, PrefixData, ProblemData, TopicData},
};
use anyhow::{Context, Result};
use sqlx::PgPool;

/// Generic trait for handling methods that update relationships
/// (linking tables).
trait Relationship {
    const TABLE_NAME: &'static str;
    const PARENT_COLUMN: &'static str;
    const CHILD_COLUMN: &'static str;
}

struct CourseChapters;
impl Relationship for CourseChapters {
    const TABLE_NAME: &'static str = "course_chapters";
    const PARENT_COLUMN: &'static str = "course_id";
    const CHILD_COLUMN: &'static str = "chapter_id";
}

struct ChapterTopics;
impl Relationship for ChapterTopics {
    const TABLE_NAME: &'static str = "chapter_topics";
    const PARENT_COLUMN: &'static str = "chapter_id";
    const CHILD_COLUMN: &'static str = "topic_id";
}

struct TopicProblems;
impl Relationship for TopicProblems {
    const TABLE_NAME: &'static str = "topic_problems";
    const PARENT_COLUMN: &'static str = "topic_id";
    const CHILD_COLUMN: &'static str = "problem_id";
}

/// Generic helper for updating many-to-many relationships with ordering
async fn update_relationships<R: Relationship>(
    pool: &PgPool,
    parent_id: i32,
    child_ids: &[i32],
) -> Result<()> {
    // Clear existing relationships
    let delete_query = format!(
        "DELETE FROM {} WHERE {} = $1",
        R::TABLE_NAME,
        R::PARENT_COLUMN
    );
    sqlx::query(&delete_query)
        .bind(parent_id)
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to clear relationships in {} for {} = {}",
                R::TABLE_NAME,
                R::PARENT_COLUMN,
                parent_id
            )
        })?;

    // Start transaction for bulk insert
    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    // Insert new relationships with ordering
    let insert_query = format!(
        "INSERT INTO {} ({}, {}, order_index) VALUES ($1, $2, $3)",
        R::TABLE_NAME,
        R::PARENT_COLUMN,
        R::CHILD_COLUMN
    );

    for (order, child_id) in child_ids.iter().enumerate() {
        sqlx::query(&insert_query)
            .bind(parent_id)
            .bind(child_id)
            .bind((order + 1) as i32)
            .execute(&mut *tx)
            .await
            .with_context(|| {
                format!(
                    "Failed to add relationship: {} {} -> {} {}",
                    R::PARENT_COLUMN,
                    parent_id,
                    R::CHILD_COLUMN,
                    child_id
                )
            })?;
    }

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}

//###############################
//#          COURSES            #
//###############################
pub async fn get_all_courses() -> Result<Vec<CourseData>> {
    let pool = db::get_pool();
    let courses = sqlx::query_as!(
        CourseData,
        r#"SELECT id, name, desc_sv, desc_en
            FROM courses ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(courses)
}

pub async fn get_all_course_data() -> Result<Vec<CourseData>> {
    let pool = db::get_pool();

    let course_data = sqlx::query_as!(
        CourseData,
        r#"SELECT id, name, desc_sv, desc_en
            FROM courses ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(course_data)
}

pub async fn get_course(id: i32) -> Result<CourseData> {
    let pool = db::get_pool();
    let course = sqlx::query_as!(
        CourseData,
        r#"SELECT id, name, desc_sv, desc_en        
            FROM courses
            WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to get course with id {id}"))?;

    Ok(course)
}

pub async fn create_course(course: CourseData) -> Result<CourseData> {
    let pool = db::get_pool();
    let created = sqlx::query_as!(
            CourseData,
            r#"INSERT INTO courses (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id, name, desc_sv, desc_en"#,
            course.name,
            course.desc_sv,
            course.desc_en,
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to create course '{}'", course.name))?;

    Ok(created)
}

pub async fn update_course(course: CourseData) -> Result<CourseData> {
    let pool = db::get_pool();
    let updated = sqlx::query_as!(
            CourseData,
            r#"UPDATE courses SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 RETURNING id, name, desc_sv, desc_en"#,
            course.name,
            course.desc_sv,
            course.desc_en,
            course.id
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to update course with id {}", course.id))?;

    Ok(updated)
}

pub async fn delete_course(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM courses WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to delete course with id {id}"))?;

    Ok(result.name)
}

//###############################
//#          CHAPTERS           #
//###############################

pub async fn get_all_chapter_data() -> Result<Vec<ChapterData>> {
    let pool = db::get_pool();
    let chapter_data = sqlx::query_as!(
        ChapterData,
        r#"SELECT id, name, desc_sv, desc_en
            FROM chapters ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(chapter_data)
}

pub async fn get_course_chapters(course_id: i32) -> Result<Vec<ChapterData>> {
    let pool = db::get_pool();
    let chapters = sqlx::query_as!(
        ChapterData,
        r#"SELECT ch.id, ch.name, ch.desc_sv, ch.desc_en
        FROM chapters ch
        JOIN course_chapters cc ON ch.id = cc.chapter_id
        WHERE cc.course_id = $1
        ORDER BY cc.order_index, ch.name"#,
        course_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters for course {course_id}"))?;

    Ok(chapters)
}

pub async fn update_course_chapters(course_id: i32, chapter_ids: Vec<i32>) -> Result<()> {
    let pool = db::get_pool();
    update_relationships::<CourseChapters>(pool, course_id, &chapter_ids).await
}

pub async fn get_chapter(id: i32) -> Result<ChapterData> {
    let pool = db::get_pool();
    let chapter = sqlx::query_as!(
        ChapterData,
        r#"SELECT id, name, desc_sv, desc_en
                FROM chapters
                WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to get chapter with id {id}"))?;

    Ok(chapter)
}

pub async fn get_chapters(ids: &[i32]) -> Result<Vec<ChapterData>> {
    let pool = db::get_pool();
    let chapters = sqlx::query_as!(
        ChapterData,
        r#"SELECT id, name, desc_sv, desc_en FROM chapters
            WHERE id = ANY($1)
            ORDER BY name"#,
        ids,
    )
    .fetch_all(pool)
    .await?;

    Ok(chapters)
}

pub async fn create_chapter(chapter: ChapterData) -> Result<ChapterData> {
    let pool = db::get_pool();
    let created = sqlx::query_as!(
            ChapterData,
            r#"INSERT INTO chapters (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id, name, desc_sv, desc_en"#,
            chapter.name,
            chapter.desc_sv,
            chapter.desc_en,
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to create chapter '{}'", chapter.name))?;

    Ok(created)
}

pub async fn update_chapter(chapter: ChapterData) -> Result<ChapterData> {
    let pool = db::get_pool();
    let updated = sqlx::query_as!(
            ChapterData,
            r#"UPDATE chapters SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 RETURNING id, name, desc_sv, desc_en"#,
            chapter.name,
            chapter.desc_sv,
            chapter.desc_en,
            chapter.id
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to update chapter with id {}", chapter.id))?;

    Ok(updated)
}

pub async fn delete_chapter(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM chapters WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to delete chapter with id {id}"))?;

    Ok(result.name)
}

//###############################
//#          TOPICS             #
//###############################

pub async fn create_topic(topic: TopicData) -> Result<TopicData> {
    let pool = db::get_pool();
    let created = sqlx::query_as!(
            TopicData,
            r#"INSERT INTO topics (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id, name, desc_sv, desc_en"#,
            topic.name,
            topic.desc_sv,
            topic.desc_en,
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to create topic '{}'", topic.name))?;

    Ok(created)
}

pub async fn update_topic(topic: TopicData) -> Result<TopicData> {
    let pool = db::get_pool();
    let updated = sqlx::query_as!(
            TopicData,
            r#"UPDATE topics SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 RETURNING id, name, desc_sv, desc_en"#,
            topic.name,
            topic.desc_sv,
            topic.desc_en,
            topic.id
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to update topic with id {}", topic.id))?;

    Ok(updated)
}

pub async fn get_all_topic_data() -> Result<Vec<TopicData>> {
    let pool = db::get_pool();
    let topics = sqlx::query_as!(
        TopicData,
        r#"SELECT id, name, desc_sv, desc_en
            FROM topics ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(topics)
}

pub async fn get_chapter_topics(chapter_id: i32) -> Result<Vec<TopicData>> {
    let pool = db::get_pool();
    let topics = sqlx::query_as!(
        TopicData,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en
        FROM topics t
        JOIN chapter_topics ct ON t.id = ct.topic_id
        WHERE ct.chapter_id = $1
        ORDER BY ct.order_index, t.name"#,
        chapter_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get topics for chapter {chapter_id}"))?;

    Ok(topics)
}

pub async fn update_chapter_topics(chapter_id: i32, topic_ids: Vec<i32>) -> Result<()> {
    let pool = db::get_pool();
    update_relationships::<ChapterTopics>(pool, chapter_id, &topic_ids).await
}

pub async fn get_topic(id: i32) -> Result<TopicData> {
    let pool = db::get_pool();
    let topic = sqlx::query_as!(
        TopicData,
        r#"SELECT id, name, desc_sv, desc_en
                FROM topics
                WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to get topic with id {id}"))?;

    Ok(topic)
}

pub async fn get_topics(ids: &[i32]) -> Result<Vec<TopicData>> {
    let pool = db::get_pool();
    let topics = sqlx::query_as!(
        TopicData,
        r#"SELECT id, name, desc_sv, desc_en 
                FROM topics
                WHERE id = ANY($1)
                ORDER BY name"#,
        ids,
    )
    .fetch_all(pool)
    .await?;

    Ok(topics)
}

pub async fn delete_topic(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM topics WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to delete topic with id {id}"))?;

    Ok(result.name)
}

//###############################
//#          PROBLEMS           #
//###############################

pub async fn create_problem(problem: ProblemData) -> Result<i32> {
    let pool = db::get_pool();
    let result = sqlx::query!(
        r#"INSERT INTO problems (name, desc_sv, desc_en, difficulty, module,
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) 
            RETURNING id"#,
        problem.name,
        problem.desc_sv,
        problem.desc_en,
        problem.difficulty,
        problem.module,
        problem.question_sv,
        problem.question_en,
        problem.answer_sv,
        problem.answer_en,
        problem.solution_sv,
        problem.solution_en,
        problem.prefix_id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to create problem '{}'", problem.name))?;

    Ok(result.id)
}

pub async fn update_problem(problem: ProblemData) -> Result<i32> {
    let pool = db::get_pool();
    let result = sqlx::query!(
            r#"UPDATE problems SET name = $2, difficulty = $12, desc_sv = $3, desc_en = $4, module = $5,
            question_sv = $6, question_en = $7, answer_sv = $8, answer_en = $9, solution_sv = $10, solution_en = $11, prefix_id = $13
            WHERE id = $1
            RETURNING id"#,
            problem.id,
            problem.name,
            problem.desc_sv,
            problem.desc_en,
            problem.module,
            problem.question_sv,
            problem.question_en,
            problem.answer_sv,
            problem.answer_en,
            problem.solution_sv,
            problem.solution_en,
            problem.difficulty,
            problem.prefix_id,
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to update problem with id {}", problem.id))?;

    Ok(result.id)
}

pub async fn get_all_problem_data() -> Result<Vec<ProblemData>> {
    let pool = db::get_pool();
    let problems = sqlx::query_as!(
        ProblemData,
        r#"SELECT id, name, difficulty, desc_sv, desc_en, question_sv, question_en,
            answer_sv, answer_en, solution_sv, solution_en, prefix_id, module
            FROM problems ORDER BY module"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(problems)
}

/// For PDF generation, we need the full names (module+problem) of all the problems
pub async fn get_problem_names_and_difficulties_from_topics(
    topic_ids: Vec<i32>,
    exclusions: Vec<i32>,
) -> Result<Vec<(String, u8)>> {
    let pool = db::get_pool();
    let problems = sqlx::query!(
        r#"SELECT p.module, p.name, p.difficulty 
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = ANY($1)
          AND NOT p.id = ANY($2)"#,
        &topic_ids,
        &exclusions
    )
    .fetch_all(pool)
    .await?;

    Ok(problems
        .iter()
        .map(|record| {
            (
                format!("{}_{}", record.module, record.name),
                record.difficulty as u8,
            )
        })
        .collect())
}

pub async fn get_topic_problems(topic_id: i32) -> Result<Vec<ProblemData>> {
    let pool = db::get_pool();
    let problems = sqlx::query_as!(
            ProblemData,
            r#"SELECT p.id, p.name, p.difficulty, p.desc_sv, p.desc_en, p.module, 
            p.question_sv, p.question_en, p.answer_sv, p.answer_en, p.solution_sv, p.solution_en, p.prefix_id
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = $1
        ORDER BY tp.order_index, p.name"#,
            topic_id
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to get problems for topic {topic_id}"))?;

    Ok(problems)
}

pub async fn update_topic_problems(topic_id: i32, problem_ids: Vec<i32>) -> Result<()> {
    let pool = db::get_pool();
    update_relationships::<TopicProblems>(pool, topic_id, &problem_ids).await
}

pub async fn get_topic_problems_in_difficulty_range(
    topic_ids: Vec<i32>,
    starting_difficulty: i32,
    ending_difficulty: i32,
) -> Result<Vec<ProblemData>> {
    let pool = db::get_pool();
    let problems = sqlx::query_as!(
            ProblemData,
            r#"SELECT DISTINCT p.id, p.name, p.difficulty, p.desc_sv, p.desc_en, p.module, 
            p.question_sv, p.question_en, p.answer_sv, p.answer_en, p.solution_sv, p.solution_en, p.prefix_id
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = ANY($1)
            AND p.difficulty >= $2
            AND p.difficulty <= $3
        ORDER BY p.difficulty"#,
            &topic_ids,
            starting_difficulty,
            ending_difficulty
        )
        .fetch_all(pool)
        .await?;

    Ok(problems)
}

pub async fn get_problem(id: i32) -> Result<ProblemData> {
    let pool = db::get_pool();
    let problem = sqlx::query_as!(
        ProblemData,
        r#"SELECT id, name, difficulty, desc_sv, desc_en, module, 
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id
                FROM problems
                WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to get problem with id {id}"))?;

    Ok(problem)
}

pub async fn get_problems(ids: &[i32]) -> Result<Vec<ProblemData>> {
    let pool = db::get_pool();
    let problems = sqlx::query_as!(
        ProblemData,
        r#"SELECT id, name, difficulty, desc_sv, desc_en, module, 
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id
                FROM problems
            WHERE id = ANY($1)
            ORDER BY module"#,
        ids,
    )
    .fetch_all(pool)
    .await?;

    Ok(problems)
}

pub async fn delete_problem(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM problems WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to delete problem with id {id}"))?;

    Ok(result.name)
}

//###############################
//#          PREFIXES           #
//###############################

pub async fn get_all_prefix_data() -> Result<Vec<PrefixData>> {
    let pool = db::get_pool();
    let prefixes = sqlx::query_as!(
        PrefixData,
        r#"SELECT id, name, text_sv, text_en, group_text_sv, group_text_en FROM prefixes"#
    )
    .fetch_all(pool)
    .await?;

    Ok(prefixes)
}

pub async fn create_prefix(prefix: PrefixData) -> Result<i32> {
    let pool = db::get_pool();
    let result = sqlx::query!(
        r#"INSERT INTO prefixes (name, text_sv, text_en, group_text_sv, group_text_en) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING id"#,
        prefix.name,
        prefix.text_sv,
        prefix.text_en,
        prefix.group_text_sv,
        prefix.group_text_en
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to create prefix '{}'", prefix.name))?;

    Ok(result.id)
}

pub async fn update_prefix(prefix: PrefixData) -> Result<i32> {
    let pool = db::get_pool();
    let result = sqlx::query!(
            r#"UPDATE prefixes SET name = $2, text_sv = $3, text_en = $4, group_text_sv = $5, group_text_en = $6 
            WHERE id = $1
            RETURNING id"#,
            prefix.id,
            prefix.name,
            prefix.text_sv,
            prefix.text_en,
            prefix.group_text_sv,
            prefix.group_text_en
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to update prefix with id {}", prefix.id))?;

    Ok(result.id)
}
