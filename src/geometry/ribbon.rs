//! Ribbon offset computation for multi-stem flow visualization.
//! Ported from computeRibbonOffsets / taperAlpha in assets/js/route-planner.js

use crate::geometry::frenet::FrenetFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RibbonEntry {
    pub left: Vec<[f64; 2]>,
    pub right: Vec<[f64; 2]>,
    pub center: Vec<[f64; 2]>,
}

/// Cubic ease-out taper: 1 - 3t^2 + 2t^3
/// Returns 1.0 at t=0, 0.0 at t=1, monotone decreasing.
pub fn taper_alpha(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - 3.0 * t * t + 2.0 * t * t * t
}

/// Compute ribbon offset paths along spine using Frenet frame.
/// Ports computeRibbonOffsets from route-planner.js.
///
/// For each of n_stems ribbons, computes center/left/right polylines
/// offset from the spine by the Frenet normal, modulated by taper and curvature.
pub fn compute_ribbon_offsets(
    n_stems: usize,
    spine: &[[f64; 2]],
    frame: &FrenetFrame,
    spacing: f64,
    sign: f64,
) -> Vec<RibbonEntry> {
    if n_stems == 0 || spine.is_empty() || frame.normals.len() != spine.len() {
        return Vec::new();
    }

    let max_abs_offset = ((n_stems as f64 - 1.0) / 2.0 * spacing).abs();
    let num_samples = spine.len();

    // --- Curvature cap computation ---
    // dot(t[k-1], t[k]) uses only 2 multiplies + 1 add per step. The subsequent
    // acos/division have no wasm32 SIMD equivalents, so the full curv_cap loop
    // stays scalar.
    let mut curv_cap = vec![1.0f64; num_samples];
    for k in 1..num_samples - 1 {
        let dot = frame.tangents[k - 1][0] * frame.tangents[k][0]
            + frame.tangents[k - 1][1] * frame.tangents[k][1];
        let dot = dot.clamp(-1.0, 1.0);
        let angle = dot.acos();
        let dx = spine[k][0] - spine[k - 1][0];
        let dy = spine[k][1] - spine[k - 1][1];
        let seg_len = (dx * dx + dy * dy).sqrt();
        let kappa = if seg_len > 0.001 {
            angle / seg_len
        } else {
            0.0
        };
        let r = if kappa > 0.001 { 1.0 / kappa } else { 1e6 };
        curv_cap[k] = if max_abs_offset > 0.0 {
            (r / max_abs_offset).min(1.0)
        } else {
            1.0
        };
    }

    // --- 3-point moving average smoothing ---
    // Each output[k] depends only on input[k-1..=k+1], so consecutive pairs
    // of outputs are independent and can be computed 2-at-a-time with SIMD.
    let smooth_cap = smooth_moving_average(&curv_cap);

    // --- Per-ribbon, per-sample offset computation ---
    // The hot path: for each sample k, compute center/left/right offsets.
    // All arithmetic is on (x, y) pairs, making f64x2 a natural fit.
    let mut ribbons = Vec::with_capacity(n_stems);
    for i in 0..n_stems {
        let d_i = sign * (i as f64 - (n_stems as f64 - 1.0) / 2.0) * spacing;
        let mut center = Vec::with_capacity(num_samples);
        let mut left = Vec::with_capacity(num_samples);
        let mut right = Vec::with_capacity(num_samples);

        for k in 0..num_samples {
            let t = k as f64 / (num_samples as f64 - 1.0).max(1.0);
            let alpha = taper_alpha(t) * smooth_cap[k];
            let d_i_alpha = d_i * alpha;
            let half = spacing * 0.5 * alpha;

            let (c, l, r) = ribbon_point(spine[k], frame.normals[k], d_i_alpha, half);
            center.push(c);
            left.push(l);
            right.push(r);
        }

        ribbons.push(RibbonEntry {
            left,
            right,
            center,
        });
    }

    ribbons
}

// ---------------------------------------------------------------------------
// Moving average smoothing — SIMD128 path
//
// Processes 2 interior elements per iteration. Elements k and k+1 each read
// from non-overlapping index triples when k+1 < len-1, so they are
// independent and safe to compute simultaneously.
// ---------------------------------------------------------------------------

#[cfg(target_feature = "simd128")]
fn smooth_moving_average(cap: &[f64]) -> Vec<f64> {
    use std::arch::wasm32::*;

    let n = cap.len();
    let mut out = cap.to_vec();
    let third = f64x2_splat(1.0 / 3.0);

    let mut k = 1;
    // Process pairs of interior elements.
    while k + 2 <= n - 1 {
        // Element k: (cap[k-1] + cap[k] + cap[k+1]) / 3
        let a0 = f64x2(cap[k - 1], cap[k]);
        let a1 = f64x2(cap[k], cap[k + 1]);
        let a2 = f64x2(cap[k + 1], cap[k + 2]);
        let sum = f64x2_add(f64x2_add(a0, a1), a2);
        let avg = f64x2_mul(sum, third);
        out[k] = f64x2_extract_lane::<0>(avg);
        out[k + 1] = f64x2_extract_lane::<1>(avg);
        k += 2;
    }

    // Scalar tail for any remaining interior element.
    while k < n - 1 {
        out[k] = (cap[k - 1] + cap[k] + cap[k + 1]) / 3.0;
        k += 1;
    }

    out
}

// ---------------------------------------------------------------------------
// Moving average smoothing — scalar fallback path
// ---------------------------------------------------------------------------

#[cfg(not(target_feature = "simd128"))]
fn smooth_moving_average(cap: &[f64]) -> Vec<f64> {
    let mut out = cap.to_vec();
    for k in 1..cap.len() - 1 {
        out[k] = (cap[k - 1] + cap[k] + cap[k + 1]) / 3.0;
    }
    out
}

// ---------------------------------------------------------------------------
// Ribbon point computation — SIMD128 path
//
// Treats each 2D point as a f64x2 lane pair so that x and y are computed
// simultaneously:
//   center = spine + d_i_alpha * normal
//   left   = center + half * normal
//   right  = center - half * normal
// ---------------------------------------------------------------------------

#[cfg(target_feature = "simd128")]
#[inline(always)]
fn ribbon_point(
    spine: [f64; 2],
    normal: [f64; 2],
    d_i_alpha: f64,
    half: f64,
) -> ([f64; 2], [f64; 2], [f64; 2]) {
    use std::arch::wasm32::*;

    let spine_v = f64x2(spine[0], spine[1]);
    let norm_v = f64x2(normal[0], normal[1]);
    let scale = f64x2_splat(d_i_alpha);
    let half_v = f64x2_splat(half);

    let c_v = f64x2_add(spine_v, f64x2_mul(scale, norm_v));
    let l_v = f64x2_add(c_v, f64x2_mul(half_v, norm_v));
    let r_v = f64x2_sub(c_v, f64x2_mul(half_v, norm_v));

    (
        [f64x2_extract_lane::<0>(c_v), f64x2_extract_lane::<1>(c_v)],
        [f64x2_extract_lane::<0>(l_v), f64x2_extract_lane::<1>(l_v)],
        [f64x2_extract_lane::<0>(r_v), f64x2_extract_lane::<1>(r_v)],
    )
}

// ---------------------------------------------------------------------------
// Ribbon point computation — scalar fallback path
// ---------------------------------------------------------------------------

#[cfg(not(target_feature = "simd128"))]
#[inline(always)]
fn ribbon_point(
    spine: [f64; 2],
    normal: [f64; 2],
    d_i_alpha: f64,
    half: f64,
) -> ([f64; 2], [f64; 2], [f64; 2]) {
    let cx = spine[0] + d_i_alpha * normal[0];
    let cy = spine[1] + d_i_alpha * normal[1];
    let c = [cx, cy];
    let l = [cx + half * normal[0], cy + half * normal[1]];
    let r = [cx - half * normal[0], cy - half * normal[1]];
    (c, l, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::frenet::compute_frenet_frame;
    use crate::geometry::resample::arc_length_reparameterize;

    #[test]
    fn test_taper_alpha_at_zero() {
        assert!((taper_alpha(0.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_taper_alpha_at_one() {
        assert!(taper_alpha(1.0).abs() < 1e-10);
    }

    #[test]
    fn test_taper_alpha_clamped() {
        assert!((taper_alpha(-0.5) - 1.0).abs() < 1e-10);
        assert!(taper_alpha(1.5).abs() < 1e-10);
    }

    #[test]
    fn test_taper_alpha_monotone_decreasing() {
        let steps = 100;
        for i in 0..steps {
            let t1 = i as f64 / steps as f64;
            let t2 = (i + 1) as f64 / steps as f64;
            assert!(
                taper_alpha(t1) >= taper_alpha(t2),
                "not monotone at t={}: {} > {}",
                t1,
                taper_alpha(t1),
                taper_alpha(t2)
            );
        }
    }

    #[test]
    fn test_compute_ribbon_offsets_count() {
        let spine_pts = vec![
            [0.0, 100.0],
            [50.0, 80.0],
            [100.0, 60.0],
            [150.0, 50.0],
            [200.0, 40.0],
        ];
        let resampled = arc_length_reparameterize(&spine_pts, 40);
        let frame = compute_frenet_frame(&resampled);
        let ribbons = compute_ribbon_offsets(5, &resampled, &frame, 12.0, 1.0);
        assert_eq!(ribbons.len(), 5);
        for r in &ribbons {
            assert_eq!(r.center.len(), resampled.len());
            assert_eq!(r.left.len(), resampled.len());
            assert_eq!(r.right.len(), resampled.len());
        }
    }

    #[test]
    fn test_compute_ribbon_offsets_empty() {
        let frame = FrenetFrame {
            tangents: vec![],
            normals: vec![],
        };
        let ribbons = compute_ribbon_offsets(3, &[], &frame, 12.0, 1.0);
        assert!(ribbons.is_empty());
    }

    #[test]
    fn test_compute_ribbon_offsets_zero_stems() {
        let spine = vec![[0.0, 0.0], [100.0, 0.0]];
        let frame = compute_frenet_frame(&spine);
        let ribbons = compute_ribbon_offsets(0, &spine, &frame, 12.0, 1.0);
        assert!(ribbons.is_empty());
    }

    #[test]
    fn test_ribbons_converge_at_end() {
        // At t=1 taper_alpha should be 0, so all ribbons should meet at spine end.
        let spine_pts = vec![[0.0, 0.0], [100.0, 0.0], [200.0, 0.0]];
        let resampled = arc_length_reparameterize(&spine_pts, 20);
        let frame = compute_frenet_frame(&resampled);
        let ribbons = compute_ribbon_offsets(3, &resampled, &frame, 15.0, 1.0);
        let last_idx = resampled.len() - 1;
        for r in &ribbons {
            assert!(
                (r.center[last_idx][0] - resampled[last_idx][0]).abs() < 1e-6,
                "ribbon center should converge to spine end"
            );
            assert!(
                (r.center[last_idx][1] - resampled[last_idx][1]).abs() < 1e-6,
                "ribbon center should converge to spine end"
            );
        }
    }

    #[test]
    fn test_ribbon_point_zero_offset() {
        // With d_i_alpha=0, center should equal spine; left and right should
        // be symmetric around it.
        let spine = [10.0, 20.0];
        let normal = [0.0, 1.0];
        let half = 5.0;
        let (c, l, r) = ribbon_point(spine, normal, 0.0, half);
        assert_eq!(c, spine);
        assert!((l[1] - (spine[1] + half)).abs() < 1e-10);
        assert!((r[1] - (spine[1] - half)).abs() < 1e-10);
    }

    #[test]
    fn test_smooth_moving_average_flat() {
        // A constant input should be unchanged.
        let cap = vec![1.0; 10];
        let out = smooth_moving_average(&cap);
        for v in &out {
            assert!((*v - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_smooth_moving_average_preserves_endpoints() {
        let cap = vec![0.5, 1.0, 2.0, 1.0, 0.5];
        let out = smooth_moving_average(&cap);
        assert_eq!(out[0], cap[0]);
        assert_eq!(out[cap.len() - 1], cap[cap.len() - 1]);
    }
}
