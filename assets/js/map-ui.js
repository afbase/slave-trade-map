function setTimeLabel(minYear, maxYear) {
  if (!_uiGroup) return;
  _uiGroup.selectAll(".time-label").remove();
  var cs = getComputedStyle(document.documentElement);
  _uiGroup
    .append("text")
    .attr("class", "time-label")
    .attr("x", _width / 2)
    .attr("y", _height - 8)
    .attr("text-anchor", "middle")
    .attr("fill", cs.getPropertyValue('--color-text-muted').trim() || "#666")
    .attr("font-size", "12px")
    .text(minYear + " \u2013 " + maxYear);
}

// Render legend in bottom-left of SVG
function renderLegend(datasetMode) {
  if (!_uiGroup) return;
  _uiGroup.selectAll(".legend-group").remove();
  var cs = getComputedStyle(document.documentElement);

  var g = _uiGroup.append("g").attr("class", "legend-group")
    .attr("transform", "translate(12, " + (_height - 60) + ")");

  g.append("text")
    .attr("x", 0).attr("y", 0)
    .attr("font-size", "9px").attr("font-weight", "600")
    .attr("fill", cs.getPropertyValue('--color-text-muted').trim() || "#666")
    .attr("text-transform", "uppercase")
    .attr("letter-spacing", "0.5px")
    .text("FLOW ORIGINS");

  var yOff = 14;
  if (datasetMode === "transatlantic" || datasetMode === "combined") {
    g.append("rect").attr("x", 0).attr("y", yOff).attr("width", 10).attr("height", 10)
      .attr("fill", cs.getPropertyValue('--color-ta').trim() || "#b09040").attr("rx", 2);
    g.append("text").attr("x", 14).attr("y", yOff + 8)
      .attr("font-size", "9px").attr("fill", cs.getPropertyValue('--color-text-light').trim() || "#555")
      .text("African origins (Transatlantic)");
    yOff += 16;
  }
  if (datasetMode === "interamerican" || datasetMode === "combined") {
    g.append("rect").attr("x", 0).attr("y", yOff).attr("width", 10).attr("height", 10)
      .attr("fill", cs.getPropertyValue('--color-iam').trim() || "#3a7a8a").attr("rx", 2);
    g.append("text").attr("x", 14).attr("y", yOff + 8)
      .attr("font-size", "9px").attr("fill", cs.getPropertyValue('--color-text-light').trim() || "#555")
      .text("American origins (Inter-American)");
  }
}

// Update all D3 SVG elements when theme changes (light <-> dark)
function updateMapTheme() {
  var cs = getComputedStyle(document.documentElement);
  var isDark = document.body.classList.contains('dark');

  // Background rect
  if (_svg) {
    _svg.select(".map-bg")
      .attr("fill", cs.getPropertyValue('--color-bg').trim() || "#f5f3ef");
  }

  // Destination dots
  if (_dotsGroup) {
    var accentColor = cs.getPropertyValue('--color-accent').trim() || "#5a7a8a";
    var dotStroke = isDark ? "#2a2e33" : "#fff";
    _dotsGroup.selectAll(".dest-dot").each(function() {
      var dot = d3.select(this);
      // Don't override selected (red) dots
      if (dot.attr("fill") !== "#c04030") {
        dot.attr("fill", accentColor).attr("stroke", dotStroke);
      }
    });
    // Dest labels via CSS (color: var works in SVG text via CSS)
    _dotsGroup.selectAll(".dest-label")
      .attr("fill", cs.getPropertyValue('--color-text-muted').trim() || "#888");
  }

  // Time label
  if (_uiGroup) {
    _uiGroup.selectAll(".time-label")
      .attr("fill", cs.getPropertyValue('--color-text-muted').trim() || "#666");
    // Legend text
    _uiGroup.selectAll(".legend-group text")
      .attr("fill", function() {
        return d3.select(this).text() === "FLOW ORIGINS"
          ? cs.getPropertyValue('--color-text-muted').trim() || "#666"
          : cs.getPropertyValue('--color-text-light').trim() || "#555";
      });
  }

  // Timeline axis
  d3.selectAll(".tl-axis text")
    .style("fill", cs.getPropertyValue('--color-text-muted').trim() || "#888");
  d3.selectAll(".tl-axis path, .tl-axis line")
    .style("stroke", cs.getPropertyValue('--color-border').trim() || "#ccc");

  // Stem halos should match theme
  var haloColor = isDark ? "#2a2e33" : "white";
  if (_flowGroup) {
    _flowGroup.selectAll(".stem-halo").attr("stroke", haloColor);
  }
}

// Watch for body.dark class toggle (slavecodes.org theme switch)
(function() {
  var observer = new MutationObserver(function(mutations) {
    mutations.forEach(function(m) {
      if (m.attributeName === "class") {
        updateMapTheme();
      }
    });
  });
  observer.observe(document.body, { attributes: true, attributeFilter: ["class"] });
})();

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
window.updateMapTheme = updateMapTheme;
