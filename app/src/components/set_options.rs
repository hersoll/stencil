use dioxus::prelude::*;

use crate::{components::{DifficultyPicker, NumberPicker}, frontend_types::ProblemSetData};

#[component]
pub fn SetOptions(set_data: Signal<ProblemSetData>) -> Element {
    
    rsx!{
        div { class: "initial_set_options",
            DifficultyPicker { set_data }
            NumberPicker { set_data }
        }
    }
}
