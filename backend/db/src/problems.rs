use crate::{
    Description, DescriptionTranslations, HasDesc, ID, Name, PublicFlag, error_context,
    error_context_by_name,
};
use types::{
    difficulty::{AbsoluteDifficulty, RelativeDifficulty},
    format_strings::{AnswerString, QuestionString, SolutionString},
    lang::Language,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct DBProblemRow {
    id: ID,
    name: Name,
    desc_sv: Description,
    desc_en: Description,
    module: Name,
    prefix_id: Option<ID>,
    question_sv: QuestionString,
    question_en: QuestionString,
    answer_sv: AnswerString,
    answer_en: AnswerString,
    solution_sv: SolutionString,
    solution_en: SolutionString,
    public: PublicFlag,
}

struct DBProblemRowWithTopicDifficulties {
    id: ID,
    name: Name,
    desc_sv: Description,
    desc_en: Description,
    module: Name,
    prefix_id: Option<ID>,
    question_sv: QuestionString,
    question_en: QuestionString,
    answer_sv: AnswerString,
    answer_en: AnswerString,
    solution_sv: SolutionString,
    solution_en: SolutionString,
    topic_id: ID,
    absolute_difficulty: AbsoluteDifficulty,
    relative_difficulty: RelativeDifficulty,
    public: PublicFlag,
}

impl From<DBProblemRow> for ProblemEntry {
    fn from(row: DBProblemRow) -> Self {
        ProblemEntry {
            id: row.id,
            name: row.name,
            desc: DescriptionTranslations {
                sv: row.desc_sv,
                en: row.desc_en,
            },
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
            topic_data: Vec::new(),
        }
    }
}

impl From<DBProblemRow> for ProblemEntryForEditor {
    fn from(row: DBProblemRow) -> Self {
        ProblemEntryForEditor {
            public: row.public,
            entry: ProblemEntry::from(row),
        }
    }
}

impl From<DBProblemRowWithTopicDifficulties> for ProblemEntry {
    fn from(row: DBProblemRowWithTopicDifficulties) -> Self {
        ProblemEntry {
            id: row.id,
            name: row.name,
            desc: DescriptionTranslations {
                sv: row.desc_sv,
                en: row.desc_en,
            },
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
            topic_data: vec![TopicSpecificData {
                topic_id: row.topic_id,
                absolute_difficulty: row.absolute_difficulty,
                relative_difficulty: row.relative_difficulty,
            }],
        }
    }
}

impl From<DBProblemRowWithTopicDifficulties> for ProblemEntryForEditor {
    fn from(row: DBProblemRowWithTopicDifficulties) -> Self {
        ProblemEntryForEditor {
            public: row.public,
            entry: ProblemEntry::from(row),
        }
    }
}

/// Used in problems, which contain both the related topics and the associated difficulties
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TopicSpecificData {
    pub topic_id: i32,
    pub absolute_difficulty: AbsoluteDifficulty,
    pub relative_difficulty: RelativeDifficulty,
}

/// The texts associated with a specific problem in a certain [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemTexts {
    pub question: QuestionString,
    pub answer: AnswerString,
    pub solution: SolutionString,
}

/// Contains [`ProblemTexts`] for every [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemTranslations {
    pub sv: ProblemTexts,
    pub en: ProblemTexts,
}

/// Representation of data about a problem from the DB
///
/// A common pattern is to read this data during problem generation to get question/answer/solution text
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemEntry {
    pub id: ID,
    pub name: Name,
    pub desc: DescriptionTranslations,
    pub module: Name,
    pub prefix_id: Option<ID>,
    pub translations: ProblemTranslations,
    pub topic_data: Vec<TopicSpecificData>,
}
/// The same data as [`ProblemEntry`], except it includes information about whether the entry is
/// public or not.
///
/// This is needed so the editor can edit the `public` value
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemEntryForEditor {
    #[serde(flatten)]
    pub entry: ProblemEntry,
    pub public: PublicFlag,
}
impl HasDesc for ProblemEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl ProblemEntry {
    pub fn get_question(&self, lang: Language) -> &QuestionString {
        match lang {
            Language::Sv => &self.translations.sv.question,
            Language::En => &self.translations.en.question,
        }
    }
    pub fn get_answer(&self, lang: Language) -> &AnswerString {
        match lang {
            Language::Sv => &self.translations.sv.answer,
            Language::En => &self.translations.en.answer,
        }
    }
    pub fn get_solution(&self, lang: Language) -> &SolutionString {
        match lang {
            Language::Sv => &self.translations.sv.solution,
            Language::En => &self.translations.en.solution,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProblemIdsAndDifficulties {
    pub problem_id: i32,
    pub topic_id: i32,
    pub absolute_difficulty: AbsoluteDifficulty,
    pub relative_difficulty: RelativeDifficulty,
}
impl ProblemIdsAndDifficulties {
    pub fn from_entry_and_topic_id(entry: &ProblemEntry, topic_id: i32) -> Self {
        let (absolute_difficulty, relative_difficulty) = match entry
            .topic_data
            .iter()
            .find(|topic| topic.topic_id == topic_id)
        {
            Some(topic) => (topic.absolute_difficulty, topic.relative_difficulty),
            None => (
                AbsoluteDifficulty::from_num(4),
                RelativeDifficulty::from_num(4),
            ),
        };
        Self {
            problem_id: entry.id,
            topic_id,
            absolute_difficulty,
            relative_difficulty,
        }
    }
}

/// Gets all of the info about every problem
///
/// Used for getting the list of all problems on the edit page
pub async fn get_all_problem_data() -> Result<Vec<ProblemEntryForEditor>> {
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
        DBProblemRow,
        r#"SELECT id, name, desc_sv, desc_en, question_sv, question_en,
            answer_sv, answer_en, solution_sv, solution_en, prefix_id, module, public
            FROM problems ORDER BY module, name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(problems
        .into_iter()
        .map(ProblemEntryForEditor::from)
        .collect())
}

/// Gets all of the info about every *public* problem (unless in dev mode)
///
/// Used for loading problems into registry during startup
pub async fn get_public_problem_data() -> Result<Vec<ProblemEntry>> {
    let production_mode = crate::production_mode();
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
        DBProblemRow,
        r#"SELECT id, name, desc_sv, desc_en, question_sv, question_en,
            answer_sv, answer_en, solution_sv, solution_en, prefix_id, module, public
            FROM problems 
            WHERE (NOT $1::bool OR public)
            ORDER BY module, name"#,
        production_mode
    )
    .fetch_all(pool)
    .await?;

    Ok(problems.into_iter().map(ProblemEntry::from).collect())
}

/// Get all problems (with difficulties for that topic) that are included in a certain topic.
///
/// Used for finding all the problems to list with a topic when editing topics
pub async fn get_all_topic_problems_with_difficulties(
    topic_id: &i32,
) -> Result<Vec<ProblemEntryForEditor>> {
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
            DBProblemRowWithTopicDifficulties,
            r#"SELECT p.id, p.name, p.desc_sv, p.desc_en, p.module, 
            p.question_sv, p.question_en, p.answer_sv, p.answer_en, p.solution_sv, p.solution_en, p.prefix_id,
            tp.topic_id, tp.absolute_difficulty, tp.relative_difficulty, p.public
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = $1
        ORDER BY tp.order_index, p.name"#,
            topic_id,
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to get problems for topic {}", topic_id))?;

    Ok(problems
        .into_iter()
        .map(ProblemEntryForEditor::from)
        .collect())
}

/// Get all problems (with difficulties for that topic) that are included in a certain topic.
///
/// Used for listing all the problems for the selected topics when editing sets in the frontend
pub async fn get_public_topic_problems_with_difficulties(
    topic_id: &i32,
) -> Result<Vec<ProblemEntry>> {
    let production_mode = crate::production_mode();
    let pool = crate::get_pool();
    let problems = sqlx::query_as!(
        DBProblemRowWithTopicDifficulties,
        r#"SELECT p.id, p.name, p.desc_sv, p.desc_en, p.module, 
            p.question_sv, p.question_en, p.answer_sv, p.answer_en, 
            p.solution_sv, p.solution_en, p.prefix_id, p.public,
            tp.topic_id, tp.absolute_difficulty, tp.relative_difficulty
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = $1 AND (NOT $2::bool OR p.public)
        ORDER BY tp.order_index, p.name"#,
        topic_id,
        production_mode
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get problems for topic {}", topic_id))?;

    Ok(problems.into_iter().map(ProblemEntry::from).collect())
}

/// Optimized version of [`get_all_topic_problems_with_difficulties()`] when we have many topics
/// to get data about, most notably in the topic list in the editor.
pub async fn get_topic_problems_with_difficulties_for_topics(
    topic_ids: &[i32],
) -> Result<HashMap<i32, Vec<ProblemIdsAndDifficulties>>> {
    let pool = crate::get_pool();
    let rows = sqlx::query_as!(
        DBProblemRowWithTopicDifficulties,
        r#"SELECT p.id, p.name, p.desc_sv, p.desc_en, p.module,
        p.question_sv, p.question_en, p.answer_sv, p.answer_en, p.solution_sv, p.solution_en, p.prefix_id,
        tp.topic_id, tp.absolute_difficulty, tp.relative_difficulty, p.public
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = ANY($1)
        ORDER BY tp.topic_id, tp.order_index, p.name"#,
        topic_ids,
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get problems for topics {:?}", topic_ids))?;

    let mut map: HashMap<i32, Vec<ProblemIdsAndDifficulties>> = HashMap::new();
    for row in rows {
        let topic_id = row.topic_id; // grab before `row` is consumed below
        let problem_entry = ProblemEntry::from(row);
        map.entry(topic_id)
            .or_default()
            .push(ProblemIdsAndDifficulties::from_entry_and_topic_id(
                &problem_entry,
                topic_id,
            ));
    }
    Ok(map)
}

/// Get problems from topics for PDF generation.
///
/// Similar to [`get_topic_problems()`] except it:
/// - Takes multiple `topic_ids`
/// - Allows for exclusions
/// - Filter out all problems not in the desired difficulty range
#[allow(clippy::cast_sign_loss)]
pub async fn get_valid_problems_from_pdf_request(
    topic_ids: Vec<i32>,
    exclusions: Vec<i32>,
    allowed_difficulties: Vec<AbsoluteDifficulty>,
) -> Result<Vec<ProblemIdsAndDifficulties>> {
    // In prod we only want the public rows,
    // in dev we want all
    let production_mode = cfg!(feature = "docker") || std::env::args().any(|x| x == "prod");

    let pool = crate::get_pool();
    let allowed_difficulty_numbers: Vec<i32> = allowed_difficulties
        .into_iter()
        .map(|diff| diff.number as i32)
        .collect();
    let problems = sqlx::query_as!(
        ProblemIdsAndDifficulties,
        r#"SELECT p.id AS problem_id, tp.relative_difficulty, tp.absolute_difficulty, tp.topic_id 
        FROM problems p
        JOIN topic_problems tp ON p.id = tp.problem_id
        WHERE tp.topic_id = ANY($1)
          AND NOT p.id = ANY($2)
          AND tp.absolute_difficulty = ANY($3)
          AND (NOT $4::bool OR p.public)"#,
        &topic_ids,
        &exclusions,
        &allowed_difficulty_numbers,
        production_mode,
    )
    .fetch_all(pool)
    .await?;

    Ok(problems)
}

pub async fn create_problem_from_entry(problem: &ProblemEntryForEditor) -> Result<i32> {
    let pool = crate::get_pool();
    let result = sqlx::query!(
        r#"INSERT INTO problems (name, desc_sv, desc_en, module,
            question_sv, question_en, answer_sv, answer_en, solution_sv, solution_en, prefix_id, public) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) 
            RETURNING id"#,
        problem.entry.name,
        problem.entry.desc.sv,
        problem.entry.desc.en,
        problem.entry.module,
        &problem.entry.translations.sv.question,
        &problem.entry.translations.en.question,
        &problem.entry.translations.sv.answer,
        &problem.entry.translations.en.answer,
        &problem.entry.translations.sv.solution,
        &problem.entry.translations.en.solution,
        problem.entry.prefix_id,
        problem.public,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "problem", &problem.entry.name))?;

    Ok(result.id)
}

pub async fn update_problem_from_entry(problem: ProblemEntryForEditor) -> Result<String> {
    let pool = crate::get_pool();
    let translations = problem.entry.translations;
    let result = sqlx::query!(
        r#"UPDATE problems SET name = $2, desc_sv = $3, desc_en = $4, module = $5,
            question_sv = $6, question_en = $7, answer_sv = $8, answer_en = $9, solution_sv = $10, 
            solution_en = $11, prefix_id = $12, public = $13
            WHERE id = $1
            RETURNING name"#,
        problem.entry.id,
        problem.entry.name,
        problem.entry.desc.sv,
        problem.entry.desc.en,
        problem.entry.module,
        &translations.sv.question,
        &translations.en.question,
        &translations.sv.answer,
        &translations.en.answer,
        &translations.sv.solution,
        &translations.en.solution,
        problem.entry.prefix_id,
        problem.public
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "problem", problem.entry.id))?;

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

pub async fn update_difficulties_for_problem_with_id(
    problem_id: &i32,
    topic_data: &[TopicSpecificData],
) -> Result<()> {
    let pool = crate::get_pool();
    for topic in topic_data {
        let _result = sqlx::query!(
            r#"UPDATE topic_problems SET absolute_difficulty = $3, relative_difficulty = $4
            WHERE problem_id = $1 AND topic_id = $2"#,
            problem_id,
            topic.topic_id,
            topic.absolute_difficulty.number as i32,
            topic.relative_difficulty.number as i32
        )
        .execute(pool)
        .await
        .with_context(|| "failed to update difficulty for problem")?;
    }

    Ok(())
}

/// The reason this accepts a `topic_id` even though [`ProblemIdsAndDifficulties`] contains a topic_id
/// is that when a topic is created, a new ID is generated which isn't contained in the problems
pub async fn update_difficulties_for_problems(
    topic_id: &ID,
    problems: &[ProblemIdsAndDifficulties],
) -> Result<()> {
    let pool = crate::get_pool();
    for problem in problems {
        let _result = sqlx::query!(
            r#"UPDATE topic_problems SET absolute_difficulty = $3, relative_difficulty = $4
            WHERE problem_id = $1 AND topic_id = $2"#,
            problem.problem_id,
            topic_id,
            problem.absolute_difficulty.number as i32,
            problem.relative_difficulty.number as i32
        )
        .execute(pool)
        .await
        .with_context(|| "failed to update difficulty for topic")?;
    }

    Ok(())
}

/// Makes every problem public
pub async fn publish_all_problems() -> Result<u64> {
    let pool = crate::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE problems 
        SET public = true
        WHERE public = false"#,
    )
    .execute(pool)
    .await
    .with_context(|| "Failed to publish problems")?;

    Ok(updated.rows_affected())
}
