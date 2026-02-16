//! Expandable accordion of voyages grouped by embarkation origin.

use crate::components::{AccordionHeader, FuzzySearchInput, VoyageRow};
use crate::models::VoyageRecord;
use crate::utils::fuzzy_score;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
enum SortColumn {
    Ship,
    Captain,
    Year,
    Embarked,
    Disembarked,
    Deaths,
}

#[derive(Clone, Copy, PartialEq)]
enum SortDirection {
    Asc,
    Desc,
}

fn sort_indicator(col: SortColumn, active: Option<(SortColumn, SortDirection)>) -> Element {
    let (up_class, down_class) = match active {
        Some((c, SortDirection::Asc)) if c == col => ("sort-arrow active", "sort-arrow"),
        Some((c, SortDirection::Desc)) if c == col => ("sort-arrow", "sort-arrow active"),
        _ => ("sort-arrow", "sort-arrow"),
    };
    rsx! {
        span { class: up_class, "\u{25b4}" }
        span { class: down_class, "\u{25be}" }
    }
}

fn deaths_value(v: &VoyageRecord) -> Option<i64> {
    match (v.embarked, v.disembarked) {
        (Some(e), Some(d)) if e > d => Some(e - d),
        (Some(_), Some(_)) => Some(0),
        _ => None,
    }
}

/// Renders voyage rows grouped by origin region, with expandable detail rows.
#[component]
pub fn VoyageAccordion(
    voyages: Vec<VoyageRecord>,
    mut open_origins: Signal<HashSet<i64>>,
    mut expanded_voyages: Signal<HashSet<i64>>,
) -> Element {
    let mut sort_state: Signal<HashMap<i64, (SortColumn, SortDirection)>> =
        use_signal(|| HashMap::new());
    let mut origin_search: Signal<HashMap<i64, String>> = use_signal(|| HashMap::new());

    let mut origin_order: Vec<i64> = Vec::new();
    let mut origin_names: HashMap<i64, String> = HashMap::new();
    let mut origin_voyage_counts: HashMap<i64, usize> = HashMap::new();
    let mut origin_embarked: HashMap<i64, i64> = HashMap::new();

    for v in &voyages {
        origin_names.entry(v.embark_region_code).or_insert_with(|| {
            origin_order.push(v.embark_region_code);
            v.embark_region_name.clone()
        });
        *origin_voyage_counts
            .entry(v.embark_region_code)
            .or_insert(0) += 1;
        *origin_embarked.entry(v.embark_region_code).or_insert(0) += v.embarked.unwrap_or(0);
    }

    rsx! {
        div { class: "accordion",
            for origin_code in origin_order.iter() {
                {
                    let oc = *origin_code;
                    let name = origin_names.get(&oc).cloned().unwrap_or_default();
                    let vcount = *origin_voyage_counts.get(&oc).unwrap_or(&0);
                    let emb = *origin_embarked.get(&oc).unwrap_or(&0);
                    let is_open = open_origins.read().contains(&oc);
                    let active_sort = sort_state.read().get(&oc).copied();
                    let search_text = origin_search.read().get(&oc).cloned().unwrap_or_default();

                    // Collect, filter, and sort voyages for this origin
                    let mut origin_voyages: Vec<VoyageRecord> = voyages
                        .iter()
                        .filter(|v| v.embark_region_code == oc)
                        .cloned()
                        .collect();

                    // Fuzzy filter if search text is non-empty
                    if !search_text.is_empty() {
                        origin_voyages.retain(|v| {
                            let fields = [
                                v.ship_name.as_str(),
                                v.captain_a.as_str(),
                                v.captain_b.as_str(),
                            ];
                            let year_s = v.year_arrived.map(|y| y.to_string()).unwrap_or_default();
                            let emb_s = v.embarked.map(|e| e.to_string()).unwrap_or_default();
                            fields.iter().any(|f| fuzzy_score(&search_text, f).is_some())
                                || fuzzy_score(&search_text, &year_s).is_some()
                                || fuzzy_score(&search_text, &emb_s).is_some()
                        });
                    }

                    // Sort if active
                    if let Some((col, dir)) = active_sort {
                        origin_voyages.sort_by(|a, b| {
                            let ord = match col {
                                SortColumn::Ship => a.ship_name.to_lowercase().cmp(&b.ship_name.to_lowercase()),
                                SortColumn::Captain => a.captain_a.to_lowercase().cmp(&b.captain_a.to_lowercase()),
                                SortColumn::Year => a.year_arrived.cmp(&b.year_arrived),
                                SortColumn::Embarked => a.embarked.cmp(&b.embarked),
                                SortColumn::Disembarked => a.disembarked.cmp(&b.disembarked),
                                SortColumn::Deaths => deaths_value(a).cmp(&deaths_value(b)),
                            };
                            match dir {
                                SortDirection::Asc => ord,
                                SortDirection::Desc => ord.reverse(),
                            }
                        });
                    }

                    rsx! {
                        AccordionHeader {
                            origin_code: oc,
                            name: name,
                            voyage_count: vcount,
                            embarked: emb,
                            is_open: is_open,
                            open_origins: open_origins,
                        }
                        if is_open {
                            div { class: "accordion-body",
                                div { class: "accordion-search",
                                    FuzzySearchInput {
                                        value: search_text.clone(),
                                        placeholder: "Filter voyages...".to_string(),
                                        on_change: move |val: String| {
                                            origin_search.write().insert(oc, val);
                                        },
                                        on_clear: move |_| {
                                            origin_search.write().remove(&oc);
                                        },
                                    }
                                }
                                table { class: "data-table",
                                    thead {
                                        tr {
                                            {
                                                let cols: Vec<(SortColumn, &str, bool)> = vec![
                                                    (SortColumn::Ship, "Ship", false),
                                                    (SortColumn::Captain, "Captain", false),
                                                    (SortColumn::Year, "Year", true),
                                                    (SortColumn::Embarked, "Embarked", true),
                                                    (SortColumn::Disembarked, "Disemb.", true),
                                                    (SortColumn::Deaths, "Deaths", true),
                                                ];
                                                rsx! {
                                                    for (col, label, is_num) in cols.into_iter() {
                                                        th {
                                                            class: if is_num { "num sortable" } else { "sortable" },
                                                            onclick: move |_| {
                                                                let mut ss = sort_state.write();
                                                                let new = match ss.get(&oc) {
                                                                    Some((c, SortDirection::Asc)) if *c == col => (col, SortDirection::Desc),
                                                                    Some((c, SortDirection::Desc)) if *c == col => (col, SortDirection::Asc),
                                                                    _ => (col, SortDirection::Asc),
                                                                };
                                                                ss.insert(oc, new);
                                                            },
                                                            "{label} "
                                                            {sort_indicator(col, active_sort)}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    tbody {
                                        for v in origin_voyages.iter() {
                                            VoyageRow {
                                                voyage: v.clone(),
                                                expanded_voyages: expanded_voyages,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
