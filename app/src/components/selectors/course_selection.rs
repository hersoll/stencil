use crate::{APP_LANGUAGE, TRANSLATIONS};
use crate::backend::{self, ChapterData, CourseData, HasDesc, ProblemRegistry};
use crate::components::selectors::chapter_selection::ChapterSelection;
use dioxus::prelude::{server_fn::error::ServerFnErrorErr, *};

#[component]
pub fn CourseSelection() -> Element {
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

    let heading = TRANSLATIONS().get_phrase("site_heading", APP_LANGUAGE())?;
    let selection_default = TRANSLATIONS().get_phrase("course_selector", APP_LANGUAGE())?;

    rsx! {
        div { id: "topic-picker",
            h1 { "{heading}" }

            if courses().len() > 0 {
                select {
                    onchange: move |event| {
                        // Reset chapters when we change course
                        chapters.set(Vec::new());
                        selected_course_name.set(Some(event.value().to_string()));
                    },
                    option {
                        value: "",
                        selected: selected_course_name().is_none(),
                        disabled: true,
                        "{selection_default}"
                    }
                    {
                        courses
                            .iter()
                            .map(|course| {
                                let course_desc = course.get_desc(APP_LANGUAGE())?;
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
