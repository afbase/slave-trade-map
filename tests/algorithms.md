# Stem Test Algorithms

This document explains the geometric checks used to verify that trade-route stems render correctly — no physical crossings and smooth parallel flow. The orientation and segment-intersection tests (sections 1–2) are what `westward-routes.test.mjs` uses today. The later sections describe the browser test page for the earlier spine-and-ribbon stems (`stem-intersection-test.html`), which was removed along with that code.

---

## 1. Orientation Test (CCW)

Everything downstream depends on being able to answer one question: given three points, do they make a left turn or a right turn?

Given points **A**, **B**, **C**, compute the cross product of vectors **AB** and **AC**:

```
cross = (C.y - A.y)(B.x - A.x) - (B.y - A.y)(C.x - A.x)
```

This is the z-component of **AB** x **AC**. Its sign tells us the orientation:

| Sign | Meaning |
|------|---------|
| > 0 | A → B → C is a **left turn** (counter-clockwise) |
| < 0 | A → B → C is a **right turn** (clockwise) |
| = 0 | Collinear |

The code uses the strict inequality `>`, so collinear cases return `false`. This is intentional — near-collinear segments that barely graze each other are not counted as crossings.

---

## 2. Segment Intersection

Two segments **P1P2** and **P3P4** cross if and only if:

1. **P1** and **P2** lie on **opposite sides** of the line through **P3P4**, AND
2. **P3** and **P4** lie on **opposite sides** of the line through **P1P2**

"Opposite sides" means the two orientation tests give different results:

```
segmentsCross(P1, P2, P3, P4) =
    ccw(P1, P3, P4) ≠ ccw(P2, P3, P4)      ← P1,P2 straddle line P3P4
  AND
    ccw(P1, P2, P3) ≠ ccw(P1, P2, P4)      ← P3,P4 straddle line P1P2
```

This is the standard "four-orientation" test. It handles proper crossings (segments pass through each other) but not endpoint-touching or overlap — both deliberate exclusions since we care about clear crossings, not tangencies.

```
    P1         P3
     \        /
      \      /
       \    /
        \  /
         \/  ← crossing
         /\
        /  \
       /    \
      P4     P2
```

---

## 3. SVG Path Sampling

The stems are cubic Bezier splines described as SVG path strings (`M ... C ...`). Analytic intersection of cubic Beziers is complex, so instead we **discretize**: sample each path into a polyline of evenly-spaced points.

The browser's `SVGPathElement.getTotalLength()` and `getPointAtLength(t)` do the work. For a path of arc-length `L` and `N` samples:

```
points[i] = path.getPointAtLength(L * i / N),   i = 0, 1, ..., N
```

This produces `N+1` points. Section C uses `N = 50` (sufficient for crossing detection). Section D uses `N = 100` (higher resolution gives a more meaningful Frechet coupling).

---

## 4. Polyline Intersection (Section C)

With two polylines of `N+1` points each, checking for crossings is brute-force: test every segment from polyline A against every segment from polyline B.

```
for each segment (A[i], A[i+1]):
    for each segment (B[j], B[j+1]):
        if segmentsCross(A[i], A[i+1], B[j], B[j+1]):
            record crossing at approximate midpoint
```

The approximate crossing location is the centroid of the four segment endpoints — not exact, but good enough to check whether the crossing falls inside an exclusion zone.

**Exclusion zones**: crossings near certain points are expected and harmless:
- Near the **convergence point** (where all stems deliberately merge)
- Near **source points** (stems originate from nearby coordinates)

Any crossing whose centroid is within `EXCLUSION_RADIUS` (15px) of an exclusion point is ignored.

**Complexity**: O(N^2) per pair, O(S^2 * N^2) for all pairs of S stems. With S=9 and N=50, this is ~460K segment tests — fast in practice.

---

## 5. Discrete Frechet Distance with Coupling (Section D)

### The problem

Section C asks: "do stems physically cross?" Section D asks something subtler: "do adjacent stems flow smoothly alongside each other, or do they twist?"

Two curves can avoid crossing yet still twist — imagine two helical paths on opposite sides of a cylinder. When projected, they never intersect but their relative ordering flips. The Frechet coupling detects this.

### Intuition: the dog-walking problem

Imagine a person walking along curve **P** and a dog walking along curve **Q**. They're connected by a leash. Both can only move forward (never backtrack), and they want to minimize the maximum leash length at any point during the walk.

The **Frechet distance** is the shortest possible maximum leash length over all valid walks. The **coupling** is the specific walk (pairing of positions) that achieves it.

### Discrete formulation

Discretize **P** into points `P[0], ..., P[n-1]` and **Q** into `Q[0], ..., Q[m-1]`. Define:

```
d(i, j) = ‖P[i] - Q[j]‖     (Euclidean distance)
```

Build a DP table where `dp[i][j]` is the Frechet distance considering only `P[0..i]` and `Q[0..j]`:

```
dp[0][0] = d(0, 0)
dp[i][0] = max(dp[i-1][0], d(i, 0))           — dog hasn't started, person walks
dp[0][j] = max(dp[0][j-1], d(0, j))           — person hasn't started, dog walks
dp[i][j] = max( min(dp[i-1][j-1],             — both step forward
                    dp[i-1][j],                — person steps, dog waits
                    dp[i][j-1]),               — dog steps, person waits
                d(i, j) )
```

The recurrence says: at each cell, take the best (minimum-cost) way to arrive, then the actual cost is the worse of that arrival cost and the current point distance.

The final answer is `dp[n-1][m-1]`.

### Backtracking the coupling

The DP table gives us the distance. To get the actual pairing (which point on P is "matched" to which point on Q), backtrack from `(n-1, m-1)` to `(0, 0)`:

```
Start at (i, j) = (n-1, m-1).
While not at (0, 0):
    Look at the three predecessors: (i-1, j-1), (i-1, j), (i, j-1)
    Move to whichever has the smallest dp value.
Reverse the collected path.
```

The result is a sequence of index pairs:

```
coupling = [(0,0), (1,0), (2,1), (3,2), (4,3), ...]
```

Each entry `(a, b)` means "match `P[a]` with `Q[b]`." This is the optimal walk.

**Complexity**: O(n * m) time and space. With n = m = 101 (from 100 samples), that's ~10K operations per pair — negligible.

---

## 6. Coupling Line Crossing Detection (Section D)

### The key insight

Each coupling entry `(a, b)` defines a "leash line" from `P[a]` to `Q[b]`. Consecutive coupling entries define consecutive leash lines. If two curves flow smoothly in parallel, consecutive leash lines look like the rungs of a ladder — roughly parallel, non-crossing:

```
P:  ●----●----●----●----●
    |    |    |    |    |     ← leash lines (ladder rungs)
Q:  ●----●----●----●----●
```

If the curves twist, leash lines cross:

```
P:  ●----●----●----●
    |    |  ╳  |    |         ← crossing = twist detected
Q:  ●----●----●----●
```

### The check

For consecutive coupling entries `k` and `k+1`:

```
Leash k:    P[coupling[k][0]]   → Q[coupling[k][1]]
Leash k+1:  P[coupling[k+1][0]] → Q[coupling[k+1][1]]
```

Apply `segmentsCross()` to these two line segments. If they cross, the stems are locally twisting.

### Exclusions

Two cases are skipped:

1. **Degenerate pairs**: where both coupling entries are identical `(a,b) = (a,b)` — this is a zero-length "step" in the walk and produces no meaningful leash line.

2. **Convergence zone**: near the convergence point, all stems merge to the same location. The leash lines collapse to near-zero length and can spuriously "cross" due to floating-point noise. Any pair where all four leash endpoints are within 20px of the convergence point is skipped.

### What a failure means

If coupling lines cross for stems **A** and **B**, it means the optimal Frechet walk had to "cross its leashes" — the two stems are not flowing in consistent parallel order. They may swap which one is "above" vs "below" partway through their journey. This suggests the routing algorithm is producing a twist that would look unnatural in the visualization.

---

## Summary of Test Sections

| Section | Question | Method |
|---------|----------|--------|
| **A** | Are destinations classified into the right quadrant? | Direct comparison |
| **B** | Is the convergence point over ocean? | Point-in-polygon via `d3.geoContains` |
| **C** | Do any two stems physically cross? | Brute-force polyline segment intersection |
| **D** | Do adjacent stems twist around each other? | Frechet coupling + leash-line crossing |

Sections C and D are complementary. C catches gross errors (paths that collide). D catches subtler problems (paths that weave) which would make the visualization look tangled even without actual intersections.
