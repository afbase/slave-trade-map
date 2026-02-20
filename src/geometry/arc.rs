//! SVG arc and multi-segment path generation.
//! Ported from assets/js/map-helpers.js

/// Generate a quadratic Bezier SVG path string from src to tgt.
/// Returns None if points are degenerate (distance < 1).
pub fn generate_arc(src: [f64; 2], tgt: [f64; 2], bias: Option<&str>) -> Option<String> {
    let dx = tgt[0] - src[0];
    let dy = tgt[1] - src[1];
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 1.0 {
        return None;
    }

    let curve_amount = (dist * 0.3).min(100.0);
    let mx = (src[0] + tgt[0]) / 2.0;
    let my = (src[1] + tgt[1]) / 2.0;
    let angle = dy.atan2(dx);

    let (cxp, cyp) = if bias == Some("south-atlantic") {
        (
            mx + angle.sin() * curve_amount,
            my - angle.cos() * curve_amount * 0.3,
        )
    } else {
        (
            mx - angle.sin() * curve_amount,
            my + angle.cos() * curve_amount * 0.3 - curve_amount * 0.4,
        )
    };

    Some(format!(
        "M {},{} Q {},{} {},{}",
        src[0], src[1], cxp, cyp, tgt[0], tgt[1]
    ))
}

/// Generate a smooth multi-segment path through all points (Catmull-Rom -> cubic Bezier).
/// Returns None for fewer than 2 points or degenerate inputs.
pub fn generate_multi_segment_path(points: &[[f64; 2]]) -> Option<String> {
    if points.len() < 2 {
        return None;
    }
    if points.len() == 2 {
        return generate_arc(points[0], points[1], None);
    }

    // Guard 1: Proximity filter -- remove consecutive points closer than 8px
    let min_segment_px: f64 = 8.0;
    let mut filtered: Vec<[f64; 2]> = vec![points[0]];
    for f in 1..points.len() {
        let prev = filtered[filtered.len() - 1];
        let cur = points[f];
        let fdist = ((cur[0] - prev[0]).powi(2) + (cur[1] - prev[1]).powi(2)).sqrt();
        if fdist >= min_segment_px || f == points.len() - 1 {
            filtered.push(cur);
        }
    }
    let mut pts = filtered;
    if pts.len() < 2 {
        return None;
    }
    if pts.len() == 2 {
        return generate_arc(pts[0], pts[1], None);
    }

    // Guard 2: X-monotonicity check -- drop penultimate point if last segments backtrack
    if pts.len() >= 3 {
        let last = pts[pts.len() - 1];
        let penult = pts[pts.len() - 2];
        let ante = pts[pts.len() - 3];
        let dir_ap = penult[0] - ante[0];
        let dir_pl = last[0] - penult[0];
        if (dir_ap > 0.0 && dir_pl < -5.0) || (dir_ap < 0.0 && dir_pl > 5.0) {
            pts.remove(pts.len() - 2);
        }
    }
    if pts.len() < 2 {
        return None;
    }
    if pts.len() == 2 {
        return generate_arc(pts[0], pts[1], None);
    }

    // Catmull-Rom to cubic Bezier conversion for smooth path through all points
    let n = pts.len();
    let mut d = format!("M {},{}", pts[0][0], pts[0][1]);

    for i in 0..n - 1 {
        let p0 = pts[if i == 0 { 0 } else { i - 1 }];
        let p1 = pts[i];
        let p2 = pts[i + 1];
        let p3 = pts[(i + 2).min(n - 1)];

        let (cp1, cp2) = catmull_rom_control_points(p0, p1, p2, p3);

        d.push_str(&format!(
            " C {},{} {},{} {},{}",
            cp1[0], cp1[1], cp2[0], cp2[1], p2[0], p2[1]
        ));
    }

    Some(d)
}

// ---------------------------------------------------------------------------
// Catmull-Rom control points — SIMD128 path
//
// Computes (cp1x, cp1y) and (cp2x, cp2y) simultaneously using f64x2,
// treating each point's (x, y) pair as a single SIMD lane pair.
//   cp1 = p1 + (p2 - p0) / 6
//   cp2 = p2 - (p3 - p1) / 6
// ---------------------------------------------------------------------------

#[cfg(target_feature = "simd128")]
#[inline(always)]
fn catmull_rom_control_points(
    p0: [f64; 2],
    p1: [f64; 2],
    p2: [f64; 2],
    p3: [f64; 2],
) -> ([f64; 2], [f64; 2]) {
    use std::arch::wasm32::*;

    let p0v = f64x2(p0[0], p0[1]);
    let p1v = f64x2(p1[0], p1[1]);
    let p2v = f64x2(p2[0], p2[1]);
    let p3v = f64x2(p3[0], p3[1]);
    let sixth = f64x2_splat(6.0);

    // cp1 = p1 + (p2 - p0) / 6
    let cp1v = f64x2_add(p1v, f64x2_div(f64x2_sub(p2v, p0v), sixth));
    // cp2 = p2 - (p3 - p1) / 6
    let cp2v = f64x2_sub(p2v, f64x2_div(f64x2_sub(p3v, p1v), sixth));

    (
        [f64x2_extract_lane::<0>(cp1v), f64x2_extract_lane::<1>(cp1v)],
        [f64x2_extract_lane::<0>(cp2v), f64x2_extract_lane::<1>(cp2v)],
    )
}

// ---------------------------------------------------------------------------
// Catmull-Rom control points — scalar fallback path
// ---------------------------------------------------------------------------

#[cfg(not(target_feature = "simd128"))]
#[inline(always)]
fn catmull_rom_control_points(
    p0: [f64; 2],
    p1: [f64; 2],
    p2: [f64; 2],
    p3: [f64; 2],
) -> ([f64; 2], [f64; 2]) {
    let cp1 = [p1[0] + (p2[0] - p0[0]) / 6.0, p1[1] + (p2[1] - p0[1]) / 6.0];
    let cp2 = [p2[0] - (p3[0] - p1[0]) / 6.0, p2[1] - (p3[1] - p1[1]) / 6.0];
    (cp1, cp2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_format() {
        let result = generate_arc([0.0, 0.0], [100.0, 50.0], None);
        assert!(result.is_some());
        let s = result.unwrap();
        assert!(s.starts_with("M "), "should start with M: {}", s);
        assert!(s.contains(" Q "), "should have Q: {}", s);
    }

    #[test]
    fn test_arc_degenerate() {
        let result = generate_arc([0.0, 0.0], [0.5, 0.0], None);
        assert!(result.is_none(), "degenerate should return None");
    }

    #[test]
    fn test_arc_south_atlantic_bias() {
        let result = generate_arc([0.0, 0.0], [100.0, 50.0], Some("south-atlantic"));
        assert!(result.is_some());
        let s = result.unwrap();
        assert!(s.starts_with("M "), "should start with M: {}", s);
        assert!(s.contains(" Q "), "should have Q: {}", s);
    }

    #[test]
    fn test_multi_segment() {
        let pts = vec![[0.0, 100.0], [100.0, 50.0], [200.0, 80.0], [300.0, 30.0]];
        let result = generate_multi_segment_path(&pts);
        assert!(result.is_some());
        assert!(result.unwrap().contains(" C "), "should have cubic bezier");
    }

    #[test]
    fn test_multi_segment_two_pts() {
        let pts = vec![[0.0, 0.0], [100.0, 100.0]];
        let result = generate_multi_segment_path(&pts);
        assert!(result.is_some(), "2 points should work via arc fallback");
    }

    #[test]
    fn test_multi_segment_empty() {
        let pts: Vec<[f64; 2]> = vec![];
        assert!(generate_multi_segment_path(&pts).is_none());
    }

    #[test]
    fn test_multi_segment_one_point() {
        let pts = vec![[50.0, 50.0]];
        assert!(generate_multi_segment_path(&pts).is_none());
    }

    #[test]
    fn test_proximity_filter() {
        // Points 1 and 2 are very close, should be filtered out
        let pts = vec![
            [0.0, 0.0],
            [1.0, 1.0], // too close to [0,0], should be filtered
            [100.0, 50.0],
            [200.0, 80.0],
        ];
        let result = generate_multi_segment_path(&pts);
        assert!(result.is_some());
    }

    #[test]
    fn test_catmull_rom_control_points_collinear() {
        // For equally spaced collinear points, control points should lie on the line.
        let p0 = [0.0, 0.0];
        let p1 = [10.0, 0.0];
        let p2 = [20.0, 0.0];
        let p3 = [30.0, 0.0];
        let (cp1, cp2) = catmull_rom_control_points(p0, p1, p2, p3);
        // cp1y and cp2y should be 0 (on the x-axis)
        assert!(cp1[1].abs() < 1e-10, "cp1 y should be 0: {}", cp1[1]);
        assert!(cp2[1].abs() < 1e-10, "cp2 y should be 0: {}", cp2[1]);
        // cp1x = 10 + (20-0)/6 ≈ 13.33, cp2x = 20 - (30-10)/6 ≈ 16.67
        assert!((cp1[0] - (10.0 + 20.0 / 6.0)).abs() < 1e-10);
        assert!((cp2[0] - (20.0 - 20.0 / 6.0)).abs() < 1e-10);
    }
}
