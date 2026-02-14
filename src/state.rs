//! Application state managed via Dioxus context.
//!
//! `AppState` bundles all reactive signals into a single struct provided via
//! `use_context_provider`. Child components retrieve it with `use_context::<AppState>()`.

use crate::db::Database;
use crate::js_bridge;
use crate::models::{DatasetMode, DestinationInfo, FlowData, VoyageRecord};
use crate::utils::dataset_mode_from_str;
use dioxus::prelude::*;
use std::collections::HashSet;

/// All application state as reactive signals.
#[derive(Clone, Copy)]
pub struct AppState {
    pub db: Signal<Option<Database>>,
    pub mode: Signal<String>,
    pub dataset_mode: Signal<String>,
    pub selected_dest: Signal<Option<i64>>,
    pub year_range: Signal<(i64, i64)>,
    pub current_min: Signal<i64>,
    pub current_max: Signal<i64>,
    pub flows: Signal<Vec<FlowData>>,
    pub destinations: Signal<Vec<DestinationInfo>>,
    pub loading: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
    pub search_query: Signal<String>,
    pub search_open: Signal<bool>,
    pub browse_open: Signal<bool>,
    pub voyage_data: Signal<Vec<VoyageRecord>>,
    pub open_origins: Signal<HashSet<i64>>,
    pub expanded_voyages: Signal<HashSet<i64>>,
    pub show_about: Signal<bool>,
}

impl AppState {
    /// Create a new `AppState` with default signal values.
    pub fn new() -> Self {
        Self {
            db: Signal::new(None),
            mode: Signal::new("regions".to_string()),
            dataset_mode: Signal::new("transatlantic".to_string()),
            selected_dest: Signal::new(None),
            year_range: Signal::new((1514i64, 1866i64)),
            current_min: Signal::new(1514i64),
            current_max: Signal::new(1866i64),
            flows: Signal::new(Vec::new()),
            destinations: Signal::new(Vec::new()),
            loading: Signal::new(true),
            error_msg: Signal::new(None),
            search_query: Signal::new(String::new()),
            search_open: Signal::new(false),
            browse_open: Signal::new(false),
            voyage_data: Signal::new(Vec::new()),
            open_origins: Signal::new(HashSet::new()),
            expanded_voyages: Signal::new(HashSet::new()),
            show_about: Signal::new(false),
        }
    }

    /// Select a destination by code: query flows + voyages, render on map.
    pub fn select_destination(&mut self, code: i64) {
        self.selected_dest.set(Some(code));
        self.open_origins.set(HashSet::new());
        self.expanded_voyages.set(HashSet::new());

        if let Some(db) = self.db.read().as_ref() {
            let is_regions = *self.mode.peek() == "regions";
            let min = *self.current_min.peek();
            let max = *self.current_max.peek();
            let ds = dataset_mode_from_str(&self.dataset_mode.peek());

            let result = if is_regions {
                db.get_flows_to_region(code, min, max, ds)
            } else {
                db.get_flows_to_port(code, min, max, ds)
            };

            match result {
                Ok(flow_data) => {
                    js_bridge::render_flows(&flow_data, code, ds);
                    self.flows.set(flow_data);

                    // Render timeline
                    let yearly_result = if is_regions {
                        db.get_yearly_flows_to_region(code, min, max, ds)
                    } else {
                        db.get_yearly_flows_to_port(code, min, max, ds)
                    };
                    match yearly_result {
                        Ok(yearly) => js_bridge::render_timeline(&yearly, min, max),
                        Err(_) => js_bridge::clear_timeline(),
                    }
                }
                Err(e) => {
                    self.error_msg.set(Some(format!("Query failed: {}", e)));
                }
            }

            let voyage_result = if is_regions {
                db.get_voyages_for_region(code, min, max, ds)
            } else {
                db.get_voyages_for_port(code, min, max, ds)
            };
            match voyage_result {
                Ok(voyages) => self.voyage_data.set(voyages),
                Err(_) => self.voyage_data.set(vec![]),
            }
        }
    }

    /// Refresh destination dots on the map for the given mode/dataset/time range.
    pub fn refresh_destinations(&mut self, mode_str: &str, ds: DatasetMode, min: i64, max: i64) {
        if let Some(db) = self.db.read().as_ref() {
            let result = if mode_str == "regions" {
                db.get_destination_regions(min, max, ds)
            } else {
                db.get_destination_ports(min, max, ds)
            };
            if let Ok(dests) = result {
                self.destinations.set(dests.clone());
                js_bridge::clear_flows();
                js_bridge::clear_timeline();
                js_bridge::render_destinations(&dests, mode_str);
            }
        }
    }

    /// Switch dataset mode (transatlantic/interamerican/combined), reset selection, refresh map.
    pub fn set_dataset_mode(&mut self, ds_str: &str) {
        if (self.dataset_mode)() == ds_str {
            return;
        }
        self.dataset_mode.set(ds_str.to_string());
        self.selected_dest.set(None);
        self.flows.set(vec![]);
        self.voyage_data.set(vec![]);
        self.search_query.set(String::new());

        let ds = dataset_mode_from_str(ds_str);
        if let Some(db) = self.db.read().as_ref() {
            if let Ok((min, max)) = db.get_year_range(ds) {
                self.year_range.set((min, max));
                self.current_min.set(min);
                self.current_max.set(max);
            }
        }
        let mode_str = self.mode.peek().clone();
        let min = *self.current_min.peek();
        let max = *self.current_max.peek();
        self.refresh_destinations(&mode_str, ds, min, max);
    }

    /// Switch between regions and ports mode.
    pub fn set_mode(&mut self, new_mode: &str) {
        if (self.mode)() == new_mode {
            return;
        }
        self.mode.set(new_mode.to_string());
        self.selected_dest.set(None);
        self.flows.set(vec![]);
        self.voyage_data.set(vec![]);
        self.search_query.set(String::new());

        let ds = dataset_mode_from_str(&self.dataset_mode.peek());
        let min = (self.current_min)();
        let max = (self.current_max)();
        self.refresh_destinations(new_mode, ds, min, max);
    }

    /// Handle time range change: refresh destinations, re-query flows if a dest is selected.
    pub fn refresh_after_time_change(&mut self, year_min: i64, year_max: i64) {
        if let Some(db) = self.db.read().as_ref() {
            let mode_str = self.mode.peek().clone();
            let ds = dataset_mode_from_str(&self.dataset_mode.peek());
            let result = if mode_str == "regions" {
                db.get_destination_regions(year_min, year_max, ds)
            } else {
                db.get_destination_ports(year_min, year_max, ds)
            };

            match result {
                Ok(dests) => {
                    self.destinations.set(dests.clone());
                    js_bridge::clear_flows();
                    js_bridge::clear_timeline();
                    js_bridge::render_destinations(&dests, &mode_str);
                    js_bridge::set_time_label(year_min, year_max);

                    let maybe_dest = *self.selected_dest.peek();
                    if let Some(dest_code) = maybe_dest {
                        let flow_result = if mode_str == "regions" {
                            db.get_flows_to_region(dest_code, year_min, year_max, ds)
                        } else {
                            db.get_flows_to_port(dest_code, year_min, year_max, ds)
                        };

                        match flow_result {
                            Ok(flow_data) => {
                                if flow_data.is_empty() {
                                    self.selected_dest.set(None);
                                    self.flows.set(vec![]);
                                    self.voyage_data.set(vec![]);
                                } else {
                                    js_bridge::render_flows(&flow_data, dest_code, ds);
                                    self.flows.set(flow_data);

                                    // Re-render timeline
                                    let yearly_result = if mode_str == "regions" {
                                        db.get_yearly_flows_to_region(dest_code, year_min, year_max, ds)
                                    } else {
                                        db.get_yearly_flows_to_port(dest_code, year_min, year_max, ds)
                                    };
                                    match yearly_result {
                                        Ok(yearly) => js_bridge::render_timeline(&yearly, year_min, year_max),
                                        Err(_) => js_bridge::clear_timeline(),
                                    }

                                    let vr = if mode_str == "regions" {
                                        db.get_voyages_for_region(
                                            dest_code, year_min, year_max, ds,
                                        )
                                    } else {
                                        db.get_voyages_for_port(
                                            dest_code, year_min, year_max, ds,
                                        )
                                    };
                                    self.voyage_data.set(vr.unwrap_or_default());
                                }
                            }
                            Err(_) => {
                                self.selected_dest.set(None);
                                self.flows.set(vec![]);
                                self.voyage_data.set(vec![]);
                            }
                        }
                    } else {
                        self.voyage_data.set(vec![]);
                    }
                }
                Err(e) => self.error_msg.set(Some(format!("Query failed: {}", e))),
            }
        }
    }
}
