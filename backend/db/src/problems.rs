use super::common::{error_context, error_context_by_name};
use crate::{
    Answer, DescriptionTranslations, ProblemEntry, ProblemTexts, ProblemTranslations, Question,
    Solution,
};
use anyhow::{Context, Result};
use types::difficulty::{AbsoluteDifficulty, RelativeDifficulty};

struct DbProblemRow {
    id: i32,
    name: String,
    desc_sv: String,
    desc_en: String,
    module: String,
    prefix_id: Option<i32>,
    question_sv: Question,
    question_en: Question,
    answer_sv: Answer,
    answer_en: Answer,
    solution_sv: Solution,
    solution_en: Solution,
}

struct DbProblemRowWithDifficulties {
    id: i32,
    name: String,
    desc_sv: String,
    desc_en: String,
    module: String,
    prefix_id: Option<i32>,
    question_sv: Question,
    question_en: Question,
    answer_sv: Answer,
    answer_en: Answer,
    solution_sv: Solution,
    solution_en: Solution,
    absolute_difficulty: AbsoluteDifficulty,
    relative_difficulty: RelativeDifficulty,
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
            absolute_difficulty: AbsoluteDifficulty::from_num(1),
            relative_difficulty: RelativeDifficulty::from_num(1),
            module: row.module,
            prefix_id: row.prefix_id,
            translations: ProblemTranslations {
                sv: ProblemTexts {
                    question: row.question_sv,
                    answer: row.answer_sv,
                    solution: row.solution_sv,
                },
                en: ProblemTexts {
                    question: row.question_en,
                    answer: row.answer_en,
                    solution: row.solution_en,
                },
            },
            topic_ids: Vec::new(),
        }
    }
}

impl From<DbProblemRowWithDifficulties> for ProblemEntry {
    fn from(row: DbProblemRowWithDifficulties) -> Self {
        ProblemEntry {
            id: row.id,
            name: row.name,
            desc: DescriptionTranslations {
                sv: row.desc_sv,
                en: row.desc_en,
            },
            absolute_difficulty: row.absolute_difficulty,
            relative_difficulty: row.relative_difficulty,
            module: row.module,
            prefix_id: row.prefix_id,
            translations: ProblemTranslations {
                sv: ProblemTexts {
                    question: row.question_sv,
                    answer: row.answer_sv,
                    solution: row.solution_sv,
                },
                en: ProblemTexts {
                    question: row.question_en,
                    answer: row.answer_en,
                    solution: row.solution_en,
                },
            },
            topic_ids: Vec::new(),
        }
    }
}

/// Gets all of the info about every problem
///
/// Used for:
/// - Loading problems into registry during startup  NOTE: Will need to change to include difficulty? Check if difficulty is fetched in registry
/// - Getting the list of all problems on the edit page
pub async fn get_all_problem_data() -> Result<Vec<ProblemEntry>> {
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
        DbProblemRow,
        r#"SELECT id, name, desc_sv, desc_en, question_sv, question_en,
            answer_sv, answer_en, solution_sv, solution_en, prefix_id, module
            FROM problems ORDER BY module"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(problems.into_iter().map(ProblemEntry::from).collect())
}

/// Returns the problems that correspond to certain IDs
///
/// Used for:
/// - Getting the problems that correlate to a certain topic in the editing page
/// -  NOTE: Could we not just use get_topic_problems instead? And get difficulties for free?
pub async fn get_problems_from_ids(problem_ids: &[i32]) -> Result<Vec<ProblemEntry>> {
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
        DbProblemRow,
        r#"SELECT p.id, p.name, p.desc_sv, p.desc_en, p.question_sv, p.question_en,
            p.answer_sv, p.answer_en, p.solution_sv, p.solution_en, p.prefix_id, p.module 
        FROM problems p
        WHERE p.id = ANY($1)"#,
        problem_ids
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get problems with ids {problem_ids:?}"))?;

    Ok(problems.into_iter().map(ProblemEntry::from).collect())
}

/// Get all problems (with difficulties) that are included in a certain topic.
///
/// Used for:
/// - Finding all the problems to list with a topic when editing topics
/// - Listing all the problems for the selected topics when editing sets in the frontend
pub async fn get_topic_problems(topic_id: &i32) -> Result<Vec<ProblemEntry>> {
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
            DbProblemRowWithDifficulties,
            r#"SELECT p.id, p.name, p.desc_sv, p.desc_en, p.module, 
            p.question_sv, p.question_en, p.answer_sv, p.answer_en, p.solution_sv, p.solution_en, p.prefix_id,
            tp.absolute_difficulty, tp.relative_difficulty
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

pub struct ProblemIdAndDifficulties {
    pub id: i32,
    pub absolute_difficulty: AbsoluteDifficulty,
    pub relative_difficulty: RelativeDifficulty,
}

/// Get problems from topics for PDF generation.
///
/// Similar
#[allow(clippy::cast_sign_loss)]
pub async fn get_valid_problems_from_pdf_request(
    topic_ids: Vec<i32>,
    exclusions: Vec<i32>,
) -> Result<Vec<ProblemIdAndDifficulties>> {
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
        ProblemIdAndDifficulties,
        r#"SELECT p.id, tp.relative_difficulty, tp.absolute_difficulty 
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = ANY($1)
          AND NOT p.id = ANY($2)"#,
        &topic_ids,
        &exclusions
    )
    .fetch_all(pool)
    .await?;

    Ok(problems)
}

pub async fn create_problem_from_entry(problem: &ProblemEntry) -> Result<i32> {
    let pool = crate::get_pool();
    let result = sqlx::query!(
        r#"INSERT INTO problems (name, desc_sv, desc_en, module,
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
            RETURNING id"#,
        problem.name,
        problem.desc.sv,
        problem.desc.en,
        problem.module,
        &problem.translations.sv.question,
        &problem.translations.en.question,
        &problem.translations.sv.answer,
        &problem.translations.en.answer,
        &problem.translations.sv.solution,
        &problem.translations.en.solution,
        problem.prefix_id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "problem", &problem.name))?;

    Ok(result.id)
}

pub async fn update_problem_from_entry(problem: ProblemEntry) -> Result<String> {
    let pool = crate::get_pool();
    let translations = problem.translations;
    let result = sqlx::query!(
            r#"UPDATE problems SET name = $2, desc_sv = $3, desc_en = $4, module = $5,
            question_sv = $6, question_en = $7, answer_sv = $8, answer_en = $9, solution_sv = $10, solution_en = $11, prefix_id = $12
            WHERE id = $1
            RETURNING name"#,
            problem.id,
            problem.name,
            problem.desc.sv,
            problem.desc.en,
            problem.module,
            &translations.sv.question,
            &translations.en.question,
            &translations.sv.answer,
            &translations.en.answer,
            &translations.sv.solution,
            &translations.en.solution,
            problem.prefix_id,
        )
        .fetch_one(pool)
        .await
        .with_context(|| error_context("update", "problem", problem.id))?;

    Ok(result.name)
}

pub async fn delete_problem_with_id(id: i32) -> Result<String> {
    let pool = crate::get_pool();
    let _result = sqlx::query!(r#"DELETE FROM topic_problems WHERE problem_id = $1"#, id)
        .execute(pool)
        .await
        .with_context(|| error_context("delete", "problem", id))?;
    let result = sqlx::query!(r#"DELETE FROM problems WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "problem", id))?;

    Ok(result.name)
}
