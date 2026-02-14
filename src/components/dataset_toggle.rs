//! Dataset mode toggle: Transatlantic / Inter-American / Combined.

use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn DatasetToggle() -> Element {
    let mut state = use_context::<AppState>();

    rsx! {
        div { class: "toggle-group",
            button {
                class: if (state.dataset_mode)() == "transatlantic" { "active" } else { "" },
                onclick: move |_| state.set_dataset_mode("transatlantic"),
                "Transatlantic"
            }
            button {
                class: if (state.dataset_mode)() == "interamerican" { "active" } else { "" },
                onclick: move |_| state.set_dataset_mode("interamerican"),
                "Inter-American"
            }
            button {
                class: if (state.dataset_mode)() == "combined" { "active" } else { "" },
                onclick: move |_| state.set_dataset_mode("combined"),
                "Combined"
            }
        }
    }
}
