// Render destination dots on the map.
function renderDestinations(destinationsJson, mode) {
  if (!_dotsGroup) return;
  _dotsGroup.selectAll("*").remove();

  var destinations;
  try {
    destinations = JSON.parse(destinationsJson);
  } catch (e) {
    console.error("Invalid destinations JSON", e);
    return;
  }

  var currentK = _svg ? d3.zoomTransform(_svg.node()).k : 1;
  var currentSK = Math.sqrt(currentK);

  destinations.forEach(function(dest) {
    var coords = getCoords(dest.code);
    if (!coords) return;
    if (dest.code >= 60000 && dest.code < 70000) return;

    var pt = _projection([coords.lng, coords.lat]);
    if (!pt || pt[0] == null || pt[1] == null) return;
    var x = pt[0], y = pt[1];

    var isLarge = dest.total_embarked > 50000;
    var r = isLarge ? 5 : 3.5;

    var dot = _dotsGroup
      .append("circle")
      .attr("class", "dest-dot")
      .attr("cx", x)
      .attr("cy", y)
      .attr("r", r / currentSK)
      .attr("data-base-r", r)
      .attr("fill", "#5a7a8a")
      .attr("stroke", "#fff")
      .attr("stroke-width", 1 / currentK)
      .style("cursor", "pointer");

    var labelDx = -8 / currentSK, anchor = "end";
    if (coords.lng > -40) {
      labelDx = 8 / currentSK;
      anchor = "start";
    }

    _dotsGroup
      .append("text")
      .attr("class", "dest-label")
      .attr("x", x + labelDx)
      .attr("y", y + 3 / currentSK)
      .attr("text-anchor", anchor)
      .style("font-size", (9 / currentSK) + "px")
      .text(dest.name);

    dot.on("click", function() {
      var el = document.getElementById("dest-click-receiver");
      if (el) {
        var nativeSetter = Object.getOwnPropertyDescriptor(
          window.HTMLInputElement.prototype, "value"
        ).set;
        nativeSetter.call(el, String(dest.code));
        el.dispatchEvent(new Event("input", { bubbles: true }));
      }
    });

    // Tooltip with dataset breakdown if in combined mode
    dot
      .on("mouseover", function(event) {
        _tooltipDiv.style.opacity = "1";
        var statsHtml = '<strong>' + Math.round(dest.total_embarked).toLocaleString() + '</strong> enslaved persons<br>' +
          '<strong>' + dest.voyage_count.toLocaleString() + '</strong> voyages';
        if (dest.ta_embarked > 0 && dest.iam_embarked > 0) {
          statsHtml += '<br><span style="color:#8a6828">TASTD: ' + Math.round(dest.ta_embarked).toLocaleString() + '</span>';
          statsHtml += ' · <span style="color:#2a6a7a">IAm: ' + Math.round(dest.iam_embarked).toLocaleString() + '</span>';
        }
        _tooltipDiv.innerHTML =
          '<div class="tt-route">' + dest.name + '</div>' +
          '<div class="tt-stat">' + statsHtml + '</div>';
      })
      .on("mousemove", function(event) {
        var rect = document.getElementById("map-container").getBoundingClientRect();
        _tooltipDiv.style.left = (event.clientX - rect.left + 16) + "px";
        _tooltipDiv.style.top = (event.clientY - rect.top - 10) + "px";
      })
      .on("mouseout", function() {
        _tooltipDiv.style.opacity = "0";
      });
  });
}
