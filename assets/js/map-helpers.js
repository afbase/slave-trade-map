// For ports, fall back to parent region centroid (first 3 digits * 100)
function getCoords(code) {
  var c = window.__coords;
  if (c) {
    if (c[code]) return c[code];
    return c[Math.floor(code / 100) * 100] || null;
  }
  if (regionCoords[code]) return regionCoords[code];
  return regionCoords[Math.floor(code / 100) * 100] || null;
}

function generateArc(source, target, curveBias) {
  if (!source || !target) return null;
  var dx = target[0] - source[0];
  var dy = target[1] - source[1];
  var dist = Math.sqrt(dx * dx + dy * dy);
  if (dist < 1) return null;

  var curveAmount = Math.min(dist * 0.3, 100);
  var mx = (source[0] + target[0]) / 2;
  var my = (source[1] + target[1]) / 2;
  var angle = Math.atan2(dy, dx);

  var cxp, cyp;
  if (curveBias === "south-atlantic") {
    cxp = mx + Math.sin(angle) * curveAmount;
    cyp = my - Math.cos(angle) * curveAmount * 0.3;
  } else {
    cxp = mx - Math.sin(angle) * curveAmount;
    cyp = my + Math.cos(angle) * curveAmount * 0.3 - curveAmount * 0.4;
  }

  return "M " + source[0] + "," + source[1] + " Q " + cxp + "," + cyp + " " + target[0] + "," + target[1];
}

function generateMultiSegmentPath(points) {
  if (!points || points.length < 2) return null;
  if (points.length === 2) return generateArc(points[0], points[1]);

  // Guard 1: Proximity filter — remove consecutive points closer than 8px
  var MIN_SEGMENT_PX = 8;
  var filtered = [points[0]];
  for (var f = 1; f < points.length; f++) {
    var prev = filtered[filtered.length - 1];
    var cur = points[f];
    var fdist = Math.sqrt((cur[0]-prev[0])**2 + (cur[1]-prev[1])**2);
    if (fdist >= MIN_SEGMENT_PX || f === points.length - 1) {
      filtered.push(cur);
    }
  }
  points = filtered;
  if (points.length < 2) return null;
  if (points.length === 2) return generateArc(points[0], points[1]);

  // Guard 2: X-monotonicity check — drop penultimate point if last segments backtrack
  if (points.length >= 3) {
    var last = points[points.length - 1];
    var penult = points[points.length - 2];
    var ante = points[points.length - 3];
    var dirAP = penult[0] - ante[0];
    var dirPL = last[0] - penult[0];
    if ((dirAP > 0 && dirPL < -5) || (dirAP < 0 && dirPL > 5)) {
      points.splice(points.length - 2, 1);
    }
  }
  if (points.length < 2) return null;
  if (points.length === 2) return generateArc(points[0], points[1]);

  // Catmull-Rom to cubic Bezier conversion for smooth path through all points
  var n = points.length;
  var d = "M " + points[0][0] + "," + points[0][1];

  for (var i = 0; i < n - 1; i++) {
    var p0 = points[Math.max(0, i - 1)];
    var p1 = points[i];
    var p2 = points[i + 1];
    var p3 = points[Math.min(n - 1, i + 2)];

    // Catmull-Rom to cubic Bezier control points (alpha = 0.5)
    var cp1x = p1[0] + (p2[0] - p0[0]) / 6;
    var cp1y = p1[1] + (p2[1] - p0[1]) / 6;
    var cp2x = p2[0] - (p3[0] - p1[0]) / 6;
    var cp2y = p2[1] - (p3[1] - p1[1]) / 6;

    d += " C " + cp1x + "," + cp1y + " " + cp2x + "," + cp2y + " " + p2[0] + "," + p2[1];
  }

  return d;
}

function generateRoutedPath(sourceCode, srcPt, convergePt, destCode) {
  // Try dynamic waypoints first (if route-planner.js is loaded)
  if (typeof computeDynamicWaypoints === 'function' && destCode) {
    var dynamicWps = computeDynamicWaypoints(sourceCode, destCode, convergePt);
    if (dynamicWps) {
      var allPoints = [srcPt];
      for (var i = 0; i < dynamicWps.length; i++) {
        var projected = _projection([dynamicWps[i].lng, dynamicWps[i].lat]);
        if (projected) allPoints.push(projected);
      }
      allPoints.push(convergePt);
      return _multiSegPath(allPoints);
    }
  }
  // Prefer Rust static waypoints
  var waypoints = (window.__waypoints && window.__waypoints[sourceCode])
    ? window.__waypoints[sourceCode]
    : routeWaypoints[sourceCode];
  if (waypoints) {
    var allPoints = [srcPt];
    for (var i = 0; i < waypoints.length; i++) {
      var projected = _projection([waypoints[i].lng, waypoints[i].lat]);
      if (projected) {
        allPoints.push(projected);
      }
    }
    allPoints.push(convergePt);
    return _multiSegPath(allPoints);
  }
  return (typeof window.generate_arc === 'function')
    ? window.generate_arc(srcPt[0], srcPt[1], convergePt[0], convergePt[1], "")
    : generateArc(srcPt, convergePt);

  function _multiSegPath(pts) {
    if (typeof window.generate_multi_segment_path === 'function') {
      return window.generate_multi_segment_path(JSON.stringify(pts)) || null;
    }
    return generateMultiSegmentPath(pts);
  }
}

function drawArrowhead(group, arcPath, strokeWidth, color) {
  var tempPath = document.createElementNS("http://www.w3.org/2000/svg", "path");
  tempPath.setAttribute("d", arcPath);
  var totalLen = tempPath.getTotalLength();
  if (totalLen < 1) return;

  var end = tempPath.getPointAtLength(totalLen);
  var before = tempPath.getPointAtLength(Math.max(0, totalLen - 8));
  var angle = Math.atan2(end.y - before.y, end.x - before.x);

  var arrowLen = strokeWidth * 2.2 + 4;
  var arrowW = strokeWidth * 1.1 + 2;

  var tipX = end.x;
  var tipY = end.y;
  var baseX = tipX - Math.cos(angle) * arrowLen;
  var baseY = tipY - Math.sin(angle) * arrowLen;
  var leftX = baseX - Math.sin(angle) * arrowW;
  var leftY = baseY + Math.cos(angle) * arrowW;
  var rightX = baseX + Math.sin(angle) * arrowW;
  var rightY = baseY - Math.cos(angle) * arrowW;

  group
    .append("polygon")
    .attr("class", "flow-arrowhead")
    .attr("points", tipX + "," + tipY + " " + leftX + "," + leftY + " " + rightX + "," + rightY)
    .attr("data-tip-x", tipX)
    .attr("data-tip-y", tipY)
    .attr("fill", color)
    .attr("opacity", 0.85);
}
