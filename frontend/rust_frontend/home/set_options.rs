use dioxus::prelude::*;

use crate::frontend::home::{DifficultyPicker, NumberPicker};
use crate::shared::ProblemSetData;

#[component]
pub fn SetOptions(set_data: Signal<ProblemSetData>) -> Element {
    rsx! {
        div { class: "initial_set_options",
            DifficultyPicker { set_data }
            NumberPicker { set_data }
        }
    }
}
