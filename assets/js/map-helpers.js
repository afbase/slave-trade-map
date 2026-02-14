// For ports, fall back to parent region centroid (first 3 digits * 100)
function getCoords(code) {
  if (regionCoords[code]) return regionCoords[code];
  var parent = Math.floor(code / 100) * 100;
  if (regionCoords[parent]) return regionCoords[parent];
  return null;
}

function generateArc(source, target) {
  if (!source || !target) return null;
  var dx = target[0] - source[0];
  var dy = target[1] - source[1];
  var dist = Math.sqrt(dx * dx + dy * dy);
  if (dist < 1) return null;

  var curveAmount = Math.min(dist * 0.3, 100);
  var mx = (source[0] + target[0]) / 2;
  var my = (source[1] + target[1]) / 2;
  var angle = Math.atan2(dy, dx);
  var cxp = mx - Math.sin(angle) * curveAmount;
  var cyp = my + Math.cos(angle) * curveAmount * 0.3 - curveAmount * 0.4;

  return "M " + source[0] + "," + source[1] + " Q " + cxp + "," + cyp + " " + target[0] + "," + target[1];
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
