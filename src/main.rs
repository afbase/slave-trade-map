//! Transatlantic & Inter-American Slave Trade flow visualization app.
//!
//! A Dioxus 0.7 WASM application that renders an interactive D3.js map of slave trade routes,
//! with filterable time ranges, region/port modes, and per-voyage detail expansion.

mod components;
mod db;
mod geo;
mod geometry;
mod js_bridge;
mod models;
mod routing;
mod state;
mod utils;
mod wasm_exports;

use components::{AboutTab, HeaderBar, InfoPanel, TimeSlider, VoyageDetails};
use db::Database;
use dioxus::prelude::*;
use models::DatasetMode;
use state::AppState;

const CSS_VARIABLES: Asset = asset!("/assets/css/variables.css");
const CSS_RESET: Asset = asset!("/assets/css/reset.css");
const CSS_LAYOUT: Asset = asset!("/assets/css/layout.css");
const CSS_HEADER: Asset = asset!("/assets/css/header.css");
const CSS_TOGGLE: Asset = asset!("/assets/css/toggle.css");
const CSS_SEARCH: Asset = asset!("/assets/css/search.css");
const CSS_MAP: Asset = asset!("/assets/css/map.css");
const CSS_TOOLTIP: Asset = asset!("/assets/css/tooltip.css");
const CSS_INFO_PANEL: Asset = asset!("/assets/css/info-panel.css");
const CSS_TABLES: Asset = asset!("/assets/css/tables.css");
const CSS_TIME_SLIDER: Asset = asset!("/assets/css/time-slider.css");
const CSS_VOYAGE: Asset = asset!("/assets/css/voyage.css");
const CSS_ABOUT: Asset = asset!("/assets/css/about.css");
const CSS_DATASET: Asset = asset!("/assets/css/dataset.css");
const CSS_UTILITY: Asset = asset!("/assets/css/utility.css");

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    wasm_exports::init_window_globals();
    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus::web::Config::new().rootname("main"))
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut state = use_context_provider(AppState::new);

    // Initialize database on mount
    use_effect(move || match Database::new() {
        Ok(db) => {
            if let Ok((min, max)) = db.get_year_range(DatasetMode::Transatlantic) {
                state.year_range.set((min, max));
                state.current_min.set(min);
                state.current_max.set(max);
                if let Ok(dests) = db.get_destination_regions(min, max, DatasetMode::Transatlantic)
                {
                    state.destinations.set(dests);
                }
            }
            state.db.set(Some(db));
            state.loading.set(false);
        }
        Err(e) => {
            state
                .error_msg
                .set(Some(format!("Database init failed: {}", e)));
            state.loading.set(false);
        }
    });

    // Initialize D3 map after DB is loaded
    use_effect(move || {
        if (state.loading)() || state.destinations.read().is_empty() {
            return;
        }

        js_bridge::load_map_scripts();

        let dest_json = serde_json::to_string(&*state.destinations.read()).unwrap_or_default();
        js_bridge::init_map_with_destinations(&dest_json, "regions");
    });

    let on_dest_click = move |evt: Event<FormData>| {
        let val = evt.value();
        if let Ok(code) = val.parse::<i64>() {
            state.select_destination(code);
        }
    };

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1, maximum-scale=5" }
        document::Stylesheet { href: CSS_VARIABLES }
        document::Stylesheet { href: CSS_RESET }
        document::Stylesheet { href: CSS_LAYOUT }
        document::Stylesheet { href: CSS_HEADER }
        document::Stylesheet { href: CSS_TOGGLE }
        document::Stylesheet { href: CSS_SEARCH }
        document::Stylesheet { href: CSS_MAP }
        document::Stylesheet { href: CSS_TOOLTIP }
        document::Stylesheet { href: CSS_INFO_PANEL }
        document::Stylesheet { href: CSS_TABLES }
        document::Stylesheet { href: CSS_TIME_SLIDER }
        document::Stylesheet { href: CSS_VOYAGE }
        document::Stylesheet { href: CSS_ABOUT }
        document::Stylesheet { href: CSS_DATASET }
        document::Stylesheet { href: CSS_UTILITY }

        div { class: "app-container",
            input {
                id: "dest-click-receiver",
                r#type: "hidden",
                value: "",
                oninput: on_dest_click,
            }

            HeaderBar {}

            if let Some(err) = state.error_msg.read().as_ref() {
                div { class: "error-box", "{err}" }
            }

            if *state.loading.read() {
                div { class: "loading", "Loading database..." }
            } else {
                div {
                    style: if (state.show_about)() { "display: none" } else { "" },
                    div { class: "main-content",
                        div { class: "map-area",
                            div { id: "map-container" }
                        }
                        InfoPanel {}
                    }
                    TimeSlider {}
                    if (state.selected_dest)().is_some() && !state.voyage_data.read().is_empty() {
                        VoyageDetails {
                            voyages: state.voyage_data.read().clone(),
                            ds_mode_str: (state.dataset_mode)(),
                            open_origins: state.open_origins,
                            expanded_voyages: state.expanded_voyages,
                        }
                    }
                }
                if (state.show_about)() {
                    AboutTab {}
                }
            }
        }
    }
}
