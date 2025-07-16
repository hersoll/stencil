use crate::Error;
use crate::backend::{self, ChapterData, CourseData, HasDesc, ProblemRegistry};
use crate::components::selectors::chapter_selection::ChapterSelection;
use crate::{APP_LANGUAGE, TRANSLATIONS};
use dioxus::prelude::{server_fn::error::ServerFnErrorErr, *};

fn course_buttons(
    names: Vec<&'static str>,
    group_number: u8,
    courses: Vec<CourseData>,
    mut chapters: Signal<Vec<ChapterData>>,
    mut active_button: Signal<String>,
) -> Element {
    let class_string = format!("course_{group_number}");
    rsx! {
        div { class: "course_group {class_string}",
            for course_name in names {
                {
                    let selected = if active_button() == course_name.to_string() {
                        "selected"
                    } else {
                        ""
                    };
                    let course = courses
                        .iter()
                        .find(|c| c.name == course_name)
                        .ok_or(Error::NoCourseWithCourseName {
                            name: course_name.to_string(),
                        })?
                        .clone();
                    let course_desc = &course.get_desc(APP_LANGUAGE())?;
                    rsx! {
                        button {
                            key: "{course_name.clone()}",
                            class: "course_button {selected}",
                            onclick: move |_| {
                                chapters.set(course.chapters.clone());
                                active_button.set(course_name.to_string());
                            },
                            "{course_desc}"
                        }
                    }
                }
            }

        }
    }
}

#[component]
pub fn CourseSelection() -> Element {
    let registry_result: Resource<Result<ProblemRegistry, ServerFnErrorErr>> =
        use_resource(move || async move {
            let reg = backend::load_registry()
                .await
                .map_err(ServerFnErrorErr::from)?;
            Ok(reg)
        });

    let mut courses: Signal<Vec<CourseData>> = use_signal(|| Vec::new());
    use_effect(move || {
        if let Some(Ok(registry)) = registry_result() {
            courses.set(registry.courses);
        } else if let Some(Err(e)) = registry_result() {
            throw_error(e.clone())
        }
    });
    let active_button: Signal<String> = use_signal(|| String::new());

    let chapters: Signal<Vec<ChapterData>> = use_signal(|| Vec::new());

    //let heading = TRANSLATIONS().get_phrase("site_heading", APP_LANGUAGE())?;
    //let selection_default = TRANSLATIONS().get_phrase("course_selector", APP_LANGUAGE())?;

    rsx! {

            if courses().len() > 0 {
                div { class: "course_buttons",
                    {
                        course_buttons(
                            vec!["ma1a", "ma1b", "ma1c"],
                            1,
                            courses(),
                            chapters,
                            active_button,
                        )
                    }
                                //{course_buttons(vec!["ma2a","ma2b","ma2c"], 2, courses(), chapters)}
                //{course_buttons(vec!["ma3b","ma3c"], 3, courses(), chapters)}
                //{course_buttons(vec!["ma4"], 4, courses(), chapters)}
                //{course_buttons(vec!["ma5"], 5, courses(), chapters)}
                }
            }
            div { class: "picker_div",

            if chapters.len() > 0 {
                ChapterSelection { chapters }
            }
        }
    }
}
