//! Public WASM API surface — all #[wasm_bindgen] exports.
//! Functions are exposed to window.* scope via init_window_globals().
//!
//! Route planning is not here: `assets/js/route-planner.js` is the only route
//! builder. Rust supplies geo data (coords, colours) and generic path geometry.

use wasm_bindgen::prelude::*;

/// Called from main() before dioxus::launch(App).
/// Initializes geo data on window and binds Rust closures to window.* so
/// eval()'d JS can call them. Uses wasm_bindgen::Closure for proper binding
/// instead of searching for a wasm module global (which doesn't exist in Dioxus).
pub fn init_window_globals() {
    init_geo_data();

    let win = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let win = win.as_ref();

    // generate_arc(sx, sy, tx, ty, bias) -> JsValue
    let f = Closure::<dyn Fn(f64, f64, f64, f64, String) -> JsValue>::new(
        |sx, sy, tx, ty, bias: String| generate_arc(sx, sy, tx, ty, &bias),
    );
    let _ = js_sys::Reflect::set(win, &"generate_arc".into(), f.as_ref());
    f.forget();

    // generate_multi_segment_path(pts_json: String) -> JsValue
    let f = Closure::<dyn Fn(String) -> JsValue>::new(|pts_json: String| {
        generate_multi_segment_path(&pts_json)
    });
    let _ = js_sys::Reflect::set(win, &"generate_multi_segment_path".into(), f.as_ref());
    f.forget();
}

/// Serialize all geo data to JSON and push to window.* globals.
/// Called at startup so JS functions can read coords/colors/waypoints from Rust.
#[wasm_bindgen]
pub fn init_geo_data() {
    let coords_json =
        serde_json::to_string(crate::geo::coords::region_coords()).unwrap_or_default();
    let waypoints_json =
        serde_json::to_string(crate::geo::coords::static_waypoints()).unwrap_or_default();
    let colors = crate::geo::colors::ColorScheme::new();
    let african_json = serde_json::to_string(&colors.african).unwrap_or_default();
    let american_json = serde_json::to_string(&colors.american).unwrap_or_default();
    fn set_window_var(name: &str, json: &str) {
        if !json.is_empty() {
            let js = format!("window.{}={};", name, json);
            let _ = js_sys::eval(&js);
        }
    }
    set_window_var("__coords", &coords_json);
    set_window_var("__waypoints", &waypoints_json);
    set_window_var("__africanColors", &african_json);
    set_window_var("__americanColors", &american_json);
}

// Geometry exports — wired to real implementations

#[wasm_bindgen]
pub fn generate_arc(sx: f64, sy: f64, tx: f64, ty: f64, bias: &str) -> JsValue {
    let bias_opt = if bias.is_empty() { None } else { Some(bias) };
    match crate::geometry::arc::generate_arc([sx, sy], [tx, ty], bias_opt) {
        Some(s) => JsValue::from_str(&s),
        None => JsValue::null(),
    }
}

#[wasm_bindgen]
pub fn generate_multi_segment_path(pts_json: &str) -> JsValue {
    let pts: Vec<[f64; 2]> = serde_json::from_str(pts_json).unwrap_or_default();
    match crate::geometry::arc::generate_multi_segment_path(&pts) {
        Some(s) => JsValue::from_str(&s),
        None => JsValue::null(),
    }
}
