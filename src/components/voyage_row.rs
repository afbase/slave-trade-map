//! Single voyage table row with conditional detail expansion.

use crate::components::VoyageDetailRow;
use crate::models::VoyageRecord;
use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub fn VoyageRow(
    voyage: VoyageRecord,
    mut expanded_voyages: Signal<HashSet<i64>>,
) -> Element {
    let vid = voyage.voyage_id;
    let ship = if voyage.ship_name.is_empty() {
        "Unknown".to_string()
    } else {
        voyage.ship_name.clone()
    };
    let captain = if !voyage.captain_a.is_empty() {
        voyage.captain_a.clone()
    } else {
        "Unknown".to_string()
    };
    let year_str = voyage
        .year_arrived
        .map(|y| y.to_string())
        .unwrap_or("-".to_string());
    let emb_str = voyage
        .embarked
        .map(|e| format!("{}", e))
        .unwrap_or("-".to_string());
    let dis_str = voyage
        .disembarked
        .map(|d| format!("{}", d))
        .unwrap_or("-".to_string());
    let death_str = match (voyage.embarked, voyage.disembarked) {
        (Some(e), Some(d)) if e > d => format!("{}", e - d),
        (Some(_), Some(_)) => "0".to_string(),
        _ => "-".to_string(),
    };
    let is_expanded = expanded_voyages.read().contains(&vid);
    let has_detail = voyage.embarked_port1.is_some()
        || voyage.embarked_port2.is_some()
        || voyage.disembarked_place1.is_some()
        || voyage.disembarked_place2.is_some()
        || !voyage.embark_port1_name.is_empty()
        || !voyage.disembark_place1_name.is_empty()
        || voyage.linked_voyage_id.is_some();
    let row_class = if has_detail {
        "voyage-row expandable"
    } else {
        "voyage-row"
    };

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
            VoyageDetailRow { voyage: voyage.clone() }
        }
    }
}
