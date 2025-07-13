use crate::{
    backend::*, components::selectors::topic_selection::TopicSelection, frontend_types::Language,
};
use dioxus::prelude::*;

#[component]
pub fn ChapterSelection(chapters: Signal<Vec<ChapterData>>) -> Element {
    let language = use_context::<Signal<Language>>();
    let mut selected_chapter_name = use_signal(|| Option::<String>::None);
    let mut topics: Signal<Vec<TopicData>> = use_signal(|| Vec::new());
    use_effect(move || {
        if let Some(chapter_name) = selected_chapter_name() {
            if let Some(chapter) = chapters()
                .iter()
                .find(|chapter| chapter.name == chapter_name)
            {
                topics.set(chapter.topics.clone());
            } else {
                throw_error(crate::Error::NoChapterWithChapterName { name: chapter_name });
            }
        } else {
            topics.set(Vec::new());
        }
    });
    rsx! {
        // Chapters
        if chapters().len() > 0 {
            select {
                onchange: move |ev| {
                    selected_chapter_name.set(Some(ev.value().to_string()));
                },
                option {
                    value: "",
                    selected: selected_chapter_name().is_none(),
                    disabled: true,
                    "Select Chapter"
                }
                {
                    chapters
                        .iter()
                        .map(|chapter| {
                            let chapter_desc = chapter.get_desc(language().0)?;
                            rsx! {
                                option { value: chapter.name.clone(), "{chapter_desc}" }
                            }
                        })
                }
            }
        }

        if topics().len() > 0 {
            TopicSelection { topics }
        }
    }
}
