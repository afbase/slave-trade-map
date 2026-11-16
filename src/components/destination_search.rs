//! Destination search input with fuzzy-match and browse dropdown.

use crate::components::FuzzySearchInput;
use crate::js_bridge;
use crate::state::AppState;
use crate::utils::fuzzy_score;
use dioxus::prelude::*;

#[component]
pub fn DestinationSearch() -> Element {
    let mut state = use_context::<AppState>();

    rsx! {
        div { class: "search-container",
            div {
                onfocusin: move |_| {
                    state.browse_open.set(false);
                    state.search_open.set(true);
                },
                onfocusout: move |_| {
                    state.search_open.set(false);
                },
                FuzzySearchInput {
                    value: (state.search_query)(),
                    placeholder: if (state.mode)() == "regions" { "Search destination region or African port...".to_string() } else { "Search destination or African port...".to_string() },
                    on_change: move |val: String| {
                        state.search_query.set(val);
                        state.browse_open.set(false);
                        state.search_open.set(true);
                    },
                    on_clear: move |_| {
                        state.search_query.set(String::new());
                    },
                    show_magnifier: true,
                }
            }
            // List icon -- click toggles full destination browse list
            i {
                class: "fa-sharp fa-solid fa-list-dropdown search-icon-list",
                onmousedown: move |_| {
                    let cur = *state.browse_open.peek();
                    state.browse_open.set(!cur);
                    state.search_open.set(false);
                    state.search_query.set(String::new());
                },
            }
            {
                let browse = (state.browse_open)();
                let search = (state.search_open)();
                let q = state.search_query.read().clone();

                // Build the dropdown items
                // (code, name, embarked, is_african_origin)
                let dropdown_items: Vec<(i64, String, f64, bool)> = if browse && q.is_empty() {
                    let mut all: Vec<_> = state.destinations.read().clone();
                    all.sort_by(|a, b| b.total_embarked.partial_cmp(&a.total_embarked).unwrap_or(std::cmp::Ordering::Equal));
                    all.truncate(50);
                    all.into_iter().map(|d| (d.code, d.name, d.total_embarked, false)).collect()
                } else if search && !q.is_empty() {
                    // Destinations and African embarkation places in one ranked list.
                    let mut scored: Vec<((i64, String, f64, bool), i32)> = state.destinations
                        .read()
                        .iter()
                        .filter_map(|d| {
                            fuzzy_score(&q, &d.name)
                                .map(|s| ((d.code, d.name.clone(), d.total_embarked, false), s))
                        })
                        .collect();
                    scored.extend(state.origins.read().iter().filter_map(|o| {
                        fuzzy_score(&q, &o.name)
                            .map(|s| ((o.code, o.name.clone(), o.total_embarked, true), s))
                    }));
                    // Best match first; larger places win ties.
                    scored.sort_by(|a, b| {
                        b.1.cmp(&a.1).then(
                            b.0.2.partial_cmp(&a.0.2).unwrap_or(std::cmp::Ordering::Equal),
                        )
                    });
                    scored.into_iter().take(10).map(|(d, _)| d).collect()
                } else {
                    vec![]
                };

                if !dropdown_items.is_empty() {
                    rsx! {
                        div { class: "search-dropdown",
                            for item in dropdown_items.iter() {
                                {
                                    let code = item.0;
                                    let name = item.1.clone();
                                    let embarked = item.2;
                                    let is_origin = item.3;
                                    rsx! {
                                        div {
                                            class: if is_origin { "search-result search-result-origin" } else { "search-result" },
                                            onmousedown: move |_| {
                                                state.search_query.set(String::new());
                                                state.search_open.set(false);
                                                state.browse_open.set(false);
                                                if is_origin {
                                                    // Make sure the layer is visible, then zoom and select.
                                                    if !*state.show_origins.peek() {
                                                        state.show_origins.set(true);
                                                        state.apply_origin_options();
                                                    }
                                                    js_bridge::focus_origin(code);
                                                    state.select_origin(code);
                                                } else {
                                                    state.select_destination(code);
                                                }
                                            },
                                            "{name}"
                                            if is_origin {
                                                span { class: "search-result-tag", "African port" }
                                            }
                                            span { class: "search-result-stat",
                                                " ({embarked:.0} enslaved)"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}
