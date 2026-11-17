// Route planning for the slave trade flow map. This file is the only route
// builder: there is no Rust copy to keep in step.
//   - planFlowStems() is the entry point for flows out of Africa:
//       Africa -> Americas: planAtlanticStems(), westward stems into one junction
//       Africa -> Pacific coast (Peru, Chile..): same stems, trunk rounds Cape Horn
//       Africa -> Europe: planNorthboundStems(), stems round the West African
//         bulge and the trunk follows sea lanes north
//   - everything else (inter-American): waypoint chains with land avoidance,
//     buildWaypointChain() + computeDynamicWaypoints()

var MARITIME_LANDMARKS = {
  capeAgulhas: { lat: -34.83, lng: 20 },
  westOfCape: { lat: -33, lng: 10 },
  stHelena: { lat: -15.97, lng: -5.71 },
  capeVerde: { lat: 14.93, lng: -23.51 },
  pontaDoSeixas: { lat: -7.15, lng: -33.8 },
  // Offshore reference points per zone
  senegambiaOffshore: { lat: 14, lng: -18.5 },
  sierraLeoneOffshore: { lat: 8, lng: -15 },
  windwardOffshore: { lat: 4.7, lng: -11 },
  goldCoastOffshore: { lat: 3.9, lng: -2 },
  beninOffshore: { lat: 3.4, lng: 1.5 },
  biafraOffshore: { lat: 2.9, lng: 6 },
  wcaOffshore: { lat: -6.8, lng: 10 },
  seAfricaOffshore: { lat: -25, lng: 25 },
  otherAfricaCoast: { lat: -2.8, lng: 3.5 }
};

var _waypointCache = {};

// Coordinates for a place code, falling back to the parent region (xx00).
function _routeCoords(code) {
  if (typeof getCoords === 'function') return getCoords(code);
  return regionCoords[code] || regionCoords[Math.floor(code / 100) * 100] || null;
}

function classifyDestQuadrant(destCode) {
  var dc = _routeCoords(destCode);
  if (!dc) return "mid-atlantic";
  var lat = dc.lat;
  var lng = dc.lng;

  if (lat >= 8 && lat <= 25 && lng >= -100 && lng <= -59) return "caribbean";
  if (lat > 25 && lng >= -100 && lng <= -60) return "north-america";
  if (lat >= -10 && lat <= 5 && lng >= -55 && lng <= -30) return "brazil-north";
  if (lat < -10 && lat >= -25 && lng >= -55 && lng <= -30) return "brazil-south";
  if (lat < -25 && lng >= -65 && lng <= -50) return "rio-plata";
  if (lat > 35 && lng > -15) return "europe";
  if (lat < -10 && lng < -65) return "pacific-south";
  return "mid-atlantic";
}

function isBrazilQuadrant(q) {
  return q === "brazil-north" || q === "brazil-south";
}

function isWestwardQuadrant(q) {
  return q === "caribbean" || q === "north-america";
}

function classifySourceQuadrant(sourceCode) {
  var sc = _routeCoords(sourceCode);
  if (!sc) return "unknown";
  var lat = sc.lat, lng = sc.lng;
  if (lat >= 8 && lat <= 25 && lng >= -100 && lng <= -59) return "caribbean";
  if (lat > 25 && lng >= -100 && lng <= -60) return "north-america";
  if (lat >= -15 && lat <= 5 && lng >= -55 && lng <= -30) return "brazil-north";
  if (lat < -15 && lat >= -25 && lng >= -55 && lng <= -30) return "brazil-south";
  if (lat < -25 && lng >= -65 && lng <= -50) return "rio-plata";
  if (lat > 35 && lng > -15) return "europe";
  return "mid-atlantic";
}

function buildWaypointChain(sourceCode, destQuadrant) {
  var src = Math.floor(sourceCode / 100) * 100;
  var M = MARITIME_LANDMARKS;

  // African sources. Routes to the Americas are planned by planAtlanticStems();
  // these chains only serve destinations that are not to the west
  // (Europe, Atlantic islands).
  var toEurope = destQuadrant === "europe";
  switch (src) {
    case 60800: // SE Africa
      var chain = [M.capeAgulhas, M.westOfCape, M.stHelena];
      if (toEurope) chain.push({ lat: 0, lng: -16 }, { lat: 10, lng: -20 });
      return chain;
    case 60700: // W Central Africa
      return toEurope
        ? [M.wcaOffshore, M.stHelena, { lat: 8, lng: -16 }, { lat: 16, lng: -18 }]
        : [M.wcaOffshore, { lat: 0, lng: -15 }];
    case 60600: // Bight of Biafra
      return toEurope
        ? [M.biafraOffshore, { lat: 3, lng: -12 }, { lat: 14, lng: -19 }, { lat: 25, lng: -17 }]
        : [M.biafraOffshore, { lat: 0, lng: -10 }];
    case 60500: // Bight of Benin
      return toEurope
        ? [M.beninOffshore, { lat: 3.5, lng: -12 }, { lat: 16, lng: -19 }, { lat: 34, lng: -12 }]
        : [M.beninOffshore, { lat: 3, lng: -15 }];
    case 60400: // Gold Coast
      return toEurope
        ? [M.goldCoastOffshore, { lat: 4, lng: -12 }, { lat: 16, lng: -19 }, { lat: 28, lng: -15 }]
        : [M.goldCoastOffshore, { lat: 3.5, lng: -20 }];
    case 60300: // Windward Coast
      return toEurope
        ? [M.windwardOffshore, { lat: 18, lng: -19 }, { lat: 31, lng: -13 }]
        : [M.windwardOffshore, { lat: 5, lng: -25 }];
    case 60200: // Sierra Leone
      return toEurope
        ? [M.sierraLeoneOffshore, { lat: 20, lng: -20 }, { lat: 37, lng: -13 }]
        : [M.sierraLeoneOffshore, { lat: 5, lng: -28 }];
    case 60100: // Senegambia
      return toEurope
        ? [M.senegambiaOffshore, { lat: 28, lng: -18 }, { lat: 40, lng: -12 }]
        : [M.senegambiaOffshore, M.capeVerde];
    case 60900: // Other Africa
      return toEurope
        ? [M.otherAfricaCoast, { lat: 2, lng: -12 }, { lat: 14, lng: -19 }, { lat: 22, lng: -18.5 }]
        : [M.otherAfricaCoast, { lat: 0, lng: 0 }, { lat: -2, lng: -10 }];

    default:
      // Inter-American sources
      var srcQuadrant = classifySourceQuadrant(sourceCode);
      if (srcQuadrant === "unknown") return null;

      // Brazilian sources
      if (srcQuadrant === "brazil-north" || srcQuadrant === "brazil-south") {
        var srcCoord = _routeCoords(sourceCode);
        var farWest = srcCoord && srcCoord.lng < -42;

        if (destQuadrant === "rio-plata") {
          if (farWest) {
            return [{ lat: -12, lng: -36 }, { lat: -20, lng: -39 }, { lat: -28, lng: -46 }, { lat: -30, lng: -48 }];
          }
          if (srcQuadrant === "brazil-south") {
            // SE Brazil -> Rio de la Plata: direct offshore southward
            return [{ lat: -25, lng: -44 }, { lat: -30, lng: -48 }];
          }
          // Northern Brazil -> Rio de la Plata: around Ponta do Seixas, then south along coast
          return [M.pontaDoSeixas, { lat: -13, lng: -35 }, { lat: -20, lng: -38 }, { lat: -25, lng: -44 }, { lat: -30, lng: -48 }];
        }
        if (destQuadrant === "pacific-south") {
          if (farWest) {
            return [{ lat: -15, lng: -42 }, { lat: -30, lng: -48 }, { lat: -45, lng: -60 }, { lat: -50, lng: -72 }];
          }
          // Brazil -> Pacific South: around Ponta do Seixas, south past Cape Horn region
          if (srcQuadrant === "brazil-south") {
            return [{ lat: -30, lng: -48 }, { lat: -45, lng: -60 }, { lat: -50, lng: -72 }];
          }
          return [M.pontaDoSeixas, { lat: -20, lng: -38 }, { lat: -35, lng: -52 }, { lat: -45, lng: -60 }, { lat: -50, lng: -72 }];
        }
        if (isWestwardQuadrant(destQuadrant)) {
          if (farWest) {
            return [{ lat: 0, lng: -42 }, { lat: 10, lng: -50 }];
          }
          // Brazil -> Caribbean/North America: around Ponta do Seixas then north
          if (srcQuadrant === "brazil-south") {
            return [{ lat: -15, lng: -36 }, M.pontaDoSeixas, { lat: 0, lng: -40 }];
          }
          return [M.pontaDoSeixas, { lat: 0, lng: -40 }, { lat: 10, lng: -50 }];
        }
        if (isBrazilQuadrant(destQuadrant)) {
          if (farWest) {
            return [{ lat: -10, lng: -36 }];
          }
          // Intra-Brazil: short route, may need Ponta do Seixas if north<->south
          if (srcQuadrant !== destQuadrant) {
            return [M.pontaDoSeixas];
          }
          return null; // same sub-region: simple arc
        }
        return null;
      }

      // Caribbean sources (31xxx-39xxx)
      if (srcQuadrant === "caribbean") {
        if (destQuadrant === "caribbean") return null; // simple arc for short intra-Caribbean
        if (destQuadrant === "north-america") return null; // simple arc
        if (destQuadrant === "rio-plata" || destQuadrant === "brazil-south") {
          // Caribbean -> south: route through Lesser Antilles offshore, past Ponta do Seixas
          return [{ lat: 8, lng: -58 }, { lat: 0, lng: -45 }, M.pontaDoSeixas, { lat: -15, lng: -36 }, { lat: -25, lng: -44 }];
        }
        if (destQuadrant === "brazil-north") {
          return [{ lat: 8, lng: -55 }, { lat: 0, lng: -42 }];
        }
        if (destQuadrant === "pacific-south") {
          // Caribbean -> Pacific South: through Caribbean, past Central America, far south
          return [{ lat: 10, lng: -72 }, { lat: 5, lng: -78 }, { lat: -10, lng: -80 }, { lat: -35, lng: -75 }, { lat: -45, lng: -73 }];
        }
        return null;
      }

      // North American sources (20xxx-21xxx)
      if (srcQuadrant === "north-america") {
        if (destQuadrant === "caribbean") return null; // simple arc southward
        if (destQuadrant === "north-america") return null; // simple arc
        if (destQuadrant === "rio-plata" || destQuadrant === "brazil-south") {
          // North America -> south: past Florida, through Caribbean, south
          return [{ lat: 25, lng: -75 }, { lat: 15, lng: -65 }, { lat: 5, lng: -50 }, M.pontaDoSeixas, { lat: -20, lng: -38 }, { lat: -30, lng: -48 }];
        }
        if (destQuadrant === "brazil-north") {
          return [{ lat: 25, lng: -72 }, { lat: 15, lng: -60 }, { lat: 5, lng: -45 }];
        }
        if (destQuadrant === "pacific-south") {
          // North America -> Pacific: through Gulf, past Central America
          return [{ lat: 25, lng: -82 }, { lat: 15, lng: -80 }, { lat: 5, lng: -78 }, { lat: -10, lng: -80 }, { lat: -35, lng: -75 }, { lat: -45, lng: -73 }];
        }
        return null;
      }

      // European sources for I-Am dataset (rare but possible)
      if (srcQuadrant === "europe") {
        return null; // use simple arc
      }

      return null;
  }
}

function sampleSegmentOnLand(wp1, wp2, landFeatures, numSamples) {
  if (!landFeatures || typeof d3 === "undefined" || !d3.geoContains) return false;
  for (var s = 0; s <= numSamples; s++) {
    var t = s / numSamples;
    var lat = wp1.lat + (wp2.lat - wp1.lat) * t;
    var lng = wp1.lng + (wp2.lng - wp1.lng) * t;
    var pt = [lng, lat]; // GeoJSON uses [lng, lat]
    for (var f = 0; f < landFeatures.length; f++) {
      if (d3.geoContains(landFeatures[f], pt)) {
        return { lat: lat, lng: lng, index: s };
      }
    }
  }
  return false;
}

function isPointOnLand(lat, lng, landFeatures) {
  var pt = [lng, lat];
  for (var f = 0; f < landFeatures.length; f++) {
    if (d3.geoContains(landFeatures[f], pt)) return true;
  }
  return false;
}

function avoidLand(waypoints, landFeatures) {
  if (!landFeatures || !waypoints || waypoints.length < 2) return waypoints;

  var result = [waypoints[0]];
  for (var i = 0; i < waypoints.length - 1; i++) {
    var hit = sampleSegmentOnLand(waypoints[i], waypoints[i + 1], landFeatures, 12);
    if (hit) {
      var midLat = (waypoints[i].lat + waypoints[i + 1].lat) / 2;
      var midLng = (waypoints[i].lng + waypoints[i + 1].lng) / 2;
      // Try multiple push directions; pick the first that lands in ocean
      var candidates = [
        { lat: midLat, lng: midLng - 5 },           // west
        { lat: midLat, lng: midLng + 5 },           // east
        { lat: midLat - 3, lng: midLng - 5 },       // southwest
        { lat: midLat + 3, lng: midLng + 5 },       // northeast
        { lat: midLat - 5, lng: midLng },           // south
        { lat: midLat + 5, lng: midLng },           // north
        { lat: midLat, lng: midLng - 8 },           // far west
        { lat: midLat, lng: midLng + 8 },           // far east
      ];
      var bypass = null;
      for (var c = 0; c < candidates.length; c++) {
        if (!isPointOnLand(candidates[c].lat, candidates[c].lng, landFeatures)) {
          bypass = candidates[c];
          break;
        }
      }
      if (bypass) result.push(bypass);
    }
    result.push(waypoints[i + 1]);
  }
  return result;
}

function computeDynamicWaypoints(sourceCode, destCode, convergePt) {
  // Require land features for dynamic routing; fallback to static if unavailable
  if (!window._landFeatures) return null;

  // Cache lookup
  var cacheKey = sourceCode + ":" + destCode + ":" + Math.round(convergePt[0]) + ":" + Math.round(convergePt[1]);
  if (_waypointCache[cacheKey]) return _waypointCache[cacheKey];

  var destQuadrant = classifyDestQuadrant(destCode);
  var chain = buildWaypointChain(sourceCode, destQuadrant);
  if (!chain || chain.length === 0) return null;

  // Land avoidance pass
  chain = avoidLand(chain, window._landFeatures);

  // Cache and return
  _waypointCache[cacheKey] = chain;
  return chain;
}

function computeIAmConvergence(sourcePoints, destPt, destCode, destQuadrant) {
  var totalWeight = 0, wSumX = 0, wSumY = 0;
  sourcePoints.forEach(function(sp) {
    wSumX += sp.pt[0] * sp.weight;
    wSumY += sp.pt[1] * sp.weight;
    totalWeight += sp.weight;
  });
  if (totalWeight === 0) return destPt;
  var centroidX = wSumX / totalWeight;
  var centroidY = wSumY / totalWeight;

  var spread = d3.max(sourcePoints, function(s) { return s.pt[0]; })
              - d3.min(sourcePoints, function(s) { return s.pt[0]; });
  var frac = spread > 200 ? 0.35 : 0.5;

  var convergeX = centroidX + (destPt[0] - centroidX) * frac;
  var convergeY = centroidY + (destPt[1] - centroidY) * frac;

  var refLat = { "caribbean": 15, "north-america": 30, "brazil-north": 0,
    "brazil-south": -15, "rio-plata": -25, "pacific-south": -35 }[destQuadrant];
  if (refLat !== undefined && typeof _projection !== 'undefined') {
    var inverted = _projection.invert([convergeX, convergeY]);
    if (inverted) {
      var refPt = _projection([inverted[0], refLat]);
      if (refPt) convergeY = refPt[1] * 0.5 + convergeY * 0.5;
    }
  }

  return [convergeX, convergeY];
}


// ===== Westward Atlantic stems (Africa -> Americas) =====
//
// Each stem is built from its own source and never turns back east:
//   source -> offshore gate (just off its own coast) -> one cubic Bezier -> junction J
// J is placed after the sources are known, west of every gate, so nothing has
// to overshoot and hook back. Southeast Africa rounds the Cape and passes
// St Helena before joining. The trunk then runs J -> destination.

// How each African region reaches open water.
//   west:  step ~3 deg west of the source (coast faces west)
//   south: drop to a per-region "lane" latitude (coast faces south, Gulf of Guinea)
// Lane latitudes step down from west to east so stems nest instead of crossing.
var AFRICA_GATE_RULES = {
  60100: { mode: "west", maxLng: -18.5 },
  60200: { mode: "west", maxLng: -15 },
  60300: { mode: "west", maxLng: -10, eastOf: -7.7, laneLat: 4.0 },
  60400: { mode: "south", laneLat: 3.9 },
  60500: { mode: "south", laneLat: 3.4 },
  60600: { mode: "south", laneLat: 2.9 },
  60700: { mode: "west", maxLng: 10.5 },
  60900: { mode: "south", laneLat: 1.8 }   // anchor sits offshore, south of the lane
};

// Southeast Africa: down the Mozambique Channel, round the Cape, up to St Helena.
var CAPE_ROUTE = [
  { lat: -13, lng: 42 },
  { lat: -21.5, lng: 38.5 },
  { lat: -27.5, lng: 35 },
  { lat: -34, lng: 28.5 },
  { lat: -36, lng: 20 },
  { lat: -32.5, lng: 12 },
  { lat: -15.97, lng: -5.71 }
];

// Gulf of Guinea lanes run west to this longitude (past Cape Palmas) before
// turning for the junction, so they stay south of the Windward Coast stem.
var GULF_LANE_END_LNG = -11.5;

var STEM_FAN_SPACING = 3;   // px between neighbouring stems where they join the trunk
var STEM_FAN_MAX = 14;
var STEM_ARRIVAL_REACH = 22; // px, longest straight run-in to the junction      // px, total spread at the junction

function isAfricanCode(code) {
  return code >= 60000 && code < 70000;
}

// Offshore gate for one source: the point where its stem meets open water.
function africanGate(code, src) {
  var region = Math.floor(code / 100) * 100;
  var rule = AFRICA_GATE_RULES[region];
  if (!rule) return null;
  var south = rule.mode === "south" ||
    (rule.eastOf !== undefined && src.lng > rule.eastOf);
  if (south) {
    return { lat: Math.min(src.lat - 0.3, rule.laneLat), lng: src.lng - 1, lane: true };
  }
  return { lat: src.lat - 0.3, lng: Math.min(src.lng - 3, rule.maxLng) };
}

// Geographic points from the source to the place its Bezier starts.
function stemPrefix(code) {
  var src = _routeCoords(code);
  if (!src) return null;
  var region = Math.floor(code / 100) * 100;
  var pts = [{ lat: src.lat, lng: src.lng }];
  if (region === 60800) {
    for (var i = 0; i < CAPE_ROUTE.length; i++) {
      var wp = CAPE_ROUTE[i];
      var last = pts[pts.length - 1];
      // Only waypoints still ahead of the ship: further west, and further
      // south until the Cape is rounded.
      if (wp.lng >= last.lng - 1) continue;
      if (wp.lng > 19 && wp.lat >= last.lat) continue;
      pts.push(wp);
    }
    return pts;
  }
  var gate = africanGate(code, src);
  if (gate) {
    pts.push(gate);
    // Only lanes north of the equator need to clear Cape Palmas; a source
    // further south turns for the junction straight from its gate.
    if (gate.lane && gate.lat > 0 && gate.lng > GULF_LANE_END_LNG + 1) {
      pts.push({ lat: gate.lat, lng: GULF_LANE_END_LNG });
    }
  }
  return pts;
}

var _landBounds = null;
var _landBoundsFor = null;

// Point-on-land test with a bounding-box reject before d3.geoContains.
function isLandFast(lng, lat) {
  var feats = window._landFeatures;
  if (!feats || typeof d3 === "undefined") return false;
  if (_landBoundsFor !== feats) {
    _landBounds = feats.map(function(f) { return d3.geoBounds(f); });
    _landBoundsFor = feats;
  }
  for (var i = 0; i < feats.length; i++) {
    var b = _landBounds[i];
    if (lat < b[0][1] || lat > b[1][1]) continue;
    // Bounds that wrap the antimeridian have west > east.
    if (b[0][0] <= b[1][0] ? (lng < b[0][0] || lng > b[1][0])
                           : (lng < b[0][0] && lng > b[1][0])) continue;
    if (d3.geoContains(feats[i], [lng, lat])) return true;
  }
  return false;
}

function _cubicAt(a, c1, c2, b, t) {
  var u = 1 - t;
  return [
    u * u * u * a[0] + 3 * u * u * t * c1[0] + 3 * u * t * t * c2[0] + t * t * t * b[0],
    u * u * u * a[1] + 3 * u * u * t * c1[1] + 3 * u * t * t * c2[1] + t * t * t * b[1]
  ];
}

// Control point of the quadratic trunk arc (same shape as generateArc).
function trunkControlPoint(source, target, curveBias) {
  var dx = target[0] - source[0];
  var dy = target[1] - source[1];
  var dist = Math.sqrt(dx * dx + dy * dy);
  var curveAmount = Math.min(dist * 0.3, 100);
  var mx = (source[0] + target[0]) / 2;
  var my = (source[1] + target[1]) / 2;
  var angle = Math.atan2(dy, dx);
  if (curveBias === "south-atlantic") {
    return [mx + Math.sin(angle) * curveAmount, my - Math.cos(angle) * curveAmount * 0.3];
  }
  return [mx - Math.sin(angle) * curveAmount,
          my + Math.cos(angle) * curveAmount * 0.3 - curveAmount * 0.4];
}

// Where the stems meet. West of every gate, at least 10 deg east of the
// destination, at a latitude that follows the destination.
function planJunction(prefixes, dest, pacific) {
  var minLng = Infinity;
  for (var i = 0; i < prefixes.length; i++) {
    var a = prefixes[i][prefixes[i].length - 1];
    if (a.lng < minLng) minLng = a.lng;
  }
  // Pacific coast: meet in the South Atlantic, where the trunk can turn for
  // Cape Horn without touching Brazil.
  if (pacific) return { lat: PACIFIC_JUNCTION_LAT, lng: Math.min(minLng - 6, PACIFIC_JUNCTION_MAX_LNG) };
  // Capped in the north so stems run west first and the trunk carries the
  // turn toward North America, instead of stems hugging the African coast.
  var lat = Math.max(-26, Math.min(13, dest.lat * 0.75));
  var lng = minLng - 6;
  // North of the Gulf of Guinea the junction must also clear the West African bulge.
  if (lat > 2) lng = Math.min(lng, -25);
  lng = Math.max(lng, dest.lng + 10);
  if (lng > minLng - 3) {
    // A source far out in the Atlantic (Cape Verde, St Helena): tighten up,
    // as long as the junction still sits clear of the destination.
    lng = minLng - 3;
    if (lng < dest.lng + 4) return null;   // destination is not west of the sources
  }
  return { lat: lat, lng: lng };
}

// One stem: Bezier from its gate (a) into the junction slot (jo), heading west
// out of the gate and arriving along the trunk direction. Screen space.
function _stemBezier(a, jo, trunkDir, heading, southPush, pull) {
  var dx = a[0] - jo[0];              // > 0: the junction is west of the gate
  var dy = jo[1] - a[1];
  var c1;
  if (heading) {
    // Carry on along the course the stem already has (Cape route).
    var jl = Math.sqrt(dx * dx + dy * dy) || 1;
    var hx = heading[0] - dx / jl, hy = heading[1] + dy / jl;
    var hl = Math.sqrt(hx * hx + hy * hy) || 1;
    c1 = [a[0] + hx / hl * dx * 0.4, a[1] + hy / hl * dx * 0.4 + southPush];
  } else {
    // Leave heading west; lean toward the junction when it lies far to the
    // north or south, so a short hop west is not followed by a right-angle turn.
    var lean = Math.max(0, Math.min(1, (Math.abs(dy) / dx - 1) / 2));
    // Long stems hold their latitude for longer, so they pass outside the
    // shorter stems that join from nearer the junction.
    var hold = Math.max(0.4, Math.min(0.7, 0.4 + (dx - 60) / 220));
    c1 = [a[0] - dx * hold, a[1] + dy * 0.3 * lean + southPush];
  }
  // Capped so a stem from far away cannot swing wide of its neighbours on arrival.
  var reach = Math.min(dx * 0.35, STEM_ARRIVAL_REACH) * pull;
  var c2 = [jo[0] - trunkDir[0] * reach, jo[1] - trunkDir[1] * reach];
  c1[0] = Math.min(Math.max(c1[0], jo[0]), a[0]);
  c2[0] = Math.min(Math.max(c2[0], jo[0]), a[0]);
  var pts = [];
  for (var s = 1; s <= 20; s++) pts.push(_cubicAt(a, c1, c2, jo, s / 20));
  return pts;
}

function _countLandHits(pts) {
  if (!window._landFeatures || !_projection.invert) return 0;
  var hits = 0;
  // Skip the last sample: it is the junction, checked on its own.
  for (var i = 1; i < pts.length - 1; i++) {
    var geo = _projection.invert(pts[i]);
    if (geo && isLandFast(geo[0], geo[1])) hits++;
  }
  return hits;
}

// Drop any point that would step back east, so the stem only ever moves west.
function _keepWestward(pts) {
  var out = [pts[0]];
  for (var i = 1; i < pts.length; i++) {
    if (pts[i][0] <= out[out.length - 1][0]) out.push(pts[i]);
  }
  return out;
}

// Join source -> gate -> curve without a sharp corner at the gate: the gate
// vertex is replaced by two points that cut the corner.
function _roundGateCorner(pts) {
  if (pts.length < 4) return pts;
  var src = pts[0], gate = pts[1], next = pts[2];
  var nl = Math.sqrt((next[0] - gate[0]) * (next[0] - gate[0]) + (next[1] - gate[1]) * (next[1] - gate[1])) || 1;
  var cut = Math.min(0.5, 14 / nl);   // cut at most ~14px along the outgoing leg
  var inPt = [src[0] * 0.3 + gate[0] * 0.7, src[1] * 0.3 + gate[1] * 0.7];
  var outPt = [gate[0] + (next[0] - gate[0]) * cut, gate[1] + (next[1] - gate[1]) * cut];
  return [src, inPt, outPt].concat(pts.slice(2));
}

function _smoothPath(pts) {
  return d3.line().curve(d3.curveCatmullRom.alpha(0.5))(pts);
}

// A path without its leading moveto, for appending to a stem that ends where
// the path starts.
function _pathTail(d) {
  return " " + d.replace(/^\s*M[^A-Za-z]*/, "");
}

function _projectRoute(route) {
  return route.map(function(p) { return _projection([p.lng, p.lat]); });
}

// North to south along the coast, Southeast Africa last, so stems nest.
function _sortStemsNorthToSouth(stems) {
  stems.sort(function(a, b) {
    var ra = Math.floor(a.code / 100) * 100 === 60800;
    var rb = Math.floor(b.code / 100) * 100 === 60800;
    if (ra !== rb) return ra ? 1 : -1;
    var pa = a.prefix[a.prefix.length - 1], pb = b.prefix[b.prefix.length - 1];
    return (pb.lat - pa.lat) || (pa.lng - pb.lng);
  });
}

// ----- Pacific coast of the Americas (Peru, Chile, Ecuador, Panama) -----
//
// These lie west of the Americas, so one arc from the junction would cut
// straight across the continent. The stems still run west into a junction, but
// the junction sits in the South Atlantic and the trunk sails south-west,
// rounds Cape Horn and comes up the Pacific coast to the destination.

var PACIFIC_JUNCTION_LAT = -30;
var PACIFIC_JUNCTION_MAX_LNG = -27;   // junction is at least this far west
var PACIFIC_JUNCTION_MIN_LNG = -42;   // and never pushed closer to Brazil than this

// South Atlantic -> east of the Falklands -> south of Cape Horn -> Pacific.
var CAPE_HORN_ROUTE = [
  { lat: -45, lng: -46 },
  { lat: -55.5, lng: -57 },
  { lat: -58.5, lng: -67 },
  { lat: -56, lng: -77 }
];

// Offshore points up the Pacific coast, south to north. A destination uses
// the ones still south of it.
var PACIFIC_COAST_ROUTE = [
  { lat: -47, lng: -79.5 },
  { lat: -33, lng: -75.5 },
  { lat: -21, lng: -73.5 },
  { lat: -16, lng: -78 },
  { lat: -6, lng: -83 },
  { lat: 2, lng: -82 },
  { lat: 6.5, lng: -79.2 }
];

function isPacificDest(code) {
  return code >= 43000 && code < 47000;
}

function pacificTrunkRoute(dest) {
  var route = CAPE_HORN_ROUTE.slice();
  for (var i = 0; i < PACIFIC_COAST_ROUTE.length; i++) {
    if (PACIFIC_COAST_ROUTE[i].lat < dest.lat + 1) route.push(PACIFIC_COAST_ROUTE[i]);
  }
  return route;
}

// Plan every stem into one destination. Returns null when the destination is
// not west of the sources (Europe, Africa, Atlantic islands) so the caller can
// fall back to waypoint chains.
//   returns { junction: [x,y], trunk: "M..", trunkTail: "..", stems: { code: "M.." },
//             points: { code: [[x,y]..] }, order: [codes north->south] }
function planAtlanticStems(sourceCodes, destCode, yShift, trunkBias) {
  if (typeof _projection === "undefined" || !_projection) return null;
  var dest = _routeCoords(destCode);
  if (!dest || isAfricanCode(destCode) || dest.lng > -30) return null;
  var destPt = _projection([dest.lng, dest.lat]);
  if (!destPt) return null;

  var stems = [];
  for (var i = 0; i < sourceCodes.length; i++) {
    if (!isAfricanCode(sourceCodes[i])) return null;
    var prefix = stemPrefix(sourceCodes[i]);
    if (!prefix) return null;
    stems.push({ code: sourceCodes[i], prefix: prefix });
  }
  if (stems.length === 0) return null;

  // Pacific coast: the trunk is a sea route round Cape Horn, not one arc.
  var seaRoute = isPacificDest(destCode) ? pacificTrunkRoute(dest) : null;
  if (seaRoute) {
    // Southeast Africa heads for the Horn straight from the Cape; the leg up
    // to St Helena would only have to be sailed back south again.
    var stHelena = CAPE_ROUTE[CAPE_ROUTE.length - 1];
    stems.forEach(function(st) {
      if (st.prefix.length > 2 && st.prefix[st.prefix.length - 1] === stHelena) st.prefix.pop();
    });
  }
  var J = planJunction(stems.map(function(s) { return s.prefix; }), dest, !!seaRoute);
  if (!J) return null;
  var jMinLng = seaRoute ? PACIFIC_JUNCTION_MIN_LNG : dest.lng + 10;

  _sortStemsNorthToSouth(stems);

  var best = null;
  // If stems still touch land, move the junction west and try again.
  for (var attempt = 0; attempt < 5; attempt++) {
    var jGeo = { lat: J.lat, lng: Math.max(J.lng - attempt * 4, jMinLng) };
    if (attempt > 0 && isLandFast(jGeo.lng, jGeo.lat)) continue;
    var jPt = _projection([jGeo.lng, jGeo.lat]);
    jPt = [jPt[0], jPt[1] + (yShift || 0)];
    var ctrl, trunkD;
    if (seaRoute) {
      var trunkPts = [jPt].concat(_projectRoute(seaRoute), [destPt]);
      ctrl = trunkPts[1];
      trunkD = _smoothPath(trunkPts);
    } else {
      ctrl = trunkControlPoint(jPt, destPt, trunkBias);
      trunkD = "M " + jPt[0] + "," + jPt[1] + " Q " + ctrl[0] + "," + ctrl[1] +
               " " + destPt[0] + "," + destPt[1];
    }
    var tdx = ctrl[0] - jPt[0], tdy = ctrl[1] - jPt[1];
    var tlen = Math.sqrt(tdx * tdx + tdy * tdy) || 1;
    var trunkDir = [tdx / tlen, tdy / tlen];
    // Normal to the trunk, pointing to the northern side.
    var normal = trunkDir[0] <= 0 ? [trunkDir[1], -trunkDir[0]] : [-trunkDir[1], trunkDir[0]];
    if (normal[1] > 0) normal = [-normal[0], -normal[1]];
    var n = stems.length;
    var spacing = n > 1 ? Math.min(STEM_FAN_SPACING, STEM_FAN_MAX / (n - 1)) : 0;

    var totalHits = 0;
    var paths = {}, points = {}, pushes = {};
    for (var s = 0; s < n; s++) {
      var pre = stems[s].prefix.map(function(p) { return _projection([p.lng, p.lat]); });
      var a = pre[pre.length - 1];
      var off = ((n - 1) / 2 - s) * spacing;
      var jo = [jPt[0] + normal[0] * off, jPt[1] + normal[1] * off];
      var heading = null;
      if (Math.floor(stems[s].code / 100) * 100 === 60800 && pre.length > 1) {
        var prev = pre[pre.length - 2];
        var hl = Math.sqrt((a[0] - prev[0]) * (a[0] - prev[0]) + (a[1] - prev[1]) * (a[1] - prev[1])) || 1;
        heading = [(a[0] - prev[0]) / hl, (a[1] - prev[1]) / hl];
      }
      var curve = null, curveHits = Infinity, usedK = 0;
      // Bend further south, then shorten the approach, until the stem is at sea.
      for (var k = 0; k < 8 && curveHits > 0; k++) {
        var cand = _stemBezier(a, jo, trunkDir, heading, k * 7, k < 4 ? 1 : 0.4);
        var h = _countLandHits(cand);
        if (h < curveHits) { curve = cand; curveHits = h; usedK = k; }
      }
      totalHits += curveHits;
      pushes[stems[s].code] = usedK;
      var pts = _keepWestward(heading ? pre.concat(curve.slice(2))
        : _roundGateCorner(pre.concat(pre.length > 2 ? curve : curve.slice(2))));
      points[stems[s].code] = pts;
      paths[stems[s].code] = _smoothPath(pts);
    }
    if (!best || totalHits < best.hits) {
      best = {
        hits: totalHits,
        junction: jPt,
        junctionGeo: jGeo,
        trunk: trunkD,
        trunkTail: _pathTail(trunkD),
        stems: paths,
        points: points,
        pushes: pushes,
        order: stems.map(function(s) { return s.code; })
      };
    }
    if (totalHits === 0) break;
  }
  return best;
}


// ===== Northbound stems (Africa -> Europe) =====
//
// Europe lies north of every African source, so nothing here runs west.
// Each stem leaves through its usual offshore gate, joins a corridor that
// rounds the West African bulge well offshore, and meets the others at a
// junction off the Saharan coast. The trunk then follows sea lanes north:
// past Iberia, and on through the Bay of Biscay, the Channel, the North Sea
// or the Strait of Gibraltar as the destination needs.

// Corridor round the bulge, south to north. A stem joins at the first node
// that is still ahead of it.
var NORTH_CORRIDOR = [
  { lat: 2.5, lng: -13.5 },    // south-west of Cape Palmas
  { lat: 9, lng: -18.5 },      // off Sierra Leone
  { lat: 16, lng: -20.5 }      // west of Cap-Vert
];
var NORTH_JUNCTION = { lat: 23, lng: -22.5 };
// Stems share the corridor side by side: each one further down the coast runs
// further out to sea, by this multiple of their spacing at the junction.
var NORTH_LANE_SPREAD = 1.5;
var NORTH_ARRIVAL_REACH = 16;   // px of straight run-in to the junction

// Sea lanes to Europe as a tree: every node is reached from its `from` node.
var EUROPE_LANES = {
  canaries:    { lat: 30, lng: -19 },
  iberiaSW:    { lat: 36, lng: -14.5, from: "canaries" },
  cadizGulf:   { lat: 36.1, lng: -7.6, from: "iberiaSW" },
  // Mediterranean nodes only serve destinations east of them.
  gibraltar:   { lat: 35.95, lng: -5.6, from: "cadizGulf", med: true },
  alboran:     { lat: 36.3, lng: -2.2, from: "gibraltar", med: true },
  balearic:    { lat: 37.6, lng: 3.5, from: "alboran", med: true },
  sardiniaS:   { lat: 38.3, lng: 9.6, from: "balearic", med: true },
  tyrrhenian:  { lat: 40.3, lng: 12.3, from: "sardiniaS", med: true },
  portugal:    { lat: 40.5, lng: -12.5, from: "iberiaSW" },
  finisterre:  { lat: 44.6, lng: -10.8, from: "portugal" },
  biscay:      { lat: 45.8, lng: -4, from: "finisterre" },
  brittany:    { lat: 48, lng: -7, from: "finisterre" },
  celticSea:   { lat: 51, lng: -7, from: "brittany" },
  bristolChan: { lat: 51.3, lng: -4.6, from: "celticSea" },
  stGeorges:   { lat: 52, lng: -6, from: "celticSea" },
  irishSea:    { lat: 53.7, lng: -5.2, from: "stGeorges" },
  channelW:    { lat: 49.2, lng: -5.6, from: "brittany" },
  channelMid:  { lat: 50, lng: -2.9, from: "channelW" },
  seineBay:    { lat: 49.9, lng: -0.9, from: "channelMid" },
  channelE:    { lat: 50.3, lng: 0.1, from: "channelMid" },
  dover:       { lat: 51.1, lng: 1.5, from: "channelE" },
  thames:      { lat: 51.5, lng: 1.3, from: "dover" },
  zeeland:     { lat: 51.9, lng: 3, from: "dover" },
  texel:       { lat: 53.7, lng: 4.4, from: "zeeland" },
  germanBight: { lat: 54.4, lng: 7.5, from: "texel" },
  skagerrak:   { lat: 57.8, lng: 8.5, from: "texel" },
  skagen:      { lat: 57.95, lng: 10.8, from: "skagerrak" },
  kattegat:    { lat: 56.3, lng: 11.9, from: "skagen" }
};

function isEuropeanDest(code) {
  return code >= 10000 && code < 20000;
}

var _europeRouteCache = {};
var _europeRouteCacheFor = null;

var LANE_CLEARANCE = 0.4;   // degrees of sea room wanted either side of a leg

// Degrees of the straight line a -> b that lie on land or brush past it. The
// drawn trunk is a smoothed curve, so a leg needs sea room on both sides.
function _landDegrees(a, b) {
  var dLat = b.lat - a.lat, dLng = b.lng - a.lng;
  var len = Math.sqrt(dLat * dLat + dLng * dLng) || 1;
  var sLat = -dLng / len * LANE_CLEARANCE, sLng = dLat / len * LANE_CLEARANCE;
  var n = Math.max(2, Math.ceil(len / 0.1));
  var hits = 0;
  for (var i = 1; i < n; i++) {
    var lat = a.lat + dLat * i / n, lng = a.lng + dLng * i / n;
    if (isLandFast(lng, lat) || isLandFast(lng + sLng, lat + sLat) ||
        isLandFast(lng - sLng, lat - sLat)) hits++;
  }
  return hits * len / n;
}

// The lane nodes to follow to one destination: the chain whose last leg to
// the destination crosses the least land, then the shortest.
function europeTrunkRoute(destCode, dest) {
  if (_europeRouteCacheFor !== window._landFeatures) {
    _europeRouteCache = {};
    _europeRouteCacheFor = window._landFeatures;
  }
  var key = destCode + ":" + dest.lat + ":" + dest.lng;
  if (_europeRouteCache[key]) return _europeRouteCache[key];
  var best = null, bestScore = Infinity;
  for (var name in EUROPE_LANES) {
    var chain = [];
    for (var n = EUROPE_LANES[name]; n; n = EUROPE_LANES[n.from]) chain.unshift(n);
    var last = chain[chain.length - 1];
    if (last.lat > dest.lat + 1.5) continue;   // do not sail past and come back south
    if (last.med && last.lng > dest.lng) continue;
    var len = 0;
    for (var i = 1; i < chain.length; i++) {
      len += Math.sqrt(Math.pow(chain[i].lat - chain[i - 1].lat, 2) +
                       Math.pow(chain[i].lng - chain[i - 1].lng, 2));
    }
    len += Math.sqrt(Math.pow(dest.lat - last.lat, 2) + Math.pow(dest.lng - last.lng, 2));
    var score = _landDegrees(last, dest) * 25 + len;
    if (score < bestScore) { bestScore = score; best = chain; }
  }
  _europeRouteCache[key] = best;
  return best;
}

// Once a stem has reached its southernmost point (its gate, or the Cape),
// drop any point that would step back south.
function _keepNorthward(pts) {
  var south = 0;
  for (var i = 1; i < pts.length; i++) if (pts[i][1] > pts[south][1]) south = i;
  var out = pts.slice(0, south + 1);
  for (var j = south + 1; j < pts.length; j++) {
    if (pts[j][1] <= out[out.length - 1][1]) out.push(pts[j]);
  }
  return out;
}

// Plan every stem into one European destination. Same return shape as
// planAtlanticStems; null when a source lies too far north to join.
function planNorthboundStems(sourceCodes, destCode, yShift) {
  if (typeof _projection === "undefined" || !_projection) return null;
  var dest = _routeCoords(destCode);
  if (!dest || !isEuropeanDest(destCode)) return null;
  var destPt = _projection([dest.lng, dest.lat]);
  if (!destPt) return null;

  var stems = [];
  var maxEndLat = -Infinity;
  for (var i = 0; i < sourceCodes.length; i++) {
    if (!isAfricanCode(sourceCodes[i])) return null;
    var prefix = stemPrefix(sourceCodes[i]);
    if (!prefix) return null;
    stems.push({ code: sourceCodes[i], prefix: prefix });
    maxEndLat = Math.max(maxEndLat, prefix[prefix.length - 1].lat);
  }
  if (stems.length === 0) return null;
  _sortStemsNorthToSouth(stems);

  // The junction lies north of every gate (Arguim, the Canaries).
  var jGeo = { lat: Math.max(NORTH_JUNCTION.lat, maxEndLat + 2.5), lng: NORTH_JUNCTION.lng };
  if (jGeo.lat > dest.lat - 1.5) return null;
  var lanes = europeTrunkRoute(destCode, dest).filter(function(p) { return p.lat > jGeo.lat + 1; });

  var jPt = _projection([jGeo.lng, jGeo.lat]);
  jPt = [jPt[0], jPt[1] + (yShift || 0)];
  var trunkPts = [jPt].concat(_projectRoute(lanes), [destPt]);
  var trunkD = _smoothPath(trunkPts);
  var tdx = trunkPts[1][0] - jPt[0], tdy = trunkPts[1][1] - jPt[1];
  var tlen = Math.sqrt(tdx * tdx + tdy * tdy) || 1;
  var trunkDir = [tdx / tlen, tdy / tlen];
  // Normal to the trunk, pointing to the landward (east) side.
  var normal = [-trunkDir[1], trunkDir[0]];
  if (normal[0] < 0) normal = [-normal[0], -normal[1]];

  var n = stems.length;
  var spacing = n > 1 ? Math.min(STEM_FAN_SPACING, STEM_FAN_MAX / (n - 1)) : 0;
  var paths = {}, points = {};
  for (var s = 0; s < n; s++) {
    var pts = _projectRoute(stems[s].prefix);
    var end = stems[s].prefix[stems[s].prefix.length - 1];
    for (var c = 0; c < NORTH_CORRIDOR.length; c++) {
      var node = NORTH_CORRIDOR[c];
      if (node.lat <= end.lat + 1.5) continue;    // already past it
      if (node.lat >= jGeo.lat - 1.5) continue;
      if (end.lng < node.lng - 2) continue;       // already further out to sea
      end = node;
      var np = _projection([node.lng, node.lat]);
      pts.push([np[0] - s * spacing * NORTH_LANE_SPREAD, np[1]]);
    }
    // Nearest the coast on the landward side of the junction, so stems nest.
    var off = ((n - 1) / 2 - s) * spacing;
    var jo = [jPt[0] + normal[0] * off, jPt[1] + normal[1] * off];
    pts.push([jo[0] - trunkDir[0] * NORTH_ARRIVAL_REACH, jo[1] - trunkDir[1] * NORTH_ARRIVAL_REACH], jo);
    var cape = Math.floor(stems[s].code / 100) * 100 === 60800;
    pts = _keepNorthward(cape ? pts : _roundGateCorner(pts));
    points[stems[s].code] = pts;
    paths[stems[s].code] = _smoothPath(pts);
  }

  return {
    hits: 0,
    junction: jPt,
    junctionGeo: jGeo,
    trunk: trunkD,
    trunkTail: _pathTail(trunkD),
    stems: paths,
    points: points,
    order: stems.map(function(st) { return st.code; })
  };
}

// Entry point: plan the stems and trunk for flows out of Africa into one
// destination, whichever way it lies. Null means "no plan": the caller falls
// back to waypoint chains or a plain arc.
function planFlowStems(sourceCodes, destCode, yShift, trunkBias) {
  if (isEuropeanDest(destCode)) return planNorthboundStems(sourceCodes, destCode, yShift);
  return planAtlanticStems(sourceCodes, destCode, yShift, trunkBias);
}
