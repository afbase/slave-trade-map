// Dynamic route waypoints for slave trade flow map
// Computes destination-aware waypoint chains with land avoidance

var MARITIME_LANDMARKS = {
  capeAgulhas: { lat: -34.83, lng: 20 },
  westOfCape: { lat: -33, lng: 10 },
  stHelena: { lat: -15.97, lng: -5.71 },
  capeVerde: { lat: 14.93, lng: -23.51 },
  pontaDoSeixas: { lat: -7.15, lng: -34.79 },
  // Offshore reference points per zone
  senegambiaOffshore: { lat: 14.5, lng: -20 },
  sierraLeoneOffshore: { lat: 8.5, lng: -16 },
  windwardOffshore: { lat: 6, lng: -12 },
  goldCoastOffshore: { lat: 6, lng: -5 },
  beninOffshore: { lat: 6, lng: -2 },
  biafraOffshore: { lat: 4, lng: 2 },
  wcaOffshore: { lat: -8, lng: 5 },
  seAfricaOffshore: { lat: -25, lng: 25 },
  otherAfricaCoast: { lat: 2, lng: 8 }
};

var _waypointCache = {};

function classifyDestQuadrant(destCode) {
  var dc = regionCoords[destCode];
  if (!dc) return "mid-atlantic";
  var lat = dc.lat;
  var lng = dc.lng;

  if (lat >= 8 && lat <= 25 && lng >= -100 && lng <= -59) return "caribbean";
  if (lat > 25 && lng >= -100 && lng <= -60) return "north-america";
  if (lat >= -10 && lat <= 5 && lng >= -55 && lng <= -30) return "brazil-north";
  if (lat < -10 && lat >= -25 && lng >= -55 && lng <= -30) return "brazil-south";
  if (lat < -25 && lng >= -65 && lng <= -50) return "rio-plata";
  if (lat > 35 && lng > -15) return "europe";
  return "mid-atlantic";
}

function isBrazilQuadrant(q) {
  return q === "brazil-north" || q === "brazil-south";
}

function isWestwardQuadrant(q) {
  return q === "caribbean" || q === "north-america";
}

function buildWaypointChain(sourceCode, destQuadrant) {
  var src = Math.floor(sourceCode / 100) * 100;
  var M = MARITIME_LANDMARKS;

  switch (src) {
    case 60800: // SE Africa
      var chain = [M.capeAgulhas, M.westOfCape, M.stHelena];
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        chain.push({ lat: -10, lng: -20 });
      } else if (isWestwardQuadrant(destQuadrant)) {
        chain.push({ lat: 0, lng: -20 });
      } else if (destQuadrant === "europe") {
        chain.push({ lat: 10, lng: -18 });
      }
      return chain;

    case 60700: // W Central Africa
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.wcaOffshore, M.stHelena, { lat: -10, lng: -18 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.wcaOffshore, { lat: 0, lng: -15 }, { lat: 10, lng: -35 }];
      }
      if (destQuadrant === "europe") {
        return [M.wcaOffshore, M.stHelena, { lat: 10, lng: -18 }];
      }
      return [M.wcaOffshore, { lat: 0, lng: -15 }];

    case 60600: // Bight of Biafra
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.biafraOffshore, { lat: -5, lng: -5 }, { lat: -8, lng: -18 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.biafraOffshore, { lat: 8, lng: -25 }, { lat: 14, lng: -40 }];
      }
      if (destQuadrant === "europe") {
        return [M.biafraOffshore, { lat: 10, lng: -10 }, { lat: 30, lng: -15 }];
      }
      return [M.biafraOffshore, { lat: 0, lng: -10 }];

    case 60500: // Bight of Benin
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.beninOffshore, { lat: 0, lng: -10 }, { lat: -5, lng: -18 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.beninOffshore, { lat: 10, lng: -30 }, { lat: 15, lng: -45 }];
      }
      if (destQuadrant === "europe") {
        return [M.beninOffshore, { lat: 12, lng: -10 }, { lat: 35, lng: -12 }];
      }
      return [M.beninOffshore, { lat: 5, lng: -15 }];

    case 60400: // Gold Coast
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.goldCoastOffshore, { lat: 2, lng: -15 }, { lat: -3, lng: -18 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.goldCoastOffshore, { lat: 12, lng: -35 }, { lat: 16, lng: -50 }];
      }
      if (destQuadrant === "europe") {
        return [M.goldCoastOffshore, { lat: 14, lng: -12 }, { lat: 35, lng: -10 }];
      }
      return [M.goldCoastOffshore, { lat: 5, lng: -20 }];

    case 60300: // Windward Coast
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.windwardOffshore, { lat: 0, lng: -20 }, { lat: -3, lng: -20 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.windwardOffshore, { lat: 13, lng: -38 }, { lat: 16, lng: -52 }];
      }
      if (destQuadrant === "europe") {
        return [M.windwardOffshore, { lat: 15, lng: -18 }, { lat: 35, lng: -12 }];
      }
      return [M.windwardOffshore, { lat: 5, lng: -25 }];

    case 60200: // Sierra Leone
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.sierraLeoneOffshore, { lat: 2, lng: -22 }, { lat: -3, lng: -22 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.sierraLeoneOffshore, { lat: 14, lng: -40 }, { lat: 16, lng: -55 }];
      }
      if (destQuadrant === "europe") {
        return [M.sierraLeoneOffshore, { lat: 16, lng: -20 }, { lat: 38, lng: -12 }];
      }
      return [M.sierraLeoneOffshore, { lat: 5, lng: -28 }];

    case 60100: // Senegambia
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.senegambiaOffshore, M.capeVerde, { lat: 5, lng: -28 }, { lat: -3, lng: -25 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.senegambiaOffshore, M.capeVerde, { lat: 15, lng: -45 }];
      }
      if (destQuadrant === "europe") {
        return [M.senegambiaOffshore, { lat: 25, lng: -18 }, { lat: 38, lng: -12 }];
      }
      return [M.senegambiaOffshore, M.capeVerde];

    case 60900: // Other Africa / Interior — route to coast then follow Biafra path
      var coastWps = [M.otherAfricaCoast, M.biafraOffshore];
      var biafraChain = buildWaypointChain(60600, destQuadrant);
      if (biafraChain) {
        // Skip the first point (biafraOffshore) since we already have it
        for (var i = 1; i < biafraChain.length; i++) {
          coastWps.push(biafraChain[i]);
        }
      }
      return coastWps;

    default:
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

function avoidLand(waypoints, landFeatures) {
  if (!landFeatures || !waypoints || waypoints.length < 2) return waypoints;

  var result = [waypoints[0]];
  for (var i = 0; i < waypoints.length - 1; i++) {
    var hit = sampleSegmentOnLand(waypoints[i], waypoints[i + 1], landFeatures, 12);
    if (hit) {
      // Push the path offshore: subtract 5 from lng (westward) or lat (southward)
      var midLat = (waypoints[i].lat + waypoints[i + 1].lat) / 2;
      var midLng = (waypoints[i].lng + waypoints[i + 1].lng) / 2;
      // Determine push direction based on hemisphere
      if (midLat > 0) {
        // Northern hemisphere: push west
        result.push({ lat: midLat, lng: midLng - 5 });
      } else {
        // Southern hemisphere: push south and west
        result.push({ lat: midLat - 3, lng: midLng - 5 });
      }
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

function euclidean(a, b) {
  return Math.sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2);
}

function discreteFrechet(P, Q) {
  var n = P.length, m = Q.length;
  if (n === 0 || m === 0) return { distance: Infinity, coupling: [] };
  var dp = [];
  for (var i = 0; i < n; i++) {
    dp[i] = new Float64Array(m);
    for (var j = 0; j < m; j++) {
      var d = euclidean(P[i], Q[j]);
      if (i === 0 && j === 0) dp[i][j] = d;
      else if (i === 0) dp[i][j] = Math.max(dp[0][j-1], d);
      else if (j === 0) dp[i][j] = Math.max(dp[i-1][0], d);
      else dp[i][j] = Math.max(Math.min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]), d);
    }
  }
  // Backtrack coupling
  var coupling = [];
  var i = n - 1, j = m - 1;
  coupling.push([i, j]);
  while (i > 0 || j > 0) {
    if (i === 0) { j--; }
    else if (j === 0) { i--; }
    else {
      var candidates = [
        [i-1, j-1, dp[i-1][j-1]],
        [i-1, j, dp[i-1][j]],
        [i, j-1, dp[i][j-1]]
      ];
      candidates.sort(function(a, b) { return a[2] - b[2]; });
      i = candidates[0][0];
      j = candidates[0][1];
    }
    coupling.push([i, j]);
  }
  coupling.reverse();
  return { distance: dp[n-1][m-1], coupling: coupling };
}

var STEM_MIN_SPACING = 12;

function separateStems(stemInfos, convergePt, destCode) {
  if (!stemInfos || stemInfos.length < 2) return;
  if (!_projection || !_projection.invert) return;

  // Build projected waypoint paths for each stem
  var paths = stemInfos.map(function(info) {
    var destQuadrant = classifyDestQuadrant(destCode);
    var chain = buildWaypointChain(info.sourceCode, destQuadrant);
    if (!chain) return null;

    var pts = [info.srcPt];
    for (var i = 0; i < chain.length; i++) {
      var pp = _projection([chain[i].lng, chain[i].lat]);
      if (pp) pts.push(pp);
    }
    pts.push(convergePt);
    return { info: info, pts: pts, chain: chain };
  }).filter(Boolean);

  if (paths.length < 2) return;

  // For each adjacent pair, check Fréchet distance
  for (var i = 0; i < paths.length - 1; i++) {
    var pathA = paths[i];
    var pathB = paths[i + 1];
    var result = discreteFrechet(pathA.pts, pathB.pts);

    if (result.distance < STEM_MIN_SPACING) {
      var coupling = result.coupling;
      var secondary = pathB;
      var chain = secondary.chain;

      // Build per-waypoint minimum distance from coupling
      // pathB.pts indices: 0=srcPt, 1..chain.length=waypoints, last=convergePt
      // We only nudge chain waypoints (indices 1 through chain.length)
      var waypointMinDist = [];
      for (var w = 0; w < chain.length; w++) {
        waypointMinDist[w] = Infinity;
      }

      // Scan coupling for entries involving pathB waypoint indices
      for (var c = 0; c < coupling.length; c++) {
        var bIdx = coupling[c][1];  // index into pathB.pts
        var aIdx = coupling[c][0];  // index into pathA.pts
        // bIdx 0 is srcPt, bIdx chain.length+1 is convergePt — skip those
        if (bIdx >= 1 && bIdx <= chain.length) {
          var d = euclidean(pathA.pts[aIdx], pathB.pts[bIdx]);
          var wIdx = bIdx - 1;
          if (d < waypointMinDist[wIdx]) waypointMinDist[wIdx] = d;
        }
      }

      // Nudge each waypoint proportional to its actual proximity deficit
      for (var w = 0; w < chain.length; w++) {
        var wp = chain[w];
        var projected = _projection([wp.lng, wp.lat]);
        if (!projected) continue;

        var localDeficit = STEM_MIN_SPACING - waypointMinDist[w];
        if (localDeficit <= 0) continue;  // this waypoint is already far enough

        // Taper at chain endpoints so we don't distort source/convergence joins
        var t = (w + 1) / (chain.length + 1);
        var taper = Math.sin(Math.PI * t);
        var offset = localDeficit * taper;

        // Perpendicular direction: rotate flow vector 90° clockwise
        var dx = convergePt[0] - projected[0];
        var dy = convergePt[1] - projected[1];
        var len = Math.sqrt(dx * dx + dy * dy);
        if (len < 1) continue;
        var perpX = dy / len;
        var perpY = -dx / len;

        var newScreenX = projected[0] + perpX * offset;
        var newScreenY = projected[1] + perpY * offset;
        var inv = _projection.invert([newScreenX, newScreenY]);
        if (inv) {
          chain[w] = { lat: inv[1], lng: inv[0] };
        }
      }

      // Update waypoint cache with modified chain
      var cacheKey = secondary.info.sourceCode + ":" + destCode + ":" +
        Math.round(convergePt[0]) + ":" + Math.round(convergePt[1]);
      _waypointCache[cacheKey] = chain;
    }
  }
}
