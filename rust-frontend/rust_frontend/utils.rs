use crate::Error;
use crate::Result;
use crate::frontend::TRANSLATIONS;

pub fn i18n_lookup<T: Into<String>>(key: T) -> Result<String> {
    let key_str: String = key.into();
    TRANSLATIONS().get(&key_str).ok_or(Error::NoSuchKeyExists { key: key_str }).cloned()
}
