//! Controls for the African embarkation layer.

use crate::components::LabeledToggle;
use crate::js_bridge;
use crate::state::AppState;
use dioxus::prelude::*;

/// Toggles for African ports: layer on/off, region/port level,
/// "port unspecified" rings, and voyages that landed in Africa.
#[component]
pub fn OriginToggles() -> Element {
    let mut state = use_context::<AppState>();
    // The Inter-American dataset has no African embarkations.
    if (state.dataset_mode)() == "interamerican" {
        return rsx! {};
    }
    let show = (state.show_origins)();
    let level = if show {
        (state.origin_level)()
    } else {
        "off".to_string()
    };

    rsx! {
        LabeledToggle {
            label: "African Ports",
            options: vec![
                ("off", "Off"),
                ("auto", "By Zoom"),
                ("regions", "Regions"),
                ("ports", "Ports"),
            ],
            active: level,
            on_change: move |val: String| {
                if val == "off" {
                    state.show_origins.set(false);
                } else {
                    state.show_origins.set(true);
                    state.origin_level.set(val);
                }
                state.apply_origin_options();
            },
        }
        div { class: "toggle-labeled",
            span { class: "toggle-label", "Also Show" }
            div { class: "toggle-group toggle-checks",
                button {
                    class: if (state.show_unspecified)() { "active" } else { "" },
                    title: "Captives whose region is known but whose port is not (codes ending in 99)",
                    disabled: !show,
                    onclick: move |_| {
                        let cur = *state.show_unspecified.peek();
                        state.show_unspecified.set(!cur);
                        state.apply_origin_options();
                    },
                    "Port Unspecified"
                }
                button {
                    class: if (state.show_african_landings)() { "active" } else { "" },
                    title: "Voyages that disembarked in Africa, such as liberated Africans landed at Sierra Leone",
                    onclick: move |_| {
                        let cur = *state.show_african_landings.peek();
                        state.show_african_landings.set(!cur);
                        js_bridge::set_african_landings(!cur);
                    },
                    "African Landings"
                }
            }
        }
    }
}
