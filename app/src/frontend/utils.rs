use crate::Result;
use crate::frontend::APP_LANGUAGE;
use crate::frontend::TRANSLATIONS;

pub fn i18n_lookup<T: Into<String>>(key: T) -> Result<String> {
    TRANSLATIONS().get_phrase(&key.into(), APP_LANGUAGE())
}
