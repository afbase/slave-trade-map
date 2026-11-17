# Plan: African embarkation ports + westward flow routing

**Context.** Daniel Domingues da Silva (SlaveVoyages host, Rice) reported that "the African ports are not showing" and suggested highlighting them. Separately, the flow lines from Africa to the Americas jut out in odd directions (see `screenshot.png` and `Screenshot 2026-11-16 at 3.02.47 PM.heic`). Both problems were investigated read-only on 2026-11-16. This doc records the root causes and the fixes to make. Line numbers are as of that date; re-check them before editing.

Codebook reference: `SPSS_Codebook_2023-11-06.pdf`, appendix p.28-30. A place code is 5 digits: the 1st digit is the broad region (6xxxx = Africa), the 2nd-3rd digits are the region (60100 Senegambia … 60900 Other Africa), and the last 2 digits are the place. `xx99` means "port unspecified".

---

## Problem 1: African ports never appear

### Root causes
1. **Markers are only drawn for landing places, and African codes are skipped.**
   - `assets/js/map-destinations.js:23`: `if (dest.code >= 60000 && dest.code < 70000) return;`
   - `renderDestinations` is the only code that draws dots and labels. Its input is landing places only (`MJSELIMP`/`MJSLPTIMP`; `src/db.rs:392-600`, called from `src/state.rs:113-125` and `src/main.rs:82-83`).
   - The same line hides 623 voyages that landed at African places (28 places, e.g. Sierra Leone landings of liberated Africans).
2. **Origins only exist as 9 regions.**
   - `build.rs:204` keys every flow on `MAJBYIMP` (imputed region of purchase), which has only 9 African values.
   - `MJBYPTIMP` (imputed principal port of purchase) is never read. It is not in `needed_vars` (`build.rs:22-48`).
   - `port_flows` is keyed `(embark_region, disembark_port, year)` (`build.rs:231-235`). The SQL groups by `embark_region` (`src/db.rs:661-680` and `src/db.rs:752-775`).
   - So "Ports" mode only means ports in the Americas.
3. **Port-level fields only reach the voyage table.** `PLAC1TRA/2/3` (`build.rs:259-267`) are used only in voyage detail rows (`src/db.rs:973`, `src/components/voyage_detail_row.rs:38`).
4. **Two region centre points are misleading.**
   - 60300 Windward Coast `(6,-8)` is inland.
   - 60900 Other Africa `(1,20)` is in the Congo interior, yet it is the 2nd-largest origin (8,454 voyages).

Ruled out: a minimum-count filter, z-order (dots are drawn above flows, `assets/js/map-core.js:74-76`), and missing coordinates (`getCoords` in `assets/js/map-helpers.js:2-10` falls back to the region centre).

### Numbers (`tastdb-exp-2020-raw.csv`, 36,108 voyages)
| Variable | Non-null | Distinct African codes |
|---|---|---|
| `EMBPORT` | 15,721 | 149 |
| `MAJBUYPT` | 21,958 | 178 |
| **`MJBYPTIMP`** | **34,421** | **179** (170 specific + 9 xx99) |
| `MAJBYIMP` (used today) | 34,421 | 9 |

- **Use `MJBYPTIMP`.** It covers the same voyages as `MAJBYIMP`, at port level.
- Embarked captives (`SLAXIMP`) at African ports total 10.63M:
  - 3.44M (32%) are at xx99 "unspecified" codes. 60999 "Africa, port unspecified" alone is 2.15M.
  - 7.14M are at the 95 specific ports that have coordinates.
  - 51k are at the 75 specific ports with **no** coordinates. The largest of those: Kilongo 60722, Rio Zaire 60718, Mpinda 60729, Rio Nun 60526, Alampo 60405, Albreda 60101, Corisco 60615, Bissagos 60103, Cess 60310.
- Top ports: Luanda 60734 (1.40M), Costa da Mina 60502 (648k), Bonny 60605 (433k), Whydah 60515 (408k), Benguela 60707 (374k), Cabinda 60711 (369k), Calabar 60608 (270k), Mozambique 60820 (214k), Malembo 60725 (195k), Anomabu 60408 (171k).

---

## Problem 2: Flow lines jut out instead of running west

Data and projection are fine. The shapes come from the routing code, which was reproduced in Node with the app's JS, d3 NaturalEarth1 and 110m land data. The scratch renders were in the old session's scratchpad and may be gone.

### Root causes
1. **Every West African line first goes to one shared centre point (main cause).**
   - `computeSpine` (`assets/js/route-planner.js:515-529`) starts the shared route at the average position of all sources, about lng 0-3, lat 5-9 (the Benin/Togo coast).
   - `computeRibbonOffsets` (`:562-574`) starts each line there, plus a sideways offset.
   - `generateRoutedPath` (`assets/js/map-helpers.js:96-101`) puts the real source in front, so each line is drawn source → centre → spine.
   - Result: Senegambia goes 16° east over land and then hairpins west. Sierra Leone goes 13° east. West Central Africa cuts north-west across Gabon. Other Africa crosses the Congo basin.
2. **The join point is east of the last waypoints, so lines overshoot and hook back.**
   - `map-flows.js:29-34` puts the join point halfway between the westernmost source and the destination. For Barbados that is lng -37.8.
   - Caribbean waypoint lists end at -42 to -50 (`route-planner.js:118` etc.), so every line turns back east into the join point.
   - The guard against this (`map-helpers.js:55-65`, duplicated in `src/geometry/arc.rs:66-76`) removes only one point.
3. **Southeast Africa makes a U-turn.**
   - The Cape route (`route-planner.js:66-75`) reaches the mid-Atlantic, at [-20,-14] for Brazil.
   - `computeJunction` (`:578-602`) then picks its join point by direction alone, within the first 70% of the spine. It lands back in the Gulf of Guinea, about [0,1].
   - So the line jumps about 20° east and runs west again.
4. **Smaller problems:**
   - The offshore waypoints are 3-6° too far west (`route-planner.js:12-16`). `beninOffshore` is off Ivory Coast and `goldCoastOffshore` is off Liberia.
   - All lines in a group follow one middle source's waypoints (`:503-512`), which gives S-bends.
   - The land-avoidance fallback can push a detour east (`:306-315`).
   - The port-code direction classifiers don't fall back to the parent region (`route-planner.js:25`, `src/routing/quadrant.rs:61-63`), so unmatched ports get default routes.
   - There are two route builders (JS `buildWaypointChain` and Rust `build_waypoint_chain`) that can drift apart.
   - Catmull-Rom smoothing (`map-helpers.js:73-86`, `arc.rs:88-99`) overshoots when points reverse.

---

## Design decision: how to present 170 ports without clutter

**Don't draw 170 lines. Separate the markers from the lines.**

The data is fine-grained, but drawing every port-to-destination line at once would bury the map. About 400 pairs of African ports are less than 8 px apart at default zoom, and only about 25 separate markers fit along the coast. The approach has three layers:

1. **Lines stay at region level by default** (9 African sources). This is the overview historians expect, and it stays readable once Problem 2 is fixed.
2. **Port markers zoom in and out by the codebook's place levels.** This is the "quadtree-like" layer, but it is keyed on the code levels, not on spatial cells:

   | Zoom k | African markers shown |
   |---|---|
   | < ~2 | 9 **region bubbles**, sized by embarked total (including xx99) |
   | ~2-5 | **ports**, sized by embarked count. Ports still on top of each other are merged with a `d3.quadtree` / pixel-distance pass *within one region only* (e.g. Cape Coast / Elmina / Anomabu) |
   | > ~5 | **every individual port** |

   - Clicking a region bubble expands it into its ports (animated split) at any zoom.
   - Labels go through the existing collision pass (`recomputeLabelVisibility`, `assets/js/map-core.js:204`), with importance = embarked count, so only the large ports are labelled when zoomed out.
   - Why the code levels rather than a pure spatial quadtree: they match how SlaveVoyages and historians group places, they keep xx99 totals in the right region, and they need only `floor(code/100)*100`.
   - There is no performance reason for a quadtree with fewer than about 450 points. Use `d3.quadtree` only where it is cheap and useful: the mid-zoom merge and, optionally, replacing the O(n²) label-collision loop (`assets/js/map-destinations.js:147-179`).
3. **Port-level lines only on selection.** Clicking (or searching for) an African port draws only that port's lines to its destinations, using the new routing. Other lines dim. This mirrors the existing click-a-destination behaviour. At most a few dozen lines are on screen at once.

**Also add search, as a complement.** Historians usually arrive with a name ("Bonny", "Ouidah", "Cabinda"):
- a typeahead over all ports, both African and American, using `MJBYPTIMP`/`MJSLPTIMP` labels;
- selecting a result zooms to the port, expands its region, and selects it (layer 3);
- match spelling variants in the label, e.g. "Whydah (Ouidah)", "Luanda (St. Paul de Loanda)".

Search does not replace the visual layer, because Daniel's point is that the ports should be *visible*. It makes the visual layer usable.

---

## Implementation steps (in order)

### Step 1: Routing rewrite (do first; port-level lines depend on it)
1. **One offshore gate per region** (lng, lat), replacing `route-planner.js:12-16`:
   ```
   60100 Senegambia [-18.5,14]   60200 Sierra Leone [-15,8]   60300 Windward [-10,5]
   60400 Gold Coast [-1,4.5]     60500 Benin [2.5,5.5]        60600 Biafra [6.5,3]
   60700 WCA [10.5,-6.5]         60900 Other Africa [8,2]
   ```
   - Ports use their region's gate (`floor(code/100)*100`).
   - The line runs from the port's real coordinate to the gate, then on.
2. **Move centre points off land:** 60900 Other Africa to the coast (or an offshore "Africa, unspecified" point), and 60300 Windward to the coast. Change both `assets/js/coords.js` and `src/geo/coords.rs`.
3. **Place the join point J after the sources are known:**
   - `lng = max(min(gate lngs, -5.7 if SE Africa present) - 6, dest.lng + 10)`
   - `lat` = the destination band (Caribbean 10, North America 28, Brazil-north -4, Brazil-south -12, Río de la Plata -22).
4. **Each line is one cubic Bezier from gate to J, always moving west.**
   - Clamp both control points' longitude to `[J.lng, gate.lng]`.
   - The 1st control point heads due west. The 2nd lies along the direction of the trunk (the shared line from J to the destination).
   - Fan the lines into J by a small latitude offset, ordered north to south by gate latitude, so they don't cross.
   - Sample in lng/lat, project, then draw with `d3.curveMonotoneX`.
5. **Southeast Africa (60800):** source → Cape Agulhas [20,-35] → [10,-33] → St Helena [-5.7,-16] → the same Bezier into J.
6. **Trunk:** J → destination.
7. **Delete** `computeSpine`, `computeRibbonOffsets`, `computeJunction`, and the ribbon/spine path in `generateRoutedPath`.
   - Replace the one-point backtrack guard with a filter that keeps x non-increasing, in both JS and `arc.rs`.
   - Optional: check sampled points with `isPointOnLand` and nudge J south or west only, never east.
8. **Direction classifiers:** fall back to the parent region code in `route-planner.js:25` and `src/routing/quadrant.rs:61-63`.
9. **Keep one route builder:** either call the Rust builder everywhere or delete it, and say which in `CLAUDE.md`.

Reference sketch from the investigation (replaces `computeRibbonStems` + `generateRoutedPath` for Atlantic lines):
```js
var GATES = {60100:[-18.5,14],60200:[-15,8],60300:[-10,5],60400:[-1,4.5],
             60500:[2.5,5.5],60600:[6.5,3],60700:[10.5,-6.5],60900:[8,2]};
var CAPE = [[20,-35],[10,-33],[-5.7,-16]];
var BAND = {caribbean:10,"north-america":28,"brazil-north":-4,"brazil-south":-12,
            "rio-plata":-22,"pacific-south":-30,"mid-atlantic":5};

function planJunction(srcCodes, dest, q) {
  var lngs = srcCodes.map(function(c){ var r=Math.floor(c/100)*100;
    return r===60800 ? -5.7 : (GATES[r]||[0])[0]; });
  var lng = Math.max(Math.min.apply(null,lngs) - 6, dest.lng + 10);
  return [lng, BAND[q] != null ? BAND[q] : dest.lat];
}

function stemPath(code, J, trunkDir, rank, n) {
  var r = Math.floor(code/100)*100, src = getCoords(code);
  var pre = [[src.lng, src.lat]].concat(r===60800 ? CAPE : [GATES[r]]);
  var A = pre[pre.length-1];
  var Jo = [J[0], J[1] + (rank - (n-1)/2) * 0.8];
  var dx = A[0] - Jo[0];
  var c1 = [A[0] - dx*0.4, A[1]];
  var c2 = [Jo[0] - trunkDir[0]*dx*0.35, Jo[1] - trunkDir[1]*dx*0.35];
  c2[0] = Math.min(Math.max(c2[0], Jo[0]), A[0]);
  var pts = pre.slice();
  for (var t = 0.05; t <= 1.0001; t += 0.05) {
    var u=1-t; pts.push([0,1].map(function(k){
      return u*u*u*A[k]+3*u*u*t*c1[k]+3*u*t*t*c2[k]+t*t*t*Jo[k]; }));
  }
  return d3.line().curve(d3.curveMonotoneX)(pts.map(_projection));
}
```

**Acceptance criteria:** for Barbados (34200), Maranhão (50102) and one port in North America, every African line's projected x never increases from gate to destination. No line crosses inland Africa. The SE Africa line never comes back east of St Helena. Add a test that checks the x-never-increases property for every pair of region and destination band.

### Step 2: Port-level origin data
1. In `build.rs`:
   - add `MJBYPTIMP` to `needed_vars`;
   - carry it as `embark_port` in `voyages.csv`, keeping `MAJBYIMP` as `embark_region`;
   - add an `origin_port_flows (embark_port, embark_region, disembark_port, disembark_region, year, embarked, disembarked, voyages)` table;
   - add `origins.csv` (`embark_port, year, embarked, voyages`).
2. In `src/db.rs`:
   - add `get_origin_ports(min_year, max_year, dataset)`, returning `code, label, region, embarked, voyages, is_unspecified (code % 100 == 99)`;
   - add `get_origin_port_flows(port_code, ...)` for selection mode (design layer 3).
3. Expose both through `src/wasm_exports.rs` / `js_bridge.rs` and `state.rs`, following the existing pattern for destinations.

### Step 3: Map presentation (see Design decision)
1. Add a new `renderOrigins()` in its own `_originsGroup`, drawn above the flows.
   - Use a different shape or style from destination dots and the region colours from `window.__africanColors`.
   - Size by `d3.scaleSqrt` on embarked count.
2. Show region bubbles, ports, or all ports by zoom level. Hook into the zoom handler (`assets/js/map-core.js:144-187`).
3. Click a region bubble to expand it. Click a port to show only its lines (dims the rest) and a tooltip with its totals and top destinations.
4. **xx99 "unspecified":** draw a hollow ring at the region anchor (e.g. "Gold Coast, port unspecified"), with no fake port point. The legend states the unspecified share (~32%) so the totals add up.
5. **`map-destinations.js:23`:** stop silently skipping African landing places. Draw them in a separate style (e.g. "liberated Africans landed"), behind a toggle.

### Step 4: Search
- Header typeahead (a new component next to `mode_toggle.rs` / `dataset_toggle.rs`) over all African and American port labels.
- On select: zoom to the port, expand its region, and select it.

### Step 5: Toggles and gaps
- Toggles: "Show African ports", "Origins: region / port", "Include unspecified", "Show African landings".
- Add coordinates for the ~75 African ports that lack them (see the list above). Low priority, since they total about 51k captives.

## Verification
- `cargo test`, `cargo clippy`, and the existing tests in `tests/`.
- Run the app (`dx serve`) and compare before/after screenshots:
  - Ports mode at default zoom, then zoomed to the Gold Coast and to West Central Africa;
  - Barbados and Maranhão selected;
  - Luanda, Bonny and Whydah selected through search.
- Check totals: the sum of African port markers plus the xx99 rings equals the region totals for the same year range.
- When done, reply to Daniel with what changed and why: the map grouped departures into 9 regions and only drew markers for landing places; it now shows `MJBYPTIMP` ports.

---

## Addendum (2026-11-16): Peru and Europe

The westward planner drew one arc from the junction to the destination, which cut across South America for Peru, and it declined Europe altogether, leaving the old waypoint chains (which crossed Iberia and each other). Both are now special cases in `assets/js/route-planner.js`, reached through `planFlowStems()`:

- **Pacific coast (43xxx-46xxx: Peru, Chile, Ecuador, Pacific Central America).** Stems still only move west, into a junction in the South Atlantic (30 S). The trunk follows `CAPE_HORN_ROUTE` south of Tierra del Fuego and then `PACIFIC_COAST_ROUTE` north to the destination. Southeast Africa skips the St Helena leg and heads for the Horn from the Cape.
- **Europe (1xxxx).** `planNorthboundStems()`: each stem leaves through its usual gate, joins `NORTH_CORRIDOR` round the West African bulge, and meets the others off the Saharan coast (23 N). The trunk follows `EUROPE_LANES`, a small tree of sea lanes (Iberia, Biscay, Channel, North Sea, Gibraltar); the lane into each destination is the one whose last leg crosses the least land.
- **Known limits.** Country anchors that sit inland (Spain, France, Scotland..) still get a last stretch over land from the nearest lane; ports do not. Baltic destinations lie past the last lane. Madeira and the Canaries as *sources* to Europe fall back to a plain arc. None of the last two occur in the voyage data.
- **Test:** `tests/special-routes.test.mjs` samples the drawn curves (not only their control points) for land, direction and crossings.
