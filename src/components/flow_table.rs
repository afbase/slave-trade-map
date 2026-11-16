//! Reusable data table for origin flow breakdowns.

use crate::models::FlowData;
use dioxus::prelude::*;

/// Renders a table of origin flows with enslaved counts, percentages, and voyage counts.
#[component]
pub fn FlowTable(
    flows: Vec<FlowData>,
    section_total: f64,
    /// List each flow by its destination (used when an African place is selected).
    #[props(default = false)]
    by_destination: bool,
) -> Element {
    rsx! {
        table { class: "data-table",
            thead {
                tr {
                    th { if by_destination { "Destination" } else { "Origin" } }
                    th { "Enslaved" }
                    th { "%" }
                    th { "Voyages" }
                }
            }
            tbody {
                for flow in flows.iter() {
                    {
                        let pct = if section_total > 0.0 {
                            flow.total_embarked / section_total * 100.0
                        } else {
                            0.0
                        };
                        // Rows and map lines cross-highlight on this code.
                        let sc = if by_destination {
                            flow.target_code.to_string()
                        } else {
                            flow.source_code.to_string()
                        };
                        let name = if by_destination {
                            flow.target_name.clone()
                        } else {
                            flow.source_name.clone()
                        };
                        let ds = flow.dataset.clone();
                        rsx! {
                            tr {
                                "data-source-code": sc,
                                "data-dataset": ds,
                                td { "{name}" }
                                td { class: "num", "{flow.total_embarked:.0}" }
                                td { class: "num", "{pct:.1}" }
                                td { class: "num", "{flow.voyage_count}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
