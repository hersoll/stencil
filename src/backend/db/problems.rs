use crate::{
    backend::db::get_pool,
    shared::{
        ChapterData, ChapterInfo, CourseData, CourseInfo, ProblemData, ProblemInfo, TopicData,
        TopicInfo,
    },
    Error, Result,
};

pub struct ProblemDatabase;

impl ProblemDatabase {
    //###############################
    //#          COURSES            #
    //###############################
    pub async fn get_all_courses(lang: &str) -> Result<Vec<CourseInfo>> {
        let pool = get_pool();
        sqlx::query_as!(
            CourseInfo,
            r#" SELECT id, name, COALESCE( CASE 
                                                WHEN $1 = 'sv' THEN desc_sv
                                                WHEN $1 = 'en' THEN desc_en  
                                                ELSE desc_sv
                                            END,
                'No description') as "desc!"
            FROM courses ORDER BY name"#,
            lang
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

    pub async fn get_course(id: i32, lang: &str) -> Result<CourseInfo> {
        let pool = get_pool();
        sqlx::query_as!(
            CourseInfo,
            r#"SELECT id, name, 
                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN desc_sv
                            WHEN $1 = 'en' THEN desc_en  
                            ELSE desc_sv
                        END,
                'No description') as "desc!"

        FROM courses
        WHERE id = $2"#,
            lang,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadCourses {
            error: e.to_string(),
        })
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

    pub async fn get_course_chapters(course_id: i32, lang: &str) -> Result<Vec<ChapterInfo>> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterInfo,
            r#"SELECT ch.id, ch.name,
                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN ch.desc_sv
                            WHEN $1 = 'en' THEN ch.desc_en  
                            ELSE ch.desc_sv
                        END,
                'No description') as "desc!"
        FROM chapters ch
        JOIN course_chapters cc ON ch.id = cc.chapter_id
        WHERE cc.course_id = $2
        ORDER BY cc.order_index, ch.name"#,
            lang,
            course_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
    }

    pub async fn get_chapter(id: i32, lang: &str) -> Result<ChapterInfo> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterInfo,
            r#"SELECT id, name, 
                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN desc_sv
                            WHEN $1 = 'en' THEN desc_en  
                            ELSE desc_sv
                        END,
                'No description') as "desc!"

        FROM chapters
        WHERE id = $2"#,
            lang,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
    }

    pub async fn get_chapters(ids: &Vec<i32>, lang: &str) -> Result<Vec<ChapterInfo>> {
        let pool = get_pool();
        sqlx::query_as!(
            ChapterInfo,
            r#"SELECT id, name, 
                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN desc_sv
                            WHEN $1 = 'en' THEN desc_en  
                            ELSE desc_sv
                        END,
                'No description') as "desc!"

        FROM chapters
        WHERE id = ANY($2)"#,
            lang,
            ids,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
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

    pub async fn get_chapter_topics(chapter_id: i32, lang: &str) -> Result<Vec<TopicInfo>> {
        let pool = get_pool();
        sqlx::query_as!(
            TopicInfo,
            r#"SELECT t.id, t.name,
                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN t.desc_sv
                            WHEN $1 = 'en' THEN t.desc_en  
                            ELSE t.desc_sv
                        END,
                'No description') as "desc!"
        FROM topics t
        JOIN chapter_topics ct ON t.id = ct.topic_id
        WHERE ct.chapter_id = $2
        ORDER BY ct.order_index, t.name"#,
            lang,
            chapter_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }

    pub async fn get_topic(id: i32, lang: &str) -> Result<TopicInfo> {
        let pool = get_pool();
        sqlx::query_as!(
            TopicInfo,
            r#"SELECT id, name, 
                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN desc_sv
                            WHEN $1 = 'en' THEN desc_en  
                            ELSE desc_sv
                        END,
                'No description') as "desc!"

        FROM topics
        WHERE id = $2"#,
            lang,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }

    pub async fn get_topics(ids: &Vec<i32>, lang: &str) -> Result<Vec<TopicInfo>> {
        let pool = get_pool();
        sqlx::query_as!(
            TopicInfo,
            r#"SELECT id, name, 
                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN desc_sv
                            WHEN $1 = 'en' THEN desc_en  
                            ELSE desc_sv
                        END,
                'No description') as "desc!"

        FROM topics
        WHERE id = ANY($2)"#,
            lang,
            &ids,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
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

    pub async fn get_topic_problems(topic_id: i32, lang: &str) -> Result<Vec<ProblemInfo>> {
        let pool = get_pool();
        sqlx::query_as!(
            ProblemInfo,
            r#"SELECT p.id, p.name, p.difficulty, 

                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN p.desc_sv
                            WHEN $1 = 'en' THEN p.desc_en  
                            ELSE p.desc_sv
                        END,
                'No description') as "desc!"

        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = $2
        ORDER BY tp.order_index, p.name"#,
            lang,
            topic_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FailedToLoadProblems {
            error: e.to_string(),
        })
    }

    pub async fn get_topic_problems_in_difficulty_range(
        topic_ids: Vec<i32>,
        starting_difficulty: i32,
        ending_difficulty: i32,
        lang: &str,
    ) -> Result<Vec<ProblemInfo>> {
        let pool = get_pool();
        sqlx::query_as!(
            ProblemInfo,
            r#"SELECT DISTINCT p.id, p.name, p.difficulty, 

                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN p.desc_sv
                            WHEN $1 = 'en' THEN p.desc_en  
                            ELSE p.desc_sv
                        END,
                'No description') as "desc!"

        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = ANY($2)
            AND p.difficulty >= $3
            AND p.difficulty <= $4
        ORDER BY p.difficulty"#,
            lang,
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

    pub async fn get_problem(id: i32, lang: &str) -> Result<ProblemInfo> {
        let pool = get_pool();
        sqlx::query_as!(
            ProblemInfo,
            r#"SELECT id, name, difficulty, 

                COALESCE( CASE 
                            WHEN $1 = 'sv' THEN desc_sv
                            WHEN $1 = 'en' THEN desc_en  
                            ELSE desc_sv
                        END,
                'No description') as "desc!"

        FROM problems
        WHERE id = $2"#,
            lang,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FailedToLoadProblems {
            error: e.to_string(),
        })
    }
}
