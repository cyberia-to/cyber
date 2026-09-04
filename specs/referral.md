---
tags: cyber, specs, money, cybernomics
crystal-type: spec
crystal-domain: cyber
alias: referral, referral system, referrer share, dunbar decay
status: draft
---

# referral

growth is work, and the protocol pays for work: a bind-once referrer earns a share of every settled reward of its referee. the share is the 10% pool [[li]] already reserves; the decay makes it honest at scale; the witness gate makes it [[sybil-proof]].

## the curve

parameters in micros: pool $P = 100\,000$ (10%), floor $F = 10\,000$ (1%), Dunbar scale $D = 150$ — Dunbar's number, an anthropological estimate rather than a derived constant — activity window $W = 30$ epochs.

$$s(n) = \max\!\left(F,\ \frac{P \cdot D}{D + 9n}\right)$$

$n$ — direct referees active within $W$. the slope $9 = P/F - 1$ is forced by the boundary conditions $s(0) = P$ and $s(D) = F$; beyond $D$ the share stays at the floor. the referrer cut of a settled reward $A$ is $\lfloor A \cdot s(n)/10^6 \rfloor$; the referee keeps the rest.

hyperbolic, not linear: income $n \cdot s(n)$ must rise with every referee — the percentage falls, the volume grows, nobody is paid to stop recruiting. a linear descent to the floor peaks at $n \approx 83$ and then pays less for each newcomer; on the hyperbola each active referee adds at least the floor, and in integer micros monotonicity survives quantization for all $n \le D$. a large node lives on volume, a newcomer on margin, and [[dunbar number|Dunbar]] is the equilibrium cell size rather than a limit: past $D$ growth continues by delegation — new people head their own branches.

hover the chart, or a milestone card.

<div id="ref-curve"></div>

<style>
#ref-curve{--bg:#000;--s1:#0a0a0a;--s2:#111;--ln:#222;--tx:#f0f0f0;--mut:#8b948c;--neon:#22c55e;--cyan:#06b6d4;--amb:#eab308;--vio:#8b5cf6;background:transparent;color:var(--tx);font-family:var(--font-body,'Play',system-ui,sans-serif);border:none;box-shadow:none;box-sizing:border-box;width:100%;max-width:100%;margin:16px 0 28px;padding:0}
#ref-curve .panel{background:var(--s1);border:1px solid var(--ln);border-radius:10px;padding:14px 14px 10px}
#ref-curve .head{display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:10px 16px;margin:0 0 12px}
#ref-curve .title{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;color:var(--neon);letter-spacing:2px;text-transform:uppercase;text-shadow:0 0 10px rgba(34,197,94,.45)}
#ref-curve .legend{display:flex;flex-wrap:wrap;gap:10px 16px;font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;color:var(--mut)}
#ref-curve .legend i{display:inline-block;width:18px;height:0;border-top:2.4px solid;margin-right:6px;vertical-align:middle;border-radius:1px}
#ref-curve .legend .s{border-color:var(--neon)}
#ref-curve .legend .i{border-color:var(--cyan)}
#ref-curve .legend .sl{border-color:var(--amb);border-top-style:dashed}
#ref-curve .legend .il{border-color:var(--vio);border-top-style:dashed}
#ref-curve .milestones{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:6px;margin:0 0 12px}
#ref-curve .ms{background:var(--s2);border:1px solid var(--ln);border-radius:8px;padding:7px 8px;min-width:0;cursor:pointer;transition:border-color .12s,background .12s}
#ref-curve .ms:hover,#ref-curve .ms.on{border-color:var(--neon);background:rgba(34,197,94,.07)}
#ref-curve .ms .a{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:10px;color:var(--mut);letter-spacing:.3px;margin-bottom:3px}
#ref-curve .ms .b{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:13px;font-weight:600;color:var(--neon)}
#ref-curve .ms .c{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:10px;color:var(--cyan);margin-top:2px}
#ref-curve .stats{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:8px;margin:0 0 12px}
#ref-curve .stat{background:var(--s2);border:1px solid var(--ln);border-radius:8px;padding:8px 10px;min-width:0}
#ref-curve .stat .l{font-size:10px;color:var(--mut);letter-spacing:.4px;margin-bottom:3px}
#ref-curve .stat .v{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:14px;font-weight:600;word-break:break-word}
#ref-curve .stat .v.s{color:var(--neon);text-shadow:0 0 12px rgba(34,197,94,.25)}
#ref-curve .stat .v.i{color:var(--cyan);text-shadow:0 0 12px rgba(6,182,212,.25)}
#ref-curve .chart-wrap{position:relative;width:100%;min-height:300px}
#ref-curve .chart-wrap svg{width:100%;height:auto;display:block;cursor:crosshair}
#ref-curve svg text{font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:10px;fill:var(--mut)}
#ref-curve .tip{position:absolute;pointer-events:none;z-index:5;background:#111;border:1px solid #333;color:#f0f0f0;font-family:var(--font-mono,'JetBrains Mono',monospace);font-size:11px;padding:7px 10px;border-radius:6px;box-shadow:0 0 20px rgba(34,197,94,.15);white-space:nowrap;display:none;line-height:1.45}
#ref-curve .note{font-size:11px;color:var(--mut);margin:10px 0 0;line-height:1.5}
@media(max-width:640px){
  #ref-curve .stats{grid-template-columns:repeat(2,minmax(0,1fr))}
  #ref-curve .milestones{grid-template-columns:repeat(2,minmax(0,1fr))}
  #ref-curve .chart-wrap{min-height:260px}
}
</style>

<script>
(function(){
  var root = document.getElementById("ref-curve");
  if (!root) return;

  var P = 100000, F = 10000, D = 150, SLOPE = P / F - 1; // 9
  var N_MAX = 220;
  var PEAK = P * D / (2 * (P - F)); // linear-comparison peak, n ~= 83.33

  function shareHyp(n) { return Math.max(F, P * D / (D + SLOPE * n)); } // micros
  function shareLin(n) { return n >= D ? F : P - (P - F) * n / D; } // micros
  function incomeHyp(n) { return n * shareHyp(n) / 1e6; } // xA
  function incomeLin(n) { return n * shareLin(n) / 1e6; } // xA

  function at(n) {
    return { n: n, sHyp: shareHyp(n), sLin: shareLin(n), iHyp: incomeHyp(n), iLin: incomeLin(n) };
  }

  var S_MAX = 10; // percent axis ceiling (P = 10%)
  var I_MAX = 4.5; // xA axis ceiling

  var MILESTONES = [
    { label: "n = 0", n: 0 },
    { label: "n ≈ " + Math.round(PEAK) + " (linear peak)", n: PEAK },
    { label: "n = D = " + D, n: D },
    { label: "n = " + N_MAX, n: N_MAX }
  ];

  function pctFmt(microShare) { return (microShare / 1e6 * 100).toFixed(2) + "%"; }
  function xFmt(x) { return x.toFixed(2) + "×"; }

  var W = 960, H = 380;
  var left = 54, right = 58, topPad = 14, bottom = 36;
  var plotW = W - left - right, plotH = H - topPad - bottom;

  function xOf(n) { return left + plotW * (n / N_MAX); }
  function yShare(microShare) { return topPad + plotH * (1 - (microShare / 1e6 * 100) / S_MAX); }
  function yIncome(x) { return topPad + plotH * (1 - x / I_MAX); }

  function nFromClientX(svg, clientX) {
    var rect = svg.getBoundingClientRect();
    var px = (clientX - rect.left) / rect.width * W;
    var u = (px - left) / plotW;
    u = Math.max(0, Math.min(1, u));
    return u * N_MAX;
  }

  function card(label, value, cls) {
    return '<div class="stat"><div class="l">' + label + '</div><div class="v ' + (cls || "") + '">' + value + "</div></div>";
  }

  function buildChart() {
    var grid = "";
    var xD = xOf(D);
    grid += '<line x1="' + xD.toFixed(1) + '" y1="' + topPad + '" x2="' + xD.toFixed(1) + '" y2="' + (topPad + plotH) + '" stroke="#3f6b4a" stroke-width="1" stroke-dasharray="3 3"></line>';
    grid += '<text x="' + xD.toFixed(1) + '" y="' + (topPad + 12) + '" text-anchor="middle" fill="#3f6b4a" font-size="9">D</text>';

    var xPk = xOf(PEAK);
    grid += '<line x1="' + xPk.toFixed(1) + '" y1="' + topPad + '" x2="' + xPk.toFixed(1) + '" y2="' + (topPad + plotH) + '" stroke="#6b5a1f" stroke-width="1" stroke-dasharray="2 4"></line>';
    grid += '<text x="' + xPk.toFixed(1) + '" y="' + (topPad + 12) + '" text-anchor="middle" fill="#6b5a1f" font-size="9">linear peak</text>';

    for (var g = 0; g <= 4; g++) {
      var sv = g / 4 * S_MAX;
      var yy = yShare(sv / 100 * 1e6);
      grid += '<line x1="' + left + '" y1="' + yy + '" x2="' + (W - right) + '" y2="' + yy + '" stroke="#222" stroke-width="0.5"></line>';
      grid += '<text x="' + (left - 6) + '" y="' + (yy + 3) + '" text-anchor="end">' + sv.toFixed(1) + "%</text>";
    }
    for (var gi = 0; gi <= 4; gi++) {
      var iv = gi / 4 * I_MAX;
      var yyi = yIncome(iv);
      grid += '<text x="' + (W - right + 6) + '" y="' + (yyi + 3) + '" text-anchor="start">' + iv.toFixed(1) + "×</text>";
    }
    var nTicks = [0, 50, 100, 150, 200];
    for (var t = 0; t < nTicks.length; t++) {
      var nn = nTicks[t], xx = xOf(nn);
      grid += '<line x1="' + xx + '" y1="' + topPad + '" x2="' + xx + '" y2="' + (topPad + plotH) + '" stroke="#1a1a1a" stroke-width="0.5"></line>';
      grid += '<text x="' + xx + '" y="' + (H - 10) + '" text-anchor="middle">' + nn + "</text>";
    }

    var STEPS = 200;
    var ptsSH = [], ptsSL = [], ptsIH = [], ptsIL = [];
    for (var j = 0; j <= STEPS; j++) {
      var n = j / STEPS * N_MAX;
      var p = at(n);
      ptsSH.push(xOf(n).toFixed(2) + "," + yShare(p.sHyp).toFixed(2));
      ptsSL.push(xOf(n).toFixed(2) + "," + yShare(p.sLin).toFixed(2));
      ptsIH.push(xOf(n).toFixed(2) + "," + yIncome(p.iHyp).toFixed(2));
      ptsIL.push(xOf(n).toFixed(2) + "," + yIncome(p.iLin).toFixed(2));
    }

    return (
      '<svg id="ref-curve-svg" viewBox="0 0 ' + W + " " + H + '" preserveAspectRatio="xMidYMid meet">' +
      grid +
      '<polyline fill="none" stroke="#eab308" stroke-width="1.6" stroke-dasharray="5 4" points="' + ptsSL.join(" ") + '"></polyline>' +
      '<polyline fill="none" stroke="#8b5cf6" stroke-width="1.6" stroke-dasharray="5 4" points="' + ptsIL.join(" ") + '"></polyline>' +
      '<polyline fill="none" stroke="#22c55e" stroke-width="2.2" points="' + ptsSH.join(" ") + '"></polyline>' +
      '<polyline fill="none" stroke="#06b6d4" stroke-width="2" points="' + ptsIH.join(" ") + '"></polyline>' +
      '<circle id="ref-mk-sh" cx="0" cy="0" r="4.2" fill="#0a0a0a" stroke="#22c55e" stroke-width="1.8" opacity="0"></circle>' +
      '<circle id="ref-mk-ih" cx="0" cy="0" r="4.2" fill="#0a0a0a" stroke="#06b6d4" stroke-width="1.8" opacity="0"></circle>' +
      '<line id="ref-curve-guide" x1="0" y1="' + topPad + '" x2="0" y2="' + (topPad + plotH) +
      '" stroke="#444" stroke-width="1" stroke-dasharray="3 3" opacity="0"></line>' +
      '<rect id="ref-curve-hit" x="' + left + '" y="' + topPad + '" width="' + plotW + '" height="' + plotH +
      '" fill="transparent"></rect>' +
      "</svg>"
    );
  }

  function render(p) {
    if (!p) p = at(D);
    root.querySelector("#ref-curve-stats").innerHTML =
      card("referees n", Math.round(p.n), "") +
      card("share s(n)", pctFmt(p.sHyp), "s") +
      card("income n·s(n)", xFmt(p.iHyp), "i") +
      card("linear income", xFmt(p.iLin), "");
  }

  function milestonesHtml() {
    return MILESTONES.map(function (m, idx) {
      var p = at(m.n);
      return (
        '<div class="ms" data-n="' + m.n + '" data-i="' + idx + '">' +
        '<div class="a">' + m.label + "</div>" +
        '<div class="b">' + pctFmt(p.sHyp) + "</div>" +
        '<div class="c">' + xFmt(p.iHyp) + " income</div>" +
        "</div>"
      );
    }).join("");
  }

  function bind() {
    var wrap = root.querySelector("#ref-curve-chart");
    var svg = root.querySelector("#ref-curve-svg");
    if (!wrap || !svg) return;
    var tip = document.createElement("div");
    tip.className = "tip";
    wrap.appendChild(tip);

    var hit = svg.querySelector("#ref-curve-hit");
    var guide = svg.querySelector("#ref-curve-guide");
    var mkSh = svg.querySelector("#ref-mk-sh");
    var mkIh = svg.querySelector("#ref-mk-ih");

    function setActiveMilestone(n) {
      root.querySelectorAll(".ms").forEach(function (el) {
        var mn = +el.getAttribute("data-n");
        el.classList.toggle("on", Math.abs(mn - n) < 3);
      });
    }

    function show(clientX, clientY, nOpt) {
      var n = nOpt != null ? nOpt : nFromClientX(svg, clientX);
      var p = at(n);
      tip.style.display = "block";
      tip.innerHTML =
        "n " + Math.round(p.n) +
        "<br>share, hyperbolic " + pctFmt(p.sHyp) +
        "<br>share, linear " + pctFmt(p.sLin) +
        "<br>income, hyperbolic " + xFmt(p.iHyp) +
        "<br>income, linear " + xFmt(p.iLin);
      var wrapRect = wrap.getBoundingClientRect();
      var tipW = tip.offsetWidth || 190;
      var tipH = tip.offsetHeight || 90;
      var cx = clientX - wrapRect.left;
      var cy = clientY - wrapRect.top;
      var L = cx + 14, T = cy - tipH - 10;
      if (L + tipW > wrapRect.width - 4) L = cx - tipW - 14;
      if (L < 4) L = 4;
      if (T < 4) T = cy + 16;
      tip.style.left = L + "px";
      tip.style.top = T + "px";

      var xx = xOf(p.n);
      mkSh.setAttribute("cx", xx);
      mkSh.setAttribute("cy", yShare(p.sHyp));
      mkSh.setAttribute("opacity", "1");
      mkIh.setAttribute("cx", xx);
      mkIh.setAttribute("cy", yIncome(p.iHyp));
      mkIh.setAttribute("opacity", "1");
      guide.setAttribute("x1", xx);
      guide.setAttribute("x2", xx);
      guide.setAttribute("opacity", "1");
      render(p);
      setActiveMilestone(p.n);
    }
    function hide() {
      tip.style.display = "none";
      mkSh.setAttribute("opacity", "0");
      mkIh.setAttribute("opacity", "0");
      guide.setAttribute("opacity", "0");
      root.querySelectorAll(".ms").forEach(function (el) { el.classList.remove("on"); });
    }

    hit.addEventListener("mousemove", function (e) { show(e.clientX, e.clientY); });
    hit.addEventListener("mouseenter", function (e) { show(e.clientX, e.clientY); });
    hit.addEventListener("mouseleave", hide);

    root.querySelectorAll(".ms").forEach(function (el) {
      el.addEventListener("mouseenter", function () {
        var n = +el.getAttribute("data-n");
        var rect = wrap.getBoundingClientRect();
        var svgRect = svg.getBoundingClientRect();
        show(svgRect.left + (xOf(n) / W) * svgRect.width, rect.top + 40, n);
      });
      el.addEventListener("click", function (e) {
        var n = +el.getAttribute("data-n");
        var svgRect = svg.getBoundingClientRect();
        show(svgRect.left + (xOf(n) / W) * svgRect.width, e.clientY, n);
      });
    });
  }

  root.innerHTML =
    '<div class="panel">' +
    '<div class="head"><div class="title">referral share s(n)</div>' +
    '<div class="legend"><span><i class="s"></i>share s(n)</span><span><i class="i"></i>income n·s(n)</span><span><i class="sl"></i>share, linear</span><span><i class="il"></i>income, linear</span></div></div>' +
    '<div class="milestones" id="ref-curve-ms">' + milestonesHtml() + "</div>" +
    '<div class="stats" id="ref-curve-stats"></div>' +
    '<div class="chart-wrap" id="ref-curve-chart"></div>' +
    '<p class="note">s(n) = max(F, P·D/(D+9n)), micros; P=100,000 (10%), F=10,000 (1%), D=150. Green: share s(n). Cyan: income n·s(n) in units of mean referee reward A. Dashed amber/violet: a linear descent to the same floor, for comparison — it peaks near n≈' + Math.round(PEAK) + ' then pays less per newcomer down to n=D, where both curves meet.</p>' +
    "</div>";

  root.querySelector("#ref-curve-chart").innerHTML = buildChart();
  render(at(D));
  bind();
})();
</script>

## the witness gate

decay alone pays for splitting: a referrer at $n = 150$ earns $150F$ per unit of mean referee reward, two fresh identities at 75 each earn $2 \cdot 75 \cdot s(75) \approx 1.82\times$ more. so the curve opens only to a witnessed identity — one [[attested genome protocol|nullifier]], one curve — and an unwitnessed referrer earns the flat floor at any $n$. splitting across unwitnessed identities returns exactly the floor; splitting across witnessed identities requires distinct humans, which is recruiting — the attack becomes the behavior the pool buys. the curve inherits its security from the attestation primitive, the way reward inherits sybil-resistance from karma non-transferability in [[tru]].

[[moon passport]] resolves names, not persons — one owner holds many passports — so the gate mounts on the passport's proof slot, not on the passport itself.

## wiring

binding. the referrer arrives in boot.dat ([[cyb-boot]]) and binds on first sync. bind-once: self-binding, rebinding, and upline cycles are rejected.

payout. every settle path converges in `apply_settle_receipt` ([[cyb]] core): the epoch's [[Shapley value|Shapley]] share splits — the referee is marked active, the referrer cut is credited with a matching [[tok]] mint leg (cut + net = share, conservation), the net mints to the referee under clock-B escrow. [[sense]] and sigma read `ReferralAccrued`.

witness. `attest_witness` admits a neuron to the curve; the verifying implementation — a [[moon passport]] extension proof or a genome nullifier — is the mount point, trusted-local until it lands.

## open

- destination of the freed margin $P - s(n)$: treasury, or cashback to the referee — a counter-pull that equilibrates branches around $D$
- turnover-driven decay instead of the activity window
- referrer-side clock-B escrow

see [[specs/money-loop|money loop]] for the settle path · [[attested genome protocol]] for the witness primitive

discover all [[concepts]]
