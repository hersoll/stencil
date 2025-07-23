// Updated populator for your JSON structure
use sqlx::{PgPool, Pool, Postgres};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Your existing structs (keeping them as-is)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemRegistry {
    pub courses: Vec<CourseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub chapters: Vec<ChapterData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub topics: Vec<TopicData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub problems: Vec<ProblemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemData {
    pub name: String,
    pub desc: HashMap<String, String>,
    #[serde(default)]
    pub question: HashMap<String, String>,
    #[serde(default)]
    pub answer: HashMap<String, String>,
    #[serde(default)]
    pub solution: HashMap<String, String>,
    #[serde(default)]
    pub difficulty: Option<u8>,
    #[serde(default)]
    pub prefix: Option<HashMap<String, String>>,
    #[serde(default)]
    pub group_prefix: Option<HashMap<String, String>>,
}

pub struct DatabasePopulator {
    pool: PgPool,
}

impl DatabasePopulator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn populate_from_json_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = std::fs::read_to_string(file_path)?; // Use std::fs instead of tokio::fs
        let registry: ProblemRegistry = serde_json::from_str(&json_content)?;
        self.populate_from_registry(registry).await
    }

    pub async fn populate_from_json_string(&self, json_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let registry: ProblemRegistry = serde_json::from_str(json_str)?;
        self.populate_from_registry(registry).await
    }

    async fn populate_from_registry(&self, registry: ProblemRegistry) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;

        // Clear existing data
        sqlx::query("DELETE FROM topic_problems").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM chapter_topics").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM course_chapters").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM problems").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM topics").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM chapters").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM courses").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM prefixes").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM group_prefixes").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM i18n").execute(&mut *tx).await?;

        // Reset sequences
        sqlx::query("ALTER SEQUENCE courses_id_seq RESTART WITH 1").execute(&mut *tx).await?;
        sqlx::query("ALTER SEQUENCE chapters_id_seq RESTART WITH 1").execute(&mut *tx).await?;
        sqlx::query("ALTER SEQUENCE topics_id_seq RESTART WITH 1").execute(&mut *tx).await?;
        sqlx::query("ALTER SEQUENCE problems_id_seq RESTART WITH 1").execute(&mut *tx).await?;
        sqlx::query("ALTER SEQUENCE prefixes_id_seq RESTART WITH 1").execute(&mut *tx).await?;
        sqlx::query("ALTER SEQUENCE group_prefixes_id_seq RESTART WITH 1").execute(&mut *tx).await?;

        // Only populate i18n for UI text (buttons, labels, etc.)
        // Add some common UI translations
        let ui_translations = vec![
            ("welcome", "Välkommen", "Welcome"),
            ("course", "Kurs", "Course"),
            ("chapter", "Kapitel", "Chapter"), 
            ("topic", "Ämne", "Topic"),
            ("problem", "Problem", "Problem"),
            ("difficulty", "Svårighet", "Difficulty"),
            ("question", "Fråga", "Question"),
            ("answer", "Svar", "Answer"),
            ("solution", "Lösning", "Solution"),
        ];

        for (key, sv, en) in ui_translations {
            sqlx::query!(
                "INSERT INTO i18n (key, sv, en) VALUES ($1, $2, $3)",
                key, Some(sv), Some(en)
            ).execute(&mut *tx).await?;
        }

        println!("Inserted {} UI translations", 9);

        // Collect and insert prefixes and group_prefixes
        let mut prefixes = HashMap::new();
        let mut group_prefixes = HashMap::new();
        
        // Collect unique prefixes from all problems
        for course in &registry.courses {
            for chapter in &course.chapters {
                for topic in &chapter.topics {
                    for problem in &topic.problems {
                        // Collect prefixes
                        if let Some(prefix_map) = &problem.prefix {
                            // Use the name as key, or generate one from content
                            let prefix_key = format!("prefix_{}", prefixes.len() + 1);
                            prefixes.insert(prefix_key, prefix_map.clone());
                        }
                        
                        // Collect group_prefixes
                        if let Some(group_prefix_map) = &problem.group_prefix {
                            let group_prefix_key = format!("group_prefix_{}", group_prefixes.len() + 1);
                            group_prefixes.insert(group_prefix_key, group_prefix_map.clone());
                        }
                    }
                }
            }
        }

        // Insert prefixes
        let mut prefix_ids = HashMap::new();
        for (key, prefix_texts) in &prefixes {
            let prefix_id = sqlx::query_scalar!(
                "INSERT INTO prefixes (name, text_sv, text_en) VALUES ($1, $2, $3) RETURNING id",
                key,
                prefix_texts.get("sv"),
                prefix_texts.get("en")
            ).fetch_one(&mut *tx).await?;
            
            prefix_ids.insert(key.clone(), prefix_id);
        }

        // Insert group_prefixes
        let mut group_prefix_ids = HashMap::new();
        for (key, group_prefix_texts) in &group_prefixes {
            let group_prefix_id = sqlx::query_scalar!(
                "INSERT INTO group_prefixes (name, text_sv, text_en) VALUES ($1, $2, $3) RETURNING id",
                key,
                group_prefix_texts.get("sv"),
                group_prefix_texts.get("en")
            ).fetch_one(&mut *tx).await?;
            
            group_prefix_ids.insert(key.clone(), group_prefix_id);
        }

        println!("Inserted {} prefixes and {} group_prefixes", prefixes.len(), group_prefixes.len());

        // Track entity IDs to handle reuse
        let mut chapter_ids = HashMap::new();
        let mut topic_ids = HashMap::new(); 
        let mut problem_ids = HashMap::new();

        // Process courses
        for course in &registry.courses {
            // Insert course with both languages
            let course_id = sqlx::query_scalar!(
                "INSERT INTO courses (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id",
                course.name,
                course.desc.get("sv"),
                course.desc.get("en")
            ).fetch_one(&mut *tx).await?;

            println!("Inserted course: {} (ID: {})", course.name, course_id);

            // Process chapters
            for (chapter_idx, chapter) in course.chapters.iter().enumerate() {
                let chapter_id = if let Some(&existing_id) = chapter_ids.get(&chapter.name) {
                    existing_id
                } else {
                    let id = sqlx::query_scalar!(
                        "INSERT INTO chapters (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id",
                        chapter.name,
                        chapter.desc.get("sv"),
                        chapter.desc.get("en")
                    ).fetch_one(&mut *tx).await?;
                    
                    chapter_ids.insert(chapter.name.clone(), id);
                    println!("  Inserted chapter: {} (ID: {})", chapter.name, id);
                    id
                };

                // Link course to chapter
                sqlx::query!(
                    "INSERT INTO course_chapters (course_id, chapter_id, order_index) VALUES ($1, $2, $3)
                     ON CONFLICT (course_id, chapter_id) DO NOTHING",
                    course_id,
                    chapter_id,
                    chapter_idx as i32 + 1
                ).execute(&mut *tx).await?;

                // Process topics
                for (topic_idx, topic) in chapter.topics.iter().enumerate() {
                    let topic_id = if let Some(&existing_id) = topic_ids.get(&topic.name) {
                        existing_id
                    } else {
                        let id = sqlx::query_scalar!(
                            "INSERT INTO topics (name, desc_sv, desc_en) VALUES ($1, $2, $3) RETURNING id",
                            topic.name,
                            topic.desc.get("sv"),
                            topic.desc.get("en")
                        ).fetch_one(&mut *tx).await?;
                        
                        topic_ids.insert(topic.name.clone(), id);
                        println!("    Inserted topic: {} (ID: {})", topic.name, id);
                        id
                    };

                    // Link chapter to topic
                    sqlx::query!(
                        "INSERT INTO chapter_topics (chapter_id, topic_id, order_index) VALUES ($1, $2, $3)
                         ON CONFLICT (chapter_id, topic_id) DO NOTHING",
                        chapter_id,
                        topic_id,
                        topic_idx as i32 + 1
                    ).execute(&mut *tx).await?;

                    // Process problems
                    for (problem_idx, problem) in topic.problems.iter().enumerate() {
                        let problem_id = if let Some(&existing_id) = problem_ids.get(&problem.name) {
                            existing_id
                        } else {
                            // Find prefix IDs for this problem
                            let prefix_id = if let Some(prefix_map) = &problem.prefix {
                                // Find matching prefix by content (simple approach)
                                prefixes.iter().find_map(|(key, stored_map)| {
                                    if stored_map == prefix_map {
                                        prefix_ids.get(key).copied()
                                    } else {
                                        None
                                    }
                                })
                            } else {
                                None
                            };

                            let group_prefix_id = if let Some(group_prefix_map) = &problem.group_prefix {
                                // Find matching group_prefix by content
                                group_prefixes.iter().find_map(|(key, stored_map)| {
                                    if stored_map == group_prefix_map {
                                        group_prefix_ids.get(key).copied()
                                    } else {
                                        None
                                    }
                                })
                            } else {
                                None
                            };

                            let id = sqlx::query_scalar!(
                                "INSERT INTO problems (name, desc_sv, desc_en, question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, difficulty, prefix_id, group_prefix_id) 
                                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id",
                                problem.name,
                                problem.desc.get("sv"),
                                problem.desc.get("en"),
                                problem.question.get("sv"),
                                problem.question.get("en"),
                                problem.answer.get("sv"),
                                problem.answer.get("en"),
                                problem.solution.get("sv"),
                                problem.solution.get("en"),
                                problem.difficulty.map(|d| d as i32),
                                prefix_id,
                                group_prefix_id
                            ).fetch_one(&mut *tx).await?;
                            
                            problem_ids.insert(problem.name.clone(), id);
                            println!("      Inserted problem: {} (ID: {})", problem.name, id);
                            id
                        };

                        // Link topic to problem
                        sqlx::query!(
                            "INSERT INTO topic_problems (topic_id, problem_id, order_index) VALUES ($1, $2, $3)
                             ON CONFLICT (topic_id, problem_id) DO NOTHING",
                            topic_id,
                            problem_id,
                            problem_idx as i32 + 1
                        ).execute(&mut *tx).await?;
                    }
                }
            }
        }

        tx.commit().await?;

        println!("Successfully populated database!");
        println!("Courses: {}", registry.courses.len());
        println!("Unique chapters: {}", chapter_ids.len());
        println!("Unique topics: {}", topic_ids.len());
        println!("Unique problems: {}", problem_ids.len());

        Ok(())
    }
}

// Example usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            println!("DATABASE_URL not found, using default...");
            "postgresql://postgres:password@localhost:5432/courses_db".to_string()
        });
    
    println!("Connecting to: {}", database_url);
    let pool = Pool::<Postgres>::connect(&database_url).await?;
    let populator = DatabasePopulator::new(pool);

    // Populate from your JSON file
    populator.populate_from_json_file("registry.json").await?;

    Ok(())
}
