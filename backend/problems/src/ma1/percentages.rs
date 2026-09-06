use types::lang::Language;

mod calculating_change;
mod decimal_form;
mod identify_change_factors;
mod multiple_changes;

const NEW_SV: &str = "\"nya\"";
const OLD_SV: &str = "\"gamla\"";
const FF_SV: &str = "\"ff\"";
const TIME_SV: &str = "\" tid\"";
const NEW_EN: &str = "\"new\"";
const OLD_EN: &str = "\"old\"";
const FF_EN: &str = "\"change\"";
const TIME_EN: &str = "\" time\"";

pub(crate) fn new_label(lang: Language) -> &'static str {
    use Language::*;
    match lang {
        Sv => NEW_SV,
        En => NEW_EN,
    }
}
pub(crate) fn old_label(lang: Language) -> &'static str {
    use Language::*;
    match lang {
        Sv => OLD_SV,
        En => OLD_EN,
    }
}
pub(crate) fn ff_label(lang: Language) -> &'static str {
    use Language::*;
    match lang {
        Sv => FF_SV,
        En => FF_EN,
    }
}
pub(crate) fn time_label(lang: Language) -> &'static str {
    use Language::*;
    match lang {
        Sv => TIME_SV,
        En => TIME_EN,
    }
}

pub(crate) fn labels(lang: Language) -> (&'static str, &'static str, &'static str, &'static str) {
    (
        new_label(lang),
        old_label(lang),
        ff_label(lang),
        time_label(lang),
    )
}
