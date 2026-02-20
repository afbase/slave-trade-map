//! Arc-length reparameterization for ribbon geometry.
//! Ported from arcLengthReparameterize in assets/js/route-planner.js

/// Resample `points` to `n+1` evenly-spaced samples along arc length.
/// First and last points are preserved exactly.
// The loop variables are used arithmetically (s as f64 / n, j-1), not just as subscripts.
#[allow(clippy::needless_range_loop)]
pub fn arc_length_reparameterize(points: &[[f64; 2]], n: usize) -> Vec<[f64; 2]> {
    if points.len() < 2 || n == 0 {
        return points.to_vec();
    }

    // Build cumulative arc-length array.
    //
    // SIMD pass: compute all segment distances in parallel (2 consecutive
    // differences per iteration), then scalar prefix-sum to accumulate.
    // The prefix sum has a sequential dependency and cannot be vectorized.
    let m = points.len();
    let mut dists = vec![0.0f64; m - 1];
    segment_distances(points, &mut dists);

    let mut cum_len = vec![0.0f64; m];
    for i in 1..m {
        cum_len[i] = cum_len[i - 1] + dists[i - 1];
    }

    let total_len = cum_len[m - 1];
    if total_len < 1.0 {
        return points.to_vec();
    }

    let mut samples = vec![[0.0f64; 2]; n + 1];
    samples[0] = points[0];
    samples[n] = points[m - 1];

    for s in 1..n {
        let target_len = (s as f64 / n as f64) * total_len;

        // Linear scan to find containing segment (matches JS behavior).
        let mut seg_idx = m - 2;
        for j in 1..m {
            if cum_len[j] >= target_len {
                seg_idx = j - 1;
                break;
            }
        }

        let seg_len = cum_len[seg_idx + 1] - cum_len[seg_idx];
        let t = if seg_len > 0.0 {
            (target_len - cum_len[seg_idx]) / seg_len
        } else {
            0.0
        };

        // Lerp both coordinates simultaneously.
        samples[s] = lerp_point(points[seg_idx], points[seg_idx + 1], t);
    }

    samples
}

// ---------------------------------------------------------------------------
// Segment distance computation — SIMD128 path
//
// Processes 2 segments per iteration. Each segment difference (dx, dy) is
// naturally a f64x2, so len = sqrt(dx² + dy²) maps directly to:
//   diff = p[i] - p[i-1]           (f64x2 subtract)
//   len  = sqrt(dot(diff, diff))   (f64x2 mul + add + sqrt)
// ---------------------------------------------------------------------------

#[cfg(target_feature = "simd128")]
fn segment_distances(points: &[[f64; 2]], dists: &mut [f64]) {
    use std::arch::wasm32::*;

    let n = dists.len(); // = points.len() - 1
    let mut i = 0;

    // Process 2 segments per iteration.
    while i + 2 <= n {
        // Segment i: points[i+1] - points[i]
        let ax = f64x2(points[i][0], points[i][1]);
        let bx = f64x2(points[i + 1][0], points[i + 1][1]);
        let d0 = f64x2_sub(bx, ax);

        // Segment i+1: points[i+2] - points[i+1]
        let cx = f64x2(points[i + 1][0], points[i + 1][1]);
        let dx = f64x2(points[i + 2][0], points[i + 2][1]);
        let d1 = f64x2_sub(dx, cx);

        // len for each segment: sqrt(dx² + dy²).
        // d0 = (dx0, dy0) → dot with itself gives dx0²+dy0² ... but we need
        // the horizontal sum per vector, not a cross-product.
        // Use mul + add of the two lanes.
        let sq0 = f64x2_mul(d0, d0); // (dx0², dy0²)
        let sq1 = f64x2_mul(d1, d1); // (dx1², dy1²)

        let len0 = f64x2_extract_lane::<0>(sq0) + f64x2_extract_lane::<1>(sq0);
        let len1 = f64x2_extract_lane::<0>(sq1) + f64x2_extract_lane::<1>(sq1);

        // Pack the two squared lengths and take sqrt together.
        let lens_sq = f64x2(len0, len1);
        let lens = f64x2_sqrt(lens_sq);

        dists[i] = f64x2_extract_lane::<0>(lens);
        dists[i + 1] = f64x2_extract_lane::<1>(lens);

        i += 2;
    }

    // Scalar tail.
    while i < n {
        let dx = points[i + 1][0] - points[i][0];
        let dy = points[i + 1][1] - points[i][1];
        dists[i] = (dx * dx + dy * dy).sqrt();
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// Segment distance computation — scalar fallback path
// ---------------------------------------------------------------------------

#[cfg(not(target_feature = "simd128"))]
fn segment_distances(points: &[[f64; 2]], dists: &mut [f64]) {
    for i in 0..dists.len() {
        let dx = points[i + 1][0] - points[i][0];
        let dy = points[i + 1][1] - points[i][1];
        dists[i] = (dx * dx + dy * dy).sqrt();
    }
}

// ---------------------------------------------------------------------------
// Point lerp — SIMD128 path
//
// Interpolates both coordinates simultaneously: result = a + t * (b - a).
// ---------------------------------------------------------------------------

#[cfg(target_feature = "simd128")]
#[inline(always)]
fn lerp_point(a: [f64; 2], b: [f64; 2], t: f64) -> [f64; 2] {
    use std::arch::wasm32::*;
    let av = f64x2(a[0], a[1]);
    let bv = f64x2(b[0], b[1]);
    let tv = f64x2_splat(t);
    let rv = f64x2_add(av, f64x2_mul(tv, f64x2_sub(bv, av)));
    [f64x2_extract_lane::<0>(rv), f64x2_extract_lane::<1>(rv)]
}

// ---------------------------------------------------------------------------
// Point lerp — scalar fallback path
// ---------------------------------------------------------------------------

#[cfg(not(target_feature = "simd128"))]
#[inline(always)]
fn lerp_point(a: [f64; 2], b: [f64; 2], t: f64) -> [f64; 2] {
    [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_n_plus_1_points() {
        let pts = vec![[0.0, 0.0], [50.0, 0.0], [100.0, 0.0]];
        let result = arc_length_reparameterize(&pts, 40);
        assert_eq!(result.len(), 41);
    }

    #[test]
    fn test_endpoints_preserved() {
        let pts = vec![[10.0, 20.0], [50.0, 30.0], [90.0, 10.0]];
        let result = arc_length_reparameterize(&pts, 10);
        assert_eq!(result[0], pts[0]);
        assert_eq!(result[10], pts[2]);
    }

    #[test]
    fn test_single_point() {
        let pts = vec![[5.0, 5.0]];
        let result = arc_length_reparameterize(&pts, 10);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], [5.0, 5.0]);
    }

    #[test]
    fn test_zero_n() {
        let pts = vec![[0.0, 0.0], [100.0, 0.0]];
        let result = arc_length_reparameterize(&pts, 0);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_collinear_even_spacing() {
        // For a straight line, resampled points should be evenly spaced.
        let pts = vec![[0.0, 0.0], [100.0, 0.0]];
        let result = arc_length_reparameterize(&pts, 4);
        assert_eq!(result.len(), 5);
        for i in 0..5 {
            let expected_x = 25.0 * i as f64;
            assert!(
                (result[i][0] - expected_x).abs() < 1e-10,
                "point {} x: expected {}, got {}",
                i,
                expected_x,
                result[i][0]
            );
            assert!(result[i][1].abs() < 1e-10, "point {} y should be ~0", i);
        }
    }

    #[test]
    fn test_degenerate_distance() {
        // All points at the same location.
        let pts = vec![[5.0, 5.0], [5.0, 5.0], [5.0, 5.0]];
        let result = arc_length_reparameterize(&pts, 10);
        // total_len < 1.0, returns original points.
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_lerp_point_midpoint() {
        let a = [0.0, 0.0];
        let b = [10.0, 20.0];
        let mid = lerp_point(a, b, 0.5);
        assert!((mid[0] - 5.0).abs() < 1e-10);
        assert!((mid[1] - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_lerp_point_endpoints() {
        let a = [3.0, 7.0];
        let b = [9.0, 1.0];
        let start = lerp_point(a, b, 0.0);
        let end = lerp_point(a, b, 1.0);
        assert_eq!(start, a);
        assert_eq!(end, b);
    }
}
