//! Expandable accordion of voyages grouped by embarkation origin.

use crate::models::VoyageRecord;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

/// Renders voyage rows grouped by origin region, with expandable detail rows.
#[component]
pub fn VoyageAccordion(
    voyages: Vec<VoyageRecord>,
    mut open_origins: Signal<HashSet<i64>>,
    mut expanded_voyages: Signal<HashSet<i64>>,
) -> Element {
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
                    let arrow = if is_open { "\u{25be}" } else { "\u{25b8}" };
                    let v_plural = if vcount != 1 { "s" } else { "" };
                    let header_stats = format!(" ({} voyage{}, {} enslaved)", vcount, v_plural, emb);
                    rsx! {
                        div {
                            class: "accordion-header",
                            onclick: move |_| {
                                let mut set = open_origins.write();
                                if set.contains(&oc) {
                                    set.remove(&oc);
                                } else {
                                    set.insert(oc);
                                }
                            },
                            span { class: "accordion-arrow", "{arrow} " }
                            span { class: "accordion-origin", "{name}" }
                            span { class: "accordion-stats", "{header_stats}" }
                        }
                        if is_open {
                            div { class: "accordion-body",
                                table { class: "data-table",
                                    thead {
                                        tr {
                                            th { "Ship" }
                                            th { "Captain" }
                                            th { "Year" }
                                            th { class: "num", "Embarked" }
                                            th { class: "num", "Disemb." }
                                            th { class: "num", "Deaths" }
                                        }
                                    }
                                    tbody {
                                        for v in voyages.iter().filter(|v| v.embark_region_code == oc) {
                                            {
                                                let vid = v.voyage_id;
                                                let ship = if v.ship_name.is_empty() { "Unknown".to_string() } else { v.ship_name.clone() };
                                                let captain = if !v.captain_a.is_empty() {
                                                    v.captain_a.clone()
                                                } else {
                                                    "Unknown".to_string()
                                                };
                                                let year_str = v.year_arrived.map(|y| y.to_string()).unwrap_or("-".to_string());
                                                let emb_str = v.embarked.map(|e| format!("{}", e)).unwrap_or("-".to_string());
                                                let dis_str = v.disembarked.map(|d| format!("{}", d)).unwrap_or("-".to_string());
                                                let death_str = match (v.embarked, v.disembarked) {
                                                    (Some(e), Some(d)) if e > d => format!("{}", e - d),
                                                    (Some(_), Some(_)) => "0".to_string(),
                                                    _ => "-".to_string(),
                                                };
                                                let is_expanded = expanded_voyages.read().contains(&vid);
                                                let has_detail = v.embarked_port1.is_some() || v.embarked_port2.is_some()
                                                    || v.disembarked_place1.is_some() || v.disembarked_place2.is_some()
                                                    || !v.embark_port1_name.is_empty() || !v.disembark_place1_name.is_empty()
                                                    || v.linked_voyage_id.is_some();
                                                let row_class = if has_detail { "voyage-row expandable" } else { "voyage-row" };

                                                rsx! {
                                                    tr {
                                                        class: row_class,
                                                        onclick: move |_| {
                                                            let mut set = expanded_voyages.write();
                                                            if set.contains(&vid) {
                                                                set.remove(&vid);
                                                            } else {
                                                                set.insert(vid);
                                                            }
                                                        },
                                                        td { "{ship}" }
                                                        td { "{captain}" }
                                                        td { class: "num", "{year_str}" }
                                                        td { class: "num", "{emb_str}" }
                                                        td { class: "num", "{dis_str}" }
                                                        td { class: "num", "{death_str}" }
                                                    }
                                                    if is_expanded && has_detail {
                                                        {
                                                            let mut detail_lines: Vec<String> = Vec::new();

                                                            if !v.captain_b.is_empty() || !v.captain_c.is_empty() {
                                                                let mut caps = vec![v.captain_a.clone()];
                                                                if !v.captain_b.is_empty() { caps.push(v.captain_b.clone()); }
                                                                if !v.captain_c.is_empty() { caps.push(v.captain_c.clone()); }
                                                                detail_lines.push(format!("Captains: {}", caps.join(", ")));
                                                            }

                                                            let embark_stops: Vec<String> = [
                                                                (&v.embark_port1_name, &v.embarked_port1),
                                                                (&v.embark_port2_name, &v.embarked_port2),
                                                                (&v.embark_port3_name, &v.embarked_port3),
                                                            ].iter().filter_map(|(name, count)| {
                                                                if !name.is_empty() || count.is_some() {
                                                                    let n = if name.is_empty() { "Unknown port" } else { name.as_str() };
                                                                    let c = count.map(|c| format!(" ({})", c)).unwrap_or_default();
                                                                    Some(format!("{}{}", n, c))
                                                                } else {
                                                                    None
                                                                }
                                                            }).collect();
                                                            if !embark_stops.is_empty() {
                                                                detail_lines.push(format!("Embark stops: {}", embark_stops.join(" \u{2192} ")));
                                                            }

                                                            let disembark_stops: Vec<String> = [
                                                                (&v.disembark_place1_name, &v.disembarked_place1),
                                                                (&v.disembark_place2_name, &v.disembarked_place2),
                                                                (&v.disembark_place3_name, &v.disembarked_place3),
                                                            ].iter().filter_map(|(name, count)| {
                                                                if !name.is_empty() || count.is_some() {
                                                                    let n = if name.is_empty() { "Unknown place" } else { name.as_str() };
                                                                    let c = count.map(|c| format!(" ({})", c)).unwrap_or_default();
                                                                    Some(format!("{}{}", n, c))
                                                                } else {
                                                                    None
                                                                }
                                                            }).collect();
                                                            if !disembark_stops.is_empty() {
                                                                detail_lines.push(format!("Disembark stops: {}", disembark_stops.join(" \u{2192} ")));
                                                            }

                                                            if let Some(linked) = v.linked_voyage_id {
                                                                detail_lines.push(format!("Linked to TASTD voyage #{}", linked));
                                                            }

                                                            rsx! {
                                                                tr { class: "voyage-expand-row",
                                                                    td { colspan: "6",
                                                                        div { class: "voyage-detail-content",
                                                                            for line in detail_lines.iter() {
                                                                                div { "{line}" }
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
                            }
                        }
                    }
                }
            }
        }
    }
}
