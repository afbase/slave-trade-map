// Checks the two destinations groups that are not a straight run west, in
// assets/js/route-planner.js. Run by `npm test` after westward-routes.test.mjs.
//
//   Pacific coast (Peru, Chile, Ecuador, Panama): stems still only move west,
//     and the trunk rounds Cape Horn instead of crossing South America.
//   Europe: stems only move north once they have left their coast (or rounded
//     the Cape), and the trunk follows sea lanes north.
//
// In both, stems and trunk must stay off land and stems must not cross. The
// drawn SVG paths are sampled, not just their control points, so a curve that
// swings over a coast between two points is caught.
import * as d3 from 'd3';
import * as topojson from 'topojson-client';
import fs from 'node:fs';
import vm from 'node:vm';

const here = p => new URL(p, import.meta.url);
const world = JSON.parse(fs.readFileSync(here('./node_modules/world-atlas/countries-110m.json')));
const projection = d3.geoNaturalEarth1().center([-15, 5]).scale(210).translate([600, 320]);

const ctx = { d3, Math, JSON, console, window: {}, _projection: projection };
ctx.window._landFeatures = topojson.feature(world, world.objects.countries).features;
vm.createContext(ctx);
for (const f of ['coords.js', 'map-helpers.js', 'route-planner.js']) {
  vm.runInContext(fs.readFileSync(here('../assets/js/' + f), 'utf8'), ctx, { filename: f });
}
const run = (code, args) => { ctx.__a = args; return vm.runInContext(code, ctx); };
const isLand = pt => { const g = projection.invert(pt); return run('isLandFast(__a[0], __a[1])', g); };

const coords = run('regionCoords');
const codes = Object.keys(coords).map(Number);
const REGIONS = [60100, 60200, 60300, 60400, 60500, 60600, 60700, 60800, 60900];
const africanPlaces = codes.filter(c => c >= 60000 && c < 70000);
const pacificDests = codes.filter(c => c >= 43000 && c < 47000);
const europeDests = codes.filter(c => c >= 10000 && c < 20000);

// Places that are inland by nature; their first stretch crosses land.
const INLAND_SOURCES = new Set([60107]);
// Atlantic islands north of the stem junction: no shared northbound plan, the
// map falls back to a plain arc.
const NO_NORTH_PLAN = new Set([60128, 60141]);
// How far from the destination dot (px at zoom 1) the trunk may be over land.
// Ports sit on the coarse 110m coastline or a little way up a river.
const PORT_REACH = 5;
// Country anchors that sit inland (Spain = Madrid, France = central France..):
// the trunk comes in from the nearest sea lane and the last stretch is on land.
const ANCHOR_REACH = { 10100: 18, 10300: 8, 10500: 12, 10700: 16, 10799: 16, 10800: 8, 11000: 10,
  11200: 8, 11300: 30, 11500: 75, 44000: 8, 45000: 10 };
// Baltic destinations lie past the last sea lane (Kattegat).
const BALTIC = new Set([10900, 10901, 11300, 11500]);

// Points along an SVG path made of M, L, Q and C commands (absolute).
function samplePath(d, perSegment = 12) {
  const out = [];
  const re = /([MLQC])([^MLQC]*)/g;
  let m, cur = null;
  while ((m = re.exec(d))) {
    const n = m[2].trim().split(/[\s,]+/).filter(Boolean).map(Number);
    if (m[1] === 'M') { cur = [n[0], n[1]]; out.push(cur); continue; }
    const step = { L: 2, Q: 4, C: 6 }[m[1]];
    for (let i = 0; i + step <= n.length; i += step) {
      const p = n.slice(i, i + step), a = cur;
      for (let s = 1; s <= perSegment; s++) {
        const t = s / perSegment, u = 1 - t;
        if (step === 2) out.push([a[0] + (p[0] - a[0]) * t, a[1] + (p[1] - a[1]) * t]);
        else if (step === 4) out.push([0, 1].map(k => u * u * a[k] + 2 * u * t * p[k] + t * t * p[2 + k]));
        else out.push([0, 1].map(k => u * u * u * a[k] + 3 * u * u * t * p[k] + 3 * u * t * t * p[2 + k] + t * t * t * p[4 + k]));
      }
      cur = [p[step - 2], p[step - 1]];
    }
  }
  return out;
}

function segmentsCross(a, b, c, d) {
  const o = (p, q, r) => (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0]);
  return o(a, b, c) * o(a, b, d) < 0 && o(c, d, a) * o(c, d, b) < 0;
}

// Stems: direction, land, crossings. `direction` is 'west' or 'north'.
function checkStems(plan, direction, problems) {
  const stemCodes = Object.keys(plan.points).map(Number);
  for (const code of stemCodes) {
    const pts = plan.points[code];
    if (direction === 'west') {
      for (let i = 1; i < pts.length; i++) {
        if (pts[i][0] > pts[i - 1][0] + 1e-6) { problems.push(`${code}: steps east at point ${i}`); break; }
      }
    } else {
      // Northward from the southernmost point on (the gate, or the Cape).
      let south = 0;
      for (let i = 1; i < pts.length; i++) if (pts[i][1] > pts[south][1]) south = i;
      for (let i = south + 1; i < pts.length; i++) {
        if (pts[i][1] > pts[i - 1][1] + 1e-6) { problems.push(`${code}: steps south at point ${i}`); break; }
      }
    }
    if (INLAND_SOURCES.has(code)) continue;
    // The drawn curve, from where it has left its own coast: skip the legs
    // from the source out to its offshore gate (the first three points).
    const from = pts[Math.min(3, pts.length - 1)];
    const drawn = samplePath(plan.stems[code]);
    let start = 0, bestD = Infinity;
    drawn.forEach((p, i) => {
      const dd = Math.hypot(p[0] - from[0], p[1] - from[1]);
      if (dd < bestD) { bestD = dd; start = i; }
    });
    const onLand = drawn.slice(start).filter(isLand).length;
    if (onLand) problems.push(`${code}: stem has ${onLand} samples on land`);
  }
  for (let i = 0; i < stemCodes.length; i++) {
    for (let j = i + 1; j < stemCodes.length; j++) {
      const A = plan.points[stemCodes[i]], B = plan.points[stemCodes[j]];
      let crossings = 0;
      for (let a = 1; a < A.length - 1; a++) {
        for (let b = 1; b < B.length - 1; b++) {
          if (segmentsCross(A[a - 1], A[a], B[b - 1], B[b])) crossings++;
        }
      }
      if (crossings) problems.push(`${stemCodes[i]} x ${stemCodes[j]}: ${crossings} crossings`);
    }
  }
}

// Trunk: starts at the junction, ends on the destination, and is at sea
// everywhere except within `reach` px of the destination dot.
function checkTrunk(plan, dest, problems) {
  const destPt = projection([coords[dest].lng, coords[dest].lat]);
  const trunk = samplePath(plan.trunk);
  const first = trunk[0], last = trunk[trunk.length - 1];
  if (Math.hypot(first[0] - plan.junction[0], first[1] - plan.junction[1]) > 0.01) problems.push('trunk does not start at the junction');
  if (Math.hypot(last[0] - destPt[0], last[1] - destPt[1]) > 0.01) problems.push('trunk does not end on the destination');
  const reach = ANCHOR_REACH[dest] || PORT_REACH;
  const onLand = trunk.filter(p => Math.hypot(p[0] - destPt[0], p[1] - destPt[1]) > reach && isLand(p)).length;
  if (onLand) problems.push(`trunk has ${onLand} samples on land`);
  if (plan.trunkTail.trim().startsWith('M') || !plan.trunk.endsWith(plan.trunkTail.trim())) problems.push('trunkTail is not the trunk without its moveto');
  return trunk;
}

function checkPacific(dest, sources) {
  const plan = run('planFlowStems(__a.sources, __a.dest, 0, "south-atlantic")', { dest, sources });
  if (!plan) return ['no plan'];
  const problems = [];
  checkStems(plan, 'west', problems);
  const trunk = checkTrunk(plan, dest, problems);
  // Round the Horn: south of Tierra del Fuego (56 S), reached west of the
  // junction, and the Pacific leg stays west of the continent.
  const lats = trunk.map(p => projection.invert(p));
  const southmost = lats.reduce((a, b) => (b[1] < a[1] ? b : a));
  if (southmost[1] > -56.5) problems.push(`trunk only reaches ${southmost[1].toFixed(1)} S, not round Cape Horn`);
  if (trunk.some(p => p[0] > plan.junction[0] + 0.01)) problems.push('trunk goes east of the junction');
  return problems;
}

function checkEurope(dest, sources) {
  const plan = run('planFlowStems(__a.sources, __a.dest, 0, null)', { dest, sources });
  if (!plan) return ['no plan'];
  const problems = [];
  checkStems(plan, 'north', problems);
  const trunk = checkTrunk(plan, dest, problems);
  // Generally northward: ends north of the junction and never runs back south
  // by more than a few degrees (px; e.g. down the Kattegat to Copenhagen).
  let highest = trunk[0][1], worstDrop = 0;
  for (const p of trunk) {
    if (p[1] < highest) highest = p[1];
    worstDrop = Math.max(worstDrop, p[1] - highest);
  }
  if (trunk[trunk.length - 1][1] >= trunk[0][1]) problems.push('trunk does not end north of the junction');
  if (worstDrop > 8) problems.push(`trunk runs back south by ${worstDrop.toFixed(1)} px`);
  return problems;
}

let cases = 0;
const failures = [];
const name = c => `${coords[c].name} (${c})`;

for (const dest of pacificDests) {
  cases++;
  let problems = checkPacific(dest, REGIONS);
  if (problems.length) failures.push(`all regions -> ${name(dest)}: ${problems.join('; ')}`);
  for (const place of africanPlaces) {
    cases++;
    problems = checkPacific(dest, [place]);
    if (problems.length) failures.push(`${name(place)} -> ${name(dest)}: ${problems.join('; ')}`);
  }
}

for (const dest of europeDests) {
  if (BALTIC.has(dest)) continue;
  cases++;
  let problems = checkEurope(dest, REGIONS);
  if (problems.length) failures.push(`all regions -> ${name(dest)}: ${problems.join('; ')}`);
}
// Every African place on its own, to the European ports the voyages name.
// Cadiz, Lisbon, Bristol, Le Havre, Middelburg
const SAMPLE_EUROPE = [10104, 10203, 10404, 10713, 10850].filter(c => coords[c]);
for (const place of africanPlaces) {
  for (const dest of SAMPLE_EUROPE) {
    cases++;
    if (NO_NORTH_PLAN.has(place)) {
      if (run('planFlowStems(__a.sources, __a.dest, 0, null)', { dest, sources: [place] }) &&
          checkEurope(dest, [place]).length) failures.push(`${name(place)} -> ${name(dest)}: bad plan`);
      continue;
    }
    const problems = checkEurope(dest, [place]);
    if (problems.length) failures.push(`${name(place)} -> ${name(dest)}: ${problems.join('; ')}`);
  }
}

if (failures.length) {
  console.error(failures.slice(0, 60).join('\n'));
  console.error(`\n${failures.length} of ${cases} special route cases failed`);
  process.exit(1);
}
console.log(`${cases} special route cases passed: ${pacificDests.length} Pacific and ` +
  `${europeDests.filter(c => !BALTIC.has(c)).length} European destinations`);
