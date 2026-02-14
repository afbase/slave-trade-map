//! Dual-range time slider with year display.

use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn TimeSlider() -> Element {
    let mut state = use_context::<AppState>();

    let on_min_change = move |evt: Event<FormData>| {
        if let Ok(val) = evt.value().parse::<i64>() {
            let max_val = (state.current_max)();
            let new_min = val.min(max_val - 1);
            state.current_min.set(new_min);
            state.refresh_after_time_change(new_min, max_val);
        }
    };

    let on_max_change = move |evt: Event<FormData>| {
        if let Ok(val) = evt.value().parse::<i64>() {
            let min_val = (state.current_min)();
            let new_max = val.max(min_val + 1);
            state.current_max.set(new_max);
            state.refresh_after_time_change(min_val, new_max);
        }
    };

    rsx! {
        div { class: "time-slider-area",
            div { id: "timeline-chart", class: "timeline-chart" }
            div { class: "slider-label",
                span { "Time Period" }
                span { class: "range-display",
                    "{(state.current_min)()} \u{2013} {(state.current_max)()}"
                }
            }
            div { class: "slider-container",
                input {
                    r#type: "range",
                    min: "{(state.year_range)().0}",
                    max: "{(state.year_range)().1}",
                    value: "{(state.current_min)()}",
                    oninput: on_min_change,
                }
                input {
                    r#type: "range",
                    min: "{(state.year_range)().0}",
                    max: "{(state.year_range)().1}",
                    value: "{(state.current_max)()}",
                    oninput: on_max_change,
                }
            }
        }
    }
}
