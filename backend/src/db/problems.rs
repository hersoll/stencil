use super::common::{error_context, error_context_by_name};
use crate::db::{
    self, DescriptionTranslations, ProblemEntry, ProblemTranslations, TranslatedProblem,
};
use anyhow::{Context, Result};

struct DbProblemRow {
    id: i32,
    name: String,
    desc_sv: String,
    desc_en: String,
    difficulty: i32,
    module: String,
    prefix_id: Option<i32>,
    question_sv: Option<String>,
    question_en: Option<String>,
    answer_sv: Option<String>,
    answer_en: Option<String>,
    solution_sv: Option<String>,
    solution_en: Option<String>,
}

impl From<DbProblemRow> for ProblemEntry {
    fn from(row: DbProblemRow) -> Self {
        ProblemEntry {
            id: row.id,
            name: row.name,
            desc: DescriptionTranslations {
                sv: row.desc_sv,
                en: row.desc_en,
            },
            difficulty: row.difficulty,
            module: row.module,
            prefix_id: row.prefix_id,
            translations: ProblemTranslations {
                sv: crate::db::TranslatedProblem {
                    question: row.question_sv,
                    answer: row.answer_sv,
                    solution: row.solution_sv,
                },
                en: TranslatedProblem {
                    question: row.question_en,
                    answer: row.answer_en,
                    solution: row.solution_en,
                },
            },
        }
    }
}

/// Get all problems ordered by module
pub async fn get_all_problem_data() -> Result<Vec<ProblemEntry>> {
    let pool = db::get_pool();
    let problems = sqlx::query_as!(
        DbProblemRow,
        r#"SELECT id, name, difficulty, desc_sv, desc_en, question_sv, question_en,
            answer_sv, answer_en, solution_sv, solution_en, prefix_id, module
            FROM problems ORDER BY module"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(problems.into_iter().map(ProblemEntry::from).collect())
}

/// Get all problems for a specific topic, ordered by topic order_index
pub async fn get_topic_problems(topic_id: i32) -> Result<Vec<ProblemEntry>> {
    let pool = db::get_pool();
    let problems = sqlx::query_as!(
            DbProblemRow,
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
        .with_context(|| format!("Failed to get problems for topic {}", topic_id))?;

    Ok(problems.into_iter().map(ProblemEntry::from).collect())
}

/// Query builder for fetching problems with filters
pub struct ProblemQuery {
    topic_ids: Vec<i32>,
    min_difficulty: Option<i32>,
    max_difficulty: Option<i32>,
    exclusions: Vec<i32>,
}

impl ProblemQuery {
    /// Create a new problem query for the given topics
    pub fn for_topics(topic_ids: Vec<i32>) -> Self {
        Self {
            topic_ids,
            min_difficulty: None,
            max_difficulty: None,
            exclusions: Vec::new(),
        }
    }

    /// Filter by minimum difficulty (inclusive)
    pub fn min_difficulty(mut self, difficulty: i32) -> Self {
        self.min_difficulty = Some(difficulty);
        self
    }

    /// Filter by maximum difficulty (inclusive)
    pub fn max_difficulty(mut self, difficulty: i32) -> Self {
        self.max_difficulty = Some(difficulty);
        self
    }

    /// Set difficulty range (inclusive on both ends)
    pub fn difficulty_range(mut self, min: i32, max: i32) -> Self {
        self.min_difficulty = Some(min);
        self.max_difficulty = Some(max);
        self
    }

    /// Exclude specific problem IDs
    pub fn exclude(mut self, problem_ids: Vec<i32>) -> Self {
        self.exclusions = problem_ids;
        self
    }

    /// Execute the query and return matching problems
    pub async fn fetch(self) -> Result<Vec<ProblemEntry>> {
        let pool = db::get_pool();
        let min_diff = self.min_difficulty.unwrap_or(i32::MIN);
        let max_diff = self.max_difficulty.unwrap_or(i32::MAX);

        let problems = sqlx::query_as!(
                DbProblemRow,
                r#"SELECT DISTINCT p.id, p.name, p.difficulty, p.desc_sv, p.desc_en, p.module, 
                p.question_sv, p.question_en, p.answer_sv, p.answer_en, p.solution_sv, p.solution_en, p.prefix_id
            FROM problems p
            JOIN topic_problems tp ON p.id = tp.problem_id
            WHERE tp.topic_id = ANY($1)
                AND p.difficulty >= $2
                AND p.difficulty <= $3
                AND NOT p.id = ANY($4)
            ORDER BY p.difficulty"#,
                &self.topic_ids,
                min_diff,
                max_diff,
                &self.exclusions
            )
            .fetch_all(pool)
            .await?;

        Ok(problems.into_iter().map(ProblemEntry::from).collect())
    }
}

/// Get problems from topics within a difficulty range
///
/// This is a convenience function. For more control, use `ProblemQuery::for_topics()`.
pub async fn get_topic_problems_in_difficulty_range(
    topic_ids: Vec<i32>,
    starting_difficulty: i32,
    ending_difficulty: i32,
) -> Result<Vec<ProblemEntry>> {
    ProblemQuery::for_topics(topic_ids)
        .difficulty_range(starting_difficulty, ending_difficulty)
        .fetch()
        .await
}

/// Get problem names and difficulties from topics for PDF generation
///
/// # Arguments
/// * `topic_ids` - The topics to get problems from
/// * `exclusions` - Problem IDs to exclude from the results
///
/// # Returns
/// A vector of tuples containing (full_problem_name, difficulty)
/// where full_problem_name is formatted as "module_name"
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

/// Get a single problem by ID
pub async fn get_problem(id: i32) -> Result<ProblemEntry> {
    let pool = db::get_pool();
    let problem = sqlx::query_as!(
        DbProblemRow,
        r#"SELECT id, name, difficulty, desc_sv, desc_en, module, 
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id
                FROM problems
                WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("get", "problem", id))?;

    Ok(ProblemEntry::from(problem))
}

/// Get multiple problems by IDs
pub async fn get_problems(ids: &[i32]) -> Result<Vec<ProblemEntry>> {
    let pool = db::get_pool();
    let problems = sqlx::query_as!(
        DbProblemRow,
        r#"SELECT id, name, difficulty, desc_sv, desc_en, module, 
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id
                FROM problems
            WHERE id = ANY($1)
            ORDER BY module"#,
        ids,
    )
    .fetch_all(pool)
    .await?;

    Ok(problems.into_iter().map(ProblemEntry::from).collect())
}

/// Create a new problem
pub async fn create_problem(problem: ProblemEntry) -> Result<i32> {
    let pool = db::get_pool();
    let desc = problem.desc;
    let translations = problem.translations;
    let result = sqlx::query!(
        r#"INSERT INTO problems (name, desc_sv, desc_en, difficulty, module,
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) 
            RETURNING id"#,
        problem.name,
        desc.sv,
        desc.en,
        problem.difficulty,
        problem.module,
        translations.sv.question,
        translations.en.question,
        translations.sv.answer,
        translations.en.answer,
        translations.sv.solution,
        translations.en.solution,
        problem.prefix_id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "problem", &problem.name))?;

    Ok(result.id)
}

/// Update an existing problem
pub async fn update_problem(problem: ProblemEntry) -> Result<i32> {
    let pool = db::get_pool();
    let desc = problem.desc;
    let translations = problem.translations;
    let result = sqlx::query!(
            r#"UPDATE problems SET name = $2, difficulty = $12, desc_sv = $3, desc_en = $4, module = $5,
            question_sv = $6, question_en = $7, answer_sv = $8, answer_en = $9, solution_sv = $10, solution_en = $11, prefix_id = $13
            WHERE id = $1
            RETURNING id"#,
            problem.id,
            problem.name,
            desc.sv,
            desc.en,
            problem.module,
            translations.sv.question,
            translations.en.question,
            translations.sv.answer,
            translations.en.answer,
            translations.sv.solution,
            translations.en.solution,
            problem.difficulty,
            problem.prefix_id,
        )
        .fetch_one(pool)
        .await
        .with_context(|| error_context("update", "problem", problem.id))?;

    Ok(result.id)
}

/// Delete a problem by ID, returns the deleted problem name
pub async fn delete_problem(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM problems WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "problem", id))?;

    Ok(result.name)
}
