//! Destination search input with fuzzy-match and browse dropdown.

use crate::state::AppState;
use crate::utils::fuzzy_score;
use dioxus::prelude::*;

#[component]
pub fn DestinationSearch() -> Element {
    let mut state = use_context::<AppState>();

    rsx! {
        div { class: "search-container",
            // Decorative magnifying glass
            i { class: "fa-sharp-duotone fa-light fa-magnifying-glass search-icon" }
            input {
                class: "search-input",
                r#type: "text",
                placeholder: if (state.mode)() == "regions" { "Search destination region..." } else { "Search destination port..." },
                value: "{state.search_query}",
                oninput: move |e| {
                    state.search_query.set(e.value());
                    state.browse_open.set(false);
                    state.search_open.set(true);
                },
                onfocusin: move |_| {
                    state.browse_open.set(false);
                    state.search_open.set(true);
                },
                onfocusout: move |_| {
                    state.search_open.set(false);
                },
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
                let dropdown_items = if browse && q.is_empty() {
                    // Browse mode: all destinations sorted by volume
                    let mut all: Vec<_> = state.destinations.read().clone();
                    all.sort_by(|a, b| b.total_embarked.partial_cmp(&a.total_embarked).unwrap_or(std::cmp::Ordering::Equal));
                    all.truncate(50);
                    all
                } else if search && !q.is_empty() {
                    // Search mode: fuzzy-matched results
                    let mut scored: Vec<_> = state.destinations
                        .read()
                        .iter()
                        .filter_map(|d| {
                            fuzzy_score(&q, &d.name).map(|s| (d.clone(), s))
                        })
                        .collect();
                    scored.sort_by_key(|b| std::cmp::Reverse(b.1));
                    scored.into_iter().take(8).map(|(d, _)| d).collect::<Vec<_>>()
                } else {
                    vec![]
                };

                if !dropdown_items.is_empty() {
                    rsx! {
                        div { class: "search-dropdown",
                            for dest in dropdown_items.iter() {
                                {
                                    let code = dest.code;
                                    let name = dest.name.clone();
                                    let embarked = dest.total_embarked;
                                    rsx! {
                                        div {
                                            class: "search-result",
                                            onmousedown: move |_| {
                                                state.search_query.set(String::new());
                                                state.search_open.set(false);
                                                state.browse_open.set(false);
                                                state.select_destination(code);
                                            },
                                            "{name}"
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
