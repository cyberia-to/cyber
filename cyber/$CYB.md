---
tags: cyber, cybernomics
alias: $CYB, CYB, cyber energy
crystal-type: entity
crystal-domain: economics
icon: "⚡"
---
the root [[token]] of [[cyber]] — the energy of [[focus]]. stake, fees, and rewards for teaching the [[cybergraph]]. emission follows proven Δφ*; supply is a law of the field. full model in the [[whitepaper]] and below

## focus is the value

cyber organizes one quantity: [[soft3/tru/specs/focus|focus]] (φ*), the collective attention distribution — the fixed point the [[tri-kernel]] drives the graph toward. A [[cyberlink]] that earns focus is knowledge the network found worth attending to. Focus is the scarce thing, the measured thing, the thing every other mechanism serves.

## $CYB is the energy of focus

Moving focus costs work; creating focus is work done. $CYB is that work made fungible — the energy a [[cybics/crystal/neuron|neuron]] spends to write a [[cyberlink]], compute, and reach [[cybics/crystal/consensus|consensus]], and the energy it earns for raising the graph's focus. Δφ* is the gradient of the system's free energy, so $CYB is that free energy in transferable form.

## supply is a law of the field

Value and computation share one arithmetic: balances are elements of the [[nebu|Goldilocks field]], the field [[soft3/nox|nox]] computes in. So total supply is the field's own order:

p = 2⁶⁴ − 2³² + 1 = 18,446,744,069,414,584,321

The cap is how many elements the field has — arithmetic, not a governance number. (On the [[bootloader/bostrom|bostrom]] [[bootloader]] today this energy circulates as [[$C]].)

## genesis

At the first block, [[$C]] holders hold 187,416,084,623,451,570 $CYB, ≈ 1% of supply: their snapshot of 281,405,532,467,645, lifted 666×.

## emission answers to time alone

Supply at age t is M(t): a function of the clock and nothing else — identical on every node, known in full from genesis. With time as the only input, the schedule is a fixed commitment, predictable in advance and immune to forgery.

Focus enters on the other side. The clock sets how much $CYB exists; focus sets who earns it (see allocation). Supply is a law of time, reward a law of φ* — kept apart.

## emission follows the network's own law

cyber is scale-free: degrees follow a power law, focus follows Zipf. The token is issued by the same law its graph obeys — a power law:

M(t) = p · (1 − (1 + t/τ)^(−k)),    τ = 0.33 year,   k = 0.5

(t in years). A power law is also the one schedule that holds a hot head and a heavy tail at once — an exponential halving shares a single rate between the two and cannot. From one formula, two phases:

- a bootstrap head — about half the supply in the first year (~11% in the first month), spread across the year so price discovers and the first miners (days, weeks, months) are paid, with no single-day flood. The initial rate is finite (k/τ ≈ 152%/yr), not a spike.
- a heavy tail — polynomial, never exponential: still issuing past a century (~4% of supply unissued at 200 years), always under the cap.

### emission (log years, 300y)

Cumulative supply as % of field cap (left axis). Yearly inflation = new supply ÷ circulating at year start (right axis; year 1 is the genesis → half-cap jump, shown in the readout). Time axis is logarithmic so the bootstrap head stays readable. Hover any year.

<div id="cyb-emi"></div>

<style>
#cyb-emi{--bg:#000;--s1:#0a0a0a;--s2:#111;--ln:#222;--tx:#f0f0f0;--mut:#8b948c;--neon:#22c55e;--cyan:#06b6d4;background:transparent;color:var(--tx);font-family:var(--font-body,'Play',system-ui,sans-serif);border:none;box-shadow:none;box-sizing:border-box;width:100%;max-width:100%;margin:20px 0 28px;padding:0}
#cyb-emi .panel{background:var(--s1);border:1px solid var(--ln);border-radius:10px;padding:14px 14px 10px}
#cyb-emi .head{display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:10px 16px;margin:0 0 12px}
#cyb-emi .title{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;color:var(--neon);letter-spacing:2px;text-transform:uppercase;text-shadow:0 0 10px rgba(34,197,94,.45)}
#cyb-emi .legend{display:flex;flex-wrap:wrap;gap:12px 18px;font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;color:var(--mut)}
#cyb-emi .legend i{display:inline-block;width:18px;height:0;border-top:2.4px solid;margin-right:6px;vertical-align:middle;border-radius:1px}
#cyb-emi .legend .s{border-color:var(--neon)}
#cyb-emi .legend .i{border-color:var(--cyan)}
#cyb-emi .stats{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:8px;margin:0 0 12px}
#cyb-emi .stat{background:var(--s2);border:1px solid var(--ln);border-radius:8px;padding:8px 10px;min-width:0}
#cyb-emi .stat .l{font-size:10px;color:var(--mut);letter-spacing:.4px;margin-bottom:3px}
#cyb-emi .stat .v{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:14px;font-weight:600;word-break:break-word}
#cyb-emi .stat .v.s{color:var(--neon);text-shadow:0 0 12px rgba(34,197,94,.25)}
#cyb-emi .stat .v.i{color:var(--cyan);text-shadow:0 0 12px rgba(6,182,212,.25)}
#cyb-emi .chart-wrap{position:relative;width:100%;min-height:300px}
#cyb-emi .chart-wrap svg{width:100%;height:auto;display:block}
#cyb-emi svg text{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:10px;fill:var(--mut)}
#cyb-emi .tip{position:absolute;pointer-events:none;z-index:5;background:#111;border:1px solid #333;color:#f0f0f0;font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;padding:7px 10px;border-radius:6px;box-shadow:0 0 20px rgba(34,197,94,.15);white-space:nowrap;display:none;line-height:1.45}
#cyb-emi .note{font-size:11px;color:var(--mut);margin:10px 0 0;line-height:1.5}
#cyb-emi .pt{cursor:crosshair}
@media(max-width:640px){
  #cyb-emi .stats{grid-template-columns:repeat(2,minmax(0,1fr))}
  #cyb-emi .chart-wrap{min-height:260px}
}
</style>

<script>
(function(){
  var root = document.getElementById("cyb-emi");
  if (!root) return;

  var TAU = 0.33;
  var K = 0.5;
  var GENESIS = 0.01;
  var YEARS = 300;

  function supplyFrac(t) {
    if (t <= 0) return GENESIS;
    var m = 1 - Math.pow(1 + t / TAU, -K);
    return Math.max(GENESIS, m);
  }

  function inflationAt(y) {
    var prev = supplyFrac(y - 1);
    var cur = supplyFrac(y);
    if (prev <= 0) return 0;
    return (cur - prev) / prev;
  }

  var series = [];
  for (var y = 0; y <= YEARS; y++) {
    series.push({ y: y, supply: supplyFrac(y), infl: y === 0 ? null : inflationAt(y) });
  }

  function pct(x, d) {
    if (x == null || !isFinite(x)) return "\u2014";
    var p = x * 100;
    if (p >= 10) return p.toFixed(d == null ? 1 : d) + "%";
    if (p >= 1) return p.toFixed(d == null ? 1 : d) + "%";
    return p.toFixed(d == null ? 2 : d) + "%";
  }

  function card(label, value, cls) {
    return '<div class="stat"><div class="l">' + label + '</div><div class="v ' + (cls || "") + '">' + value + "</div></div>";
  }

  function buildChart() {
    var W = 960, H = 340;
    var left = 52, right = 52, top = 14, bottom = 32;
    var plotW = W - left - right, plotH = H - top - bottom;

    var inflMax = 0;
    for (var i = 2; i < series.length; i++) {
      if (series[i].infl > inflMax) inflMax = series[i].infl;
    }
    inflMax = Math.max(inflMax * 1.08, 0.05);

    // log time: log1p(y) stretches the bootstrap head, keeps a 300y tail
    var logMax = Math.log1p(YEARS);
    function x(y) { return left + plotW * (Math.log1p(y) / logMax); }
    function yS(s) { return top + plotH * (1 - s); }
    function yI(inf) { return top + plotH * (1 - Math.min(inf, inflMax) / inflMax); }

    var grid = "";
    for (var g = 0; g <= 4; g++) {
      var sv = g / 4;
      var yy = yS(sv);
      grid += '<line x1="' + left + '" y1="' + yy + '" x2="' + (W - right) + '" y2="' + yy + '" stroke="#222" stroke-width="0.5"></line>';
      grid += '<text x="' + (left - 6) + '" y="' + (yy + 3) + '" text-anchor="end">' + Math.round(sv * 100) + "%</text>";
      var iv = inflMax * (g / 4);
      grid += '<text x="' + (W - right + 6) + '" y="' + (yy + 3) + '" text-anchor="start">' + (iv * 100).toFixed(iv >= 0.1 ? 0 : 1) + "%</text>";
    }
    var yTicks = [0, 1, 2, 5, 10, 20, 50, 100, 200, 300];
    for (var t = 0; t < yTicks.length; t++) {
      var yr = yTicks[t];
      var xx = x(yr);
      grid += '<line x1="' + xx + '" y1="' + top + '" x2="' + xx + '" y2="' + (top + plotH) + '" stroke="#1a1a1a" stroke-width="0.5"></line>';
      grid += '<text x="' + xx + '" y="' + (H - 10) + '" text-anchor="middle">' + yr + "y</text>";
    }

    var ptsS = series.map(function (p) {
      return x(p.y).toFixed(1) + "," + yS(p.supply).toFixed(1);
    }).join(" ");

    var ptsI = [];
    for (var j = 2; j < series.length; j++) {
      ptsI.push(x(series[j].y).toFixed(1) + "," + yI(series[j].infl).toFixed(1));
    }

    // Full-height hit strips per year so hover works for both lines (log spacing).
    var hits = "";
    for (var k = 0; k < series.length; k++) {
      var p = series[k];
      var cx = x(p.y);
      var prevX = k === 0 ? left : (x(series[k - 1].y) + cx) / 2;
      var nextX = k === series.length - 1 ? left + plotW : (cx + x(series[k + 1].y)) / 2;
      var hx = prevX;
      var hw = Math.max(nextX - prevX, 2);
      hits +=
        '<rect class="pt" data-y="' + p.y + '" x="' + hx.toFixed(1) + '" y="' + top +
        '" width="' + hw.toFixed(1) + '" height="' + plotH +
        '" fill="transparent"></rect>';
    }

    // Markers on both series (drawn under hits, above polylines via order: lines then marks then hits)
    var marks = "";
    for (var m = 0; m < series.length; m++) {
      var q = series[m];
      var mx = x(q.y).toFixed(1);
      marks +=
        '<circle class="mk-s" data-y="' + q.y + '" cx="' + mx + '" cy="' + yS(q.supply).toFixed(1) +
        '" r="3.5" fill="#0a0a0a" stroke="#22c55e" stroke-width="1.6" opacity="0"></circle>';
      if (q.y >= 2) {
        marks +=
          '<circle class="mk-i" data-y="' + q.y + '" cx="' + mx + '" cy="' + yI(q.infl).toFixed(1) +
          '" r="3.5" fill="#0a0a0a" stroke="#06b6d4" stroke-width="1.6" opacity="0"></circle>';
      }
    }

    return (
      '<svg id="cyb-emi-svg" viewBox="0 0 ' + W + " " + H + '" preserveAspectRatio="xMidYMid meet">' +
      grid +
      '<polyline fill="none" stroke="#22c55e" stroke-width="2.2" points="' + ptsS + '"></polyline>' +
      '<polyline fill="none" stroke="#06b6d4" stroke-width="2" points="' + ptsI.join(" ") + '"></polyline>' +
      marks +
      '<line id="cyb-emi-guide" x1="0" y1="' + top + '" x2="0" y2="' + (top + plotH) +
      '" stroke="#444" stroke-width="1" stroke-dasharray="3 3" opacity="0"></line>' +
      hits +
      "</svg>"
    );
  }

  function render(hoverY) {
    var y = hoverY == null ? 10 : hoverY;
    var p = series[y];
    var inflLabel = y === 0 ? "\u2014" : (y === 1 ? "~50\u00d7 from genesis" : pct(p.infl, 1));
    root.querySelector("#cyb-emi-stats").innerHTML =
      card("year", y + " / " + YEARS, "") +
      card("supply", pct(p.supply, 1), "s") +
      card("inflation", inflLabel, "i") +
      card("unissued", pct(1 - p.supply, 1), "");
  }

  function bindTip() {
    var wrap = root.querySelector("#cyb-emi-chart");
    var svg = root.querySelector("#cyb-emi-svg");
    if (!wrap || !svg) return;
    var tip = root.querySelector("#cyb-emi-tip");
    if (!tip) {
      tip = document.createElement("div");
      tip.id = "cyb-emi-tip";
      tip.className = "tip";
      wrap.appendChild(tip);
    }
    var guide = svg.querySelector("#cyb-emi-guide");
    function clearMarks() {
      svg.querySelectorAll(".mk-s, .mk-i").forEach(function (el) { el.setAttribute("opacity", "0"); });
      if (guide) guide.setAttribute("opacity", "0");
    }
    function show(hit, year, clientX, clientY) {
      var p = series[year];
      var infl = year === 0 ? "\u2014" : year === 1 ? "~50\u00d7 (genesis \u2192 half cap)" : pct(p.infl, 2);
      tip.style.display = "block";
      tip.innerHTML = "year " + year + "<br>supply " + pct(p.supply, 2) + "<br>inflation " + infl;
      var wrapRect = wrap.getBoundingClientRect();
      var tipW = tip.offsetWidth || 160;
      var tipH = tip.offsetHeight || 48;
      var cx = clientX - wrapRect.left;
      var cy = clientY - wrapRect.top;
      var left = cx + 14;
      var top = cy - tipH - 10;
      if (left + tipW > wrapRect.width - 4) left = cx - tipW - 14;
      if (left < 4) left = 4;
      if (top < 4) top = cy + 16;
      tip.style.left = left + "px";
      tip.style.top = top + "px";
      clearMarks();
      svg.querySelectorAll('.mk-s[data-y="' + year + '"], .mk-i[data-y="' + year + '"]').forEach(function (el) {
        el.setAttribute("opacity", "1");
      });
      if (guide) {
        var gx = hit.getAttribute("x");
        var gw = hit.getAttribute("width");
        var mid = (+gx) + (+gw) / 2;
        // better: use mark x if available
        var mk = svg.querySelector('.mk-s[data-y="' + year + '"]');
        if (mk) mid = +mk.getAttribute("cx");
        guide.setAttribute("x1", mid);
        guide.setAttribute("x2", mid);
        guide.setAttribute("opacity", "1");
      }
      render(year);
    }
    function hide() {
      tip.style.display = "none";
      clearMarks();
    }
    svg.querySelectorAll("rect.pt").forEach(function (c) {
      var year = +c.getAttribute("data-y");
      c.addEventListener("mouseenter", function (e) { show(c, year, e.clientX, e.clientY); });
      c.addEventListener("mousemove", function (e) { show(c, year, e.clientX, e.clientY); });
      c.addEventListener("mouseleave", hide);
    });
  }

  root.innerHTML =
    '<div class="panel">' +
    '<div class="head"><div class="title">CYB emission schedule</div>' +
    '<div class="legend"><span><i class="s"></i>supply % of cap</span><span><i class="i"></i>yearly inflation</span></div></div>' +
    '<div class="stats" id="cyb-emi-stats"></div>' +
    '<div class="chart-wrap" id="cyb-emi-chart"></div>' +
    '<p class="note">M(t)/p = 1 \u2212 (1 + t/\u03c4)^(\u2212k), \u03c4 = 0.33 y, k = 0.5. Genesis floor 1% of cap. Time axis: log(1 + years) over 300y. Inflation axis scaled from year 2 (year 1 is the bootstrap expansion). Left: cumulative supply. Right: yearly inflation.</p>' +
    "</div>";

  root.querySelector("#cyb-emi-chart").innerHTML = buildChart();
  render(10);
  bindTip();
})();
</script>

Inflation drops below a flat-issuance design (Bittensor sits near 16% for years) by year three, while the heavy tail keeps issuing far longer than any halving.

## allocation is focus

Emission says how much; focus says who. Each freshly emitted unit is split by stake-weighted Δφ* — paid for the focus a contribution created, weighted by stake so forging identities buys nothing. This is where focus, kept out of the schedule, does its work: not in printing the money, but in directing it.

see [[cybernomics]] for the economic model
