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
    var dotStroke = cs.getPropertyValue('--color-dot-stroke').trim() || (isDark ? "#3a4258" : "#fff");
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

// Three-way cross-highlight: FlowTable rows ↔ timeline bars ↔ accordion headers
(function initCrossHighlight() {
  var _rowHoverActive = false;

  // --- Helpers ---

  function getOpenAccordionCodes() {
    var codes = [];
    document.querySelectorAll('.accordion-body').forEach(function(body) {
      var hdr = body.previousElementSibling;
      if (hdr && hdr.classList.contains('accordion-header')) {
        var code = hdr.getAttribute('data-origin-code');
        if (code) codes.push(code);
      }
    });
    return codes;
  }

  // Restore tl-bar opacity based on current accordion state
  function restoreBarHighlight() {
    var openCodes = getOpenAccordionCodes();
    if (openCodes.length === 1) {
      d3.selectAll('.tl-bar').attr('opacity', 0.2);
      d3.selectAll('.tl-bar[data-source-code="' + openCodes[0] + '"]').attr('opacity', 1);
    } else {
      d3.selectAll('.tl-bar').attr('opacity', 0.85);
    }
  }

  function clearAccordionHeaderHighlight() {
    document.querySelectorAll('.accordion-header.cross-highlight').forEach(function(h) {
      h.classList.remove('cross-highlight');
    });
  }

  function clearFlowRowTransientHighlight() {
    document.querySelectorAll('.data-table tr.tl-bar-hover').forEach(function(r) {
      r.classList.remove('tl-bar-hover');
    });
  }

  // Highlight the stem matching code+dataset (and its halo) and dim the other trunk
  function highlightStemAndTrunk(code, ds) {
    if (!window._flowGroup) return;
    _flowGroup.selectAll('.stem-halo').remove();
    _flowGroup.selectAll('.stem').attr('opacity', 0.15);
    _flowGroup.selectAll('.stem[data-source-code="' + code + '"][data-dataset="' + ds + '"]')
      .attr('opacity', 1)
      .each(function() {
        var baseSW = +this.getAttribute('data-base-sw') || 2;
        _flowGroup.insert('path', '.stem')
          .attr('class', 'stem-halo')
          .attr('d', d3.select(this).attr('d'))
          .attr('stroke', 'white')
          .attr('stroke-width', baseSW + 7)
          .attr('fill', 'none')
          .attr('stroke-linecap', 'round')
          .attr('opacity', 0.9);
        d3.select(this).attr('stroke-width', baseSW + 3);
      });
    _flowGroup.selectAll('.trunk').attr('opacity', 0.15);
    _flowGroup.selectAll('.trunk[data-dataset="' + ds + '"]').attr('opacity', 0.8);
  }

  // Highlight all stems for a source code regardless of dataset (used when only code is known)
  function highlightStemByCode(code) {
    if (!window._flowGroup) return;
    _flowGroup.selectAll('.stem-halo').remove();
    _flowGroup.selectAll('.stem').attr('opacity', 0.15);
    _flowGroup.selectAll('.stem[data-source-code="' + code + '"]')
      .attr('opacity', 1)
      .each(function() {
        var baseSW = +this.getAttribute('data-base-sw') || 2;
        _flowGroup.insert('path', '.stem')
          .attr('class', 'stem-halo')
          .attr('d', d3.select(this).attr('d'))
          .attr('stroke', 'white')
          .attr('stroke-width', baseSW + 7)
          .attr('fill', 'none')
          .attr('stroke-linecap', 'round')
          .attr('opacity', 0.9);
        d3.select(this).attr('stroke-width', baseSW + 3);
      });
    // Trunks stay full opacity — dataset unknown so can't target one
  }

  // Restore stems and trunks to their resting state
  function restoreStemsAndTrunk() {
    if (!window._flowGroup) return;
    _flowGroup.selectAll('.stem-halo').remove();
    _flowGroup.selectAll('.stem').attr('opacity', 0.7)
      .each(function() {
        var baseSW = +this.getAttribute('data-base-sw') || 2;
        var k = _svg ? d3.zoomTransform(_svg.node()).k : 1;
        d3.select(this).attr('stroke-width', baseSW / Math.sqrt(k));
      });
    _flowGroup.selectAll('.trunk').attr('opacity', 0.75);
  }

  // Re-apply accordion-driven highlights after Dioxus re-renders
  function refreshAccordionHighlight() {
    var openCodes = getOpenAccordionCodes();

    // Update flow-table row persistent tints
    document.querySelectorAll('.data-table tr.accordion-active').forEach(function(r) {
      r.classList.remove('accordion-active');
    });
    openCodes.forEach(function(code) {
      document.querySelectorAll('.data-table tr[data-source-code="' + code + '"]').forEach(function(r) {
        r.classList.add('accordion-active');
      });
    });

    // Update tl-bar opacity (skip if row hover owns it)
    if (!_rowHoverActive) {
      restoreBarHighlight();
      // Update stem/trunk to match accordion state
      if (openCodes.length === 1) {
        highlightStemByCode(openCodes[0]);
      } else {
        restoreStemsAndTrunk();
      }
    }
  }

  // --- Direction 1: FlowTable row hover → tl-bars + accordion header ---

  document.addEventListener('mouseover', function(e) {
    var row = e.target.closest && e.target.closest('.data-table tr[data-source-code]');
    if (!row) return;
    var code = row.getAttribute('data-source-code');
    var ds = row.getAttribute('data-dataset');
    if (!code || !ds) return;
    _rowHoverActive = true;
    d3.selectAll('.tl-bar').attr('opacity', 0.2);
    d3.selectAll('.tl-bar[data-source-code="' + code + '"][data-dataset="' + ds + '"]')
      .attr('opacity', 1);
    clearAccordionHeaderHighlight();
    var hdr = document.querySelector('.accordion-header[data-origin-code="' + code + '"]');
    if (hdr) hdr.classList.add('cross-highlight');
    highlightStemAndTrunk(code, ds);
  });

  document.addEventListener('mouseout', function(e) {
    var row = e.target.closest && e.target.closest('.data-table tr[data-source-code]');
    if (!row) return;
    _rowHoverActive = false;
    restoreBarHighlight();
    clearAccordionHeaderHighlight();
    restoreStemsAndTrunk();
  });

  // --- Direction 2: Accordion click → tl-bars + FlowTable rows ---
  // setTimeout(50) waits for Dioxus to finish re-rendering after the click.

  document.addEventListener('click', function(e) {
    var hdr = e.target.closest && e.target.closest('.accordion-header');
    if (!hdr) return;
    setTimeout(refreshAccordionHighlight, 50);
  });

  // --- Direction 3: tl-bar hover → FlowTable rows + accordion header ---
  // The existing D3 handler in map-timeline.js handles bar/stem opacity.
  // This handler adds the HTML-side highlights.

  document.addEventListener('mouseover', function(e) {
    if (!e.target || !e.target.classList) return;
    if (!e.target.classList.contains('tl-bar')) return;
    var code = e.target.getAttribute('data-source-code');
    var ds = e.target.getAttribute('data-dataset');
    if (!code || !ds) return;
    clearFlowRowTransientHighlight();
    document.querySelectorAll('.data-table tr[data-source-code="' + code + '"][data-dataset="' + ds + '"]')
      .forEach(function(r) { r.classList.add('tl-bar-hover'); });
    clearAccordionHeaderHighlight();
    var hdr = document.querySelector('.accordion-header[data-origin-code="' + code + '"]');
    if (hdr) hdr.classList.add('cross-highlight');
  });

  document.addEventListener('mouseout', function(e) {
    if (!e.target || !e.target.classList) return;
    if (!e.target.classList.contains('tl-bar')) return;
    clearFlowRowTransientHighlight();
    clearAccordionHeaderHighlight();
  });

  // --- Direction 4: Accordion header hover → tl-bars + FlowTable rows + stem ---

  document.addEventListener('mouseover', function(e) {
    var hdr = e.target.closest && e.target.closest('.accordion-header[data-origin-code]');
    if (!hdr) return;
    var code = hdr.getAttribute('data-origin-code');
    if (!code) return;
    // Dim all tl-bars, brighten matching source (across all datasets)
    d3.selectAll('.tl-bar').attr('opacity', 0.2);
    d3.selectAll('.tl-bar[data-source-code="' + code + '"]').attr('opacity', 1);
    // Highlight matching flow table rows transiently
    clearFlowRowTransientHighlight();
    document.querySelectorAll('.data-table tr[data-source-code="' + code + '"]')
      .forEach(function(r) { r.classList.add('tl-bar-hover'); });
    // Highlight matching stems (all datasets)
    highlightStemByCode(code);
  });

  document.addEventListener('mouseout', function(e) {
    var hdr = e.target.closest && e.target.closest('.accordion-header[data-origin-code]');
    if (!hdr) return;
    // Ignore mouse movement between child elements of the same header
    if (hdr.contains(e.relatedTarget)) return;
    clearFlowRowTransientHighlight();
    if (!_rowHoverActive) {
      restoreBarHighlight();
      var openCodes = getOpenAccordionCodes();
      if (openCodes.length === 1) {
        highlightStemByCode(openCodes[0]);
      } else {
        restoreStemsAndTrunk();
      }
    }
  });
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
