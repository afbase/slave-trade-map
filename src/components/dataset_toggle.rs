//! Dataset mode toggle: Transatlantic / Inter-American / Combined.

use crate::components::LabeledToggle;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn DatasetToggle() -> Element {
    let mut state = use_context::<AppState>();
    rsx! {
        LabeledToggle {
            label: "Trade Route",
            options: vec![
                ("transatlantic", "Transatlantic"),
                ("interamerican", "Inter-American"),
                ("combined", "Combined"),
            ],
            active: (state.dataset_mode)(),
            on_change: move |val: String| state.set_dataset_mode(&val),
        }
    }
}
