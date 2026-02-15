// Render a single flow group (stems + trunk) for one dataset
function renderFlowGroup(flows, destPt, dataset, zOffset) {
  if (flows.length === 0) return;

  var totalEmbarked = d3.sum(flows, function(d) { return d.total_embarked; });
  var maxEmbarked = d3.max(flows, function(d) { return d.total_embarked; });
  var strokeScale = d3.scaleSqrt().domain([0, maxEmbarked]).range([1.5, 14]);

  // Compute convergence point
  var cx = 0, cy = 0, totalWeight = 0;
  flows.forEach(function(f) {
    var sc = getCoords(f.source_code);
    if (!sc) return;
    var pt = _projection([sc.lng, sc.lat]);
    if (!pt) return;
    cx += pt[0] * f.total_embarked;
    cy += pt[1] * f.total_embarked;
    totalWeight += f.total_embarked;
  });
  if (totalWeight === 0) return;
  cx /= totalWeight;
  cy /= totalWeight;

  // Shift convergence toward destination
  var convergePt = [(cx + destPt[0]) / 2, (cy + destPt[1]) / 2];

  // For combined mode, offset convergence slightly to separate trunk groups
  if (zOffset !== 0) {
    convergePt[1] += zOffset * 15;
  }

  // Sort largest first
  flows.sort(function(a, b) { return b.total_embarked - a.total_embarked; });

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

    var stemPath = generateArc(srcPt, convergePt);
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
  var trunkPath = generateArc(convergePt, destPt);
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
    renderFlowGroup(taFlows, destPt, "transatlantic", hasBoth ? -1 : 0);
  }
  if (iamFlows.length > 0) {
    renderFlowGroup(iamFlows, destPt, "interamerican", hasBoth ? 1 : 0);
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
