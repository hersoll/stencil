use std::collections::HashMap;

use crate::{
    Error, Result,
    shared::{ChapterInfo, CourseInfo, ProblemInfo, TopicInfo},
};
use sqlx::{Pool, Postgres};

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_i18n(&self, lang: &str) -> Result<HashMap<String, String>> {
        let data = sqlx::query!(
            "SELECT key, CASE 
                            WHEN $1 = 'sv' THEN sv
                            WHEN $1 = 'en' THEN en
                            ELSE sv
                        END as value
            FROM i18n", lang). fetch_all(&self.pool).await.map_err(|_| Error::FailedToLoadTranslations)?;
        let map = data.into_iter().map(|row| (row.key, row.value.unwrap_or_default())).collect();

        Ok(map)
    }

    pub async fn get_all_courses(&self, lang: &str) -> Result<Vec<CourseInfo>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::FailedToLoadCourses {
            error: e.to_string(),
        })
    }

    pub async fn get_course_chapters(
        &self,
        course_id: i32,
        lang: &str,
    ) -> Result<Vec<ChapterInfo>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::FailedToLoadChapters {
            error: e.to_string(),
        })
    }

    pub async fn get_chapter_topics(&self, chapter_id: i32, lang: &str) -> Result<Vec<TopicInfo>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }

    pub async fn get_topic_problems(&self, topic_id: i32, lang: &str) -> Result<Vec<ProblemInfo>> {
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::FailedToLoadTopics {
            error: e.to_string(),
        })
    }
}
