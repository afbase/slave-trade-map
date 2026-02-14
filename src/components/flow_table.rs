//! Reusable data table for origin flow breakdowns.

use crate::models::FlowData;
use dioxus::prelude::*;

/// Renders a table of origin flows with enslaved counts, percentages, and voyage counts.
#[component]
pub fn FlowTable(flows: Vec<FlowData>, section_total: f64) -> Element {
    rsx! {
        table { class: "data-table",
            thead {
                tr {
                    th { "Origin" }
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
                        rsx! {
                            tr {
                                td { "{flow.source_name}" }
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
