//! Region / Port mode toggle.

use crate::components::LabeledToggle;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ModeToggle() -> Element {
    let mut state = use_context::<AppState>();
    rsx! {
        LabeledToggle {
            label: "View By",
            options: vec![
                ("regions", "Regions"),
                ("ports", "Ports"),
            ],
            active: (state.mode)(),
            on_change: move |val: String| {
                state.show_about.set(false);
                state.set_mode(&val);
            },
        }
    }
}
