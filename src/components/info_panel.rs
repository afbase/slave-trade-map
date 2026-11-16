//! Right sidebar showing flow details for a selected destination.

use crate::components::FlowTable;
use crate::models::FlowData;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn InfoPanel() -> Element {
    let state = use_context::<AppState>();

    let total_embarked: f64 = state.flows.read().iter().map(|f| f.total_embarked).sum();

    let ta_flows: Vec<FlowData> = state
        .flows
        .read()
        .iter()
        .filter(|f| f.dataset == "transatlantic")
        .cloned()
        .collect();
    let iam_flows: Vec<FlowData> = state
        .flows
        .read()
        .iter()
        .filter(|f| f.dataset == "interamerican")
        .cloned()
        .collect();
    let ta_total: f64 = ta_flows.iter().map(|f| f.total_embarked).sum();
    let iam_total: f64 = iam_flows.iter().map(|f| f.total_embarked).sum();
    let is_combined =
        (state.dataset_mode)() == "combined" && !ta_flows.is_empty() && !iam_flows.is_empty();

    let min_year: Option<i64> = state.flows.read().iter().flat_map(|f| f.first_year).min();
    let max_year: Option<i64> = state.flows.read().iter().flat_map(|f| f.last_year).max();

    let ta_min: Option<i64> = ta_flows.iter().flat_map(|f| f.first_year).min();
    let ta_max: Option<i64> = ta_flows.iter().flat_map(|f| f.last_year).max();
    let iam_min: Option<i64> = iam_flows.iter().flat_map(|f| f.first_year).min();
    let iam_max: Option<i64> = iam_flows.iter().flat_map(|f| f.last_year).max();

    rsx! {
        div { class: "info-panel",
            h2 { "Flow Details" }

            if let Some(origin_code) = (state.selected_origin)() {
                {
                    let origin_info = state.origins
                        .read()
                        .iter()
                        .find(|o| o.code == origin_code)
                        .cloned();
                    let fl: Vec<FlowData> = state.flows.read().clone();
                    let voyages: i64 = fl.iter().map(|f| f.voyage_count).sum();
                    let dest_kind = if (state.mode)() == "regions" { "regions" } else { "ports" };
                    rsx! {
                        if let Some(ref info) = origin_info {
                            div { class: "dest-name", "{info.name}" }
                            div { class: "origin-kind",
                                if info.is_unspecified {
                                    "African embarkation, port not named in the sources"
                                } else {
                                    "African embarkation port"
                                }
                            }
                        }
                        div { class: "dest-stats",
                            "{total_embarked:.0} enslaved persons embarked · {voyages} voyages"
                            if let (Some(y1), Some(y2)) = (min_year, max_year) {
                                if y1 == y2 { " · {y1}" } else { " · {y1} \u{2013} {y2}" }
                            }
                        }
                        div { class: "dataset-section-header ta-header", "Taken to ({dest_kind})" }
                        FlowTable { flows: fl, section_total: total_embarked, by_destination: true }
                        div { class: "methodology-footnote",
                            "Port of embarkation is the imputed principal port of purchase (MJBYPTIMP). Voyages with no recorded destination are not listed."
                        }
                    }
                }
            } else if let Some(dest_code) = (state.selected_dest)() {
                {
                    let dest_info = state.destinations
                        .read()
                        .iter()
                        .find(|d| d.code == dest_code)
                        .cloned();
                    rsx! {
                        if let Some(ref info) = dest_info {
                            div { class: "dest-name", "{info.name}" }
                            if is_combined {
                                div { class: "dest-stats",
                                    "{total_embarked:.0} total enslaved persons · {info.voyage_count} voyages"
                                    if let (Some(y1), Some(y2)) = (min_year, max_year) {
                                        if y1 == y2 { " · {y1}" } else { " · {y1} \u{2013} {y2}" }
                                    }
                                }
                                div { class: "dest-stats-breakdown",
                                    span { class: "ta-accent", "TASTD: {ta_total:.0}" }
                                    if let (Some(y1), Some(y2)) = (ta_min, ta_max) {
                                        span { class: "ta-years", " ({y1}\u{2013}{y2})" }
                                    }
                                    " · "
                                    span { class: "iam-accent", "IAm: {iam_total:.0}" }
                                    if let (Some(y1), Some(y2)) = (iam_min, iam_max) {
                                        span { class: "iam-years", " ({y1}\u{2013}{y2})" }
                                    }
                                }
                            } else {
                                div { class: "dest-stats",
                                    "{total_embarked:.0} enslaved persons · {info.voyage_count} voyages"
                                    if let (Some(y1), Some(y2)) = (min_year, max_year) {
                                        if y1 == y2 { " · {y1}" } else { " · {y1} \u{2013} {y2}" }
                                    }
                                }
                            }
                        }

                        if is_combined {
                            if !ta_flows.is_empty() {
                                div { class: "dataset-section-header ta-header", "Transatlantic Origins" }
                                FlowTable { flows: ta_flows.clone(), section_total: ta_total }
                            }
                            if !iam_flows.is_empty() {
                                div { class: "dataset-section-header iam-header", "Inter-American Origins" }
                                FlowTable { flows: iam_flows.clone(), section_total: iam_total }
                            }
                        } else if !state.flows.read().is_empty() {
                            {
                                let fl: Vec<FlowData> = state.flows.read().clone();
                                rsx! { FlowTable { flows: fl, section_total: total_embarked } }
                            }
                        }

                        div { class: "methodology-footnote",
                            "Transatlantic: embarked count; Inter-American: disembarked count."
                        }
                    }
                }
            } else {
                div { class: "no-selection",
                    "Click a destination to see where its captives came from, or an African port to see where they were taken. You can also search above."
                }
            }
        }
    }
}
