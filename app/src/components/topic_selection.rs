use crate::backend::{self, HasDesc, ProblemData, ProblemRegistry};
use crate::backend::{ChapterData, CourseData, TopicData};
use crate::frontend_types::Language;
use dioxus::prelude::{server_fn::error::ServerFnErrorErr, *};

#[component]
pub fn TopicSelection() -> Element {
    let language = use_context::<Signal<Language>>();
    let registry_result: Resource<Result<ProblemRegistry, ServerFnErrorErr>> =
        use_resource(move || async move {
            let reg = backend::load_registry()
                .await
                .map_err(ServerFnErrorErr::from)?;
            Ok(reg)
        });
    if let Some(Err(e)) = registry_result.read().as_ref() {
        throw_error(e.clone())
    }
    let mut courses: Signal<Vec<CourseData>> = use_signal(|| Vec::new());
    // Load the courses when we get the Registry
    use_effect(move || {
        if let Some(Ok(registry)) = registry_result() {
            courses.set(registry.courses);
        }
    });
    let mut selected_course_name = use_signal(|| Option::<String>::None);
    let mut chapters: Signal<Vec<ChapterData>> = use_signal(|| Vec::new());
    use_effect(move || {
        if let Some(course_name) = selected_course_name() {
            if let Some(course) = courses().iter().find(|course| course.name == course_name) {
                chapters.set(course.chapters.clone());
            } else {
                throw_error(crate::Error::NoCourseWithCourseName { name: course_name });
            }
        } else {
            chapters.set(Vec::new());
        }
    });
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
    let mut selected_topic_name = use_signal(|| Option::<String>::None);
    let mut problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());
    use_effect(move || {
        if let Some(topic_name) = selected_topic_name() {
            if let Some(topic) = topics().iter().find(|topic| topic.name == topic_name) {
                problems.set(topic.problems.clone());
            } else {
                throw_error(crate::Error::NoTopicWithTopicName { name: topic_name });
            }
        } else {
            problems.set(Vec::new())
        }
    });

    rsx! {
        div { id: "topic-picker",
            h1 { "Topic picker" }

            if courses().len() > 0 {
                select {
                    onchange: move |event| {
                        selected_course_name.set(Some(event.value().to_string()));
                        selected_chapter_name.set(None);
                        selected_topic_name.set(None);
                    },
                    option {
                        value: "",
                        selected: selected_course_name().is_none(),
                        disabled: true,
                        "Select Course"
                    }
                    {
                        courses
                            .iter()
                            .map(|course| {
                                let course_desc = course.get_desc(language().0)?;
                                rsx! {
                                    option { value: course.name.clone(), "{course_desc}" }
                                }
                            })
                    }
                }
            }
            // Chapters
            if chapters().len() > 0 {
                select {
                    onchange: move |ev| {
                        selected_chapter_name.set(Some(ev.value().to_string()));
                        selected_topic_name.set(None);
                    },
                    option {
                        value: "",
                        selected: selected_chapter_name().is_none(),
                        disabled: true,
                        "Select Chapter"
                    }
                    {chapters.iter().map(|chapter| {
                        let chapter_desc = chapter.get_desc(language().0)?;
                        rsx! {
                            option { value: chapter.name.clone(), "{chapter_desc}" }
                    }})}
                }
                // Topics
                if topics().len() > 0 {
                    select {
                        onchange: move |ev| {
                            selected_topic_name.set(Some(ev.value().to_string()));
                        },
                        option {
                            value: "",
                            selected: selected_topic_name().is_none(),
                            disabled: true,
                            "Select Topic"
                        }
                        {topics.iter().map(|topic| {
                            let topic_desc = topic.get_desc(language().0)?;
                            rsx! {
                                option { value: topic.name.clone(), "{topic_desc}" }
                        }})}
                    }
                    // Problems
                    if problems().len() > 0 {
                        ul {
                            {problems.iter().map(|problem| {
                                let problem_desc = problem.get_desc(language().0)?;
                                rsx! {
                                li { "{problem_desc}" }
                            }})}
                        }
                    }
                }
            }
        }
    }
}
