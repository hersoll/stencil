use dioxus::prelude::*;

use crate::frontend::i18n_lookup;
use crate::shared::DocumentOptions;

#[component]
pub fn DocumentOptionsLanguage(options: Signal<DocumentOptions>) -> Element {
    let languages = vec!["sv", "en"];
    let mut value = use_signal(|| String::from("sv"));
    use_effect(move || {
        value.set(options().lang);
    });
    rsx! {
        div {
            p { "{i18n_lookup(\"document_option_language\")?}:" }
            select {
                value: value(),
                class: "select with_arrow",
                onchange: move |evt| {
                    options.write().lang = evt.value();
                },
                for lang in languages {
                    option { value: lang, "{i18n_lookup(\"language_\".to_string() + lang)?}" }
                }
            }
        }
    }
}
