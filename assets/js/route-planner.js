// Dynamic route waypoints for slave trade flow map
// Computes destination-aware waypoint chains with land avoidance

var MARITIME_LANDMARKS = {
  capeAgulhas: { lat: -34.83, lng: 20 },
  westOfCape: { lat: -33, lng: 10 },
  stHelena: { lat: -15.97, lng: -5.71 },
  capeVerde: { lat: 14.93, lng: -23.51 },
  pontaDoSeixas: { lat: -7.15, lng: -33.8 },
  // Offshore reference points per zone
  senegambiaOffshore: { lat: 14.5, lng: -20 },
  sierraLeoneOffshore: { lat: 8.5, lng: -16 },
  windwardOffshore: { lat: 6, lng: -12 },
  goldCoastOffshore: { lat: 4, lng: -7 },
  beninOffshore: { lat: 5, lng: -4 },
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
  var sc = regionCoords[sourceCode] || regionCoords[Math.floor(sourceCode/100)*100];
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

  switch (src) {
    case 60800: // SE Africa
      var chain = [M.capeAgulhas, M.westOfCape, M.stHelena];
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        chain.push({ lat: -14, lng: -20 });
      } else if (isWestwardQuadrant(destQuadrant)) {
        chain.push({ lat: -6, lng: -18 }, { lat: -4, lng: -26 });
      } else if (destQuadrant === "europe") {
        chain.push({ lat: 0, lng: -16 }, { lat: 10, lng: -20 });
      }
      return chain;

    case 60700: // W Central Africa
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.wcaOffshore, M.stHelena, { lat: -11, lng: -18 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.wcaOffshore, { lat: -1, lng: -12 }, { lat: 2, lng: -32 }];
      }
      if (destQuadrant === "europe") {
        return [M.wcaOffshore, M.stHelena, { lat: 8, lng: -16 }, { lat: 16, lng: -18 }];
      }
      return [M.wcaOffshore, { lat: 0, lng: -15 }];

    case 60600: // Bight of Biafra
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.biafraOffshore, { lat: -5, lng: -5 }, { lat: -8, lng: -17 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.biafraOffshore, { lat: 7, lng: -20 }, { lat: 8, lng: -38 }];
      }
      if (destQuadrant === "europe") {
        return [M.biafraOffshore, { lat: 14, lng: -10 }, { lat: 25, lng: -14 }];
      }
      return [M.biafraOffshore, { lat: 0, lng: -10 }];

    case 60500: // Bight of Benin
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.beninOffshore, { lat: 0, lng: -10 }, { lat: -4, lng: -18 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.beninOffshore, { lat: 11, lng: -25 }, { lat: 14, lng: -42 }];
      }
      if (destQuadrant === "europe") {
        return [M.beninOffshore, { lat: 16, lng: -10 }, { lat: 34, lng: -12 }];
      }
      return [M.beninOffshore, { lat: 5, lng: -15 }];

    case 60400: // Gold Coast
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.goldCoastOffshore, { lat: -2, lng: -14 }, { lat: -6, lng: -18 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.goldCoastOffshore, { lat: 9, lng: -28 }, { lat: 10, lng: -43 }];
      }
      if (destQuadrant === "europe") {
        return [M.goldCoastOffshore, { lat: 16, lng: -12 }, { lat: 28, lng: -11 }];
      }
      return [M.goldCoastOffshore, { lat: 5, lng: -20 }];

    case 60300: // Windward Coast
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.windwardOffshore, { lat: -1, lng: -18 }, { lat: -5, lng: -20 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.windwardOffshore, { lat: 10, lng: -30 }, { lat: 12, lng: -45 }];
      }
      if (destQuadrant === "europe") {
        return [M.windwardOffshore, { lat: 18, lng: -16 }, { lat: 31, lng: -13 }];
      }
      return [M.windwardOffshore, { lat: 5, lng: -25 }];

    case 60200: // Sierra Leone
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.sierraLeoneOffshore, { lat: 2, lng: -20 }, { lat: -2, lng: -22 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.sierraLeoneOffshore, { lat: 13, lng: -35 }, { lat: 16, lng: -50 }];
      }
      if (destQuadrant === "europe") {
        return [M.sierraLeoneOffshore, { lat: 20, lng: -20 }, { lat: 37, lng: -13 }];
      }
      return [M.sierraLeoneOffshore, { lat: 5, lng: -28 }];

    case 60100: // Senegambia
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.senegambiaOffshore, M.capeVerde, { lat: 5, lng: -28 }, { lat: 0, lng: -24 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.senegambiaOffshore, M.capeVerde, { lat: 18, lng: -45 }];
      }
      if (destQuadrant === "europe") {
        return [M.senegambiaOffshore, { lat: 28, lng: -18 }, { lat: 40, lng: -12 }];
      }
      return [M.senegambiaOffshore, M.capeVerde];

    case 60900: // Other Africa / Interior — separate corridor from 60600
      if (isBrazilQuadrant(destQuadrant) || destQuadrant === "rio-plata") {
        return [M.otherAfricaCoast, { lat: 0, lng: 0 }, { lat: -7, lng: -8 }, { lat: -9, lng: -16 }];
      }
      if (isWestwardQuadrant(destQuadrant)) {
        return [M.otherAfricaCoast, { lat: 2, lng: 0 }, { lat: 5, lng: -20 }, { lat: 6, lng: -36 }];
      }
      if (destQuadrant === "europe") {
        return [M.otherAfricaCoast, { lat: 2, lng: 0 }, { lat: 12, lng: -10 }, { lat: 22, lng: -15 }];
      }
      return [M.otherAfricaCoast, { lat: 0, lng: 0 }, { lat: -2, lng: -10 }];

    default:
      // Inter-American sources
      var srcQuadrant = classifySourceQuadrant(sourceCode);
      if (srcQuadrant === "unknown") return null;

      // Brazilian sources
      if (srcQuadrant === "brazil-north" || srcQuadrant === "brazil-south") {
        var srcCoord = regionCoords[sourceCode] || regionCoords[Math.floor(sourceCode/100)*100];
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

// ===== Spine-and-Ribbon System for Non-Crossing Stems =====

function taperAlpha(t) {
  t = Math.max(0, Math.min(1, t));
  return 1 - 3 * t * t + 2 * t * t * t;
}

function arcLengthReparameterize(spinePoints, numSamples) {
  numSamples = numSamples || 40;
  if (spinePoints.length < 2) return spinePoints.slice();
  var cumLen = [0];
  for (var i = 1; i < spinePoints.length; i++) {
    var dx = spinePoints[i][0] - spinePoints[i - 1][0];
    var dy = spinePoints[i][1] - spinePoints[i - 1][1];
    cumLen.push(cumLen[i - 1] + Math.sqrt(dx * dx + dy * dy));
  }
  var totalLen = cumLen[cumLen.length - 1];
  if (totalLen < 1) return spinePoints.slice();
  var samples = [];
  for (var s = 0; s <= numSamples; s++) {
    var targetLen = (s / numSamples) * totalLen;
    var segIdx = cumLen.length - 2;
    for (var j = 1; j < cumLen.length; j++) {
      if (cumLen[j] >= targetLen) { segIdx = j - 1; break; }
    }
    var segLen = cumLen[segIdx + 1] - cumLen[segIdx];
    var t = segLen > 0 ? (targetLen - cumLen[segIdx]) / segLen : 0;
    samples.push([
      spinePoints[segIdx][0] + t * (spinePoints[segIdx + 1][0] - spinePoints[segIdx][0]),
      spinePoints[segIdx][1] + t * (spinePoints[segIdx + 1][1] - spinePoints[segIdx][1])
    ]);
  }
  return samples;
}

function computeFrenetFrame(samples) {
  var n = samples.length;
  var tangents = [];
  var normals = [];
  for (var i = 0; i < n; i++) {
    var tx, ty;
    if (i === 0) {
      tx = samples[1][0] - samples[0][0];
      ty = samples[1][1] - samples[0][1];
    } else if (i === n - 1) {
      tx = samples[n - 1][0] - samples[n - 2][0];
      ty = samples[n - 1][1] - samples[n - 2][1];
    } else {
      tx = samples[i + 1][0] - samples[i - 1][0];
      ty = samples[i + 1][1] - samples[i - 1][1];
    }
    var len = Math.sqrt(tx * tx + ty * ty);
    if (len < 0.001) { tx = 1; ty = 0; len = 1; }
    tx /= len;
    ty /= len;
    tangents.push([tx, ty]);
    normals.push([-ty, tx]);
  }
  return { tangents: tangents, normals: normals };
}

function computeSpine(stemInfos, convergePt, destCode) {
  if (!_projection || !_projection.invert) return null;
  var destQuadrant = classifyDestQuadrant(destCode);
  var groupA = stemInfos.filter(function(info) {
    var src = Math.floor(info.sourceCode / 100) * 100;
    return src !== 60800;
  });
  if (groupA.length === 0) return null;

  // Find median source by latitude
  var sorted = groupA.slice().sort(function(a, b) {
    var scA = getCoords(a.sourceCode);
    var scB = getCoords(b.sourceCode);
    return (scA ? scA.lat : 0) - (scB ? scB.lat : 0);
  });
  var medianSource = sorted[Math.floor(sorted.length / 2)];

  var chain = buildWaypointChain(medianSource.sourceCode, destQuadrant);
  if (!chain || chain.length === 0) return null;
  chain = avoidLand(chain, window._landFeatures);

  // Centroid of Group A source positions (screen space)
  var sumX = 0, sumY = 0;
  for (var i = 0; i < groupA.length; i++) {
    sumX += groupA[i].srcPt[0];
    sumY += groupA[i].srcPt[1];
  }
  var centroid = [sumX / groupA.length, sumY / groupA.length];

  // Spine: centroid -> projected waypoints -> convergePt
  var spinePoints = [centroid];
  for (var i = 0; i < chain.length; i++) {
    var pp = _projection([chain[i].lng, chain[i].lat]);
    if (pp) spinePoints.push(pp);
  }
  spinePoints.push(convergePt);
  return spinePoints;
}

function computeRibbonOffsets(sortedInfos, spineSamples, frame, spacing, sign) {
  var n = sortedInfos.length;
  var maxAbsOffset = Math.abs((n - 1) / 2 * spacing);

  // Compute curvature cap at each sample
  var curvCap = [];
  for (var k = 0; k < spineSamples.length; k++) {
    if (k === 0 || k === spineSamples.length - 1) {
      curvCap[k] = 1.0;
      continue;
    }
    var dot = frame.tangents[k-1][0] * frame.tangents[k][0] +
              frame.tangents[k-1][1] * frame.tangents[k][1];
    dot = Math.max(-1, Math.min(1, dot));
    var angle = Math.acos(dot);
    var dx = spineSamples[k][0] - spineSamples[k-1][0];
    var dy = spineSamples[k][1] - spineSamples[k-1][1];
    var segLen = Math.sqrt(dx * dx + dy * dy);
    var kappa = segLen > 0.001 ? angle / segLen : 0;
    var R = kappa > 0.001 ? 1 / kappa : 1e6;
    curvCap[k] = maxAbsOffset > 0 ? Math.min(1.0, R / maxAbsOffset) : 1.0;
  }

  // Smooth the cap to avoid abrupt transitions (3-point moving average)
  var smoothCap = curvCap.slice();
  for (var k = 1; k < curvCap.length - 1; k++) {
    smoothCap[k] = (curvCap[k-1] + curvCap[k] + curvCap[k+1]) / 3;
  }

  var ribbons = [];
  for (var i = 0; i < n; i++) {
    var d_i = sign * (i - (n - 1) / 2) * spacing;
    var ribbonPts = [];
    for (var k = 0; k < spineSamples.length; k++) {
      var t = k / (spineSamples.length - 1);
      var alpha = taperAlpha(t) * smoothCap[k];
      ribbonPts.push([
        spineSamples[k][0] + d_i * alpha * frame.normals[k][0],
        spineSamples[k][1] + d_i * alpha * frame.normals[k][1]
      ]);
    }
    ribbons.push({ info: sortedInfos[i], points: ribbonPts, offset: d_i });
  }
  return ribbons;
}

function computeJunction(capeEndPt, spineSamples, frame, landFeatures) {
  var maxIdx = Math.floor(spineSamples.length * 0.7);
  var bestIdx = Math.floor(spineSamples.length * 0.3);
  var bestScore = -Infinity;
  for (var k = 1; k <= maxIdx; k++) {
    var dx = spineSamples[k][0] - capeEndPt[0];
    var dy = spineSamples[k][1] - capeEndPt[1];
    var len = Math.sqrt(dx * dx + dy * dy);
    if (len < 1) continue;
    dx /= len; dy /= len;
    var alignment = Math.abs(dx * frame.tangents[k][0] + dy * frame.tangents[k][1]);
    if (_projection && _projection.invert && landFeatures) {
      var geo = _projection.invert(spineSamples[k]);
      if (geo) {
        var onLand = false;
        for (var f = 0; f < landFeatures.length; f++) {
          if (d3.geoContains(landFeatures[f], [geo[0], geo[1]])) { onLand = true; break; }
        }
        if (onLand) continue;
      }
    }
    if (alignment > bestScore) { bestScore = alignment; bestIdx = k; }
  }
  return bestIdx;
}

function taperAlpha60800(t, tJ) {
  if (t < tJ || tJ >= 1) return 0;
  return taperAlpha((t - tJ) / (1 - tJ));
}

function ribbonToGeoWaypoints(screenPts) {
  var geoWps = [];
  var lastX = -Infinity, lastY = -Infinity;
  for (var j = 0; j < screenPts.length; j++) {
    var dx = screenPts[j][0] - lastX;
    var dy = screenPts[j][1] - lastY;
    if (Math.sqrt(dx * dx + dy * dy) < 10 && j < screenPts.length - 1) continue;
    var inv = _projection.invert(screenPts[j]);
    if (inv) {
      geoWps.push({ lat: inv[1], lng: inv[0] });
      lastX = screenPts[j][0];
      lastY = screenPts[j][1];
    }
  }
  return geoWps;
}

function ribbonToScreenWaypoints(screenPts) {
  var result = [];
  var lastX = -Infinity, lastY = -Infinity;
  for (var j = 0; j < screenPts.length; j++) {
    var dx = screenPts[j][0] - lastX;
    var dy = screenPts[j][1] - lastY;
    if (Math.sqrt(dx * dx + dy * dy) < 10 && j < screenPts.length - 1) continue;
    result.push([screenPts[j][0], screenPts[j][1]]);
    lastX = screenPts[j][0];
    lastY = screenPts[j][1];
  }
  return result;
}

function computeRibbonStems(stemInfos, convergePt, destCode) {
  if (!stemInfos || stemInfos.length < 2) return;
  if (!_projection || !_projection.invert) return;

  // Only apply ribbon to transatlantic African sources
  var hasAfrican = stemInfos.some(function(info) {
    var src = Math.floor(info.sourceCode / 100) * 100;
    return src >= 60100 && src <= 60900;
  });
  if (!hasAfrican) return;

  var destQuadrant = classifyDestQuadrant(destCode);

  // Separate Group A (West Coast) and 60800 (SE Africa)
  var groupA = [];
  var se60800 = null;
  for (var i = 0; i < stemInfos.length; i++) {
    var src = Math.floor(stemInfos[i].sourceCode / 100) * 100;
    if (src === 60800) { se60800 = stemInfos[i]; }
    else { groupA.push(stemInfos[i]); }
  }
  if (groupA.length === 0) return;

  // Sort Group A north-to-south (matching map-flows.js sort)
  groupA.sort(function(a, b) {
    var scA = getCoords(a.sourceCode);
    var scB = getCoords(b.sourceCode);
    return (scB ? scB.lat : 0) - (scA ? scA.lat : 0);
  });

  // Step 1: Compute spine
  var spinePoints = computeSpine(stemInfos, convergePt, destCode);
  if (!spinePoints || spinePoints.length < 2) return;

  // Step 2: Arc-length reparameterize
  var NUM_SAMPLES = 80;
  var spineSamples = arcLengthReparameterize(spinePoints, NUM_SAMPLES);
  var frame = computeFrenetFrame(spineSamples);

  // Step 3: Determine offset sign so offsets match source spread direction
  var firstSrc = groupA[0].srcPt;
  var lastSrc = groupA[groupA.length - 1].srcPt;
  var spreadDx = lastSrc[0] - firstSrc[0];
  var spreadDy = lastSrc[1] - firstSrc[1];
  var dot = spreadDx * frame.normals[0][0] + spreadDy * frame.normals[0][1];
  var sign = dot >= 0 ? 1 : -1;

  // Step 4: Compute ribbon offsets
  var spacing = STEM_MIN_SPACING;
  var ribbons = computeRibbonOffsets(groupA, spineSamples, frame, spacing, sign);

  // Land check with retry (reduce spacing if ribbons cross land)
  for (var attempt = 0; attempt < 3; attempt++) {
    var anyOnLand = false;
    if (window._landFeatures && _projection.invert) {
      for (var r = 0; r < ribbons.length && !anyOnLand; r++) {
        for (var k = Math.floor(ribbons[r].points.length * 0.15);
             k < Math.floor(ribbons[r].points.length * 0.85) && !anyOnLand; k += 3) {
          var geo = _projection.invert(ribbons[r].points[k]);
          if (geo && isPointOnLand(geo[1], geo[0], window._landFeatures)) {
            anyOnLand = true;
          }
        }
      }
    }
    if (!anyOnLand) break;
    spacing *= 0.8;
    ribbons = computeRibbonOffsets(groupA, spineSamples, frame, spacing, sign);
  }

  // Step 5: Cache waypoints for each Group A stem
  var SNAP_THRESHOLD = 4; // pixels — snap ribbon points to spine when offset is tiny
  for (var i = 0; i < ribbons.length; i++) {
    var ribbon = ribbons[i];
    var info = ribbon.info;
    // Ribbon waypoints: skip last point (it equals convergePt since taper→0)
    var screenPts = [];
    for (var k = 0; k < ribbon.points.length - 1; k++) {
      var t = k / (spineSamples.length - 1);
      var alpha = taperAlpha(t);
      var effectiveOffset = Math.abs(ribbon.offset * alpha);
      if (effectiveOffset < SNAP_THRESHOLD) {
        screenPts.push(spineSamples[k]); // snap to spine
      } else {
        screenPts.push(ribbon.points[k]);
      }
    }
    var screenWps = ribbonToScreenWaypoints(screenPts);
    var cacheKey = info.sourceCode + ":" + destCode + ":" +
      Math.round(convergePt[0]) + ":" + Math.round(convergePt[1]);
    _waypointCache[cacheKey] = { _screen: true, points: screenWps };
  }

  // Step 6: Handle 60800 (SE Africa) junction
  if (se60800) {
    var chain60800 = buildWaypointChain(60800, destQuadrant);
    if (chain60800 && chain60800.length > 0) {
      chain60800 = avoidLand(chain60800, window._landFeatures);
      var lastWp = chain60800[chain60800.length - 1];
      var capeEndPt = _projection([lastWp.lng, lastWp.lat]);
      if (capeEndPt) {
        var jIdx = computeJunction(capeEndPt, spineSamples, frame, window._landFeatures);
        var tJ = jIdx / (spineSamples.length - 1);

        // 60800 gets the last offset position
        var totalStems = groupA.length + 1;
        var d_60800 = sign * (groupA.length - (totalStems - 1) / 2) * spacing;

        // Junction target with offset
        var junctionTarget = [
          spineSamples[jIdx][0] + d_60800 * frame.normals[jIdx][0],
          spineSamples[jIdx][1] + d_60800 * frame.normals[jIdx][1]
        ];

        // Build ribbon segment from junction to convergePt
        var ribbonSeg = [];
        for (var k = jIdx; k < spineSamples.length - 1; k++) {
          var t = k / (spineSamples.length - 1);
          var alpha = taperAlpha60800(t, tJ);
          var effectiveOffset = Math.abs(d_60800 * alpha);
          if (effectiveOffset < SNAP_THRESHOLD) {
            ribbonSeg.push(spineSamples[k].slice());
          } else {
            ribbonSeg.push([
              spineSamples[k][0] + d_60800 * alpha * frame.normals[k][0],
              spineSamples[k][1] + d_60800 * alpha * frame.normals[k][1]
            ]);
          }
        }

        // Full 60800 screen path: cape waypoints → junction → ribbon segment
        var fullScreen = [];
        for (var w = 0; w < chain60800.length; w++) {
          var pp = _projection([chain60800[w].lng, chain60800[w].lat]);
          if (pp) fullScreen.push(pp);
        }
        fullScreen.push(junctionTarget);
        for (var k = 0; k < ribbonSeg.length; k++) {
          fullScreen.push(ribbonSeg[k]);
        }

        var screenWps = ribbonToScreenWaypoints(fullScreen);
        var cacheKey = se60800.sourceCode + ":" + destCode + ":" +
          Math.round(convergePt[0]) + ":" + Math.round(convergePt[1]);
        _waypointCache[cacheKey] = { _screen: true, points: screenWps };
      }
    }
  }
}
