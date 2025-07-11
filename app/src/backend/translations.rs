use crate::{Error, Result};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

pub static TRANSLATIONS: Lazy<Arc<Mutex<Translations>>> = Lazy::new(|| {
    let data = fs::read_to_string("translations.json").expect("Failed to read translations.json");
    let table: TranslationTable =
        serde_json::from_str(&data).expect("Failed to parse translation JSON");
    Arc::new(Mutex::new(Translations::new(table, "sv")))
});

pub type TranslationTable = HashMap<String, HashMap<String, HashMap<String, String>>>;

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
    pub fn get_phrase(&self, group: &str, key: &str) -> Result<String> {
        match self
            .table
            .get(group)
            .and_then(|group_map| group_map.get(key))
            .and_then(|lang_map| lang_map.get(&self.lang))
            .cloned()
        {
            Some(val) => Ok(val),
            None => Err(Error::InvalidTranslationKey {
                group: group.to_string(),
                key: key.to_string(),
                lang: self.lang.to_string(),
            }),
        }
    }
    pub fn get_placeholder_phrase(
        &self,
        group: &str,
        key: &str,
        args: HashMap<&str, &str>,
    ) -> Result<String> {
        if let Some(val) = self
            .table
            .get(group)
            .and_then(|group_map| group_map.get(key))
            .and_then(|lang_map| lang_map.get(&self.lang))
            .cloned()
        {
            Ok(Self::fill_args(&val, &args))
        } else {
            Err(Error::InvalidTranslationKey {
                group: group.to_string(),
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
