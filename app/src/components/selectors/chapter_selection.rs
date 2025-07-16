use crate::{
    APP_LANGUAGE, TRANSLATIONS, backend::*, components::selectors::topic_selection::TopicSelection,
};
use dioxus::prelude::*;

#[component]
pub fn ChapterSelection(chapters: Signal<Vec<ChapterData>>) -> Element {
    let mut selected_chapter_name = use_signal(|| Option::<String>::None);
    let mut topics: Signal<Vec<TopicData>> = use_signal(|| Vec::new());
    // If the chapter list has been reset from above
    use_effect(move || {
        if chapters().len() == 0 {
            selected_chapter_name.set(None);
        }
    });
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

    let selection_default = TRANSLATIONS().get_phrase("chapter_selector", APP_LANGUAGE())?;

    rsx! {
        // Chapters
        if chapters().len() > 0 {
            select {
                onchange: move |ev| {
                    topics.set(Vec::new());
                    selected_chapter_name.set(Some(ev.value().to_string()));
                },
                option {
                    value: "",
                    selected: selected_chapter_name().is_none(),
                    disabled: true,
                    "{selection_default}"
                }
                {
                    chapters
                        .iter()
                        .map(|chapter| {
                            let chapter_desc = chapter.get_desc(APP_LANGUAGE())?;
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
