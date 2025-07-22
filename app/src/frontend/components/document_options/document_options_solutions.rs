use dioxus::prelude::*;

use crate::shared::{DocumentOptions, WriteSolutions};

#[component]
pub fn DocumentOptionsWriteSolution(options: Signal<DocumentOptions>) -> Element {
    let str_to_solution = |val: &str| match val {
        "first" => WriteSolutions::First,
        "all" => WriteSolutions::All,
        _ => WriteSolutions::None,
    };
    let solution_to_str = |val: WriteSolutions| match val {
        WriteSolutions::First => "first",
        WriteSolutions::All => "all",
        WriteSolutions::None => "none",
    };
    let mut value = use_signal(|| "first");
    use_effect(move || {
        value.set(solution_to_str(options().write_solutions));
    });
    rsx! {
        div {
            p { "Lösningar:" }
            select {
                value: "{value()}",
                class: "select with_arrow",
                onchange: move |evt| {
                    options.write().write_solutions = str_to_solution(&evt.value());
                },
                option { value: "all", "Alltid" }
                option { value: "first", "Första" }
                option { value: "none", "Aldrig" }
            }
        }
    }
}
