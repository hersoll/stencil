use serde::Deserialize;
use std::{collections::HashMap, fs};

pub type TranslationTable = HashMap<String, HashMap<String, HashMap<String, String>>>;

#[derive(Debug, Deserialize)]
pub struct Translations {
    lang: String,
    table: TranslationTable,
}

impl Translations {
    pub fn new(lang: &str) -> Translations {
        let data =
            fs::read_to_string("translations.json").expect("Failed to read translations.json");
        let table: TranslationTable =
            serde_json::from_str(&data).expect("Failed to parse translation JSON");
        Translations {
            lang: lang.to_string(),
            table: table,
        }
    }
    pub fn get_phrase(&self, group: &str, key: &str) -> String {
        match self
            .table
            .get(group)
            .and_then(|group_map| group_map.get(key))
            .and_then(|lang_map| lang_map.get(&self.lang))
            .cloned()
        {
            Some(val) => val,
            None => panic!(
                "No translation of group {group}, key {key} and lang {}",
                self.lang
            ),
        }
    }
    pub fn get_placeholder_phrase(
        &self,
        group: &str,
        key: &str,
        args: HashMap<&str, &str>,
    ) -> String {
        if let Some(val) = self
            .table
            .get(group)
            .and_then(|group_map| group_map.get(key))
            .and_then(|lang_map| lang_map.get(&self.lang))
            .cloned()
        {
            Self::fill_args(&val, &args)
        } else {
            panic!(
                "No translation of group {group}, key {key} and lang {}",
                self.lang
            )
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
