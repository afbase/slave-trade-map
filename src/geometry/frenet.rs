//! Frenet frame computation (tangents + normals) for ribbon geometry.
//! Ported from computeFrenetFrame in assets/js/route-planner.js

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrenetFrame {
    pub tangents: Vec<[f64; 2]>,
    pub normals: Vec<[f64; 2]>,
}

/// Compute tangent and normal vectors at each sample point.
/// Uses central differences for interior, forward/backward at endpoints.
/// Normal = [-ty, tx] (perpendicular to tangent).
pub fn compute_frenet_frame(samples: &[[f64; 2]]) -> FrenetFrame {
    let n = samples.len();
    if n == 0 {
        return FrenetFrame {
            tangents: vec![],
            normals: vec![],
        };
    }

    // Scalar finite-difference pass — endpoint branches don't vectorize cleanly.
    let mut dxs = vec![0.0f64; n];
    let mut dys = vec![0.0f64; n];

    for i in 0..n {
        let (dx, dy) = if n == 1 {
            (1.0, 0.0)
        } else if i == 0 {
            (samples[1][0] - samples[0][0], samples[1][1] - samples[0][1])
        } else if i == n - 1 {
            (
                samples[n - 1][0] - samples[n - 2][0],
                samples[n - 1][1] - samples[n - 2][1],
            )
        } else {
            (
                samples[i + 1][0] - samples[i - 1][0],
                samples[i + 1][1] - samples[i - 1][1],
            )
        };
        dxs[i] = dx;
        dys[i] = dy;
    }

    let mut tangents = vec![[0.0f64; 2]; n];
    let mut normals = vec![[0.0f64; 2]; n];

    normalize(&dxs, &dys, &mut tangents, &mut normals);

    FrenetFrame { tangents, normals }
}

// ---------------------------------------------------------------------------
// Normalization — SIMD128 path
// ---------------------------------------------------------------------------

#[cfg(target_feature = "simd128")]
fn normalize(dxs: &[f64], dys: &[f64], tangents: &mut [[f64; 2]], normals: &mut [[f64; 2]]) {
    use std::arch::wasm32::*;

    let n = dxs.len();
    let threshold = f64x2_splat(0.001);
    let fallback_x = f64x2_splat(1.0);
    let fallback_y = f64x2_splat(0.0);

    let mut i = 0;

    while i + 2 <= n {
        let vdx = f64x2(dxs[i], dxs[i + 1]);
        let vdy = f64x2(dys[i], dys[i + 1]);

        let len_sq = f64x2_add(f64x2_mul(vdx, vdx), f64x2_mul(vdy, vdy));
        let len = f64x2_sqrt(len_sq);

        let degen = f64x2_lt(len, threshold);
        let safe_len = f64x2_pmax(len, threshold);

        let tx = f64x2_div(vdx, safe_len);
        let ty = f64x2_div(vdy, safe_len);

        // Where degenerate: use fallback (1, 0), else use normalized result.
        let tx = v128_bitselect(fallback_x, tx, degen);
        let ty = v128_bitselect(fallback_y, ty, degen);

        // Normal = (-ty, tx)
        let nx = f64x2_neg(ty);
        let ny = tx;

        tangents[i] = [f64x2_extract_lane::<0>(tx), f64x2_extract_lane::<0>(ty)];
        tangents[i + 1] = [f64x2_extract_lane::<1>(tx), f64x2_extract_lane::<1>(ty)];
        normals[i] = [f64x2_extract_lane::<0>(nx), f64x2_extract_lane::<0>(ny)];
        normals[i + 1] = [f64x2_extract_lane::<1>(nx), f64x2_extract_lane::<1>(ny)];

        i += 2;
    }

    // Scalar tail for odd-length inputs.
    if i < n {
        normalize_one(dxs[i], dys[i], &mut tangents[i], &mut normals[i]);
    }
}

// ---------------------------------------------------------------------------
// Normalization — scalar fallback path
// ---------------------------------------------------------------------------

#[cfg(not(target_feature = "simd128"))]
fn normalize(dxs: &[f64], dys: &[f64], tangents: &mut [[f64; 2]], normals: &mut [[f64; 2]]) {
    for i in 0..dxs.len() {
        normalize_one(dxs[i], dys[i], &mut tangents[i], &mut normals[i]);
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

#[inline(always)]
fn normalize_one(dx: f64, dy: f64, t: &mut [f64; 2], n: &mut [f64; 2]) {
    let len = (dx * dx + dy * dy).sqrt();
    let (tx, ty) = if len < 0.001 {
        (1.0, 0.0)
    } else {
        (dx / len, dy / len)
    };
    *t = [tx, ty];
    *n = [-ty, tx];
}

// ---------------------------------------------------------------------------
// Tests — run against whichever path is compiled in
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let frame = compute_frenet_frame(&[]);
        assert!(frame.tangents.is_empty());
        assert!(frame.normals.is_empty());
    }

    #[test]
    fn test_single_point() {
        let frame = compute_frenet_frame(&[[5.0, 5.0]]);
        assert_eq!(frame.tangents.len(), 1);
        assert_eq!(frame.normals.len(), 1);
        assert_eq!(frame.tangents[0], [1.0, 0.0]);
        assert_eq!(frame.normals[0], [0.0, 1.0]);
    }

    #[test]
    fn test_normals_perpendicular() {
        let samples = vec![
            [0.0, 0.0],
            [10.0, 5.0],
            [20.0, 15.0],
            [30.0, 10.0],
            [40.0, 0.0],
        ];
        let frame = compute_frenet_frame(&samples);
        assert_eq!(frame.tangents.len(), samples.len());
        assert_eq!(frame.normals.len(), samples.len());

        for i in 0..samples.len() {
            let dot = frame.tangents[i][0] * frame.normals[i][0]
                + frame.tangents[i][1] * frame.normals[i][1];
            assert!(
                dot.abs() < 1e-10,
                "tangent and normal at {} not perpendicular: dot = {}",
                i,
                dot
            );
        }
    }

    #[test]
    fn test_tangents_unit_length() {
        let samples = vec![[0.0, 0.0], [30.0, 40.0], [60.0, 20.0]];
        let frame = compute_frenet_frame(&samples);
        for i in 0..samples.len() {
            let len = (frame.tangents[i][0].powi(2) + frame.tangents[i][1].powi(2)).sqrt();
            assert!(
                (len - 1.0).abs() < 1e-10,
                "tangent at {} not unit length: {}",
                i,
                len
            );
        }
    }

    #[test]
    fn test_normals_unit_length() {
        let samples = vec![[0.0, 0.0], [30.0, 40.0], [60.0, 20.0]];
        let frame = compute_frenet_frame(&samples);
        for i in 0..samples.len() {
            let len = (frame.normals[i][0].powi(2) + frame.normals[i][1].powi(2)).sqrt();
            assert!(
                (len - 1.0).abs() < 1e-10,
                "normal at {} not unit length: {}",
                i,
                len
            );
        }
    }

    #[test]
    fn test_horizontal_line() {
        let samples = vec![[0.0, 0.0], [10.0, 0.0], [20.0, 0.0]];
        let frame = compute_frenet_frame(&samples);
        for i in 0..3 {
            assert!(
                (frame.tangents[i][0] - 1.0).abs() < 1e-10,
                "horizontal tangent x"
            );
            assert!(frame.tangents[i][1].abs() < 1e-10, "horizontal tangent y");
            assert!(frame.normals[i][0].abs() < 1e-10, "horizontal normal x");
            assert!(
                (frame.normals[i][1] - 1.0).abs() < 1e-10,
                "horizontal normal y"
            );
        }
    }
}
