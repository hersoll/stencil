// src/bin/setup_db.rs - Run this first to create tables
use sqlx::{Pool, Postgres};

type PgPool = Pool<Postgres>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    
    // Drop existing tables (clean slate)
    println!("Dropping existing tables...");
    sqlx::query("DROP TABLE IF EXISTS topic_problems CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS chapter_topics CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS course_chapters CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS problems CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS topics CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS chapters CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS courses CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS prefixes CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS group_prefixes CASCADE").execute(&pool).await?;
    sqlx::query("DROP TABLE IF EXISTS i18n CASCADE").execute(&pool).await?;
    
    println!("Creating new tables...");
    
    // Create prefix tables first (referenced by problems)
    println!("Creating prefixes table...");
    sqlx::query(r#"
        CREATE TABLE prefixes (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE,
            text_sv TEXT,
            text_en TEXT,
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#).execute(&pool).await?;

    println!("Creating group_prefixes table...");
    sqlx::query(r#"
        CREATE TABLE group_prefixes (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE,
            text_sv TEXT,
            text_en TEXT,
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#).execute(&pool).await?;

    // Entity tables with inline translations
    println!("Creating courses table...");
    sqlx::query(r#"
        CREATE TABLE courses (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE,
            desc_sv TEXT NOT NULL,
            desc_en TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#).execute(&pool).await?;

    println!("Creating chapters table...");
    sqlx::query(r#"
        CREATE TABLE chapters (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE,
            desc_sv TEXT NOT NULL,
            desc_en TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#).execute(&pool).await?;

    println!("Creating topics table...");
    sqlx::query(r#"
        CREATE TABLE topics (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE,
            desc_sv TEXT NOT NULL,
            desc_en TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#).execute(&pool).await?;

    println!("Creating problems table with prefix columns...");
    sqlx::query(r#"
        CREATE TABLE problems (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL UNIQUE,
            desc_sv TEXT NOT NULL,
            desc_en TEXT NOT NULL,
            question_sv TEXT,
            question_en TEXT,
            answer_sv TEXT,
            answer_en TEXT,
            solution_sv TEXT,
            solution_en TEXT,
            difficulty INTEGER NOT NULL,
            prefix_id INTEGER REFERENCES prefixes(id),
            group_prefix_id INTEGER REFERENCES group_prefixes(id),
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#).execute(&pool).await?;

    println!("Creating relationship tables...");
    sqlx::query(r#"
        CREATE TABLE course_chapters (
            course_id INTEGER REFERENCES courses(id) ON DELETE CASCADE,
            chapter_id INTEGER REFERENCES chapters(id) ON DELETE CASCADE,
            order_index INTEGER,
            PRIMARY KEY (course_id, chapter_id)
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE chapter_topics (
            chapter_id INTEGER REFERENCES chapters(id) ON DELETE CASCADE,
            topic_id INTEGER REFERENCES topics(id) ON DELETE CASCADE,
            order_index INTEGER,
            PRIMARY KEY (chapter_id, topic_id)
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE topic_problems (
            topic_id INTEGER REFERENCES topics(id) ON DELETE CASCADE,
            problem_id INTEGER REFERENCES problems(id) ON DELETE CASCADE,
            order_index INTEGER,
            PRIMARY KEY (topic_id, problem_id)
        )
    "#).execute(&pool).await?;

    println!("Creating i18n table...");
    sqlx::query(r#"
        CREATE TABLE i18n (
            key VARCHAR PRIMARY KEY,
            sv TEXT,
            en TEXT
        )
    "#).execute(&pool).await?;

    println!("Database tables created successfully!");
    println!("Tables created:");
    println!("- prefixes (id, name, text_sv, text_en)");
    println!("- group_prefixes (id, name, text_sv, text_en)");
    println!("- courses (id, name, desc_sv, desc_en)");
    println!("- chapters (id, name, desc_sv, desc_en)");
    println!("- topics (id, name, desc_sv, desc_en)");
    println!("- problems (id, name, desc_sv, desc_en, question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, difficulty, prefix_id, group_prefix_id)");
    println!("- course_chapters, chapter_topics, topic_problems (relationship tables)");
    println!("- i18n (key, sv, en)");
    
    Ok(())
}
