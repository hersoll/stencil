use crate::{Error, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// A map between problem names (simple-equations-default) and ProblemTypes
pub static PROBLEM_REGISTRY: Lazy<Mutex<HashMap<String, super::ProblemType>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

//#################################
//#       COURSE STRUCTURE        #
//#################################

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
impl HasDesc for CourseData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub topics: Vec<TopicData>,
}
impl HasDesc for ChapterData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicData {
    pub name: String,
    pub desc: HashMap<String, String>,
    pub problems: Vec<ProblemData>,
}
impl HasDesc for TopicData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}
pub trait HasDesc {
    fn desc(&self) -> &HashMap<String, String>;
    fn name(&self) -> &String;
    fn get_desc<T: Into<String>>(&self, lang: T) -> Result<String> {
        let lang_str: String = lang.into();
        let desc = self
            .desc()
            .get(&lang_str)
            .ok_or(Error::NoDescriptionForLang {
                name: self.name().clone(),
                lang: lang_str,
            })?
            .clone();
        Ok(desc)
    }
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
}
impl ProblemData {
    pub fn get_question(&self, lang: String) -> Result<String> {
        let question = self
            .question            
            .get(&lang)
            .ok_or(Error::NoQuestionForLang {
                name: self.name().clone(),
                lang,
            })?
            .clone();
        Ok(question)
    }

    pub fn get_answer(&self, lang: String) -> Result<String> {
        let answer = self
            .answer            
            .get(&lang)
            .ok_or(Error::NoAnswerForLang {
                name: self.name().clone(),
                lang,
            })?
            .clone();
        Ok(answer)
    }

    pub fn get_solution(&self, lang: String) -> Result<String> {
        let solution = self
            .solution            
            .get(&lang)
            .ok_or(Error::NoSolutionForLang {
                name: self.name().clone(),
                lang,
            })?
            .clone();
        Ok(solution)
    }
}
impl HasDesc for ProblemData {
    fn name(&self) -> &String {
        &self.name
    }
    fn desc(&self) -> &HashMap<String, String> {
        &self.desc
    }
}


// OLD SER/DESER. KEEP JUST IN CASE
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
