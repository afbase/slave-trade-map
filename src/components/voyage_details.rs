//! Voyage details section with dataset-split accordion display.

use crate::components::VoyageAccordion;
use crate::models::VoyageRecord;
use dioxus::prelude::*;
use std::collections::HashSet;

/// Renders the voyage details area, splitting by dataset in combined mode.
#[component]
pub fn VoyageDetails(
    voyages: Vec<VoyageRecord>,
    ds_mode_str: String,
    open_origins: Signal<HashSet<i64>>,
    expanded_voyages: Signal<HashSet<i64>>,
) -> Element {
    let is_combined = ds_mode_str == "combined";

    let ta_voyages: Vec<VoyageRecord> = voyages
        .iter()
        .filter(|v| v.dataset == "transatlantic")
        .cloned()
        .collect();
    let iam_voyages: Vec<VoyageRecord> = voyages
        .iter()
        .filter(|v| v.dataset == "interamerican")
        .cloned()
        .collect();

    let total_voyages = voyages.len();
    let mut origin_set: HashSet<i64> = HashSet::new();
    for v in &voyages {
        origin_set.insert(v.embark_region_code);
    }
    let total_origins = origin_set.len();
    let origin_plural = if total_origins != 1 { "s" } else { "" };
    let summary = format!(
        "{} voyages from {} origin{}",
        total_voyages, total_origins, origin_plural
    );

    rsx! {
        div { class: "voyage-details-area",
            h3 { "Voyage Details" }
            p { class: "methodology-note",
                "Embarked/disembarked totals are imputed estimates. Per-stop counts (shown in expanded rows) are documented values where available."
            }
            p { class: "voyage-count", "{summary}" }

            if is_combined && !ta_voyages.is_empty() && !iam_voyages.is_empty() {
                div { class: "dataset-section-header ta-header", "Transatlantic Voyages ({ta_voyages.len()})" }
                VoyageAccordion { voyages: ta_voyages.clone(), open_origins, expanded_voyages }
                div { class: "dataset-section-header iam-header", "Inter-American Voyages ({iam_voyages.len()})" }
                VoyageAccordion { voyages: iam_voyages.clone(), open_origins, expanded_voyages }
            } else {
                VoyageAccordion { voyages: voyages.clone(), open_origins, expanded_voyages }
            }
        }
    }
}
