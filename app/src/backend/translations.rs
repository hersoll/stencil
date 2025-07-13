use crate::{
    backend::{ChapterData, CourseData, ProblemData, ProblemRegistry, TopicData}, Error, Result
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    fs,
};

pub static GENERAL_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let data = fs::read_to_string("translations.json").expect("Failed to read translations.json");
    let table: TranslationTable =
        serde_json::from_str(&data).expect("Failed to parse translation JSON");
    Translations::new(table)
});

pub static REGISTRY_TRANSLATIONS: Lazy<ProblemRegistry> = Lazy::new(|| {
    let json = std::fs::read_to_string("registry.json").expect("Failed to read registry.json");
    let parsed: crate::backend::ProblemRegistry =
        serde_json::from_str(&json).expect("Failed to parse registry JSON");
    parsed
});

// TODO: COURSE, CHAPTER, TOPIC, PROBLEM might not be needed??
pub static COURSE_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        table.insert(course.name, course.desc);
    }
    Translations::new(table)
});

pub static CHAPTER_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            table.insert(chapter.name, chapter.desc);
        }
    }
    Translations::new(table)
});

pub static TOPIC_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            for topic in chapter.topics {
                table.insert(topic.name, topic.desc);
            }
        }
    }
    Translations::new(table)
});

pub static PROBLEM_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            for topic in chapter.topics {
                for problem in topic.problems {
                    let combined_name = topic.name.clone() + "_" + problem.name.as_str();
                    table.insert(combined_name, problem.desc);
                }
            }
        }
    }
    Translations::new(table)
});

pub static QUESTION_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            for topic in chapter.topics {
                for problem in topic.problems {
                    if !problem.question.is_empty() {
                        let combined_name = topic.name.clone() + "_" + problem.name.as_str();
                        table.insert(combined_name, problem.question);
                    }
                }
            }
        }
    }
    Translations::new(table)
});

pub static ANSWER_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            for topic in chapter.topics {
                for problem in topic.problems {
                    if !problem.answer.is_empty() {
                        let combined_name = topic.name.clone() + "_" + problem.name.as_str();
                        table.insert(combined_name, problem.answer);
                    }
                }
            }
        }
    }
    Translations::new(table)
});

pub static SOLUTION_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            for topic in chapter.topics {
                for problem in topic.problems {
                    if !problem.solution.is_empty() {
                        let combined_name = topic.name.clone() + "_" + problem.name.as_str();
                        table.insert(combined_name, problem.solution);
                    }
                }
            }
        }
    }
    Translations::new(table)
});

pub type TranslationTable = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Deserialize)]
pub struct Translations {
    table: TranslationTable,
}

// NOTE: To deal with language, have a "global" in the front_end, and pass it on every api call.
//       Or, when you set the language in the frontend, have an effect that calls backend and sets
//       it for every TranslationTable

impl Translations {
    pub fn new(table: TranslationTable) -> Translations {
        Translations {
            table: table,
        }
    }
    pub fn get_phrase(&self, key: &str, lang: &str) -> Result<String> {
        match self
            .table
            .get(key)
            .and_then(|lang_map| lang_map.get(lang))
            .cloned()
        {
            Some(val) => Ok(val),
            None => Err(Error::InvalidTranslationKey {
                key: key.to_string(),
                lang: lang.to_string(),
            }),
        }
    }
    pub fn get_placeholder_phrase(&self, key: &str, args: HashMap<&str, String>, lang: &str) -> Result<String> {
        if let Some(val) = self
            .table
            .get(key)
            .and_then(|lang_map| lang_map.get(lang))
            .cloned()
        {
            Ok(Self::fill_args(&val, &args))
        } else {
            Err(Error::InvalidTranslationKey {
                key: key.to_string(),
                lang: lang.to_string(),
            })
        }
    }

    fn fill_args(placeholder_text: &str, args: &HashMap<&str, String>) -> String {
        let mut placeholder_str = placeholder_text.to_string();
        for (key, value) in args {
            let placeholder = format!("{{{}}}", key);
            placeholder_str = placeholder_str.replace(&placeholder, value);
        }
        placeholder_str
    }
}
// impl Serialize for ProblemRegistry {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         // Create the JSON structure directly
//         let json_structure = serde_json::json!({
//             "courses": self.courses.iter().map(|course| {
//                 (course.name.clone(), serde_json::json!({
//                     "desc": course.desc,
//                     "chapters": course.chapters.iter().map(|chapter| {
//                         (chapter.name.clone(), serde_json::json!({
//                             "desc": chapter.desc,
//                             "topics": chapter.topics.iter().map(|topic| {
//                                 (topic.name.clone(), serde_json::json!({
//                                     "desc": topic.desc,
//                                     "problems": topic.problems.iter().map(|problem| {
//                                         let mut problem_map = serde_json::Map::new();
//                                         problem_map.insert("desc".to_string(), json!(problem.desc));
//
//                                         if problem.question.len() > 0 {
//                                             problem_map.insert("question".to_string(), json!(problem.question));
//                                         }
//                                         if problem.answer.len() > 0 {
//                                             problem_map.insert("answer".to_string(), json!(problem.answer));
//                                         }
//                                         if problem.solution.len() > 0 {
//                                             problem_map.insert("solution".to_string(), json!(problem.solution));
//                                         }
//
//                                         (problem.name.clone(), json!(problem_map))
//                                     }).collect::<std::collections::HashMap<_,_>>()
//                                 }))
//                             }).collect::<std::collections::HashMap<_, _>>()
//                         }))
//                     }).collect::<std::collections::HashMap<_, _>>()
//                 }))
//             }).collect::<std::collections::HashMap<_, _>>()
//         });
//
//         json_structure.serialize(serializer)
//     }
// }
//
// impl<'de> Deserialize<'de> for ProblemRegistry {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         struct CourseDataHelper {
//             courses: HashMap<String, CourseHelper>,
//         }
//
//         #[derive(Deserialize)]
//         struct CourseHelper {
//             desc: HashMap<String, String>,
//             chapters: HashMap<String, ChapterHelper>,
//         }
//
//         #[derive(Deserialize)]
//         struct ChapterHelper {
//             desc: HashMap<String, String>,
//             topics: HashMap<String, TopicHelper>,
//         }
//
//         #[derive(Deserialize)]
//         struct TopicHelper {
//             desc: HashMap<String, String>,
//             problems: HashMap<String, ProblemHelper>,
//         }
//
//         #[derive(Deserialize)]
//         struct ProblemHelper {
//             desc: HashMap<String, String>,
//             #[serde(default)]
//             question: HashMap<String, String>,
//             #[serde(default)]
//             answer: HashMap<String, String>,
//             #[serde(default)]
//             solution: HashMap<String, String>,
//         }
//
//         let helper = CourseDataHelper::deserialize(deserializer)?;
//
//         let courses = helper
//             .courses
//             .into_iter()
//             .map(|(course_name, course_helper)| {
//                 let chapters = course_helper
//                     .chapters
//                     .into_iter()
//                     .map(|(chapter_name, chapter_helper)| {
//                         let topics = chapter_helper
//                             .topics
//                             .into_iter()
//                             .map(|(topic_name, topic_helper)| {
//                                 let problems = topic_helper
//                                     .problems
//                                     .into_iter()
//                                     .map(|(problem_name, problem_helper)| {
//                                         ProblemData {
//                                             name: problem_name,
//                                             desc: problem_helper.desc,
//                                             question: problem_helper.question,
//                                             answer: problem_helper.answer,
//                                             solution: problem_helper.solution,
//                                         }
//                                     }).collect();
//
//                                 TopicData {
//                                 name: topic_name,
//                                 desc: topic_helper.desc,
//                                 problems,
//                                 }
//                             })
//                             .collect();
//
//                         ChapterData {
//                             name: chapter_name,
//                             desc: chapter_helper.desc,
//                             topics,
//                         }
//                     })
//                     .collect();
//                 CourseData {
//                     name: course_name,
//                     desc: course_helper.desc,
//                     chapters,
//                 }
//             })
//             .collect();
//
//         Ok(ProblemRegistry { courses })
//     }
// }
