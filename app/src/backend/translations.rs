use crate::{
    backend::ProblemRegistry, Error, Result
};
use once_cell::sync::Lazy;
use serde::{Deserialize};
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

// pub static COURSE_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
//     let mut table: TranslationTable = HashMap::new();
//     for course in REGISTRY_TRANSLATIONS.clone().courses {
//         table.insert(course.name, course.desc);
//     }
//     Translations::new(table)
// });
//
// pub static CHAPTER_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
//     let mut table: TranslationTable = HashMap::new();
//     for course in REGISTRY_TRANSLATIONS.clone().courses {
//         for chapter in course.chapters {
//             table.insert(chapter.name, chapter.desc);
//         }
//     }
//     Translations::new(table)
// });
//
// pub static TOPIC_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
//     let mut table: TranslationTable = HashMap::new();
//     for course in REGISTRY_TRANSLATIONS.clone().courses {
//         for chapter in course.chapters {
//             for topic in chapter.topics {
//                 table.insert(topic.name, topic.desc);
//             }
//         }
//     }
//     Translations::new(table)
// });
//
// pub static PROBLEM_TRANSLATIONS: Lazy<Translations> = Lazy::new(|| {
//     let mut table: TranslationTable = HashMap::new();
//     for course in REGISTRY_TRANSLATIONS.clone().courses {
//         for chapter in course.chapters {
//             for topic in chapter.topics {
//                 for problem in topic.problems {
//                     let combined_name = topic.name.clone() + "_" + problem.name.as_str();
//                     table.insert(combined_name, problem.desc);
//                 }
//             }
//         }
//     }
//     Translations::new(table)
// });

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

