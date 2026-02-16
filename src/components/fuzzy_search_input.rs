//! Reusable fuzzy search input with clear button.

use dioxus::prelude::*;

#[component]
pub fn FuzzySearchInput(
    value: String,
    placeholder: String,
    on_change: EventHandler<String>,
    on_clear: EventHandler<()>,
    #[props(default = false)] show_magnifier: bool,
    #[props(default = "search-input")] input_class: &'static str,
) -> Element {
    let has_value = !value.is_empty();
    rsx! {
        div { class: "fuzzy-search-row",
            if show_magnifier {
                i { class: "fa-sharp-duotone fa-light fa-magnifying-glass search-icon" }
            }
            input {
                class: "{input_class}",
                r#type: "text",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| on_change.call(e.value()),
            }
            if has_value {
                i {
                    class: "fa-sharp fa-regular fa-circle-xmark search-clear-btn",
                    onmousedown: move |_| on_clear.call(()),
                }
            }
        }
    }
}
