// Render a single flow group (stems + trunk) for one dataset
function renderFlowGroup(flows, destPt, dataset, zOffset, destCode) {
  if (flows.length === 0) return;

  var totalEmbarked = d3.sum(flows, function(d) { return d.total_embarked; });
  var maxEmbarked = d3.max(flows, function(d) { return d.total_embarked; });
  var strokeScale = d3.scaleSqrt().domain([0, maxEmbarked]).range([1.5, 14]);

  // Compute convergence point — positioned mid-Atlantic
  var westmostSourceX = Infinity;
  var sourcePoints = [];
  flows.forEach(function(f) {
    var sc = getCoords(f.source_code);
    if (!sc) return;
    var pt = _projection([sc.lng, sc.lat]);
    if (!pt) return;
    sourcePoints.push({ pt: pt, code: f.source_code, weight: f.total_embarked });
    if (pt[0] < westmostSourceX) westmostSourceX = pt[0];
  });
  if (sourcePoints.length === 0) return;

  var convergePt;
  var destQuadrant = typeof classifyDestQuadrant === 'function'
    ? classifyDestQuadrant(destCode) : "mid-atlantic";

  if (dataset === "interamerican" && typeof computeIAmConvergence === 'function') {
    convergePt = computeIAmConvergence(sourcePoints, destPt, destCode, destQuadrant);
  } else {
    var convergeX = (westmostSourceX + destPt[0]) / 2;
    var refLat = { "caribbean": 8, "north-america": 25, "brazil-north": -3,
      "brazil-south": -12, "rio-plata": -20, "pacific-south": -30, "europe": 35 }[destQuadrant] || 5;
    var refPt = _projection([0, refLat]);
    var convergeY = refPt ? refPt[1] : destPt[1];
    convergePt = [convergeX, convergeY];
  }

  // Ensure convergence point is over ocean
  if (window._landFeatures && _projection.invert) {
    var geo = _projection.invert(convergePt);
    if (geo) {
      var onLand = false;
      for (var li = 0; li < window._landFeatures.length; li++) {
        if (d3.geoContains(window._landFeatures[li], geo)) { onLand = true; break; }
      }
      if (onLand) {
        // Shift convergence eastward (toward mid-Atlantic) until off land
        var safeGeo = [geo[0], geo[1]];
        var shiftDir = (destQuadrant === "pacific-south") ? -1 : 1;
        for (var shift = 5; shift <= 40; shift += 5) {
          safeGeo[0] = geo[0] + shift * shiftDir;
          var still = false;
          for (var li2 = 0; li2 < window._landFeatures.length; li2++) {
            if (d3.geoContains(window._landFeatures[li2], safeGeo)) { still = true; break; }
          }
          if (!still) break;
        }
        var safePt = _projection(safeGeo);
        if (safePt) convergePt = [safePt[0], safePt[1]];
      }
    }
  }

  // For combined mode, offset convergence slightly to separate trunk groups
  if (zOffset !== 0) {
    convergePt[1] += zOffset * 15;
  }

  // Sort stems by source latitude (south→north), SE Africa always at bottom
  flows.sort(function(a, b) {
    var scA = getCoords(a.source_code);
    var scB = getCoords(b.source_code);
    var srcA = Math.floor(a.source_code / 100) * 100;
    var srcB = Math.floor(b.source_code / 100) * 100;
    if (srcA === 60800 && srcB !== 60800) return 1;
    if (srcB === 60800 && srcA !== 60800) return -1;
    return (scB ? scB.lat : 0) - (scA ? scA.lat : 0);
  });

  // Spine-and-ribbon stem separation (non-crossing by construction)
  if (typeof computeRibbonStems === 'function') {
    var stemInfos = flows.map(function(f) {
      var sc = getCoords(f.source_code);
      if (!sc) return null;
      var srcPt = _projection([sc.lng, sc.lat]);
      if (!srcPt) return null;
      return { sourceCode: f.source_code, srcPt: srcPt, destCode: destCode };
    }).filter(Boolean);
    computeRibbonStems(stemInfos, convergePt, destCode);
  }

  var cs = getComputedStyle(document.documentElement);
  var trunkColor = dataset === "transatlantic"
    ? (cs.getPropertyValue('--color-ta').trim() || "#6a5830")
    : (cs.getPropertyValue('--color-iam').trim() || "#2a5a6a");

  // Draw stems
  flows.forEach(function(flow) {
    var sc = getCoords(flow.source_code);
    if (!sc) return;
    var srcPt = _projection([sc.lng, sc.lat]);
    if (!srcPt) return;

    var sw = strokeScale(flow.total_embarked);
    var color = getSourceColor(flow.source_code, dataset);
    var pct = ((flow.total_embarked / totalEmbarked) * 100).toFixed(1);
    var dsLabel = dataset === "transatlantic" ? "Transatlantic" : "Inter-American";

    var stemPath = generateRoutedPath(flow.source_code, srcPt, convergePt, destCode);
    if (!stemPath) return;

    var stemEl = _flowGroup
      .append("path")
      .attr("class", "flow-path stem")
      .attr("d", stemPath)
      .attr("stroke", color)
      .attr("stroke-width", sw)
      .attr("data-base-sw", sw)
      .attr("data-source-code", flow.source_code)
      .attr("data-dataset", dataset)
      .attr("fill", "none")
      .attr("stroke-linecap", "round")
      .attr("opacity", 0.7);

    // Year range string for tooltip
    var yearStr = '';
    if (flow.first_year != null && flow.last_year != null) {
      yearStr = flow.first_year === flow.last_year
        ? '<br>Year: ' + flow.first_year
        : '<br>Active: ' + flow.first_year + ' \u2013 ' + flow.last_year;
    }

    stemEl
      .on("mouseover", function(event) {
        var self = d3.select(this);
        var curSW = +self.attr("stroke-width");

        // Dim all other stems, brighten this one
        _flowGroup.selectAll(".stem").attr("opacity", 0.15);
        self.attr("opacity", 1).attr("stroke-width", curSW + 3);

        // Insert a white halo path behind the hovered stem
        _flowGroup.insert("path", ".stem")
          .attr("class", "stem-halo")
          .attr("d", self.attr("d"))
          .attr("stroke", "white")
          .attr("stroke-width", curSW + 7)
          .attr("fill", "none")
          .attr("stroke-linecap", "round")
          .attr("opacity", 0.9);

        _tooltipDiv.style.opacity = "1";
        _tooltipDiv.innerHTML =
          '<div class="tt-route">' + flow.source_name + ' &rarr; ' + flow.target_name +
          ' <span style="font-size:10px;color:#888">(' + dsLabel + ')</span></div>' +
          '<div class="tt-stat">' +
          '<strong>' + Math.round(flow.total_embarked).toLocaleString() + '</strong> enslaved persons (' + pct + '%)<br>' +
          '<strong>' + flow.voyage_count.toLocaleString() + '</strong> voyages' +
          yearStr +
          '</div>';

        // Cross-highlight: dim all timeline bars, brighten matching source
        d3.selectAll(".tl-bar").attr("opacity", 0.2);
        d3.selectAll('.tl-bar[data-source-code="' + flow.source_code + '"][data-dataset="' + dataset + '"]')
          .attr("opacity", 1);
      })
      .on("mousemove", function(event) {
        var rect = document.getElementById("map-container").getBoundingClientRect();
        _tooltipDiv.style.left = (event.clientX - rect.left + 16) + "px";
        _tooltipDiv.style.top = (event.clientY - rect.top - 10) + "px";
      })
      .on("mouseout", function() {
        // Remove halo
        _flowGroup.selectAll(".stem-halo").remove();

        // Restore all stems to normal opacity and zoom-adjusted width
        _flowGroup.selectAll(".stem").attr("opacity", 0.7)
          .each(function() {
            var baseSW = +this.getAttribute("data-base-sw") || 2;
            var k = _svg ? d3.zoomTransform(_svg.node()).k : 1;
            d3.select(this).attr("stroke-width", baseSW / Math.sqrt(k));
          });

        _tooltipDiv.style.opacity = "0";

        // Restore all timeline bars
        d3.selectAll(".tl-bar").attr("opacity", 0.85);
      });
  });

  // Draw trunk
  var trunkWidth = Math.min(strokeScale(totalEmbarked) * 0.8, 20);
  var bias = null;
  if (destQuadrant === "rio-plata" || destQuadrant === "pacific-south") {
    bias = "south-atlantic";
  }
  var trunkPath = (typeof window.generate_arc === 'function')
    ? window.generate_arc(convergePt[0], convergePt[1], destPt[0], destPt[1], bias || "")
    : generateArc(convergePt, destPt, bias);
  if (trunkPath) {
    _flowGroup
      .append("path")
      .attr("class", "flow-path trunk")
      .attr("d", trunkPath)
      .attr("stroke", trunkColor)
      .attr("stroke-width", trunkWidth)
      .attr("data-base-sw", trunkWidth)
      .attr("fill", "none")
      .attr("stroke-linecap", "round")
      .attr("opacity", 0.75);

    drawArrowhead(_flowGroup, trunkPath, trunkWidth, trunkColor);
  }
}

// Render flow arrows — supports dataset field for dual rendering.
function renderFlows(flowsJson, destCode) {
  if (!_flowGroup) return;
  _flowGroup.selectAll("*").remove();

  if (typeof _waypointCache !== 'undefined') {
    for (var k in _waypointCache) delete _waypointCache[k];
  }

  var flows;
  try {
    flows = JSON.parse(flowsJson);
  } catch (e) {
    console.error("Invalid flows JSON", e);
    return;
  }

  if (flows.length === 0) return;

  var destCoords = getCoords(destCode);
  if (!destCoords) return;
  var destPt = _projection([destCoords.lng, destCoords.lat]);
  if (!destPt) return;

  // Split by dataset
  var taFlows = flows.filter(function(f) { return f.dataset === "transatlantic"; });
  var iamFlows = flows.filter(function(f) { return f.dataset === "interamerican"; });

  // If no dataset field, treat all as transatlantic (backward compat)
  if (taFlows.length === 0 && iamFlows.length === 0) {
    taFlows = flows;
  }

  var hasBoth = taFlows.length > 0 && iamFlows.length > 0;

  // Render transatlantic first (behind), then inter-American (in front)
  if (taFlows.length > 0) {
    renderFlowGroup(taFlows, destPt, "transatlantic", hasBoth ? -1 : 0, destCode);
  }
  if (iamFlows.length > 0) {
    renderFlowGroup(iamFlows, destPt, "interamerican", hasBoth ? 1 : 0, destCode);
  }

  // Highlight the destination dot
  var currentTransform = d3.zoomTransform(_svg.node());
  var k = currentTransform.k;
  var sk = Math.sqrt(k);
  _dotsGroup.selectAll(".dest-dot").each(function() {
    var dot = d3.select(this);
    var dotX = +dot.attr("cx");
    var dotY = +dot.attr("cy");
    if (Math.abs(dotX - destPt[0]) < 2 && Math.abs(dotY - destPt[1]) < 2) {
      dot.attr("fill", "#c04030").attr("r", 6 / sk).attr("data-base-r", 6);
    } else {
      var baseR = +this.getAttribute("data-base-r") || 3.5;
      dot.attr("fill", getComputedStyle(document.documentElement).getPropertyValue('--color-accent').trim() || "#5a7a8a").attr("r", baseR / sk);
    }
  });
}

function clearFlows() {
  if (_flowGroup) _flowGroup.selectAll("*").remove();
  if (_dotsGroup) {
    var k = _svg ? d3.zoomTransform(_svg.node()).k : 1;
    var sk = Math.sqrt(k);
    _dotsGroup.selectAll(".dest-dot").each(function() {
      var baseR = +this.getAttribute("data-base-r") || 3.5;
      d3.select(this).attr("fill", getComputedStyle(document.documentElement).getPropertyValue('--color-accent').trim() || "#5a7a8a").attr("r", baseR / sk).attr("data-base-r", 3.5);
    });
  }
  // Remove legend when flows are cleared
  if (_uiGroup) _uiGroup.selectAll(".legend-group").remove();
}
