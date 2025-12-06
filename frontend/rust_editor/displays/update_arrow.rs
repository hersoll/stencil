use dioxus::prelude::*;

#[component]
pub fn UpdateArrow<T: 'static + PartialEq>(
    future: Resource<Result<Vec<T>, ServerFnError>>,
) -> Element {
    rsx! {
        div { class: "update_arrow", onclick: move |_| future.restart(),
            "{char::from_u32(0x27F3).unwrap()}"
        }
    }
}
