//! Public WASM API surface — all #[wasm_bindgen] exports.
//! Functions are exposed to window.* scope via init_window_globals().

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

    // classify_dest_quadrant(code: f64) -> String
    let f = Closure::<dyn Fn(f64) -> String>::new(classify_dest_quadrant);
    let _ = js_sys::Reflect::set(win, &"classify_dest_quadrant".into(), f.as_ref());
    f.forget();

    // build_waypoint_chain(source_code: f64, dest_quadrant: String) -> JsValue
    let f = Closure::<dyn Fn(f64, String) -> JsValue>::new(|code, q: String| {
        build_waypoint_chain(code, &q)
    });
    let _ = js_sys::Reflect::set(win, &"build_waypoint_chain".into(), f.as_ref());
    f.forget();

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

    // arc_length_reparameterize(pts_json: String, n: u32) -> String
    // Note: usize not directly usable at JS boundary; use u32 and cast internally
    let f = Closure::<dyn Fn(String, u32) -> String>::new(|pts_json: String, n: u32| {
        arc_length_reparameterize(&pts_json, n as usize)
    });
    let _ = js_sys::Reflect::set(win, &"arc_length_reparameterize".into(), f.as_ref());
    f.forget();

    // compute_frenet_frame(pts_json: String) -> String
    let f = Closure::<dyn Fn(String) -> String>::new(|pts_json: String| {
        compute_frenet_frame(&pts_json)
    });
    let _ = js_sys::Reflect::set(win, &"compute_frenet_frame".into(), f.as_ref());
    f.forget();

    // compute_ribbon_offsets(spine_json, frame_json, n, spacing, sign) -> String
    let f = Closure::<dyn Fn(String, String, u32, f64, f64) -> String>::new(
        |spine_json: String, frame_json: String, n: u32, spacing, sign| {
            compute_ribbon_offsets(&spine_json, &frame_json, n as usize, spacing, sign)
        },
    );
    let _ = js_sys::Reflect::set(win, &"compute_ribbon_offsets".into(), f.as_ref());
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

// Routing exports — wired to real implementations
#[wasm_bindgen]
pub fn classify_dest_quadrant(dest_code: f64) -> String {
    let code = dest_code as i64;
    let coords = crate::geo::coords::region_coords();
    crate::routing::quadrant::classify_dest_quadrant(code, coords)
        .as_str()
        .to_string()
}

#[wasm_bindgen]
pub fn build_waypoint_chain(source_code: f64, dest_quadrant_str: &str) -> JsValue {
    let code = source_code as i64;
    let quadrant = crate::routing::quadrant::Quadrant::from_str(dest_quadrant_str);
    let coords = crate::geo::coords::region_coords();
    match crate::routing::waypoints::build_waypoint_chain(code, &quadrant, coords) {
        Some(chain) => {
            let json = serde_json::to_string(&chain).unwrap_or_default();
            JsValue::from_str(&json)
        }
        None => JsValue::null(),
    }
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

#[wasm_bindgen]
pub fn arc_length_reparameterize(pts_json: &str, n: usize) -> String {
    let pts: Vec<[f64; 2]> = serde_json::from_str(pts_json).unwrap_or_default();
    let result = crate::geometry::resample::arc_length_reparameterize(&pts, n);
    serde_json::to_string(&result).unwrap_or_default()
}

#[wasm_bindgen]
pub fn compute_frenet_frame(pts_json: &str) -> String {
    let pts: Vec<[f64; 2]> = serde_json::from_str(pts_json).unwrap_or_default();
    let frame = crate::geometry::frenet::compute_frenet_frame(&pts);
    serde_json::to_string(&frame).unwrap_or_default()
}

#[wasm_bindgen]
pub fn compute_ribbon_offsets(
    spine_json: &str,
    frame_json: &str,
    n: usize,
    spacing: f64,
    sign: f64,
) -> String {
    let spine: Vec<[f64; 2]> = serde_json::from_str(spine_json).unwrap_or_default();
    let frame: crate::geometry::frenet::FrenetFrame = serde_json::from_str(frame_json)
        .unwrap_or_else(|_| crate::geometry::frenet::compute_frenet_frame(&spine));
    let result = crate::geometry::ribbon::compute_ribbon_offsets(n, &spine, &frame, spacing, sign);
    serde_json::to_string(&result).unwrap_or_default()
}
