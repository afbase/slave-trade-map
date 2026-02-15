// Transatlantic & Inter-American Slave Trade — D3.js Flow Map
// Module state and map initialization

var WORLD_URL = "https://cdn.jsdelivr.net/npm/world-atlas@2/countries-110m.json";

var _svg, _projection, _path, _flowGroup, _dotsGroup, _tooltipDiv, _width, _height;
var _masterGroup, _uiGroup, _zoom;
window.mapReady = false;

function initMap() {
  _width = 1200;
  _height = 620;

  var container = document.getElementById("map-container");
  if (!container) {
    console.error("map-container not found");
    return;
  }

  // Clear any previous
  container.querySelectorAll("svg").forEach(function(s) { s.remove(); });
  var existingTooltip = container.querySelector(".tooltip");
  if (existingTooltip) existingTooltip.remove();
  var existingBtn = container.querySelector(".map-reset-btn");
  if (existingBtn) existingBtn.remove();

  // Tooltip
  _tooltipDiv = document.createElement("div");
  _tooltipDiv.className = "tooltip";
  container.appendChild(_tooltipDiv);

  _svg = d3
    .select("#map-container")
    .append("svg")
    .attr("viewBox", "0 0 " + _width + " " + _height)
    .attr("preserveAspectRatio", "xMidYMid meet");

  // Background
  _svg
    .append("rect")
    .attr("class", "map-bg")
    .attr("width", _width)
    .attr("height", _height)
    .attr("fill", getComputedStyle(document.documentElement).getPropertyValue('--color-bg').trim() || "#f5f3ef");

  // Master group that zoom will transform
  _masterGroup = _svg.append("g").attr("class", "map-content");

  // UI group for fixed elements (time label, legend) — not affected by zoom
  _uiGroup = _svg.append("g").attr("class", "ui-group");

  // Projection centered on Atlantic
  _projection = d3
    .geoNaturalEarth1()
    .center([-15, 5])
    .scale(210)
    .translate([_width / 2, _height / 2 + 10]);

  _path = d3.geoPath().projection(_projection);

  // Graticule (inside masterGroup)
  var graticule = d3.geoGraticule().step([15, 15]);
  _masterGroup
    .append("path")
    .datum(graticule())
    .attr("class", "graticule")
    .attr("d", _path);

  // Groups for layering (inside masterGroup)
  var landGroup = _masterGroup.append("g").attr("class", "land-group");
  _flowGroup = _masterGroup.append("g").attr("class", "flows");
  _dotsGroup = _masterGroup.append("g").attr("class", "dots");

  // Arrow marker
  var defs = _svg.append("defs");
  defs
    .append("marker")
    .attr("id", "arrowhead")
    .attr("viewBox", "0 0 10 10")
    .attr("refX", 8)
    .attr("refY", 5)
    .attr("markerWidth", 4)
    .attr("markerHeight", 4)
    .attr("orient", "auto")
    .append("path")
    .attr("d", "M 0 2 L 8 5 L 0 8 Z")
    .attr("fill", "#8a6828");

  // Load world map
  d3.json(WORLD_URL)
    .then(function(world) {
      var countries = topojson.feature(world, world.objects.countries);

      landGroup
        .selectAll("path")
        .data(countries.features)
        .enter()
        .append("path")
        .attr("class", "land")
        .attr("d", _path);

      // Ocean label
      _masterGroup
        .append("text")
        .attr("class", "ocean-label")
        .attr("x", _width * 0.35)
        .attr("y", _height * 0.58)
        .attr("text-anchor", "middle")
        .text("ATLANTIC OCEAN");

      // Continent labels
      var labelData = [
        { text: "NORTH AMERICA", coords: [-95, 48] },
        { text: "SOUTH AMERICA", coords: [-55, -30] },
        { text: "AFRICA", coords: [20, 18] },
        { text: "EUROPE", coords: [15, 52] },
      ];
      labelData.forEach(function(l) {
        var pt = _projection(l.coords);
        _masterGroup
          .append("text")
          .attr("class", "continent-label")
          .attr("x", pt[0])
          .attr("y", pt[1])
          .attr("text-anchor", "middle")
          .text(l.text);
      });

      window.mapReady = true;
    })
    .catch(function(err) {
      console.warn("Could not load world topology:", err);
      window.mapReady = true;
    });

  // Zoom behavior
  _zoom = d3.zoom()
    .scaleExtent([1, 12])
    .on("zoom", function(event) {
      _masterGroup.attr("transform", event.transform);
      var k = event.transform.k;
      var sk = Math.sqrt(k);
      _masterGroup.selectAll(".dest-label")
        .style("font-size", (9 / sk) + "px");
      _masterGroup.selectAll(".ocean-label")
        .style("font-size", (18 / sk) + "px");
      _masterGroup.selectAll(".continent-label")
        .style("font-size", (12 / sk) + "px")
        .style("letter-spacing", (4 / sk) + "px");
      _masterGroup.selectAll(".dest-dot")
        .each(function() {
          var baseR = +this.getAttribute("data-base-r") || 3.5;
          d3.select(this).attr("r", baseR / sk).attr("stroke-width", 1 / k);
        });
      _masterGroup.selectAll(".flow-path")
        .each(function() {
          var baseSW = +this.getAttribute("data-base-sw");
          if (baseSW) {
            d3.select(this).attr("stroke-width", baseSW / sk);
          }
        });
      _masterGroup.selectAll(".flow-arrowhead")
        .each(function() {
          var el = d3.select(this);
          var tx = +el.attr("data-tip-x");
          var ty = +el.attr("data-tip-y");
          var s = 1 / sk;
          el.attr("transform",
            "translate(" + tx + "," + ty + ") scale(" + s + ") translate(" + (-tx) + "," + (-ty) + ")");
        });
      _masterGroup.selectAll(".land")
        .attr("stroke-width", 0.4 / k);
      _masterGroup.selectAll(".graticule")
        .attr("stroke-width", 0.3 / k);
    });

  _svg.call(_zoom);

  // Reset zoom button
  d3.select("#map-container").append("button")
    .attr("class", "map-reset-btn")
    .text("Reset Zoom")
    .on("click", function() { window.resetZoom(); });
}

function resetZoom() {
  if (_svg && _zoom) {
    _svg.transition().duration(500).call(_zoom.transform, d3.zoomIdentity);
  }
}
