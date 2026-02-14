//! Typed wrappers around JS interop via `js_sys::eval()`.
//!
//! D3.js map functions are split across `assets/js/*.js` and concatenated at compile time.
//! They are evaluated as globals (no ES modules) and exposed via `window.*`.
//! This module provides safe Rust wrappers that serialize data and call those globals.

use crate::models::{DatasetMode, DestinationInfo, FlowData, YearlySourceFlow};

static COORDS_JS: &str = include_str!("../assets/js/coords.js");
static COLORS_JS: &str = include_str!("../assets/js/colors.js");
static MAP_HELPERS_JS: &str = include_str!("../assets/js/map-helpers.js");
static MAP_CORE_JS: &str = include_str!("../assets/js/map-core.js");
static MAP_DESTINATIONS_JS: &str = include_str!("../assets/js/map-destinations.js");
static MAP_FLOWS_JS: &str = include_str!("../assets/js/map-flows.js");
static MAP_TIMELINE_JS: &str = include_str!("../assets/js/map-timeline.js");
static MAP_UI_JS: &str = include_str!("../assets/js/map-ui.js");

/// Execute arbitrary JS, wrapping in try/catch to avoid panics.
pub fn call_js(code: &str) {
    let wrapped = format!(
        "try {{ {} }} catch(e) {{ console.warn('JS call failed:', e); }}",
        code
    );
    let _ = js_sys::eval(&wrapped);
}

/// Load and evaluate all map JS scripts (concatenated from split files). Call once at startup.
pub fn load_map_scripts() {
    let all_js = [
        COORDS_JS,
        COLORS_JS,
        MAP_HELPERS_JS,
        MAP_CORE_JS,
        MAP_DESTINATIONS_JS,
        MAP_FLOWS_JS,
        MAP_TIMELINE_JS,
        MAP_UI_JS,
    ]
    .join("\n");
    let _ = js_sys::eval(&all_js);
}

/// Initialize the D3 map and render initial destinations once D3/topojson are available.
pub fn init_map_with_destinations(dest_json: &str, mode: &str) {
    let init_js = format!(
        r#"
        (function() {{
            var waitForD3 = setInterval(function() {{
                if (typeof d3 !== 'undefined' && typeof topojson !== 'undefined') {{
                    clearInterval(waitForD3);
                    initMap();
                    var waitForMap = setInterval(function() {{
                        if (window.mapReady) {{
                            clearInterval(waitForMap);
                            window.__destData = {dest_json};
                            renderDestinations(JSON.stringify(window.__destData), '{mode}');
                        }}
                    }}, 200);
                }}
            }}, 100);
        }})();
        "#
    );
    let _ = js_sys::eval(&init_js);
}

/// Render destination dots on the map.
pub fn render_destinations(destinations: &[DestinationInfo], mode: &str) {
    let json = serde_json::to_string(destinations).unwrap_or_default();
    call_js(&format!(
        "window.__destData = {json}; renderDestinations(JSON.stringify(window.__destData), '{mode}');"
    ));
}

/// Render flow arrows for a selected destination, including legend.
pub fn render_flows(flows: &[FlowData], dest_code: i64, ds: DatasetMode) {
    let json = serde_json::to_string(flows).unwrap_or_default();
    let ds_label = ds.label();
    call_js(&format!(
        "window.__flowData = {json}; renderFlows(JSON.stringify(window.__flowData), {dest_code}); renderLegend('{ds_label}');"
    ));
}

/// Remove all flow arrows and reset dot highlights.
pub fn clear_flows() {
    call_js("clearFlows()");
}

/// Update the time period label on the map.
pub fn set_time_label(min_year: i64, max_year: i64) {
    call_js(&format!("setTimeLabel({min_year}, {max_year})"));
}

/// Render the flow-origin legend.
#[allow(dead_code)]
pub fn render_legend(ds: DatasetMode) {
    let ds_label = ds.label();
    call_js(&format!("renderLegend('{ds_label}')"));
}

/// Render the timeline stacked bar chart for a destination's yearly flows.
pub fn render_timeline(yearly: &[YearlySourceFlow], min_year: i64, max_year: i64) {
    let json = serde_json::to_string(yearly).unwrap_or_default();
    call_js(&format!(
        "window.__yearlyData = {json}; renderTimeline(JSON.stringify(window.__yearlyData), {min_year}, {max_year});"
    ));
}

/// Clear the timeline chart.
pub fn clear_timeline() {
    call_js("clearTimeline()");
}
