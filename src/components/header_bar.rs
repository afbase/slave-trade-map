//! Top header bar: title, subtitle, and controls (search, toggles, about).

use crate::components::{DatasetToggle, DestinationSearch, ModeToggle, OriginToggles};
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn HeaderBar() -> Element {
    let mut state = use_context::<AppState>();

    let ds_mode_str = (state.dataset_mode)();
    let title_text = match ds_mode_str.as_str() {
        "interamerican" => "Inter-American Slave Trade \u{2014} Flow Map",
        "combined" => "Transatlantic & Inter-American Slave Trade \u{2014} Flow Map",
        _ => "Transatlantic Slave Trade \u{2014} Flow Map",
    };
    let subtitle_text = match ds_mode_str.as_str() {
        "interamerican" => "Based on 11,521 voyages from the Intra-American Slave Trade Database",
        "combined" => "36,108 transatlantic + 11,521 inter-American voyages from SlaveVoyages.org",
        _ => "Based on 36,108 voyages from the Trans-Atlantic Slave Trade Database",
    };

    rsx! {
        div { class: "header",
            div {
                h1 { "{title_text}" }
                div { class: "subtitle", "{subtitle_text}" }
            }
        }
        div { class: "header-controls",
            DestinationSearch {}
            DatasetToggle {}
            ModeToggle {}
            OriginToggles {}
            // About toggle
            div { class: "toggle-labeled",
                span { class: "toggle-label", "Info" }
                button {
                    class: if (state.show_about)() { "about-btn active" } else { "about-btn" },
                    onclick: move |_| {
                        let cur = *state.show_about.peek();
                        state.show_about.set(!cur);
                    },
                    "About"
                }
            }
        }
    }
}
