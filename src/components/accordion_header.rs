//! Accordion header row showing origin name, arrow toggle, and stats.

use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub fn AccordionHeader(
    origin_code: i64,
    name: String,
    voyage_count: usize,
    embarked: i64,
    is_open: bool,
    mut open_origins: Signal<HashSet<i64>>,
) -> Element {
    let arrow = if is_open { "\u{25be}" } else { "\u{25b8}" };
    let v_plural = if voyage_count != 1 { "s" } else { "" };
    let header_stats = format!(
        " ({} voyage{}, {} enslaved)",
        voyage_count, v_plural, embarked
    );

    rsx! {
        div {
            class: "accordion-header",
            onclick: move |_| {
                let mut set = open_origins.write();
                if set.contains(&origin_code) {
                    set.remove(&origin_code);
                } else {
                    set.insert(origin_code);
                }
            },
            span { class: "accordion-arrow", "{arrow} " }
            span { class: "accordion-origin", "{name}" }
            span { class: "accordion-stats", "{header_stats}" }
        }
    }
}
