// Stacked bar chart timeline for voyage volume by origin region over time.
// Renders into #timeline-chart, colors match map stems via getSourceColor().
// Uses adaptive binning (decade/5-year/individual) based on year span.

function renderTimeline(yearlyFlowsJson, globalMinYear, globalMaxYear) {
  clearTimeline();

  var data;
  try {
    data = JSON.parse(yearlyFlowsJson);
  } catch (e) {
    console.error("Invalid timeline JSON", e);
    return;
  }
  if (!data || data.length === 0) return;

  var container = document.getElementById("timeline-chart");
  if (!container) return;

  // Adaptive bin size based on year span
  var span = globalMaxYear - globalMinYear;
  var binSize = span > 100 ? 10 : span > 30 ? 5 : 1;

  // Fixed logical dimensions — CSS scales via viewBox
  var margin = { top: 4, right: 0, bottom: 18, left: 0 };
  var logicalW = 800;
  var logicalH = 60;
  var innerW = logicalW - margin.left - margin.right;
  var innerH = logicalH - margin.top - margin.bottom;

  // Create bin boundaries
  var startBin = Math.floor(globalMinYear / binSize) * binSize;
  var bins = [];
  for (var b = startBin; b <= globalMaxYear; b += binSize) {
    bins.push(b);
  }

  // Collect unique source keys (source_code + dataset)
  var sourceKeys = [];
  var sourceSet = {};
  data.forEach(function(d) {
    var sk = d.source_code + "|" + d.dataset;
    if (!sourceSet[sk]) {
      sourceSet[sk] = true;
      sourceKeys.push({ code: d.source_code, dataset: d.dataset, name: d.source_name });
    }
  });

  // Aggregate data into bins: { binStart -> { sourceKey -> { embarked, source_code, dataset, source_name } } }
  var byBin = {};
  bins.forEach(function(b) { byBin[b] = {}; });

  data.forEach(function(d) {
    var b = Math.floor(d.year / binSize) * binSize;
    if (!byBin[b]) byBin[b] = {};
    var sk = d.source_code + "|" + d.dataset;
    if (!byBin[b][sk]) {
      byBin[b][sk] = {
        embarked: 0,
        source_code: d.source_code,
        dataset: d.dataset,
        source_name: d.source_name
      };
    }
    byBin[b][sk].embarked += d.embarked;
  });

  // Build stacked bar data
  var stackedBars = [];
  var maxBinTotal = 0;

  bins.forEach(function(b) {
    var entries = byBin[b] || {};
    var cumY = 0;
    // Iterate source keys in consistent order
    sourceKeys.forEach(function(sk) {
      var key = sk.code + "|" + sk.dataset;
      var entry = entries[key];
      if (entry && entry.embarked > 0) {
        stackedBars.push({
          bin: b,
          binEnd: b + binSize - 1,
          source_code: entry.source_code,
          dataset: entry.dataset,
          source_name: entry.source_name,
          y0: cumY,
          y1: cumY + entry.embarked,
          embarked: entry.embarked
        });
        cumY += entry.embarked;
      }
    });
    if (cumY > maxBinTotal) maxBinTotal = cumY;
  });

  if (maxBinTotal === 0) return;

  var svg = d3.select("#timeline-chart")
    .append("svg")
    .attr("viewBox", "0 0 " + logicalW + " " + logicalH)
    .attr("preserveAspectRatio", "xMidYMid meet");

  var g = svg.append("g")
    .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

  var x = d3.scaleBand()
    .domain(bins)
    .range([0, innerW])
    .padding(0.08);

  var y = d3.scaleLinear()
    .domain([0, maxBinTotal])
    .range([innerH, 0]);

  // Draw bars
  g.selectAll(".tl-bar")
    .data(stackedBars)
    .enter()
    .append("rect")
    .attr("class", "tl-bar")
    .attr("x", function(d) { return x(d.bin); })
    .attr("y", function(d) { return y(d.y1); })
    .attr("width", x.bandwidth())
    .attr("height", function(d) { return Math.max(0, y(d.y0) - y(d.y1)); })
    .attr("fill", function(d) { return getSourceColor(d.source_code, d.dataset); })
    .attr("data-source-code", function(d) { return d.source_code; })
    .attr("data-dataset", function(d) { return d.dataset; })
    .attr("data-bin-start", function(d) { return d.bin; })
    .attr("data-bin-size", binSize)
    .attr("opacity", 0.85)
    .on("mouseover", function() {
      var code = d3.select(this).attr("data-source-code");
      var ds = d3.select(this).attr("data-dataset");
      // Dim all timeline bars, brighten matching source
      d3.selectAll(".tl-bar").attr("opacity", 0.2);
      d3.selectAll('.tl-bar[data-source-code="' + code + '"][data-dataset="' + ds + '"]')
        .attr("opacity", 1);
      // Highlight matching stem on map with halo effect
      if (_flowGroup) {
        _flowGroup.selectAll(".stem-halo").remove();
        _flowGroup.selectAll(".stem").attr("opacity", 0.15);
        _flowGroup.selectAll('.stem[data-source-code="' + code + '"][data-dataset="' + ds + '"]')
          .attr("opacity", 1)
          .each(function() {
            var baseSW = +this.getAttribute("data-base-sw") || 2;
            var el = d3.select(this);
            // Insert white halo behind the stem
            _flowGroup.insert("path", ".stem")
              .attr("class", "stem-halo")
              .attr("d", el.attr("d"))
              .attr("stroke", "white")
              .attr("stroke-width", baseSW + 7)
              .attr("fill", "none")
              .attr("stroke-linecap", "round")
              .attr("opacity", 0.9);
            el.attr("stroke-width", baseSW + 3);
          });
      }
    })
    .on("mouseout", function() {
      // Restore all timeline bars
      d3.selectAll(".tl-bar").attr("opacity", 0.85);
      // Restore all stems and remove halos
      if (_flowGroup) {
        _flowGroup.selectAll(".stem-halo").remove();
        _flowGroup.selectAll(".stem").attr("opacity", 0.7)
          .each(function() {
            var baseSW = +this.getAttribute("data-base-sw") || 2;
            var k = _svg ? d3.zoomTransform(_svg.node()).k : 1;
            d3.select(this).attr("stroke-width", baseSW / Math.sqrt(k));
          });
      }
    });

  // X axis — show bin labels
  var tickInterval = binSize >= 10 ? 1 : (binSize >= 5 ? 2 : 10);
  var axisTicks = bins.filter(function(b, i) { return i % tickInterval === 0; });

  var xAxis = d3.axisBottom(x)
    .tickValues(axisTicks)
    .tickSize(3)
    .tickFormat(function(d) { return d; });

  var cs = getComputedStyle(document.documentElement);
  g.append("g")
    .attr("class", "tl-axis")
    .attr("transform", "translate(0," + innerH + ")")
    .call(xAxis)
    .selectAll("text")
    .style("font-size", "8px")
    .style("fill", cs.getPropertyValue('--color-text-muted').trim() || "#888");

  g.selectAll(".tl-axis path, .tl-axis line")
    .style("stroke", cs.getPropertyValue('--color-border').trim() || "#ccc");
}

function clearTimeline() {
  var container = document.getElementById("timeline-chart");
  if (container) {
    container.innerHTML = "";
  }
}
