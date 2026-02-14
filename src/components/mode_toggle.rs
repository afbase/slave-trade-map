//! Region / Port mode toggle.

use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ModeToggle() -> Element {
    let mut state = use_context::<AppState>();

    rsx! {
        div { class: "toggle-group",
            button {
                class: if (state.mode)() == "regions" { "active" } else { "" },
                onclick: move |_| state.set_mode("regions"),
                "Regions"
            }
            button {
                class: if (state.mode)() == "ports" { "active" } else { "" },
                onclick: move |_| state.set_mode("ports"),
                "Ports"
            }
        }
    }
}
