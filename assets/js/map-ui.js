function setTimeLabel(minYear, maxYear) {
  if (!_uiGroup) return;
  _uiGroup.selectAll(".time-label").remove();
  _uiGroup
    .append("text")
    .attr("class", "time-label")
    .attr("x", _width / 2)
    .attr("y", _height - 8)
    .attr("text-anchor", "middle")
    .attr("fill", "#666")
    .attr("font-size", "12px")
    .text(minYear + " \u2013 " + maxYear);
}

// Render legend in bottom-left of SVG
function renderLegend(datasetMode) {
  if (!_uiGroup) return;
  _uiGroup.selectAll(".legend-group").remove();

  var g = _uiGroup.append("g").attr("class", "legend-group")
    .attr("transform", "translate(12, " + (_height - 60) + ")");

  g.append("text")
    .attr("x", 0).attr("y", 0)
    .attr("font-size", "9px").attr("font-weight", "600")
    .attr("fill", "#666").attr("text-transform", "uppercase")
    .attr("letter-spacing", "0.5px")
    .text("FLOW ORIGINS");

  var yOff = 14;
  if (datasetMode === "transatlantic" || datasetMode === "combined") {
    g.append("rect").attr("x", 0).attr("y", yOff).attr("width", 10).attr("height", 10)
      .attr("fill", "#b09040").attr("rx", 2);
    g.append("text").attr("x", 14).attr("y", yOff + 8)
      .attr("font-size", "9px").attr("fill", "#555")
      .text("African origins (Transatlantic)");
    yOff += 16;
  }
  if (datasetMode === "interamerican" || datasetMode === "combined") {
    g.append("rect").attr("x", 0).attr("y", yOff).attr("width", 10).attr("height", 10)
      .attr("fill", "#3a7a8a").attr("rx", 2);
    g.append("text").attr("x", 14).attr("y", yOff + 8)
      .attr("font-size", "9px").attr("fill", "#555")
      .text("American origins (Inter-American)");
  }
}

// Expose functions globally
window.initMap = initMap;
window.renderDestinations = renderDestinations;
window.renderFlows = renderFlows;
window.clearFlows = clearFlows;
window.setTimeLabel = setTimeLabel;
window.renderLegend = renderLegend;
window.resetZoom = resetZoom;
window.renderTimeline = renderTimeline;
window.clearTimeline = clearTimeline;
