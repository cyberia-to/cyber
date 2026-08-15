---
tags: cyber, cybernomics
alias: $CYB, CYB, cyber energy
crystal-type: entity
crystal-domain: economics
icon: "⚡"
---
root [[token]] of cyber. unit of stake, fees, and rewards for proven contribution to [[focus]] φ*. emission schedule is a function of time alone; who receives emission is a function of φ*. see [[whitepaper]], [[mining]], [[staking]]

## role

[[Focus]] is the scarce object: the unique [[fixed point]] of the [[tri-kernel]] over the [[cybergraph]]. Moving and creating focus costs work. [[CYB]] makes that work transferable — spend to link and prove, earn when your links raise collective focus (proven Δφ*). It is not a fee token bolted onto ranking; ranking and payment share one physics

## cap

Total supply is the order of the [[nebu|Goldilocks field]] used by [[nox]] and proofs:

p = 2⁶⁴ − 2³² + 1 = 18,446,744,069,414,584,321

Cap is arithmetic, not a governance vote. On the [[bootloader]] ([[bostrom]]) the same energy currently circulates as [[$C]]

## genesis

At block 0, [[$C]] holders receive 187,416,084,623,451,570 CYB ≈ 1% of p (281,405,532,467,645 snapshot × 666). Continuity of prior stake into the soft3 field; remaining mass is reserved for the emission schedule

## emission

How much exists at network age t (years) is fixed by the clock — identical on every honest node, no oracle, no forgery surface:

M(t) = p · (1 − (1 + t/τ)^(−k)),    τ = 0.33 year,   k = 0.5

π(t) = M′(t)/M(t) is instantaneous inflation (1/year). There is no discrete year step and no halving epoch

### why this shape

The graph needs proving and settlement capacity most when it is empty and growing. A power-law head puts most of the emission in the first years so early useful work (mining division and fold, early links that set topology) is paid while the network is still cheap to dominate with capital alone. Finite initial rate k/τ ≈ 152% of cap per year avoids a single-block flood; the year-1 integral still reaches ≈ half of p, so the main prize for compute is front-loaded without a cliff

The polynomial tail keeps residual issuance for centuries under the same cap: long-run security budget without reintroducing a policy rate. Scale-free graphs and Zipf focus already are power laws; emission matches the same family so the money supply does not impose a foreign timescale (e.g. fixed halvings) on a structure that does not have one

Clock and focus stay separated: M(t) only answers how much is available; stake-weighted Δφ* answers who mints (see allocation)

### schedule

<div id="cyb-emi"></div>

<style>
#cyb-emi{--bg:#000;--s1:#0a0a0a;--s2:#111;--ln:#222;--tx:#f0f0f0;--mut:#8b948c;--neon:#22c55e;--cyan:#06b6d4;--amb:#eab308;background:transparent;color:var(--tx);font-family:var(--font-body,'Play',system-ui,sans-serif);border:none;box-shadow:none;box-sizing:border-box;width:100%;max-width:100%;margin:16px 0 28px;padding:0}
#cyb-emi .panel{background:var(--s1);border:1px solid var(--ln);border-radius:10px;padding:14px 14px 10px}
#cyb-emi .head{display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:10px 16px;margin:0 0 12px}
#cyb-emi .title{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;color:var(--neon);letter-spacing:2px;text-transform:uppercase;text-shadow:0 0 10px rgba(34,197,94,.45)}
#cyb-emi .legend{display:flex;flex-wrap:wrap;gap:12px 18px;font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;color:var(--mut)}
#cyb-emi .legend i{display:inline-block;width:18px;height:0;border-top:2.4px solid;margin-right:6px;vertical-align:middle;border-radius:1px}
#cyb-emi .legend .s{border-color:var(--neon)}
#cyb-emi .legend .i{border-color:var(--cyan)}
#cyb-emi .milestones{display:grid;grid-template-columns:repeat(7,minmax(0,1fr));gap:6px;margin:0 0 12px}
#cyb-emi .ms{background:var(--s2);border:1px solid var(--ln);border-radius:8px;padding:7px 8px;min-width:0;cursor:pointer;transition:border-color .12s,background .12s}
#cyb-emi .ms:hover,#cyb-emi .ms.on{border-color:var(--neon);background:rgba(34,197,94,.07)}
#cyb-emi .ms .a{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:10px;color:var(--mut);letter-spacing:.3px;margin-bottom:3px}
#cyb-emi .ms .b{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:13px;font-weight:600;color:var(--neon)}
#cyb-emi .ms .c{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:10px;color:var(--cyan);margin-top:2px}
#cyb-emi .stats{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:8px;margin:0 0 12px}
#cyb-emi .stat{background:var(--s2);border:1px solid var(--ln);border-radius:8px;padding:8px 10px;min-width:0}
#cyb-emi .stat .l{font-size:10px;color:var(--mut);letter-spacing:.4px;margin-bottom:3px}
#cyb-emi .stat .v{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:14px;font-weight:600;word-break:break-word}
#cyb-emi .stat .v.s{color:var(--neon);text-shadow:0 0 12px rgba(34,197,94,.25)}
#cyb-emi .stat .v.i{color:var(--cyan);text-shadow:0 0 12px rgba(6,182,212,.25)}
#cyb-emi .chart-wrap{position:relative;width:100%;min-height:300px}
#cyb-emi .chart-wrap svg{width:100%;height:auto;display:block;cursor:crosshair}
#cyb-emi svg text{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:10px;fill:var(--mut)}
#cyb-emi .tip{position:absolute;pointer-events:none;z-index:5;background:#111;border:1px solid #333;color:#f0f0f0;font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;padding:7px 10px;border-radius:6px;box-shadow:0 0 20px rgba(34,197,94,.15);white-space:nowrap;display:none;line-height:1.45}
#cyb-emi .note{font-size:11px;color:var(--mut);margin:10px 0 0;line-height:1.5}
@media(max-width:900px){
  #cyb-emi .milestones{grid-template-columns:repeat(4,minmax(0,1fr))}
}
@media(max-width:640px){
  #cyb-emi .stats{grid-template-columns:repeat(2,minmax(0,1fr))}
  #cyb-emi .milestones{grid-template-columns:repeat(2,minmax(0,1fr))}
  #cyb-emi .chart-wrap{min-height:260px}
}
</style>

<script>
(function(){
  var root = document.getElementById("cyb-emi");
  if (!root) return;

  var TAU = 0.33;
  var K = 0.5;
  var YEARS = 300;
  var T_MIN = 1 / 365;

  function s(t) {
    if (t <= 0) return 0;
    return 1 - Math.pow(1 + t / TAU, -K);
  }
  function sPrime(t) {
    if (t < 0) return 0;
    return (K / TAU) * Math.pow(1 + t / TAU, -(K + 1));
  }
  function pi(t) {
    var st = s(t);
    if (st <= 1e-15) return Infinity;
    return sPrime(t) / st;
  }
  function at(t) {
    return { t: t, supply: s(t), infl: pi(t), rate: sPrime(t) };
  }

  function sampleLog(t0, t1, n) {
    var a = Math.log1p(t0), b = Math.log1p(t1), out = [];
    for (var i = 0; i < n; i++) {
      var u = n === 1 ? 0 : i / (n - 1);
      out.push(Math.expm1(a + u * (b - a)));
    }
    return out;
  }

  // Dense in year 1, then log-sparse to 300y
  var times = sampleLog(T_MIN, 1, 220).concat(sampleLog(1, YEARS, 300).slice(1));
  var series = times.map(at);

  var logMin = Math.log1p(T_MIN);
  var logMax = Math.log1p(YEARS);

  var MILESTONES = [
    { label: "1 day", t: 1 / 365 },
    { label: "1 week", t: 7 / 365 },
    { label: "1 month", t: 1 / 12 },
    { label: "3 month", t: 0.25 },
    { label: "1 year", t: 1 },
    { label: "2 year", t: 2 },
    { label: "4 year", t: 4 }
  ];

  function fmtYear(t) {
    if (t < 1 / 24) return (t * 365).toFixed(1) + "d";
    if (t < 1 / 12) return (t * 365).toFixed(0) + "d";
    if (t < 1) return (t * 12).toFixed(1) + "mo";
    if (t < 10) return t.toFixed(2) + "y";
    if (t < 100) return t.toFixed(1) + "y";
    return t.toFixed(0) + "y";
  }

  function pct(x, d) {
    if (x == null || !isFinite(x)) return "\u2014";
    var p = x * 100;
    if (p >= 1000) return p.toFixed(0) + "%";
    if (p >= 100) return p.toFixed(d == null ? 0 : d) + "%";
    if (p >= 10) return p.toFixed(d == null ? 1 : d) + "%";
    if (p >= 1) return p.toFixed(d == null ? 1 : d) + "%";
    if (p >= 0.01) return p.toFixed(d == null ? 2 : d) + "%";
    return p.toFixed(3) + "%";
  }

  function card(label, value, cls) {
    return '<div class="stat"><div class="l">' + label + '</div><div class="v ' + (cls || "") + '">' + value + "</div></div>";
  }

  var inflVals = series.map(function (p) { return p.infl; }).filter(function (v) { return isFinite(v) && v > 0; });
  var logI0 = Math.log10(Math.max(Math.min.apply(null, inflVals), 1e-5));
  var logI1 = Math.log10(Math.max.apply(null, inflVals) * 1.15);

  // layout constants shared by draw + hover
  var W = 960, H = 380;
  var left = 52, right = 58, topPad = 14, bottom = 36;
  var plotW = W - left - right, plotH = H - topPad - bottom;

  function xOf(t) {
    return left + plotW * ((Math.log1p(t) - logMin) / (logMax - logMin));
  }
  function yS(sv) { return topPad + plotH * (1 - sv); }
  function yI(inf) {
    if (!isFinite(inf) || inf <= 0) return topPad;
    var u = (Math.log10(inf) - logI0) / (logI1 - logI0);
    u = Math.max(0, Math.min(1, u));
    return topPad + plotH * (1 - u);
  }
  function tFromClientX(svg, clientX) {
    var rect = svg.getBoundingClientRect();
    var px = (clientX - rect.left) / rect.width * W;
    var u = (px - left) / plotW;
    u = Math.max(0, Math.min(1, u));
    return Math.expm1(logMin + u * (logMax - logMin));
  }

  function buildChart() {
    var grid = "";
    // year-1 band
    var x0 = xOf(T_MIN), x1 = xOf(1);
    grid +=
      '<rect x="' + x0.toFixed(1) + '" y="' + topPad + '" width="' + (x1 - x0).toFixed(1) +
      '" height="' + plotH + '" fill="rgba(34,197,94,0.05)" stroke="none"></rect>';
    grid +=
      '<text x="' + ((x0 + x1) / 2).toFixed(1) + '" y="' + (topPad + 12) +
      '" text-anchor="middle" fill="#3f6b4a" font-size="9">year 1 bootstrap</text>';

    for (var g = 0; g <= 4; g++) {
      var sv = g / 4;
      var yy = yS(sv);
      grid += '<line x1="' + left + '" y1="' + yy + '" x2="' + (W - right) + '" y2="' + yy + '" stroke="#222" stroke-width="0.5"></line>';
      grid += '<text x="' + (left - 6) + '" y="' + (yy + 3) + '" text-anchor="end">' + Math.round(sv * 100) + "%</text>";
    }

    var e0 = Math.ceil(logI0), e1 = Math.floor(logI1);
    for (var e = e0; e <= e1; e++) {
      var iv = Math.pow(10, e);
      var yi = yI(iv);
      grid += '<text x="' + (W - right + 6) + '" y="' + (yi + 3) + '" text-anchor="start">' + pct(iv, iv >= 0.1 ? 0 : 1) + "/y</text>";
    }

    // denser ticks in year 1, then sparse tail
    var yTicks = [
      [T_MIN, "1d"], [7 / 365, "1w"], [1 / 12, "1mo"], [0.25, "3mo"], [0.5, "6mo"], [1, "1y"],
      [2, "2y"], [5, "5y"], [10, "10y"], [20, "20y"], [50, "50y"], [100, "100y"], [200, "200y"], [300, "300y"]
    ];
    for (var t = 0; t < yTicks.length; t++) {
      var yr = yTicks[t][0], lab = yTicks[t][1];
      if (yr < T_MIN * 0.99 || yr > YEARS * 1.001) continue;
      var xx = xOf(yr);
      var strong = yr <= 1;
      grid += '<line x1="' + xx + '" y1="' + topPad + '" x2="' + xx + '" y2="' + (topPad + plotH) +
        '" stroke="' + (strong ? "#2a3a2a" : "#1a1a1a") + '" stroke-width="0.5"></line>';
      grid += '<text x="' + xx + '" y="' + (H - 10) + '" text-anchor="middle"' +
        (strong ? ' fill="#8b948c"' : "") + ">" + lab + "</text>";
    }

    var ptsS = [], ptsI = [];
    for (var j = 0; j < series.length; j++) {
      var p = series[j];
      ptsS.push(xOf(p.t).toFixed(2) + "," + yS(p.supply).toFixed(2));
      if (isFinite(p.infl) && p.infl > 0) {
        ptsI.push(xOf(p.t).toFixed(2) + "," + yI(p.infl).toFixed(2));
      }
    }

    // milestone dots on supply for year 1
    var msMarks = "";
    for (var m = 0; m < MILESTONES.length; m++) {
      var mt = MILESTONES[m].t;
      var mp = at(mt);
      msMarks +=
        '<circle cx="' + xOf(mt).toFixed(1) + '" cy="' + yS(mp.supply).toFixed(1) +
        '" r="3.2" fill="#0a0a0a" stroke="#22c55e" stroke-width="1.5"></circle>';
      if (isFinite(mp.infl)) {
        msMarks +=
          '<circle cx="' + xOf(mt).toFixed(1) + '" cy="' + yI(mp.infl).toFixed(1) +
          '" r="3.2" fill="#0a0a0a" stroke="#06b6d4" stroke-width="1.5"></circle>';
      }
    }

    return (
      '<svg id="cyb-emi-svg" viewBox="0 0 ' + W + " " + H + '" preserveAspectRatio="xMidYMid meet">' +
      grid +
      '<polyline fill="none" stroke="#22c55e" stroke-width="2.2" points="' + ptsS.join(" ") + '"></polyline>' +
      '<polyline fill="none" stroke="#06b6d4" stroke-width="2" points="' + ptsI.join(" ") + '"></polyline>' +
      msMarks +
      '<circle id="cyb-mk-s" cx="0" cy="0" r="4.2" fill="#0a0a0a" stroke="#22c55e" stroke-width="1.8" opacity="0"></circle>' +
      '<circle id="cyb-mk-i" cx="0" cy="0" r="4.2" fill="#0a0a0a" stroke="#06b6d4" stroke-width="1.8" opacity="0"></circle>' +
      '<line id="cyb-emi-guide" x1="0" y1="' + topPad + '" x2="0" y2="' + (topPad + plotH) +
      '" stroke="#444" stroke-width="1" stroke-dasharray="3 3" opacity="0"></line>' +
      '<rect id="cyb-emi-hit" x="' + left + '" y="' + topPad + '" width="' + plotW + '" height="' + plotH +
      '" fill="transparent"></rect>' +
      "</svg>"
    );
  }

  function render(p) {
    if (!p) p = at(1);
    root.querySelector("#cyb-emi-stats").innerHTML =
      card("age", fmtYear(p.t), "") +
      card("supply", pct(p.supply, 2), "s") +
      card("inflation \u03c0", isFinite(p.infl) ? pct(p.infl, 1) + "/y" : "\u2014", "i") +
      card("emit rate", pct(p.rate, 1) + " cap/y", "");
  }

  function milestonesHtml() {
    return MILESTONES.map(function (m, idx) {
      var p = at(m.t);
      return (
        '<div class="ms" data-t="' + m.t + '" data-i="' + idx + '">' +
        '<div class="a">' + m.label + "</div>" +
        '<div class="b">' + pct(p.supply, 1) + "</div>" +
        '<div class="c">\u03c0 ' + pct(p.infl, 0) + "/y</div>" +
        "</div>"
      );
    }).join("");
  }

  function bind() {
    var wrap = root.querySelector("#cyb-emi-chart");
    var svg = root.querySelector("#cyb-emi-svg");
    if (!wrap || !svg) return;
    var tip = document.createElement("div");
    tip.className = "tip";
    wrap.appendChild(tip);

    var hit = svg.querySelector("#cyb-emi-hit");
    var guide = svg.querySelector("#cyb-emi-guide");
    var mkS = svg.querySelector("#cyb-mk-s");
    var mkI = svg.querySelector("#cyb-mk-i");

    function setActiveMilestone(t) {
      root.querySelectorAll(".ms").forEach(function (el) {
        var mt = +el.getAttribute("data-t");
        var on = Math.abs(mt - t) / Math.max(t, mt, 1e-9) < 0.08 || Math.abs(mt - t) < 0.02;
        el.classList.toggle("on", on);
      });
    }

    function show(clientX, clientY, tOpt) {
      var t = tOpt != null ? tOpt : tFromClientX(svg, clientX);
      var p = at(t);
      tip.style.display = "block";
      tip.innerHTML =
        "age " + fmtYear(p.t) +
        "<br>supply " + pct(p.supply, 2) +
        "<br>\u03c0 " + (isFinite(p.infl) ? pct(p.infl, 1) + "/y" : "\u2014") +
        "<br>rate " + pct(p.rate, 1) + " cap/y";
      var wrapRect = wrap.getBoundingClientRect();
      var tipW = tip.offsetWidth || 170;
      var tipH = tip.offsetHeight || 60;
      var cx = clientX - wrapRect.left;
      var cy = clientY - wrapRect.top;
      var L = cx + 14, T = cy - tipH - 10;
      if (L + tipW > wrapRect.width - 4) L = cx - tipW - 14;
      if (L < 4) L = 4;
      if (T < 4) T = cy + 16;
      tip.style.left = L + "px";
      tip.style.top = T + "px";

      var xx = xOf(p.t);
      mkS.setAttribute("cx", xx);
      mkS.setAttribute("cy", yS(p.supply));
      mkS.setAttribute("opacity", "1");
      if (isFinite(p.infl) && p.infl > 0) {
        mkI.setAttribute("cx", xx);
        mkI.setAttribute("cy", yI(p.infl));
        mkI.setAttribute("opacity", "1");
      } else mkI.setAttribute("opacity", "0");
      guide.setAttribute("x1", xx);
      guide.setAttribute("x2", xx);
      guide.setAttribute("opacity", "1");
      render(p);
      setActiveMilestone(p.t);
    }
    function hide() {
      tip.style.display = "none";
      mkS.setAttribute("opacity", "0");
      mkI.setAttribute("opacity", "0");
      guide.setAttribute("opacity", "0");
      root.querySelectorAll(".ms").forEach(function (el) { el.classList.remove("on"); });
    }

    hit.addEventListener("mousemove", function (e) { show(e.clientX, e.clientY); });
    hit.addEventListener("mouseenter", function (e) { show(e.clientX, e.clientY); });
    hit.addEventListener("mouseleave", hide);

    root.querySelectorAll(".ms").forEach(function (el) {
      el.addEventListener("mouseenter", function (e) {
        var t = +el.getAttribute("data-t");
        var rect = wrap.getBoundingClientRect();
        var svgRect = svg.getBoundingClientRect();
        // place tip near milestone column
        show(svgRect.left + (xOf(t) / W) * svgRect.width, rect.top + 40, t);
      });
      el.addEventListener("click", function (e) {
        var t = +el.getAttribute("data-t");
        var svgRect = svg.getBoundingClientRect();
        show(svgRect.left + (xOf(t) / W) * svgRect.width, e.clientY, t);
      });
      el.addEventListener("mouseleave", function () { /* keep last hover on chart */ });
    });
  }

  root.innerHTML =
    '<div class="panel">' +
    '<div class="head"><div class="title">CYB emission schedule</div>' +
    '<div class="legend"><span><i class="s"></i>cumulative supply</span><span><i class="i"></i>instant \u03c0(t)</span></div></div>' +
    '<div class="milestones" id="cyb-emi-ms">' + milestonesHtml() + "</div>" +
    '<div class="stats" id="cyb-emi-stats"></div>' +
    '<div class="chart-wrap" id="cyb-emi-chart"></div>' +
    '<p class="note">M(t)/p = 1 \u2212 (1 + t/\u03c4)^(\u2212k), \u03c4 = 0.33 y, k = 0.5. Green: cumulative supply/cap. Cyan: log \u03c0(t) = M\u2032/M (1/y). Time: log(1+t), 1d\u2013300y. Green band = year 1 (\u224850% of cap).</p>' +
    "</div>";

  root.querySelector("#cyb-emi-chart").innerHTML = buildChart();
  render(at(1));
  bind();
})();
</script>

## utility (plumb)

[[CYB]] is a [[coin]] under [[tok]] / [[plumb]]. Every use is one of: [[pay]], [[lock]], [[mint]], [[burn]] (+ [[uber]] for non-value state). Σ balances = mints − burns; [[zheng]] rejects the rest

| op | CYB |
|----|-----|
| [[mint]] | Coin emission or Card birth ([[neuron]] / [[cyb/robot\|robot]]) |
| [[lock]] | stake / [[will]] — risk for influence |
| [[pay]] | transfer under 1% tax |
| [[burn]] | fee deflation + eternal φ*-weight |
| [[uber]] | φ*, karma — no supply change |

### mint

**Coin.** Budget M(t) and hybrid B under [[adaptive hybrid economics]] (PID α, floor, β): \(R_{\mathrm{PoW}}=B(1-\theta^\alpha)\), \(R_{\mathrm{PoS}}=B\theta^\alpha\).

| channel | risk | earn |
|---------|------|------|
| [[mining]] | none | prove Δφ* [[Shapley value\|division]] + [[fold mining\|fold]] → mint |
| [[staking]] active (v≠0) | lock CYB | stake-side mint if focus moves with the claim |

Δφ* mint only if proven Δφ* > 0. Security floor mints only to PoW + active stake; PID-decays as fees cover security. Passive lock (v=0): rank only. Spec: [[rewards]]

**Card.** mint robot/neuron = identity Card, not CYB. Create may cost pay/burn. At create freeze `creator_mint_share`, `creator_pay_share` ∈ [0,1]: residual on later Coin mints and fee-leg pays through that agent; optional royalty on card transfer. Robots redirect who receives M(t) — they do not open a second schedule

```
M(t), B, fees → hybrid α → PoW/PoS mint [− creator_mint_share if via robot]
pay volume    → 1% tax → burn β·tax + pool + creator_pay_share
```

### lock

Freeze CYB on a [[cyberlink]] ([[staking]], [[will]]). Active (v=±1): weight + stake mint + fee yield × [[karma]]. Passive (v=0): φ* influence only. Wrong active bets lose score under [[Bayesian Truth Serum|BTS]]; idle capital does not compound mint

### pay

Every transfer G: recipient \(1-\tau\), \(\tau=1\%\). Of the tax: burn \(\beta\cdot\tau\cdot G\), fee pool \((1-\beta)\tau\cdot G\) (security + service). Robot-mediated: `creator_pay_share` of the fee-pool leg to creator first. Headers, queries, DA, inference — all pays under the same tax

### burn

Fee burn (velocity → deflation); eternal [[particle]] / [[cyberlink]] (φ* floor); optional slash-to-burn. Mint makes energy transferable; burn makes influence permanent

## allocation

How much: M(t) + hybrid B. Who: Δφ* [[Shapley value]] (knowledge); fold/settlement work (PoW); active stake × karma (PoS). Zero stake and zero work → zero. [[rewards]]

## loop

Lock truth; pay under 1% tax that burns; mint Coin by hybrid work or mint Card agents with frozen creator residuals. [[tok]] · [[plumb]] · [[adaptive hybrid economics]] · [[cyb/robot]] · [[rewards]] · [[self]]
