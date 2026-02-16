//! Expandable detail row showing captains, embark/disembark stops, and linked voyage.

use crate::models::VoyageRecord;
use dioxus::prelude::*;

#[component]
pub fn VoyageDetailRow(voyage: VoyageRecord) -> Element {
    let mut detail_lines: Vec<String> = Vec::new();

    if !voyage.captain_b.is_empty() || !voyage.captain_c.is_empty() {
        let mut caps = vec![voyage.captain_a.clone()];
        if !voyage.captain_b.is_empty() {
            caps.push(voyage.captain_b.clone());
        }
        if !voyage.captain_c.is_empty() {
            caps.push(voyage.captain_c.clone());
        }
        detail_lines.push(format!("Captains: {}", caps.join(", ")));
    }

    let embark_stops: Vec<String> = [
        (&voyage.embark_port1_name, &voyage.embarked_port1),
        (&voyage.embark_port2_name, &voyage.embarked_port2),
        (&voyage.embark_port3_name, &voyage.embarked_port3),
    ]
    .iter()
    .filter_map(|(name, count)| {
        if !name.is_empty() || count.is_some() {
            let n = if name.is_empty() {
                "Unknown port"
            } else {
                name.as_str()
            };
            let c = count
                .map(|c| format!(" ({})", c))
                .unwrap_or_default();
            Some(format!("{}{}", n, c))
        } else {
            None
        }
    })
    .collect();
    if !embark_stops.is_empty() {
        detail_lines.push(format!(
            "Embark stops: {}",
            embark_stops.join(" \u{2192} ")
        ));
    }

    let disembark_stops: Vec<String> = [
        (&voyage.disembark_place1_name, &voyage.disembarked_place1),
        (&voyage.disembark_place2_name, &voyage.disembarked_place2),
        (&voyage.disembark_place3_name, &voyage.disembarked_place3),
    ]
    .iter()
    .filter_map(|(name, count)| {
        if !name.is_empty() || count.is_some() {
            let n = if name.is_empty() {
                "Unknown place"
            } else {
                name.as_str()
            };
            let c = count
                .map(|c| format!(" ({})", c))
                .unwrap_or_default();
            Some(format!("{}{}", n, c))
        } else {
            None
        }
    })
    .collect();
    if !disembark_stops.is_empty() {
        detail_lines.push(format!(
            "Disembark stops: {}",
            disembark_stops.join(" \u{2192} ")
        ));
    }

    if let Some(linked) = voyage.linked_voyage_id {
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
