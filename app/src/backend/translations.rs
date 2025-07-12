use crate::{backend::{Course, ProblemRegistry, Chapter, Topic}, Error, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

pub static GENERAL_TRANSLATIONS: Lazy<Arc<Mutex<Translations>>> = Lazy::new(|| {
    let data = fs::read_to_string("translations.json").expect("Failed to read translations.json");
    let table: TranslationTable =
        serde_json::from_str(&data).expect("Failed to parse translation JSON");
    Arc::new(Mutex::new(Translations::new(table, "sv")))
});

pub static REGISTRY_TRANSLATIONS: Lazy<ProblemRegistry> = Lazy::new(|| {
    let json = std::fs::read_to_string("registry.json").expect("Failed to read registry.json");
    let parsed: crate::backend::ProblemRegistry = serde_json::from_str(&json).expect("Failed to parse registry JSON");
    parsed
});

pub static COURSE_TRANSLATIONS: Lazy<Arc<Mutex<Translations>>>= Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        table.insert(course.name, course.desc);
    }
    Arc::new(Mutex::new(Translations::new(table, "sv")))
});

pub static CHAPTER_TRANSLATIONS: Lazy<Arc<Mutex<Translations>>>= Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            table.insert(chapter.name, chapter.desc);
        }
    }
    Arc::new(Mutex::new(Translations::new(table, "sv")))
});

pub static TOPIC_TRANSLATIONS: Lazy<Arc<Mutex<Translations>>>= Lazy::new(|| {
    let mut table: TranslationTable = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            for topic in chapter.topics {
                table.insert(topic.name, topic.desc);
            }
        }
    }
    Arc::new(Mutex::new(Translations::new(table, "sv")))
});

pub static PROBLEM_TRANSLATIONS: Lazy<HashMap<String, TranslationTable>> = Lazy::new(|| {
    let mut table: HashMap<String, TranslationTable> = HashMap::new();
    for course in REGISTRY_TRANSLATIONS.clone().courses {
        for chapter in course.chapters {
            for topic in chapter.topics {
                for (name, data) in topic.problems {
                        table.insert(name, data);
                }
            }
        }
    }
    table
});

pub type TranslationTable = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Deserialize)]
pub struct Translations {
    lang: String,
    table: TranslationTable,
}

impl Translations {
    pub fn new(table: TranslationTable, lang: &str) -> Translations {
        Translations {
            lang: lang.to_string(),
            table: table,
        }
    }
    pub fn get_phrase(&self, key: &str) -> Result<String> {
        match self
            .table
            .get(key)
            .and_then(|lang_map| lang_map.get(&self.lang))
            .cloned()
        {
            Some(val) => Ok(val),
            None => Err(Error::InvalidTranslationKey {
                key: key.to_string(),
                lang: self.lang.to_string(),
            }),
        }
    }
    pub fn get_placeholder_phrase(
        &self,
        key: &str,
        args: HashMap<&str, &str>,
    ) -> Result<String> {
        if let Some(val) = self
            .table
            .get(key)
            .and_then(|lang_map| lang_map.get(&self.lang))
            .cloned()
        {
            Ok(Self::fill_args(&val, &args))
        } else {
            Err(Error::InvalidTranslationKey {
                key: key.to_string(),
                lang: self.lang.to_string(),
            })
        }
    }
    pub fn change_language(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    fn fill_args(placeholder_text: &str, args: &HashMap<&str, &str>) -> String {
        let mut placeholder_str = placeholder_text.to_string();
        for (key, value) in args {
            let placeholder = format!("{{{}}}", key);
            placeholder_str = placeholder_str.replace(&placeholder, value);
        }
        placeholder_str
    }
}
impl Serialize for ProblemRegistry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Create the JSON structure directly
        let json_structure = serde_json::json!({
            "courses": self.courses.iter().map(|course| {
                (course.name.clone(), serde_json::json!({
                    "desc": course.desc,
                    "chapters": course.chapters.iter().map(|chapter| {
                        (chapter.name.clone(), serde_json::json!({
                            "desc": chapter.desc,
                            "topics": chapter.topics.iter().map(|topic| {
                                (topic.name.clone(), serde_json::json!({
                                    "desc": topic.desc,
                                    "problems": topic.problems
                                }))
                            }).collect::<std::collections::HashMap<_, _>>()
                        }))
                    }).collect::<std::collections::HashMap<_, _>>()
                }))
            }).collect::<std::collections::HashMap<_, _>>()
        });
        
        json_structure.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ProblemRegistry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CourseDataHelper {
            courses: HashMap<String, CourseHelper>,
        }

        #[derive(Deserialize)]
        struct CourseHelper {
            desc: HashMap<String, String>,
            chapters: HashMap<String, ChapterHelper>,
        }

        #[derive(Deserialize)]
        struct ChapterHelper {
            desc: HashMap<String, String>,
            topics: HashMap<String, TopicHelper>,
        }

        #[derive(Deserialize)]
        struct TopicHelper {
            desc: HashMap<String, String>,
            problems: HashMap<String, HashMap<String, HashMap<String, String>>>,
        }

        let helper = CourseDataHelper::deserialize(deserializer)?;
        
        let courses = helper.courses
            .into_iter()
            .map(|(course_name, course_helper)| {
                let chapters = course_helper.chapters
                    .into_iter()
                    .map(|(chapter_name, chapter_helper)| {
                        let topics = chapter_helper.topics
                            .into_iter()
                            .map(|(topic_name, topic_helper)| {
                                Topic {
                                    name: topic_name,
                                    desc: topic_helper.desc,
                                    problems: topic_helper.problems,
                                }
                            })
                            .collect();

                        Chapter {
                            name: chapter_name,
                            desc: chapter_helper.desc,
                            topics,
                        }
                    })
                    .collect();
                Course {
                    name: course_name,
                    desc: course_helper.desc,
                    chapters,
                }
            })
            .collect();

        Ok(ProblemRegistry { courses })
    }
}
