use crate::backend::{self, ChapterData, CourseData, HasDesc, ProblemRegistry};
use crate::components::selectors::chapter_selection::ChapterSelection;
use crate::frontend_types::Language;
use dioxus::prelude::{server_fn::error::ServerFnErrorErr, *};

#[component]
pub fn CourseSelection() -> Element {
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

    rsx! {
        div { id: "topic-picker",
            h1 { "Topic picker" }

            if courses().len() > 0 {
                select {
                    onchange: move |event| {
                        selected_course_name.set(Some(event.value().to_string()));
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
            if selected_course_name().is_some() {
                ChapterSelection { chapters }
            }
        }
    }
}
