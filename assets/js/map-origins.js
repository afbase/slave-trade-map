// African embarkation places on the map.
//
// The codebook's place codes are a hierarchy: 6xxxx = Africa, digits 2-3 the
// region (60100 Senegambia ... 60900 Other Africa), last two digits the place,
// with xx99 meaning "port unspecified". Markers follow that hierarchy:
//   zoomed out  -> one bubble per region (its total includes xx99)
//   zoomed in   -> individual ports; ports that still overlap are merged
//                  with their largest neighbour in the same region
//   zoom >= 5   -> every port on its own
// xx99 is drawn as a hollow ring just offshore of its region, never as a port.

var ORIGIN_PORT_ZOOM = 2;      // below this, "auto" shows region bubbles
var ORIGIN_UNMERGE_ZOOM = 5;   // at or above this, ports are never merged
var ORIGIN_MERGE_PX = 9;       // on-screen distance below which ports merge

var _originsGroup = null;
var _originData = [];
var _originOpts = { show: true, level: "auto", unspecified: true };
var _originExpanded = {};      // region code -> true once its bubble was opened
var _selectedOrigin = null;

function setOriginData(json) {
  try {
    _originData = typeof json === "string" ? JSON.parse(json) : (json || []);
  } catch (e) {
    console.error("Invalid origins JSON", e);
    _originData = [];
  }
  renderOrigins();
}

function setOriginOptions(json) {
  try {
    var o = typeof json === "string" ? JSON.parse(json) : json;
    for (var key in o) _originOpts[key] = o[key];
  } catch (e) {
    console.error("Invalid origin options", e);
  }
  renderOrigins();
}

function setSelectedOrigin(code) {
  if (_selectedOrigin === code) return;
  _selectedOrigin = code;
  // Redraw: the selected port is always shown on its own, never merged.
  renderOrigins();
}

function _originRegion(code) {
  return Math.floor(code / 100) * 100;
}

function _hasOwnCoords(code) {
  var c = window.__coords || regionCoords;
  return !!c[code];
}

// Offshore point for a region's "port unspecified" ring.
function _unspecifiedAnchor(region) {
  var anchor = getCoords(region);
  if (!anchor) return null;
  if (region === 60800) return { lat: anchor.lat, lng: anchor.lng + 4 };
  var gate = typeof africanGate === "function" ? africanGate(region, anchor) : null;
  return gate || anchor;
}

function _sendOriginClick(code) {
  var el = document.getElementById("origin-click-receiver");
  if (!el) return;
  var nativeSetter = Object.getOwnPropertyDescriptor(
    window.HTMLInputElement.prototype, "value").set;
  nativeSetter.call(el, String(code));
  el.dispatchEvent(new Event("input", { bubbles: true }));
}

function _originTooltip(sel, htmlFn) {
  sel
    .on("mouseover", function() {
      _tooltipDiv.style.opacity = "1";
      _tooltipDiv.innerHTML = htmlFn();
    })
    .on("mousemove", function(event) {
      var rect = document.getElementById("map-container").getBoundingClientRect();
      _tooltipDiv.style.left = (event.clientX - rect.left + 16) + "px";
      _tooltipDiv.style.top = (event.clientY - rect.top - 10) + "px";
    })
    .on("mouseout", function() {
      _tooltipDiv.style.opacity = "0";
    });
}

function _fmt(n) {
  return Math.round(n).toLocaleString();
}

function _statsHtml(embarked, voyages) {
  return '<strong>' + _fmt(embarked) + '</strong> enslaved persons embarked<br>' +
    '<strong>' + voyages.toLocaleString() + '</strong> voyages';
}

// Zoom the map so a point sits in the centre at scale k.
function _zoomToPoint(pt, k) {
  if (!_svg || !_zoom) return;
  var t = d3.zoomIdentity.translate(_width / 2, _height / 2).scale(k).translate(-pt[0], -pt[1]);
  _svg.transition().duration(500).call(_zoom.transform, t);
}

// Bring an African place into view: open its region and zoom to it.
function focusOrigin(code) {
  var c = getCoords(code);
  if (!c) return;
  _originExpanded[_originRegion(code)] = true;
  var isUnspecified = code % 100 === 99;
  var target = isUnspecified ? (_unspecifiedAnchor(_originRegion(code)) || c) : c;
  var pt = _projection([target.lng, target.lat]);
  if (!pt) return;
  var k = Math.max(d3.zoomTransform(_svg.node()).k, ORIGIN_UNMERGE_ZOOM);
  _zoomToPoint(pt, isUnspecified ? Math.min(k, 3) : k);
}

function renderOrigins() {
  if (!_originsGroup) return;
  _originsGroup.selectAll("*").remove();
  // A marker under the pointer is about to be replaced; its tooltip would stick.
  if (_tooltipDiv) _tooltipDiv.style.opacity = "0";
  if (_uiGroup) _uiGroup.selectAll(".origin-legend").remove();
  if (!_originOpts.show || !_originData.length) {
    _refreshLabels();
    return;
  }

  var k = _svg ? d3.zoomTransform(_svg.node()).k : 1;
  var sk = Math.sqrt(k);
  var level = _originOpts.level === "auto"
    ? (k < ORIGIN_PORT_ZOOM ? "regions" : "ports") : _originOpts.level;
  var cs = getComputedStyle(document.documentElement);
  var isDark = document.body.classList.contains("dark");
  var strokeColor = cs.getPropertyValue("--color-dot-stroke").trim() || (isDark ? "#3a4258" : "#fff");

  // Group places by region.
  var regions = {};
  var grandTotal = 0, unspecifiedTotal = 0, maxPlace = 0;
  _originData.forEach(function(o) {
    if (o.code < 60000 || o.code >= 70000) return;
    var r = _originRegion(o.code);
    var g = regions[r] || (regions[r] = { code: r, total: 0, voyages: 0, unspecified: 0, places: [] });
    g.total += o.total_embarked;
    g.voyages += o.voyage_count;
    g.places.push(o);
    grandTotal += o.total_embarked;
    if (o.is_unspecified) {
      g.unspecified += o.total_embarked;
      unspecifiedTotal += o.total_embarked;
    }
    if (o.total_embarked > maxPlace) maxPlace = o.total_embarked;
  });
  var regionList = Object.keys(regions).map(function(r) { return regions[r]; });
  var maxRegion = d3.max(regionList, function(g) { return g.total; }) || 1;
  var bubbleR = d3.scaleSqrt().domain([0, maxRegion]).range([3.5, 11]);
  var placeR = d3.scaleSqrt().domain([0, maxPlace || 1]).range([2.2, 11]);

  var labels = [];
  function addLabel(pt, r, text, importance, code) {
    labels.push({ x: pt[0] + (r + 3) / sk, y: pt[1] + 3 / k, text: text,
                  importance: importance, code: code });
  }

  regionList.forEach(function(g) {
    var color = getSourceColor(g.code, "transatlantic");
    var regionCoordsEntry = getCoords(g.code);
    var regionName = regionCoordsEntry ? regionCoordsEntry.name : String(g.code);

    // ---- Region bubble ----
    if (level === "regions" && !_originExpanded[g.code]) {
      if (!regionCoordsEntry) return;
      var bpt = _projection([regionCoordsEntry.lng, regionCoordsEntry.lat]);
      if (!bpt) return;
      var br = bubbleR(g.total);
      var named = g.places.filter(function(p) { return !p.is_unspecified; }).length;
      var bubble = _originsGroup.append("circle")
        .attr("class", "origin-bubble")
        .attr("cx", bpt[0]).attr("cy", bpt[1])
        .attr("r", br / sk).attr("data-base-r", br)
        .attr("data-code", g.code)
        .attr("fill", color).attr("fill-opacity", 0.55)
        .attr("stroke", color).attr("stroke-width", 1.2 / k)
        .style("cursor", "pointer")
        .on("click", function(event) {
          event.stopPropagation();
          _tooltipDiv.style.opacity = "0";
          _originExpanded[g.code] = true;
          renderOrigins();
          // Zoom in far enough that the region's ports separate.
          if (k < 4) _zoomToPoint(bpt, 4);
        });
      _originTooltip(bubble, function() {
        var unspecPct = g.total > 0 ? Math.round(g.unspecified / g.total * 100) : 0;
        return '<div class="tt-route">' + regionName + '</div><div class="tt-stat">' +
          _statsHtml(g.total, g.voyages) + '<br>' + named + ' named places' +
          (unspecPct > 0 ? ' · ' + unspecPct + '% port unspecified' : '') +
          '<br><em>Click to show its ports</em></div>';
      });
      addLabel(bpt, br, regionName, g.total, g.code);
      return;
    }

    // ---- Individual places ----
    var located = [], unlocated = [], unspecified = [];
    g.places.forEach(function(p) {
      if (p.is_unspecified) unspecified.push(p);
      else if (_hasOwnCoords(p.code)) located.push(p);
      else unlocated.push(p);
    });

    // Merge ports that would overlap on screen into their largest neighbour.
    located.sort(function(a, b) { return b.total_embarked - a.total_embarked; });
    var mergeDist = k >= ORIGIN_UNMERGE_ZOOM ? 0 : ORIGIN_MERGE_PX / k;
    var clusters = [];
    located.forEach(function(p) {
      var c = getCoords(p.code);
      var pt = _projection([c.lng, c.lat]);
      if (!pt) return;
      // Always keep the selected port on its own so it can be seen.
      if (mergeDist > 0 && p.code !== _selectedOrigin) {
        for (var i = 0; i < clusters.length; i++) {
          var cl = clusters[i];
          var dx = cl.pt[0] - pt[0], dy = cl.pt[1] - pt[1];
          if (Math.sqrt(dx * dx + dy * dy) < mergeDist) {
            cl.members.push(p);
            cl.total += p.total_embarked;
            cl.voyages += p.voyage_count;
            return;
          }
        }
      }
      clusters.push({ lead: p, pt: pt, members: [p], total: p.total_embarked,
                      voyages: p.voyage_count });
    });

    clusters.forEach(function(cl) {
      var r = placeR(cl.total);
      var merged = cl.members.length > 1;
      var dot = _originsGroup.append("circle")
        .attr("class", "origin-dot" + (cl.lead.code === _selectedOrigin ? " selected" : ""))
        .attr("cx", cl.pt[0]).attr("cy", cl.pt[1])
        .attr("r", r / sk).attr("data-base-r", r)
        .attr("data-code", cl.lead.code)
        .attr("fill", color)
        .attr("stroke", strokeColor).attr("stroke-width", 0.8 / k)
        .style("cursor", "pointer")
        .on("click", function(event) {
          event.stopPropagation();
          _tooltipDiv.style.opacity = "0";
          if (merged) {
            _zoomToPoint(cl.pt, Math.min(Math.max(k * 2.5, ORIGIN_UNMERGE_ZOOM), 20));
          } else {
            _sendOriginClick(cl.lead.code);
          }
        });
      _originTooltip(dot, function() {
        if (!merged) {
          return '<div class="tt-route">' + cl.lead.name + '</div><div class="tt-stat">' +
            _statsHtml(cl.total, cl.voyages) +
            '<br><em>Click to see where its captives were taken</em></div>';
        }
        var list = cl.members.slice(0, 8).map(function(m) {
          return m.name + ' (' + _fmt(m.total_embarked) + ')';
        }).join('<br>');
        if (cl.members.length > 8) list += '<br>… and ' + (cl.members.length - 8) + ' more';
        return '<div class="tt-route">' + cl.members.length + ' ports near ' + cl.lead.name +
          '</div><div class="tt-stat">' + _statsHtml(cl.total, cl.voyages) + '<br>' + list +
          '<br><em>Click to zoom in</em></div>';
      });
      addLabel(cl.pt, r, cl.lead.name + (merged ? " +" + (cl.members.length - 1) : ""),
               cl.total, cl.lead.code);
    });

    // Named places with no coordinates: one marker at the region anchor.
    if (unlocated.length && regionCoordsEntry) {
      var upt = _projection([regionCoordsEntry.lng, regionCoordsEntry.lat]);
      var uTotal = d3.sum(unlocated, function(p) { return p.total_embarked; });
      var uVoy = d3.sum(unlocated, function(p) { return p.voyage_count; });
      if (upt) {
        var size = 5;
        var sq = _originsGroup.append("rect")
          .attr("class", "origin-unlocated")
          .attr("x", upt[0] - size / 2 / sk).attr("y", upt[1] - size / 2 / sk)
          .attr("width", size / sk).attr("height", size / sk)
          .attr("data-cx", upt[0]).attr("data-cy", upt[1]).attr("data-base-size", size)
          .attr("fill", color).attr("stroke", strokeColor).attr("stroke-width", 0.8 / k);
        _originTooltip(sq, function() {
          unlocated.sort(function(a, b) { return b.total_embarked - a.total_embarked; });
          var list = unlocated.slice(0, 10).map(function(m) {
            return m.name + ' (' + _fmt(m.total_embarked) + ')';
          }).join('<br>');
          if (unlocated.length > 10) list += '<br>… and ' + (unlocated.length - 10) + ' more';
          return '<div class="tt-route">' + regionName + ': ' + unlocated.length +
            ' places not yet located on the map</div><div class="tt-stat">' +
            _statsHtml(uTotal, uVoy) + '<br>' + list + '</div>';
        });
      }
    }

    // "Port unspecified": hollow ring offshore.
    if (_originOpts.unspecified) {
      unspecified.forEach(function(p) {
        var a = _unspecifiedAnchor(g.code);
        if (!a) return;
        var rpt = _projection([a.lng, a.lat]);
        if (!rpt) return;
        var rr = placeR(p.total_embarked);
        var ring = _originsGroup.append("circle")
          .attr("class", "origin-ring" + (p.code === _selectedOrigin ? " selected" : ""))
          .attr("cx", rpt[0]).attr("cy", rpt[1])
          .attr("r", rr / sk).attr("data-base-r", rr)
          .attr("data-code", p.code)
          .attr("fill", color).attr("fill-opacity", 0.08)
          .attr("stroke", color).attr("stroke-width", 1.4 / k)
          .attr("stroke-dasharray", (3 / k) + " " + (2 / k))
          .style("cursor", "pointer")
          .on("click", function(event) {
            event.stopPropagation();
            _tooltipDiv.style.opacity = "0";
            _sendOriginClick(p.code);
          });
        _originTooltip(ring, function() {
          var share = g.total > 0 ? Math.round(p.total_embarked / g.total * 100) : 0;
          return '<div class="tt-route">' + p.name + '</div><div class="tt-stat">' +
            _statsHtml(p.total_embarked, p.voyage_count) + '<br>' + share + '% of ' +
            regionName + '. The source names the region but not the port.</div>';
        });
      });
    }
  });

  // Labels share the destination labels' class so one collision pass covers both.
  labels.forEach(function(l) {
    _originsGroup.append("text")
      .attr("class", "dest-label origin-label")
      .attr("x", l.x).attr("y", l.y)
      .attr("text-anchor", "start")
      .attr("data-importance", l.importance)
      .attr("data-code", l.code)
      // 9px at full view, easing to ~11px on screen when zoomed in.
      .style("font-size", Math.min(9 / sk, 11 / k) + "px")
      .text(l.text);
  });

  _renderOriginLegend(grandTotal, unspecifiedTotal, level);
  _refreshLabels();
}

function _refreshLabels() {
  if (typeof recomputeLabelVisibility === "function" && _svg) {
    recomputeLabelVisibility(d3.zoomTransform(_svg.node()).k);
  }
}

function _renderOriginLegend(grandTotal, unspecifiedTotal, level) {
  if (!_uiGroup) return;
  var cs = getComputedStyle(document.documentElement);
  var muted = cs.getPropertyValue("--color-text-muted").trim() || "#666";
  var light = cs.getPropertyValue("--color-text-light").trim() || "#555";
  var swatch = getSourceColor(60500, "transatlantic");
  var g = _uiGroup.append("g").attr("class", "origin-legend")
    .attr("transform", "translate(12, 16)");
  var bg = cs.getPropertyValue("--color-bg").trim() || "#f5f3ef";
  g.append("rect").attr("x", -6).attr("y", -11).attr("width", 232)
    .attr("height", unspecifiedTotal > 0 ? 50 : 34).attr("rx", 3)
    .attr("fill", bg).attr("fill-opacity", 0.85);
  g.append("text").attr("class", "legend-title").attr("x", 0).attr("y", 0)
    .attr("font-size", "9px").attr("font-weight", "600").attr("letter-spacing", "0.5px")
    .attr("fill", muted).text("AFRICAN EMBARKATION");
  g.append("circle").attr("cx", 5).attr("cy", 13).attr("r", 4.5)
    .attr("fill", swatch).attr("fill-opacity", level === "regions" ? 0.55 : 1);
  g.append("text").attr("x", 14).attr("y", 16).attr("font-size", "9px").attr("fill", light)
    .text(level === "regions"
      ? "Region (zoom in or click for ports)"
      : "Port, sized by captives embarked");
  if (unspecifiedTotal > 0 && grandTotal > 0) {
    var pct = Math.round(unspecifiedTotal / grandTotal * 100);
    g.append("circle").attr("cx", 5).attr("cy", 29).attr("r", 4.5)
      .attr("fill", "none").attr("stroke", swatch).attr("stroke-width", 1.2)
      .attr("stroke-dasharray", "2.5 1.5");
    g.append("text").attr("x", 14).attr("y", 32).attr("font-size", "9px").attr("fill", light)
      .text("Port unspecified: " + pct + "% of captives" +
        (level === "regions" ? " (counted in regions)" : ""));
  }
}
