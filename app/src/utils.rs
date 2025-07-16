use crate::APP_LANGUAGE;
use crate::Result;
use crate::TRANSLATIONS;
use crate::backend::Difficulty;

pub fn i18n_lookup<T: Into<String>>(key: T) -> Result<String> {
    TRANSLATIONS().get_phrase(&key.into(), APP_LANGUAGE())
}

pub fn str_to_enum(s: &str) -> Difficulty {
    match s {
        "difficulty_intro" => Difficulty::Intro,
        "difficulty_easy" => Difficulty::Easy,
        "difficulty_medium" => Difficulty::Medium,
        "difficulty_hard" => Difficulty::Hard,
        _ => panic!("Don't call str_to_enum with another string you dummy"),
    }
}

pub fn enum_to_str(d: &Difficulty) -> String {
    let s = match d {
        Difficulty::Intro => "difficulty_intro",
        Difficulty::Easy => "difficulty_easy",
        Difficulty::Medium => "difficulty_medium",
        Difficulty::Hard => "difficulty_hard",
    };
    s.to_string()
}
