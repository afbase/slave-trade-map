//! Generic labeled toggle group: a small header above a row of radio-style buttons.

use dioxus::prelude::*;

#[component]
pub fn LabeledToggle(
    label: &'static str,
    options: Vec<(&'static str, &'static str)>,  // (value_key, display_text)
    active: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "toggle-labeled",
            span { class: "toggle-label", "{label}" }
            div { class: "toggle-group",
                for (value, text) in options.iter() {
                    {
                        let value = *value;
                        let text = *text;
                        let is_active = active == value;
                        rsx! {
                            button {
                                class: if is_active { "active" } else { "" },
                                onclick: move |_| on_change.call(value.to_string()),
                                "{text}"
                            }
                        }
                    }
                }
            }
        }
    }
}
