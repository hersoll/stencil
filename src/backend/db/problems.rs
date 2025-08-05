use crate::{
    Error, Result,
    backend::db::get_pool,
    shared::{ChapterData, CourseData, ProblemData, TopicData},
};

pub struct ProblemDatabase;

impl ProblemDatabase {
    //###############################
    //#          COURSES            #
    //###############################
    pub async fn get_all_courses() -> Result<Vec<CourseData>> {
        let pool = get_pool();
        sqlx::query_as!(
            CourseData,
            r#" SELECT id, name, desc_sv, desc_en
            FROM courses ORDER BY name"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadCourses {
            error: e.to_string(),
        })
    }

    pub async fn get_all_course_data() -> Result<Vec<CourseData>> {
        let pool = get_pool();
        sqlx::query_as!(
            CourseData,
            r#" SELECT id, name, desc_sv, desc_en
            FROM courses ORDER BY name"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadCourses {
            error: e.to_string(),
        })
    }

    pub async fn get_course(id: i32) -> Result<CourseData> {
        let pool = get_pool();
        sqlx::query_as!(
            CourseData,
            r#"SELECT id, name, desc_sv, desc_en        
            FROM courses
            WHERE id = $1"#,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadCourses {
            error: e.to_string(),
        })
    }

    pub async fn create_course(course: CourseData) -> Result<CourseData> {
        let pool = get_pool();
        sqlx::query_as!(
            CourseData,
            r#"INSERT INTO courses (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id, name, desc_sv, desc_en
        "#,
            course.name,
            course.desc_sv,
            course.desc_en,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToUpdateRow {
            error: e.to_string(),
        })
    }

    pub async fn update_course(course: CourseData) -> Result<CourseData> {
        let pool = get_pool();
        sqlx::query_as!(
            CourseData,
            r#"UPDATE courses SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 RETURNING id, name, desc_sv, desc_en
        "#,
            course.name,
            course.desc_sv,
            course.desc_en,
            course.id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToUpdateRow { error: e.to_string() })
    }

    pub async fn delete_course(id: i32) -> Result<String> {
        let pool = get_pool();
        let result = sqlx::query!(r#"DELETE FROM courses WHERE id = $1 RETURNING name"#, id)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::FailedToUpdateRow {
                error: e.to_string(),
            })?;
        Ok(result.name)
    }

    //###############################
    //#          CHAPTERS           #
    //###############################

    pub async fn get_all_chapter_data() -> Result<Vec<ChapterData>> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterData,
            r#" SELECT id, name, desc_sv, desc_en
            FROM chapters ORDER BY name"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
    }

    pub async fn get_course_chapters(course_id: i32) -> Result<Vec<ChapterData>> {
        let pool = get_pool();
        sqlx::query_as!(
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
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
    }

    pub async fn update_course_chapters(course_id: i32, chapter_ids: Vec<i32>) -> Result<()> {
        let pool = get_pool();
        sqlx::query!(
            r#"DELETE FROM course_chapters WHERE course_id = $1"#,
            course_id,
        )
        .execute(pool)
        .await
        .map_err(|e| Error::FailedToUpdateRow {
            error: e.to_string(),
        })?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|_| Error::FailedToInitializePool)?;
        for (order, chapter_id) in chapter_ids.iter().enumerate() {
            sqlx::query!(r#"INSERT INTO course_chapters (course_id, chapter_id, order_index) VALUES ($1, $2, $3)"#, course_id, chapter_id, (order + 1) as i32)
                .execute(&mut *tx)
                .await
                .map_err(|e| Error::FailedToUpdateRow {
                    error: e.to_string(),
                })?;
        }

        tx.commit()
            .await
            .map_err(|_| Error::FailedToInitializePool)?;

        Ok(())
    }

    pub async fn get_chapter(id: i32) -> Result<ChapterData> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterData,
            r#"SELECT id, name, desc_sv, desc_en
                FROM chapters
                WHERE id = $1"#,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
    }

    pub async fn get_chapters(ids: &Vec<i32>) -> Result<Vec<ChapterData>> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterData,
            r#"SELECT id, name, desc_sv, desc_en FROM chapters
            WHERE id = ANY($1)"#,
            ids,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
    }

    pub async fn create_chapter(chapter: ChapterData) -> Result<ChapterData> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterData,
            r#"INSERT INTO chapters (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id, name, desc_sv, desc_en
        "#,
            chapter.name,
            chapter.desc_sv,
            chapter.desc_en,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToUpdateRow {
            error: e.to_string(),
        })
    }

    pub async fn update_chapter(chapter: ChapterData) -> Result<ChapterData> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterData,
            r#"UPDATE chapters SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 RETURNING id, name, desc_sv, desc_en
        "#,
            chapter.name,
            chapter.desc_sv,
            chapter.desc_en,
            chapter.id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToUpdateRow { error: e.to_string() })
    }
    pub async fn delete_chapter(id: i32) -> Result<String> {
        let pool = get_pool();
        let result = sqlx::query!(r#"DELETE FROM chapters WHERE id = $1 RETURNING name"#, id)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::FailedToUpdateRow {
                error: e.to_string(),
            })?;
        Ok(result.name)
    }

    //###############################
    //#          TOPICS             #
    //###############################

    pub async fn get_all_topic_data() -> Result<Vec<TopicData>> {
        let pool = get_pool();
        sqlx::query_as!(
            TopicData,
            r#" SELECT id, name, desc_sv, desc_en
            FROM topics ORDER BY name"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }

    pub async fn get_chapter_topics(chapter_id: i32) -> Result<Vec<TopicData>> {
        let pool = get_pool();
        sqlx::query_as!(
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
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }

    pub async fn update_chapter_topics(chapter_id: i32, topic_ids: Vec<i32>) -> Result<()> {
        let pool = get_pool();
        sqlx::query!(
            r#"DELETE FROM chapter_topics WHERE chapter_id = $1"#,
            chapter_id,
        )
        .execute(pool)
        .await
        .map_err(|e| Error::FailedToUpdateRow {
            error: e.to_string(),
        })?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|_| Error::FailedToInitializePool)?;
        for (order, topic_id) in topic_ids.iter().enumerate() {
            sqlx::query!(r#"INSERT INTO chapter_topics (chapter_id, topic_id, order_index) VALUES ($1, $2, $3)"#, chapter_id, topic_id, (order + 1) as i32)
                .execute(&mut *tx)
                .await
                .map_err(|e| Error::FailedToUpdateRow {
                    error: e.to_string(),
                })?;
        }

        tx.commit()
            .await
            .map_err(|_| Error::FailedToInitializePool)?;

        Ok(())
    }

    pub async fn get_topic(id: i32) -> Result<TopicData> {
        let pool = get_pool();
        sqlx::query_as!(
            TopicData,
            r#"SELECT id, name,  desc_sv, desc_en
                FROM topics
                WHERE id = $1"#,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }

    pub async fn get_topics(ids: &Vec<i32>) -> Result<Vec<TopicData>> {
        let pool = get_pool();
        sqlx::query_as!(
            TopicData,
            r#"SELECT id, name, desc_sv, desc_en 
                FROM topics
                WHERE id = ANY($1)"#,
            &ids,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }

    pub async fn delete_topic(id: i32) -> Result<String> {
        let pool = get_pool();
        let result = sqlx::query!(r#"DELETE FROM topics WHERE id = $1 RETURNING name"#, id)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::FailedToUpdateRow {
                error: e.to_string(),
            })?;
        Ok(result.name)
    }

    //###############################
    //#          PROBLEMS           #
    //###############################

    pub async fn get_all_problem_data() -> Result<Vec<ProblemData>> {
        let pool = get_pool();
        sqlx::query_as!(
            ProblemData,
            r#" SELECT id, name, difficulty, desc_sv, desc_en, question_sv, question_en,
            answer_sv, answer_en, solution_sv, solution_en, prefix_id, module
            FROM problems ORDER BY difficulty"#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadProblems {
            error: e.to_string(),
        })
    }

    /// For PDF generation, we need the full names (module+problem) of all the problems
    pub async fn get_problem_names_for_pdf(
        topic_ids: Vec<i32>,
        exclusions: Vec<i32>,
    ) -> Result<Vec<String>> {
        let pool = get_pool();
        let problems = sqlx::query!(
            r#"SELECT p.module, p.name 
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = ANY($1)
          AND NOT p.id = ANY($2)"#,
            &topic_ids,
            &exclusions
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadProblems {
            error: e.to_string(),
        })?;
        Ok(problems
            .iter()
            .map(|record| record.module.clone() + "_" + &record.name)
            .collect())
    }

    pub async fn get_topic_problems(topic_id: i32) -> Result<Vec<ProblemData>> {
        let pool = get_pool();
        sqlx::query_as!(
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
        .map_err(|e| Error::FailedToLoadProblems {
            error: e.to_string(),
        })
    }

    pub async fn update_topic_problems(topic_id: i32, problem_ids: Vec<i32>) -> Result<()> {
        let pool = get_pool();
        sqlx::query!(
            r#"DELETE FROM topic_problems WHERE topic_id = $1"#,
            topic_id,
        )
        .execute(pool)
        .await
        .map_err(|e| Error::FailedToUpdateRow {
            error: e.to_string(),
        })?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|_| Error::FailedToInitializePool)?;
        for (order, problem_id) in problem_ids.iter().enumerate() {
            sqlx::query!(r#"INSERT INTO topic_problems (topic_id, problem_id, order_index) VALUES ($1, $2, $3)"#, topic_id, problem_id, (order + 1) as i32)
                .execute(&mut *tx)
                .await
                .map_err(|e| Error::FailedToUpdateRow {
                    error: e.to_string(),
                })?;
        }

        tx.commit()
            .await
            .map_err(|_| Error::FailedToInitializePool)?;

        Ok(())
    }

    pub async fn get_topic_problems_in_difficulty_range(
        topic_ids: Vec<i32>,
        starting_difficulty: i32,
        ending_difficulty: i32,
    ) -> Result<Vec<ProblemData>> {
        let pool = get_pool();
        sqlx::query_as!(
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
        .await
        .map_err(|e| Error::FailedToLoadProblems {
            error: e.to_string(),
        })
    }

    pub async fn get_problem(id: i32) -> Result<ProblemData> {
        let pool = get_pool();
        sqlx::query_as!(
            ProblemData,
            r#"SELECT id, name, difficulty,  desc_sv, desc_en, module, 
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id
                FROM problems
                WHERE id = $1"#,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadProblems {
            error: e.to_string(),
        })
    }

    pub async fn delete_problem(id: i32) -> Result<String> {
        let pool = get_pool();
        let result = sqlx::query!(r#"DELETE FROM problems WHERE id = $1 RETURNING name"#, id)
            .fetch_one(pool)
            .await
            .map_err(|e| Error::FailedToUpdateRow {
                error: e.to_string(),
            })?;
        Ok(result.name)
    }
}
