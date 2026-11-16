// Checks the Africa -> Americas stem routing in assets/js/route-planner.js.
//
//   cd tests && npm install && npm test
//
// For every American destination in coords.js, with all nine African regions
// as sources, and for every African place on its own to five representative
// destinations, each stem must:
//   1. never step back east (projected x never increases),
//   2. stay off land once it has left its own coast,
//   3. not cross another stem into the same destination.
// Uses the app's projection and the same 110m land data the map draws.
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

const coords = run('regionCoords');
const isAfrican = c => c >= 60000 && c < 70000;
const REGIONS = [60100, 60200, 60300, 60400, 60500, 60600, 60700, 60800, 60900];
const destinations = Object.keys(coords).map(Number)
  .filter(c => !isAfrican(c) && coords[c].lng <= -30);
const africanPlaces = Object.keys(coords).map(Number).filter(isAfrican);
// Barbados, Bahia, Virginia, Maranhao, Rio de la Plata
const SAMPLE_DESTS = [34200, 50200, 21100, 50102, 42000].filter(c => coords[c]);

// Places that are inland by nature; their first stretch crosses land.
const INLAND = new Set([60107]);

function segmentsCross(a, b, c, d) {
  const o = (p, q, r) => (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0]);
  return o(a, b, c) * o(a, b, d) < 0 && o(c, d, a) * o(c, d, b) < 0;
}

function check(dest, sources) {
  const plan = run('planAtlanticStems(__a.sources, __a.dest, 0, null)', { dest, sources });
  if (!plan) return ['no plan'];
  const problems = [];
  const codes = Object.keys(plan.points).map(Number);
  for (const code of codes) {
    const pts = plan.points[code];
    for (let i = 1; i < pts.length; i++) {
      if (pts[i][0] > pts[i - 1][0] + 1e-6) {
        problems.push(`${code}: steps east at point ${i}`);
        break;
      }
    }
    if (INLAND.has(code)) continue;
    // Skip the legs from the source out to its offshore gate.
    let onLand = 0;
    for (let i = 3; i < pts.length; i++) {
      for (let t = 0.25; t < 1; t += 0.25) {
        const g = projection.invert([
          pts[i - 1][0] + (pts[i][0] - pts[i - 1][0]) * t,
          pts[i - 1][1] + (pts[i][1] - pts[i - 1][1]) * t,
        ]);
        if (run('isLandFast(__a[0], __a[1])', g)) onLand++;
      }
    }
    if (onLand) problems.push(`${code}: ${onLand} samples on land`);
  }
  for (let i = 0; i < codes.length; i++) {
    for (let j = i + 1; j < codes.length; j++) {
      const A = plan.points[codes[i]], B = plan.points[codes[j]];
      let crossings = 0;
      for (let a = 1; a < A.length - 1; a++) {
        for (let b = 1; b < B.length - 1; b++) {
          if (segmentsCross(A[a - 1], A[a], B[b - 1], B[b])) crossings++;
        }
      }
      if (crossings) problems.push(`${codes[i]} x ${codes[j]}: ${crossings} crossings`);
    }
  }
  return problems;
}

let cases = 0;
const failures = [];
for (const dest of destinations) {
  cases++;
  const problems = check(dest, REGIONS);
  if (problems.length) failures.push(`all regions -> ${coords[dest].name} (${dest}): ${problems.join('; ')}`);
}
for (const place of africanPlaces) {
  for (const dest of SAMPLE_DESTS) {
    cases++;
    const problems = check(dest, [place]);
    if (problems.length) failures.push(`${coords[place].name} (${place}) -> ${coords[dest].name}: ${problems.join('; ')}`);
  }
}

// Destinations that are not to the west must be declined, so the caller
// falls back to waypoint chains instead of drawing a backwards stem.
for (const dest of [60200, 10100].filter(c => coords[c])) {
  cases++;
  if (run('planAtlanticStems(__a.sources, __a.dest, 0, null)', { dest, sources: REGIONS })) {
    failures.push(`${coords[dest].name} (${dest}) should not get a westward plan`);
  }
}

if (failures.length) {
  console.error(failures.slice(0, 40).join('\n'));
  console.error(`\n${failures.length} of ${cases} route cases failed`);
  process.exit(1);
}
console.log(`${cases} route cases passed: ${destinations.length} destinations x 9 regions, ` +
  `${africanPlaces.length} African places x ${SAMPLE_DESTS.length} destinations`);
