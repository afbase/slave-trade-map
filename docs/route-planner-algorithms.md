# Route Planner Algorithms

Mathematical documentation for `assets/js/route-planner.js`.

This module routes flow-map stems from African source regions to transatlantic destinations. Each stem is a polyline in screen space (pixels after map projection). The core algorithmic problems are: (1) constructing waypoint polylines that avoid land geometry, and (2) separating stems that run too close together using the discrete Frechet distance with optimal coupling backtrack.

---

> **Status (2026-11):** Africa → Americas stems no longer use the waypoint chains,
> Fréchet separation or spine-and-ribbon offsets described in sections 3–6. Those
> sections now apply only to inter-American and Europe-bound routes (chains and land
> avoidance); the separation code (sections 5–6) has been removed. The current
> transatlantic method is section 0. The Rust copy of the route builder is gone:
> `assets/js/route-planner.js` is the only one.

## 0. Westward Atlantic stems (`planAtlanticStems`)

Goal: every stem runs from its African source to the Americas without ever turning
back east, and stems into one destination never cross.

1. **Gate.** Each source gets an offshore gate just off its own coast
   (`africanGate`). Coasts that face west (Senegambia, Sierra Leone, Liberia, West
   Central Africa) step about 3° west. Coasts that face south (Gulf of Guinea) drop
   to a per-region lane latitude, stepping down from west to east (Gold Coast 3.9°,
   Benin 3.4°, Biafra 2.9°), and follow the lane to 11.5°W, past Cape Palmas.
   Southeast Africa instead follows `CAPE_ROUTE` round the Cape to St Helena.
2. **Junction.** `planJunction` places J after the gates are known: 6° west of the
   westernmost gate, at least 10° east of the destination, at 0.75 × the
   destination's latitude (capped at 13°N and 26°S). North of the equator J is also
   kept west of 25°W so it clears the West African bulge.
3. **Stem.** One cubic Bezier per stem from gate to J (`_stemBezier`), in screen
   space. The first control point continues west at the gate's latitude; the second
   lies on the trunk's tangent, so stems merge into the trunk smoothly. Both control
   points are clamped between J and the gate in x, which keeps the curve moving
   west. Longer stems hold their latitude longer, so they pass outside shorter ones.
4. **Fan-in.** Stems are ordered north to south along the coast (Southeast Africa
   last) and arrive at J spread a few pixels apart across the trunk, in that order.
5. **Land check.** Each stem is sampled against the land polygons. On a hit, its
   first control point is pushed south and the run-in shortened; if stems still
   touch land, J moves 4° west and everything is replanned.
6. **Trunk.** A quadratic arc from J to the destination.

`planAtlanticStems` returns `null` when the destination is not west of the sources
(Europe, Africa, Atlantic islands); callers then fall back to waypoint chains.
`tests/westward-routes.test.mjs` asserts the three properties (no eastward step, no
land, no crossings) for every destination.

## 1. Coordinate Spaces

Two coordinate spaces are in play throughout:

| Space | Notation | Domain |
|-------|----------|--------|
| Geographic | `{lat, lng}` | Spherical coordinates on the globe |
| Screen | `[x, y]` array | Pixel coordinates after `d3.geoNaturalEarth1()` projection |

The map projection `_projection: R^2 -> R^2` maps `[lng, lat] -> [x, y]` (note the GeoJSON argument order). Its inverse `_projection.invert` maps back. All distance computations and nudge offsets happen in screen space; results are projected back to geographic coordinates for storage.

---

## 2. Quadrant Classification

`classifyDestQuadrant(destCode)` partitions the destination space into axis-aligned bounding boxes in geographic coordinates:

```
                        lng
            -100        -59   -50  -30   -15    0
              |          |     |    |      |    |
  lat 35  ----+----------+-----+----+------+----  europe (lat>35, lng>-15)
              |  north-  |     |    |      |
  lat 25  ----+  america |     |    |      |
              |          |     |    |      |
  lat  8  ----+ caribbean+-----+----+      |
              |          |     |    |      |
  lat  5  ----+----------+  brazil- |      |
              |          |   north  |      |
  lat -10 ----+----------+-----+----+      |
              |          |  brazil- |      |
  lat -25 ----+----------+  south  |      |
              |     rio- |         |      |
              |     plata|         |      |
```

This is a simple decision tree on `(lat, lng)` returning one of: `caribbean`, `north-america`, `brazil-north`, `brazil-south`, `rio-plata`, `europe`, `mid-atlantic`.

---

## 3. Waypoint Chain Construction

`buildWaypointChain(sourceCode, destQuadrant)` returns an ordered list of geographic waypoints `W = [w_0, w_1, ..., w_k]` forming the interior control points of a stem. The full polyline is:

```
sourcePoint -> w_0 -> w_1 -> ... -> w_k -> convergencePoint
```

Each of the 9 African source regions (Senegambia through SE Africa) has hand-authored waypoint sequences per destination quadrant. These encode historically plausible maritime corridors (trade wind routes, cape rounding, etc.).

The `60900` (Other Africa) case delegates to the `60600` (Biafra) chain via recursion, prepending coastal waypoints to form the composite route `[otherAfricaCoast, biafraOffshore, ...biafraChain[1:]]`.

---

## 4. Land Avoidance

### 4a. Segment sampling (`sampleSegmentOnLand`)

Given a line segment between two waypoints in geographic space, sample `N` evenly-spaced points along it:

```
p(t) = (1 - t) * wp1 + t * wp2,    t in {0/N, 1/N, ..., N/N}
```

This is simple linear interpolation in `(lat, lng)`. Each sample is tested against land polygons via `d3.geoContains` (point-in-polygon on the sphere).

Note: this interpolation is linear in lat/lng, not along a great circle. For the Atlantic basin at the projection scales used here, the error is negligible.

### 4b. Avoidance pass (`avoidLand`)

For each consecutive segment `(w_i, w_{i+1})` in the waypoint chain, if any sample falls on land, a detour waypoint is inserted at the segment midpoint, shifted offshore:

```
midpoint = (w_i + w_{i+1}) / 2

if midpoint.lat > 0:
    detour = (midpoint.lat,     midpoint.lng - 5)     // push 5 deg west
else:
    detour = (midpoint.lat - 3, midpoint.lng - 5)     // push 3 deg south + 5 deg west
```

The result is a refined chain with the detour spliced in: `[..., w_i, detour, w_{i+1}, ...]`. This is a single-pass heuristic, not iterative.

---

## 5. Discrete Frechet Distance with Coupling Backtrack

### 5a. Problem statement

Given two polylines in screen space:

```
P = [p_0, p_1, ..., p_{n-1}]   (points in R^2)
Q = [q_0, q_1, ..., q_{m-1}]
```

The **discrete Frechet distance** is the minimum over all monotone matchings (couplings) of the maximum matched-pair distance. Informally: a person walks along `P` and a dog walks along `Q`, both moving only forward. The Frechet distance is the minimum leash length that permits both to traverse their full paths.

### 5b. DP recurrence

Define `dp[i][j]` as the Frechet distance of subpaths `P[0..i]` and `Q[0..j]`:

```
dp[i][j] = max( d(p_i, q_j),  min(dp[i-1][j-1], dp[i-1][j], dp[i][j-1]) )
```

Base cases:
```
dp[0][0] = d(p_0, q_0)
dp[i][0] = max(dp[i-1][0], d(p_i, q_0))     for i > 0
dp[0][j] = max(dp[0][j-1], d(p_0, q_j))     for j > 0
```

where `d` is the Euclidean distance in screen-pixel space:

```
d(a, b) = || a - b ||_2 = sqrt( (a_x - b_x)^2 + (a_y - b_y)^2 )
```

The answer is `dp[n-1][m-1]`.

**Complexity**: O(nm) time and space. The `Float64Array` backing gives contiguous memory per row.

### 5c. Coupling backtrack

After filling the DP table, the **optimal coupling** is recovered by backtracking from `(n-1, m-1)` to `(0, 0)`. At each cell `(i, j)`, the predecessor is whichever of `{(i-1,j-1), (i-1,j), (i,j-1)}` has the smallest `dp` value (greedy backtrack along the min-cost path).

The result is a sequence of index pairs:

```
coupling = [(0, 0), (i_1, j_1), ..., (n-1, m-1)]
```

This is a monotonically non-decreasing path through the `n x m` grid from corner to corner. Each pair `(i_k, j_k)` says "at step k of the optimal walk, point `p_{i_k}` on path P is matched to point `q_{j_k}` on path Q."

The coupling is what makes targeted stem separation possible: it identifies *which* waypoints on one path are closest to which points on the other.

### 5d. Why Frechet rather than Hausdorff

The Hausdorff distance `max(max_p min_q d(p,q), max_q min_p d(p,q))` ignores temporal ordering. Two paths could have small Hausdorff distance despite crossing each other (imagine an X shape). The Frechet metric respects the monotone traversal order, which is exactly the constraint for flow-map stems that originate from one coast and converge to a single destination.

---

## 6. Stem Separation (`separateStems`)

### 6a. Overview

Given `k` stems sharing a convergence point, check each adjacent pair `(A, B)` and push `B` away from `A` where they pinch below threshold.

### 6b. Path construction

Each stem's screen-space polyline is:

```
pts = [srcPt, proj(w_0), proj(w_1), ..., proj(w_k), convergePt]
        ^                                              ^
     index 0                                      index k+2
```

where `proj` applies the map projection to each geographic waypoint.

### 6c. Per-waypoint deficit from coupling

After computing `discreteFrechet(pathA.pts, pathB.pts)`, if `result.distance < STEM_MIN_SPACING` (12 px), we extract per-waypoint proximity:

For each waypoint index `w` in `pathB.chain` (which corresponds to `pathB.pts[w+1]`), scan the coupling for all entries `(a, b)` where `b = w + 1`. Track the minimum:

```
waypointMinDist[w] = min over coupling entries with b = w+1 of d(pathA.pts[a], pathB.pts[b])
```

The **local deficit** at waypoint `w` is:

```
localDeficit[w] = STEM_MIN_SPACING - waypointMinDist[w]
```

If `localDeficit[w] <= 0`, this waypoint is already far enough from path A and is not nudged. This is the key improvement over uniform nudging, which applied the same offset everywhere regardless of actual proximity.

### 6d. Nudge vector computation

For each waypoint `w` with `localDeficit > 0`:

1. **Taper**: a sine window prevents discontinuities at chain endpoints:

```
t = (w + 1) / (chainLength + 1)        // in (0, 1), never hitting 0 or 1
taper = sin(pi * t)                     // 0 at endpoints, 1 at midpoint
offset = localDeficit * taper
```

This is a Hann-like window that smoothly ramps the displacement to zero at the source and convergence joins.

2. **Perpendicular direction**: the flow direction at waypoint `w` is approximated as the vector from the waypoint toward the convergence point:

```
v = convergePt - projected(w)           // flow direction vector
```

The 90-degree clockwise rotation of a 2D vector `(dx, dy)` is `(dy, -dx)`. Normalizing:

```
    | dy/||v|| |
n = |          |       (unit perpendicular)
    |-dx/||v|| |
```

This is the standard rotation matrix `R(-pi/2)` applied to the unit flow vector:

```
R(-pi/2) = | cos(-pi/2)  -sin(-pi/2) | = |  0  1 |
           | sin(-pi/2)   cos(-pi/2) |   | -1  0 |
```

Applied to `(dx, dy)/||v||` gives `(dy, -dx)/||v||`.

3. **Displacement**: the new screen position is:

```
newPos = projected(w) + offset * n
```

This is a translation in screen space along the perpendicular, then inverted back to geographic coordinates via `_projection.invert`.

### 6e. Why clockwise rotation specifically

The stems are processed in a consistent spatial order (by source latitude). Rotating clockwise pushes `pathB` to the right of the flow direction relative to `pathA`. Since sources are ordered north-to-south along the African coast, this produces a fan-like spreading where southern stems push southward and northern stems stay put, matching the visual expectation for a converging flow diagram.

---

## 7. Caching

`_waypointCache` is a string-keyed dictionary:

```
key = sourceCode + ":" + destCode + ":" + round(convergePt.x) + ":" + round(convergePt.y)
```

Both `computeDynamicWaypoints` (reads/writes) and `separateStems` (overwrites with nudged chains) use this cache. The convergence point coordinates are rounded to integers to avoid floating-point key mismatches.

---

## 8. Computational Summary

| Function | Time | Space | Notes |
|----------|------|-------|-------|
| `classifyDestQuadrant` | O(1) | O(1) | Bounding-box lookups |
| `buildWaypointChain` | O(1) | O(k) | k = chain length, max ~5 |
| `sampleSegmentOnLand` | O(N * F) | O(1) | N samples, F land features |
| `avoidLand` | O(k * N * F) | O(k) | Per-segment sampling |
| `discreteFrechet` | O(nm) | O(nm) | DP table + backtrack |
| `separateStems` | O(S^2 * nm) | O(nm) | S stems, pairwise Frechet (adjacent only so S-1 pairs) |

In practice: S = 9 stems, n and m are 3-6 points each, so the Frechet computations are negligible. The land-avoidance sampling is the most expensive per-call operation.
