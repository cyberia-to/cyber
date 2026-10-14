/* cyber — the superintelligence · interactive deck
   a frozen protocol for collective lifeforms. 7 chapters × 3 slides. the graph behind you grows with what you do.
   marks: ● measured · ○ target · ◌ mock (a browser simulation, not the chain) */
(() => {
const $ = (s, r = document) => r.querySelector(s);
const el = (h) => { const t = document.createElement('template'); t.innerHTML = h.trim(); return t.content.firstChild; };
const fmt = (n) => n.toLocaleString('en-US');
const esc = (t) => t.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
const M = (k) => `<i class="mk ${k}"></i>`; // m = measured, t = target, k = mock
const MONO = 'JetBrains Mono,Menlo,monospace', SANS = 'Play,Inter,system-ui,sans-serif';
const COLS = ['#ff1a00', '#ff7a00', '#ffc400', '#00ff41', '#00b8ff', '#1e00ff', '#7a00ff'];

/* ------------------------------------------------------------------ chapters */
const CH = [
  { id: 'si',      name: 'superintelligence', c: 'var(--c1)' },
  { id: 'anatomy', name: 'anatomy',           c: 'var(--c2)' },
  { id: 'focus',   name: 'focus',             c: 'var(--c3)' },
  { id: 'compile', name: 'compile',           c: 'var(--c4)' },
  { id: 'syntropy',name: 'syntropy',          c: 'var(--c5)' },
  { id: 'frozen',  name: 'frozen',            c: 'var(--c6)' },
  { id: 'moneys',  name: 'money', c: 'var(--c7)' },
];

/* ------------------------------------------------------------------ figures */
const svgo = (w, h, label, body) => `<svg class="fig" viewBox="0 0 ${w} ${h}" role="img" aria-label="${label}">${body}</svg>`;
const T = (x, y, t, o = {}) => `<text x="${x}" y="${y}" font-size="${o.fs || 11}" font-weight="${o.w || 400}" text-anchor="${o.a || 'start'}" fill="${o.c || 'currentColor'}" fill-opacity="${o.op || 1}" font-family="${o.ff || (o.w === 700 ? SANS : MONO)}"${o.ls ? ` letter-spacing="${o.ls}"` : ''}>${t}</text>`;

const scaleFig = () => {
  const W = 760, H = 275, X0 = 40, X1 = 720, B = 150;
  const x = (e) => X0 + (X1 - X0) * e / 24;
  const sup = (n) => String(n).replace(/\d/g, (d) => '⁰¹²³⁴⁵⁶⁷⁸⁹'[d]);
  const L = [
    ['self', 0, '1', 'one neuron, one particle', COLS[3], 1, 24], ['crystal', 3.7, '5,040', 'curated seed', COLS[3], -1, 24],
    ['bootloader', 6.5, '3,143,650', 'today · d⋆ = 31', COLS[3], 1, 70], ['cognition', 8.5, '10⁸ – 10⁹', 'concepts no one holds', COLS[4], -1, 50],
    ['thermodynamic', 12, '10¹²', 'φ⋆ is the state', COLS[4], 1, 24], ['planetary', 15, '10¹⁵ · 10¹⁰ neurons', 'design target', COLS[4], -1, 24],
    ['Avogadro', 23.78, '6.02 × 10²³', 'a mole of matter', COLS[1], 1, 70],
  ];
  let o = ''; let pts = [`${X0},${B}`];
  for (let e = 0; e <= 24; e++) pts.push(`${x(e).toFixed(1)},${(B - 2 - 50 * Math.pow(e / 24, 1.6)).toFixed(1)}`);
  pts.push(`${X1},${B}`);
  o += `<rect x="${x(12)}" y="${B - 118}" width="${X1 - x(12)}" height="118" fill="${COLS[4]}" fill-opacity=".06"/><line x1="${x(12)}" y1="${B - 118}" x2="${x(12)}" y2="${B + 24}" stroke="${COLS[4]}" stroke-dasharray="3 4"/>` + T(x(12) + 6, B - 108, 'threshold', { fs: 10, w: 700, c: COLS[4] }) + T(x(12) + 6, B - 97, 'past here only the field describes the system', { fs: 8, c: COLS[4], op: .85 }) + `<line x1="${X0}" y1="${B}" x2="${X1}" y2="${B}" stroke="currentColor" stroke-opacity=".35"/>`;
  for (let e = 0; e < 24; e += 3) o += T(x(e), B + 16, '10' + sup(e), { fs: 9, a: 'middle', op: .5 });
  o += `<line x1="${x(0)}" y1="${B}" x2="${x(6.5)}" y2="${B}" stroke="${COLS[3]}" stroke-width="3" stroke-linecap="round"/><line x1="${x(6.5)}" y1="${B}" x2="${x(15)}" y2="${B}" stroke="${COLS[4]}" stroke-width="3" stroke-dasharray="1 6" stroke-linecap="round"/>`;
  for (const [n, e, cnt, line, col, side, stem] of L) {
    const xx = x(e), r = 3.5 + e * .42;
    o += `<circle cx="${xx}" cy="${B}" r="${r}" fill="${col}" fill-opacity=".18" stroke="${col}" stroke-width="1.5"/><circle cx="${xx}" cy="${B}" r="2.2" fill="${col}"/>`;
    const y1 = side > 0 ? B - r - 2 : B + r + 2, y2 = side > 0 ? B - r - stem : B + r + stem;
    o += `<line x1="${xx}" y1="${y1}" x2="${xx}" y2="${y2}" stroke="${col}" stroke-opacity=".6"/>`;
    let a = 'middle', tx = xx; if (n === 'self') { a = 'start'; tx = X0 - 2; } if (n === 'Avogadro') { a = 'end'; tx = X1 + 2; }
    const ty = side > 0 ? y2 - 4 : y2 + 12;
    const rows = side > 0 ? [[ty - 24, n, 12.5, 'currentColor', 700], [ty - 11, cnt, 10.5, col, 400], [ty, line, 9, 'currentColor', 400]] : [[ty, n, 12.5, 'currentColor', 700], [ty + 13, cnt, 10.5, col, 400], [ty + 24, line, 9, 'currentColor', 400]];
    for (const [y, t, fs, c, w] of rows) o += T(tx, y, t, { fs, c, w, a, op: fs === 9 ? .65 : 1 });
  }
  return svgo(W, H, 'from self to Avogadro', o);
};

/* a cyberlink, exploded */
const linkFig = () => {
  const W = 760, H = 330; let o = '';
  const box = (x, y, w, h, title, sub, col) => `<rect x="${x}" y="${y}" width="${w}" height="${h}" rx="10" fill="${col}" fill-opacity=".12" stroke="${col}" stroke-width="1.5"/>` + T(x + w / 2, y + 30, title, { fs: 16, w: 700, a: 'middle' }) + T(x + w / 2, y + 50, sub, { fs: 9.5, a: 'middle', op: .65 });
  o += box(40, 110, 200, 70, 'from · particle p', 'hemera(file) · authorizes the move', COLS[1]);
  o += box(520, 110, 200, 70, 'to · particle q', 'hemera(file) · any card', COLS[1]);
  o += `<line x1="240" y1="145" x2="520" y2="145" stroke="currentColor" stroke-width="3" marker-end="url(#ah)"/><defs><marker id="ah" markerWidth="10" markerHeight="10" refX="8" refY="5" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="currentColor"/></marker></defs>`;
  o += T(380, 132, 'cyberlink', { fs: 15, w: 700, a: 'middle' });
  o += T(380, 170, '“p belongs with q”', { fs: 11, a: 'middle', op: .7 });
  const call = (x, y, t, d, up) => `<g class="hov"><rect x="${x - 120}" y="${up ? y - 30 : y + 4}" width="240" height="42" fill="transparent"/><line x1="${x}" y1="${up ? 110 : 180}" x2="${x}" y2="${y}" stroke="currentColor" stroke-opacity=".45" stroke-dasharray="2 4"/>` + T(x, up ? y - 16 : y + 18, t, { fs: 12, w: 700, a: 'middle' }) + T(x, up ? y - 3 : y + 32, d, { fs: 10, a: 'middle', op: .7 }) + '</g>';
  o += call(285, 50, 'structural · link(from, to)', 'a directed edge between two cards', true);
  o += call(500, 50, 'who · when', 'ride on the signal, not the link', true);
  o += call(270, 230, 'economic · box(coin, amount)', 'conviction · weight in A^eff', false);
  o += call(515, 230, 'epistemic · valence v ∈ {−1, 0, +1}', '0 ranks · ±1 bets the truth market', false);
  o += T(380, 300, 'five stored fields, three concepts · append-only: delete = withdraw the box and link again with v = −1 · never erasure', { fs: 10, a: 'middle', op: .6 });
  return svgo(W, H, 'anatomy of a cyberlink', o);
};

/* a signal, exploded: s = (ν, ℓ⃗, Δφ⋆, σ, prev, mc, vdf, step, nonce) */
const signalFig = () => {
  const W = 760, H = 275; let o = '';
  const F = [['ν', 'neuron', 'who speaks'], ['ℓ', 'links', 'the cyberlinks claimed'], ['Δφ⋆', 'shift', 'focus shift asserted, in bits'], ['σ', 'zheng proof', 'covers links, impulse, conviction'], ['prev', 'previous', 'author\'s last signal: a chain'], ['mc', 'merkle clock', 'causal state it was computed on'], ['vdf', 'delay', 'physical time since previous'], ['step', 'step', 'logical clock, monotonic'], ['nonce', 'nonce', 'proof randomness = mining']];
  const w = 76, x0 = 50, y = 110;
  F.forEach(([sym, name, meaning], i) => {
    const x = x0 + i * w, col = COLS[i % 7];
    o += `<g class="hov"><rect x="${x}" y="${y}" width="${w - 4}" height="56" rx="8" fill="${col}" fill-opacity=".14" stroke="${col}"/>` + T(x + (w - 4) / 2, y + 26, sym, { fs: 16, w: 700, a: 'middle', ff: MONO }) + T(x + (w - 4) / 2, y + 44, name, { fs: 9, a: 'middle', op: .7 });
    const lines = []; let cur = ''; for (const wd of meaning.split(' ')) { if ((cur + ' ' + wd).trim().length > 13 && cur) { lines.push(cur); cur = wd; } else cur = (cur + ' ' + wd).trim(); } if (cur) lines.push(cur);
    lines.slice(0, 3).forEach((ln, k) => { o += T(x + (w - 4) / 2, y + 74 + k * 11, ln, { fs: 8, a: 'middle', op: .8 }); }); o += '</g>';
  });
  const bx = x0 + 1 * w, bw = 3 * w - 4; o += `<path d="M${bx},${y - 10} v-8 h${bw} v8" fill="none" stroke="${COLS[3]}" stroke-width="1.5"/>` + T(bx + bw / 2, y - 24, 'σ · one zheng proof covers these', { fs: 10, w: 700, a: 'middle', c: COLS[3] });
  const px = x0 + 4 * w + (w - 4) / 2; o += `<path d="M${px},${y - 10} C ${px},${y - 60} ${x0 - 10},${y - 60} ${x0 - 10},${y + 28}" fill="none" stroke="${COLS[4]}" stroke-width="1.5" stroke-dasharray="3 4" marker-end="url(#ah3)"/><defs><marker id="ah3" markerWidth="8" markerHeight="8" refX="4" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8 z" fill="${COLS[4]}"/></marker></defs>` + T(px + 20, y - 46, 'prev → the author\'s last signal: every neuron is a chain', { fs: 10, w: 700, c: COLS[4] });
  o += T(380, 262, 'the proof is the claim. the claim is the mining. identical signals produce one root everywhere.', { fs: 10, a: 'middle', op: .6 });
  return svgo(W, H, 'anatomy of a signal', o);
};

/* graph → transformer */
const compileFig = () => {
  const W = 760, H = 285; let o = '';
  // graph glyph
  const gx = 110, gy = 150; const pts = [[0, -60], [52, -30], [60, 30], [10, 65], [-50, 40], [-58, -25], [0, 0], [25, 20], [-20, -30]];
  const E = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 0], [6, 0], [6, 2], [6, 4], [7, 2], [7, 3], [8, 5], [8, 6], [8, 1]];
  for (const [a, b] of E) o += `<line x1="${gx + pts[a][0]}" y1="${gy + pts[a][1]}" x2="${gx + pts[b][0]}" y2="${gy + pts[b][1]}" stroke="${COLS[3]}" stroke-opacity=".6"/>`;
  pts.forEach(([x, y], i) => { o += `<circle cx="${gx + x}" cy="${gy + y}" r="${i === 6 ? 7 : 4.5}" fill="${COLS[3]}"/>`; });
  o += T(gx, 250, 'weighted cybergraph', { fs: 12, w: 700, a: 'middle' }) + T(gx, 266, 'A^eff · stake × karma', { fs: 9, a: 'middle', op: .65 });
  // three derivations
  const rows = [[70, 'effective rank of Σ_φ⋆', 'd⋆ = exp H(σ(Σ_φ⋆))', 'embedding dimension'], [150, 'count of dialects', 'h⋆ ≥ |Dialect(G)|', 'attention heads'], [230, 'diameter × contraction', 'L⋆ = diam(G) · ⌈log(1/ε) / log(1/κ)⌉', 'layers']];
  for (const [y, from, formula, to] of rows) {
    o += `<g class="hov"><line x1="195" y1="${y}" x2="250" y2="${y}" stroke="currentColor" stroke-opacity=".4"/><line x1="470" y1="${y}" x2="538" y2="${y}" stroke="currentColor" stroke-opacity=".4" marker-end="url(#ah2)"/>`;
    o += `<rect x="250" y="${y - 24}" width="220" height="48" rx="8" fill="currentColor" fill-opacity=".06" stroke="currentColor" stroke-opacity=".3"/>` + T(360, y - 6, from, { fs: 9, a: 'middle', op: .65 }) + T(360, y + 12, formula, { fs: 11, w: 700, a: 'middle' });
    o += T(544, y + 4, to, { fs: 10, op: .85 }) + '</g>';
  }
  o += `<defs><marker id="ah2" markerWidth="8" markerHeight="8" refX="6" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8 z" fill="currentColor" fill-opacity=".6"/></marker></defs>`;
  // transformer stack glyph
  for (let i = 0; i < 7; i++) o += `<rect x="652" y="${60 + i * 26}" width="64" height="18" rx="3" fill="${COLS[3]}" fill-opacity="${.12 + i * .1}" stroke="${COLS[3]}" stroke-opacity=".7"/>`;
  o += T(684, 48, 'one layer = one kernel step', { fs: 9, w: 700, a: 'middle' });
  o += T(684, 250, 'L⋆ = 290 layers', { fs: 10.5, w: 700, a: 'middle' }) + T(684, 263, '= 290 kernel steps to φ⋆', { fs: 9, a: 'middle', op: .8 }) + T(684, 276, 'compiled, not trained', { fs: 9, a: 'middle', op: .65 });
  return svgo(W, H, 'the graph compiled into a transformer', o);
};


/* earth ↔ mars: finality is an information condition, not a clock */
const planetFig = () => {
  const W = 760, H = 258; let o = '';
  const planet = (x, y, r, col, name, sub) => `<circle cx="${x}" cy="${y}" r="${r}" fill="${col}" fill-opacity=".15" stroke="${col}" stroke-width="1.5"/>` + T(x, y + 4, name, { fs: 13, w: 700, a: 'middle' }) + T(x, y + r + 40, sub, { fs: 9, a: 'middle', op: .7 });
  o += planet(120, 120, 46, COLS[4], 'earth', 'its own pond · finalizes at earth speed');
  o += planet(640, 120, 30, COLS[1], 'mars', 'its own pond · finalizes at mars speed');
  // local gossip rings
  for (let k = 1; k <= 3; k++) o += `<circle cx="120" cy="120" r="${46 + k * 11}" fill="none" stroke="${COLS[4]}" stroke-opacity="${.35 - k * .09}" stroke-dasharray="2 5"/><circle cx="640" cy="120" r="${30 + k * 9}" fill="none" stroke="${COLS[1]}" stroke-opacity="${.35 - k * .09}" stroke-dasharray="2 5"/>`;
  // light path
  o += `<line x1="200" y1="120" x2="580" y2="120" stroke="currentColor" stroke-opacity=".35" stroke-dasharray="6 6"/>`;
  o += `<circle cx="300" cy="120" r="3" fill="#fff"><animate attributeName="cx" values="200;580" dur="6s" repeatCount="indefinite"/></circle>`;
  o += T(390, 104, '4 – 24 minutes of light, one way', { fs: 11, w: 700, a: 'middle' });
  o += T(390, 140, 'a signal pays light-time once · a vote pays it twice, every round', { fs: 9.5, a: 'middle', op: .75 });
  // rule box
  o += `<rect x="250" y="190" width="280" height="54" rx="10" fill="currentColor" fill-opacity=".06" stroke="currentColor" stroke-opacity=".3"/>`;
  o += T(390, 210, 'final ⇔ φ⋆ᵢ > τ', { fs: 14, w: 700, a: 'middle', ff: MONO });
  o += T(390, 230, 'enough attention has gathered · an information condition, not a clock', { fs: 9, a: 'middle', op: .75 });
  o += T(120, 22, 'identical signals → one root everywhere', { fs: 9.5, a: 'middle', op: .7 });
  o += T(745, 22, 'a dark link freezes trade, never invents two truths', { fs: 9.5, a: 'end', op: .7 });
  return svgo(W, H, 'earth and mars: finality survives the speed of light', o);
};

/* surprising syntropy: two filters, one quadrant pays */
const syntropyFig = () => {
  const W = 760, H = 290; let o = '';
  const x0 = 170, y0 = 40, w = 430, h = 190;
  const cell = (cx, cy, col, title, sub, pay) => `<rect x="${cx}" y="${cy}" width="${w / 2 - 6}" height="${h / 2 - 6}" rx="10" fill="${col}" fill-opacity="${pay ? .22 : .06}" stroke="${col}" stroke-opacity="${pay ? 1 : .35}" stroke-width="${pay ? 2 : 1}"/>` + T(cx + w / 4 - 3, cy + 34, title, { fs: 13, w: 700, a: 'middle' }) + T(cx + w / 4 - 3, cy + 54, sub, { fs: 9.5, a: 'middle', op: .75 }) + T(cx + w / 4 - 3, cy + 76, pay ? 'paid: R(ν) = Shapley_ν(v⋆)' : 'mints 0', { fs: 11, w: 700, a: 'middle', c: pay ? COLS[3] : 'currentColor', ff: MONO });
  o += cell(x0, y0, COLS[3], 'surprising syntropy', 'order the crowd did not expect', true);
  o += cell(x0 + w / 2 + 6, y0, 'currentColor', 'expected order', 'a copy · what everyone saw coming', false);
  o += cell(x0, y0 + h / 2 + 6, 'currentColor', 'surprising noise', 'novel, and the picture got no clearer', false);
  o += cell(x0 + w / 2 + 6, y0 + h / 2 + 6, 'currentColor', 'noise', 'idle capital · volume · presence', false);
  // axes
  o += T(x0 - 14, y0 + h / 4, 'Δφ⁺ > 0', { fs: 11, w: 700, a: 'end', ff: MONO }) + T(x0 - 14, y0 + h / 4 + 14, 'picture clearer', { fs: 9, a: 'end', op: .7 });
  o += T(x0 - 14, y0 + 3 * h / 4, 'Δφ⁺ = 0', { fs: 11, w: 700, a: 'end', ff: MONO }) + T(x0 - 14, y0 + 3 * h / 4 + 14, 'no new order', { fs: 9, a: 'end', op: .7 });
  o += T(x0 + w / 4, y0 + h + 22, 'ρ high · surprising', { fs: 11, w: 700, a: 'middle', ff: MONO }) + T(x0 + w / 4, y0 + h + 36, 'the crowd did not predict it', { fs: 9, a: 'middle', op: .7 });
  o += T(x0 + 3 * w / 4, y0 + h + 22, 'ρ low · expected', { fs: 11, w: 700, a: 'middle', ff: MONO }) + T(x0 + 3 * w / 4, y0 + h + 36, 'the crowd predicted it', { fs: 9, a: 'middle', op: .7 });
  o += T(W / 2, H - 6, 'syntropy J(φ⋆) = D_KL(φ⋆ ‖ uniform): bits of distance from noise · surprise ρ: the serum, your claim against the crowd\'s prediction of itself', { fs: 9, a: 'middle', op: .6 });
  return svgo(W, H, 'surprising syntropy: two filters', o);
};


/* the tri-kernel: three local maps blended into one contraction */
const triFig = () => {
  const W = 760, H = 230; let o = '';
  const maps = [['diffusion', 'D', 'a random walk along links', COLS[2]], ['springs', 'S', 'linked particles pull together', COLS[3]], ['heat', 'H', 'e^{−τL} · smoothing', COLS[1]]];
  maps.forEach(([n, sym, d, col], k) => {
    const cx = 110 + k * 200, cy = 90;
    o += `<circle cx="${cx}" cy="${cy}" r="52" fill="${col}" fill-opacity=".1" stroke="${col}" stroke-width="1.5"/>`;
    const pts = [[0, -30], [28, -8], [18, 26], [-18, 26], [-28, -8]];
    pts.forEach(([x, y], i) => { const [x2, y2] = pts[(i + 1) % 5]; o += `<line x1="${cx + x}" y1="${cy + y}" x2="${cx + x2}" y2="${cy + y2}" stroke="${col}" stroke-opacity=".7"/>`; });
    if (k === 0) pts.forEach(([x, y], i) => { o += `<line x1="${cx + x}" y1="${cy + y}" x2="${cx + x * .3}" y2="${cy + y * .3}" stroke="${col}" stroke-opacity=".5" marker-end="url(#ah4)"><animate attributeName="stroke-opacity" values=".2;.9;.2" dur="1.6s" begin="${i * .3}s" repeatCount="indefinite"/></line>`; });
    if (k === 1) pts.forEach(([x, y], i) => { const [x2, y2] = pts[(i + 2) % 5]; o += `<line x1="${cx + x}" y1="${cy + y}" x2="${cx + x2}" y2="${cy + y2}" stroke="${col}" stroke-opacity=".35" stroke-dasharray="2 3"><animate attributeName="stroke-dashoffset" values="0;-10" dur="1s" repeatCount="indefinite"/></line>`; });
    if (k === 2) for (let r = 10; r <= 40; r += 10) o += `<circle cx="${cx}" cy="${cy}" r="${r}" fill="none" stroke="${col}" stroke-opacity="${.5 - r / 100}"><animate attributeName="r" values="${r};${r + 10}" dur="2.4s" repeatCount="indefinite"/><animate attributeName="stroke-opacity" values="${.5 - r / 100};0" dur="2.4s" repeatCount="indefinite"/></circle>`;
    pts.forEach(([x, y]) => { o += `<circle cx="${cx + x}" cy="${cy + y}" r="3.5" fill="${col}"/>`; });
    o += T(cx, cy + 74, `${n} · ${sym}`, { fs: 13, w: 700, a: 'middle' }) + T(cx, cy + 90, d, { fs: 9, a: 'middle', op: .7 });
    if (k < 2) o += T(cx + 100, cy + 5, '+', { fs: 22, w: 700, a: 'middle', op: .6 });
  });
  o += `<defs><marker id="ah4" markerWidth="6" markerHeight="6" refX="5" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 z" fill="${COLS[2]}" fill-opacity=".7"/></marker></defs>`;
  o += T(380, 205, 'φ ← λ_d·D(φ) + λ_s·S(φ) + λ_h·H(φ)      κ = λ_d·α + λ_s·‖L‖/(‖L‖+μ) + λ_h·e^{−τλ₂} < 1', { fs: 11, w: 700, a: 'middle', ff: MONO });
  o += T(380, 222, 'three local operators, one blend; each step moves at most κ of the way — so the iteration must stop somewhere, and only in one place', { fs: 9, a: 'middle', op: .65 });
  return svgo(W, H, 'the tri-kernel', o);
};

/* ------------------------------------------------------------------ the cybergraph renderer (real data when graph.json is present) */
const cyberGraph = (() => {
  let data = null, cv, ctx, raf = 0, t0 = 0, W = 0, H = 0, mx = -1e9, my = -1e9, adj = null, lastHov = -1;
  const synth = () => { const n = 900, nodes = [], links = []; for (let i = 0; i < n; i++) nodes.push([`synth${i}`, 1, Math.random(), Math.random()]); for (let i = 1; i < n; i++) { const j = Math.floor(Math.pow(Math.random(), 2.2) * i); links.push([i, j]); nodes[j][1]++; nodes[i][1]++; } return { meta: { source: 'synthetic preferential attachment (graph.json not found)', particles_total: n, links_total: links.length, sampled_particles: n, sampled_links: links.length, synthetic: true }, nodes, links }; };
  const load = async () => { try { const r = await fetch('graph.json'); if (!r.ok) throw 0; data = await r.json(); } catch { data = synth(); } prep(); };
  let maxd = 1, cx = 0, cy = 0;
  let top5 = new Set();
  const prep = () => { adj = data.nodes.map(() => []); for (const [a, b] of data.links) { adj[a].push(b); adj[b].push(a); } maxd = Math.max(...data.nodes.map((n) => n[1])) || 1; top5 = new Set(data.nodes.map((n, i) => [n[1], i]).sort((a, b) => b[0] - a[0]).slice(0, 5).map((x) => x[1])); const xs = data.nodes.map((n) => n[2]), ys = data.nodes.map((n) => n[3]); cx = (Math.min(...xs) + Math.max(...xs)) / 2; cy = (Math.min(...ys) + Math.max(...ys)) / 2; const m = $('#cg-meta'); if (m) m.innerHTML = data.meta.synthetic ? `${M('k')} synthetic placeholder — the real sample is being extracted` : `sample: ${fmt(data.meta.sampled_particles)} particles · ${fmt(data.meta.sampled_links)} cyberlinks · 400 hubs + 800 particles linked into them · outer ring = sampled particles with no in-sample link · hover a particle ${M('m')}`; };
  const draw = () => {
    if (!cv || !data) return;
    const dpr = devicePixelRatio; W = cv.width = cv.clientWidth * dpr; H = cv.height = cv.clientHeight * dpr;
    const t = (performance.now() - t0) / 1000, ang = t * .04, s = Math.min(W, H) * .56, zoom = (1 + .05 * Math.sin(t * .4)) * userZoom;
    const P = data.nodes.map((n) => { const x = (n[2] - cx) * zoom, y = (n[3] - cy) * zoom; const c = Math.cos(ang), sn = Math.sin(ang); return [W / 2 + (x * c - y * sn) * 2 * s, H / 2 + (x * sn + y * c) * 2 * s * .92]; });
    let hov = -1, hd = 18 * dpr; P.forEach((p, i) => { const d = Math.hypot(p[0] - mx, p[1] - my); if (d < hd) { hd = d; hov = i; } }); lastHov = hov; if (hov < 0 && pinned >= 0) hov = pinned;
    ctx.clearRect(0, 0, W, H);
    ctx.lineWidth = .6 * dpr; ctx.strokeStyle = COLS[0]; ctx.globalAlpha = hov < 0 ? .22 : .08;
    ctx.beginPath(); for (const [a, b] of data.links) { ctx.moveTo(P[a][0], P[a][1]); ctx.lineTo(P[b][0], P[b][1]); } ctx.stroke();
    const nb = hov < 0 ? null : new Set(adj[hov]);
    if (nb) { ctx.lineWidth = 1 * dpr; ctx.strokeStyle = '#fff'; ctx.globalAlpha = .85; ctx.beginPath(); for (const j of nb) { ctx.moveTo(P[hov][0], P[hov][1]); ctx.lineTo(P[j][0], P[j][1]); } ctx.stroke(); }
    data.nodes.forEach((n, i) => { const d = n[1] / maxd, r = (1 + 9 * Math.sqrt(d)) * dpr; const lit = nb && (nb.has(i) || i === hov); ctx.globalAlpha = lit ? 1 : (nb ? .25 : .55 + .45 * d); ctx.fillStyle = lit ? '#fff' : d > .35 ? '#fff' : COLS[0]; ctx.beginPath(); ctx.arc(P[i][0], P[i][1], r, 0, Math.PI * 2); ctx.fill(); });
    const labs = [...top5].map((i) => [i, P[i][1]]).sort((a, b) => a[1] - b[1]); for (let k = 1; k < labs.length; k++) if (labs[k][1] - labs[k - 1][1] < 13 * dpr) labs[k][1] = labs[k - 1][1] + 13 * dpr;
    ctx.globalAlpha = .9; ctx.fillStyle = '#fff'; ctx.font = `${10 * dpr}px ${MONO}`; for (const [i, ly] of labs) { const r = (1 + 9 * Math.sqrt(data.nodes[i][1] / maxd)) * dpr; ctx.fillText(`${fmt(data.nodes[i][1])} links`, P[i][0] + r + 3 * dpr, ly + 3 * dpr); }
    if (hov >= 0) { const n = data.nodes[hov]; ctx.font = `${11 * dpr}px ${MONO}`; ctx.fillStyle = '#fff'; ctx.globalAlpha = 1; ctx.fillText(`${n[0]} · ${fmt(n[1])} links on chain · ${adj[hov].length} in sample`, 12 * dpr, 18 * dpr); }
    ctx.globalAlpha = 1; raf = requestAnimationFrame(draw);
  };
  let userZoom = 1, pinned = -1;
  const onMove = (e) => { const r = cv.getBoundingClientRect(); mx = (e.clientX - r.left) * devicePixelRatio; my = (e.clientY - r.top) * devicePixelRatio; };
  const onWheel = (e) => { e.preventDefault(); e.stopPropagation(); userZoom = Math.min(4, Math.max(.6, userZoom * (e.deltaY > 0 ? .92 : 1.08))); };
  const run = (init) => { let phi = null, lastD = 0; return { step() { if (!data) return null; if (!phi) { phi = data.nodes.map(() => init === 'random' ? Math.random() : 1); const z = phi.reduce((a, b) => a + b, 0); phi = phi.map((v) => v / z); } const next = phi.map((v, i) => { const nb = adj[i]; if (!nb.length) return .15 / data.nodes.length; let sum = 0; for (const j of nb) sum += phi[j] / Math.max(1, adj[j].length); return .15 / data.nodes.length + .85 * sum; }); let d = 0; for (let i = 0; i < phi.length; i++) d += Math.abs(next[i] - phi[i]); phi = next; const kap = lastD ? d / lastD : null; lastD = d; return { d, kap }; }, get phi() { return phi; }, reset() { phi = null; lastD = 0; } }; };
  const runA = run('uniform'); const iterate = () => runA.step();
  return { iterate, run, load() { if (!data) load(); }, start() { cv = $('#cg'); if (!cv) return; ctx = cv.getContext('2d'); cv.onmousemove = onMove; cv.onmouseleave = () => { mx = my = -1e9; }; cv.onwheel = onWheel; cv.onclick = () => { pinned = pinned === lastHov ? -1 : lastHov; sfx.tick(); }; t0 = t0 || performance.now(); if (!data) load().then(() => { if (!raf) draw(); }); else if (!raf) draw(); }, stop() { cancelAnimationFrame(raf); raf = 0; } };
})();

/* ------------------------------------------------------------------ slides */
const S = [
/* 01 · superintelligence */
{ ch: 0, cls: 'one', html: `
  <div><p class="kick">cyber · a frozen protocol for collective lifeforms</p>
  <p class="hero-word">cyber</p>
  <h2>the <em>superintelligence.</em></h2>
  <p class="lead">one focus — a single distribution over everything known — computed from every mind that pays to teach, proven to see what no member sees alone, final without a vote, so light-delay cannot split it. the graph is the training data, the weights are public, the money is minted when the network learns.</p>
  <div class="tiles"><div class="tile"><b>one focus · φ⋆</b>exists, unique, same on every node</div><div class="tile"><b>compiled, not trained</b>bostrom → transformer in 62 s · d⋆ = 31</div><div class="tile"><b>frozen foundations</b>one field · one hash · one proof</div><div class="tile"><b>a network of moneys</b>CYB is the root token</div></div>
  <p class="kbd" style="margin-top:24px">→ or space to move · you will mine as you read</p></div>` },
{ ch: 0, cls: 'ink', html: `
  <div><p class="kick">01 · superintelligence · the definition</p><h2>superintelligence<br><em>is a number.</em></h2>
  <p class="lead">run the same engine on one neuron's private view, then on the whole cybergraph, and score both on prediction. the gap is superadditivity. on benchmark it is positive against the strongest participant at every connectivity level. emergence stops being philosophy the moment it has an error bar.</p>
  <p class="mute">its existence and uniqueness is the collective focus theorem: the tri-kernel is a contraction, so φ⋆ is a fixed point, identical on every node, computable from local neighbourhoods. nobody votes on importance. no individual ant understands the colony; the colony understands itself.</p></div>
  <div class="nums"><div class="n"><big>κ &lt; 1</big><small>the tri-kernel contracts · Banach for the blend, Perron–Frobenius for the walk · a theorem</small></div><div class="n"><big>σ<span class="f">_best</span> &gt; 0</big><small>the whole graph out-predicts its strongest neuron at every connectivity level · 34-node benchmark, first evidence ${M('m')}</small></div><div class="n"><big>1</big><small>fixed point φ⋆ · the same object ranks the graph, prices a link and compiles the model</small></div><div class="n"><big>0</big><small>votes, committees, operators between a claim and its rank</small></div></div>` },
{ ch: 0, cls: 'one', bridge: 'a graph like this accepts exactly one kind of statement. that is the anatomy.', html: `
  <div><p class="kick">01 · superintelligence · the cybergraph · bostrom, sealed 2026-08-05</p>
  <h2>this is the <em>cybergraph.</em></h2>
  <div class="cgwrap"><canvas id="cg"></canvas><div class="cgcap"><span id="cg-meta">loading…</span><span>3,143,650 particles · 2,949,732 cyberlinks · 61,675 neurons · 1,735 days ${M('m')}</span></div></div>
  <p class="mute">every dot is a content-addressed file; every line a signed, staked claim that two of them belong together. no editor, no schema, no operator. the successor graph is born from this one on 2026-11-05.</p></div>` },

/* 02 · anatomy */
{ ch: 1, cls: 'one wash', html: `
  <div><p class="kick">02 · anatomy · the cyberlink</p><h2>the only statement<br><em>the graph accepts.</em></h2>
  ${linkFig()}
  <p class="mute">a file is bytes; its hemera hash is its particle. a cyberlink is a claim that two particles belong together, with a box of tokens behind it and a vote on where its truth market converges. there is no type field: a label is itself a cyberlink onto the edge, so relation types are ranked like everything else. the cyberlink is public; the author is private and rides on the signal.</p></div>` },
{ ch: 1, cls: 'one wash', html: `
  <div><p class="kick">02 · anatomy · the signal</p><h2>a signal is a message<br><em>that carries its own proof.</em></h2>
  ${signalFig()}
  <p class="mute"><span class="f">s = (ν, ℓ, Δφ⋆, σ, prev, mc, vdf, step, nonce)</span>. who, what, how much the picture sharpened — then one zheng proof that covers all of it, the author's previous signal, the causal state it was computed against, physical time, logical time, and a nonce that touches nothing but the proof's randomness. any verifier runs decide(σ) in O(log n). this is the unit of gossip, of settlement, and of mining.</p></div>` },
{ ch: 1, cls: 'ink', bridge: 'statements pile up. what makes one picture out of millions of them? the kernel.', html: `
  <div><p class="kick">02 · anatomy · do it</p><h2>link two things.<br><em>you just grew the graph.</em></h2>
  <p class="lead">the kernel reruns with your link and without it; the downhill difference in φ⋆ is yours. a perfect copy moves focus by zero and surprises no one: copying earns nothing.</p></div>
  <div class="panel"><label>this</label><input type="text" id="link-a" placeholder="a thing you know" value="proof of work"><div class="chips" id="chips-a"></div><label>belongs with</label><input type="text" id="link-b" placeholder="what it belongs with" value="Shapley sampling"><div class="chips" id="chips-b"></div><div class="row"><button class="btn" id="link-go">link · stake it</button><span class="fine">signed by a throwaway neuron in your browser ${M('k')}</span></div><div class="out" id="link-out">your links land in the graph behind this deck.</div></div>` },

/* 03 · focus */
{ ch: 2, cls: 'one wash', html: `
  <div><p class="kick">03 · focus · the tri-kernel · here the magic starts</p><h2>three local maps,<br><em>one contraction.</em></h2>
  ${triFig()}
  <div class="panel" style="margin-top:10px"><label>live · ‖Δφ‖ per step on the real cybergraph sample · each bar is one iteration</label><div class="cbars" id="kernel-chart"></div><div class="row"><button class="btn" id="kernel-run">iterate ×12</button><span class="hash" id="kernel-delta" style="margin:0">‖Δφ‖ —</span><span class="count" id="kernel-iter" style="font-size:28px;margin-left:auto">0</span></div><p class="fine">the bars shrink by the same ratio every step: that ratio is κ̂, measured here on 1,200 real bostrom particles; κ &lt; 1 is proven for the real kernel ${M('k')}</p></div></div>` },
{ ch: 2, cls: 'ink', html: `
  <div><p class="kick">03 · focus · the fixed point</p><h2>any start.<br><em>the same end.</em></h2>
  <p class="lead">a contraction has exactly one fixed point (Banach), and on a token-weighted graph that point is a positive distribution (Perron–Frobenius). call it φ⋆: collective focus. it is not an opinion about importance — it is the only place the iteration can stop.</p>
  <p class="mute">three consequences. it is the same number on every node, so nobody votes. it is computable from an O(log 1/ε)-hop neighbourhood, so a phone computes its own slice. and it takes a constant number of steps to reach, so the whole computation can be proven — φ⋆ is what the mint pays against and what the compiler reads.</p></div>
  <div class="panel"><label>two strangers, one focus · two random starts on the real sample</label><div class="cbars" id="unique-chart"></div><div class="row"><button class="btn" id="unique-run">run both</button><span class="hash" id="unique-out" style="margin:0">distance between the two runs —</span></div><p class="fine">bars = ‖φ_A − φ_B‖₁ per step. from unrelated starting points the two iterations meet to within 10⁻⁶: uniqueness, demonstrated, not asserted ${M('k')}</p></div>` },
{ ch: 2, cls: 'one wash', bridge: 'one focus, on every world. a focus is a distribution — and a distribution can be compiled.', html: `
  <div><p class="kick">03 · focus · interplanetary</p><h2>one mind <em>across worlds.</em></h2>
  ${planetFig()}
  <p class="mute">type I is a thinking planet; type II is a thinking solar system. earth to mars is 4–24 minutes one way, so every protocol that collects votes, elects leaders or waits for two thirds of a global committee is already dead at that distance: the medium is not slow software, the medium is physics. here a fact is final when <span class="f">φ⋆ᵢ &gt; τ</span> — enough attention has gathered — so agreement costs computation and local connectivity, and only propagation pays light-time. each planet is its own pond; shared facts eventually agree; a dark link stops cross-world trade and never invents two truths. designing for planets removes every global round, and the removal is what makes it work well on one.</p></div>` },

/* 04 · compile */
{ ch: 3, cls: 'one wash', html: `
  <div><p class="kick">04 · compile · the pitch</p><h2>we compile the graph<br><em>into a transformer.</em></h2>
  ${compileFig()}
  <p class="mute">attention is one step of a convergent dynamical system — the same system that computes φ⋆. so a transformer's three free parameters are not design choices once a weighted graph exists: they are read off its covariance, its dialects and its diameter. the weights are compiled from structure, not learned from text prediction.</p></div>` },
{ ch: 3, cls: 'ink', html: `
  <div><p class="kick">04 · compile · the bootloader corpus, compiled</p><h2>62 seconds.<br><em>one machine. the whole chain's mind.</em></h2>
  <p class="lead">the final bostrom snapshot — 2,949,732 cyberlinks over 3,143,650 particles — compiled into a transformer on one machine with 20 GB of RAM. every weight traces to the cyberlinks and the neurons that signed them: given any output, its authors are a lookup.</p></div>
  <div class="nums"><div class="n"><big>31</big><small>d⋆ · embedding dimension = exp H(σ(Σ_φ⋆)) ${M('m')}</small></div><div class="n"><big>≥ 12</big><small>h⋆ · attention heads · dialect lower bound ${M('m')}</small></div><div class="n"><big>290</big><small>L⋆ · layers = diam 10 × 29 iterations per hop ${M('m')}</small></div><div class="n"><big>~0.4<span class="u">M</span></big><small>parameters at current graph scale ${M('m')}</small></div><div class="n"><big>~62<span class="u">s</span></big><small>compilation · one machine · 20 GB RAM ${M('m')}</small></div><div class="n"><big>10³–10⁴</big><small>d⋆ at planetary scale · read off the saturated semantic space ${M('t')}</small></div></div>` },
{ ch: 3, bridge: 'a model worth having is worth paying for. who is paid, and for what? order.', html: `
  <div><p class="kick">04 · compile · a public mind</p><h2>the model is<br><em>a public object.</em></h2>
  <p class="lead">the weights are public tokens, the training data is an open graph, and the inference runs in consensus with proofs. verify once, trust forever. as the graph grows, each recompilation produces a structurally better model from the same pipeline — |E| raises d⋆, λ₂ lowers L⋆, dialects raise h⋆.</p></div>
  <div class="steps"><div class="step"><b>explainable</b><span>any answer resolves to the links and the neurons that produced it — attribution is a lookup, not interpretability research</span></div><div class="step"><b>aligned, as a number</b><span>human and machine neurons stake into the same graph; their divergence is a KL distance between two focus distributions, on a dashboard</span></div><div class="step"><b>paid teachers</b><span>the people whose links shaped the weights are the people the mint pays</span></div><div class="step"><b>self-improving by structure</b><span>no training run; a better graph compiles a better model</span></div></div>` },

/* 05 · syntropy */
{ ch: 4, cls: 'one wash', html: `
  <div><p class="kick">05 · syntropy · the law</p><h2>the mint pays for<br><em>surprising syntropy.</em></h2>
  ${syntropyFig()}
  <p class="mute">syntropy is order: how far collective focus has travelled from noise, in bits — the same quantity the compiler reads when it turns the graph into a model. surprise is the serum: your claim scored against the crowd's prediction of itself. two filters, one quadrant pays. <span class="f">v⋆(S) = Δφ⁺(A^eff ∪ ρ·S)</span>: a coalition's value is the proven focus shift it causes, weighted by how unexpected it was; your reward is your Shapley share of it. a copy moves focus by zero. a popular truth surprises no one. idle capital, volume and presence mint exactly nothing.</p></div>` },
{ ch: 4, html: `
  <div><p class="kick">05 · syntropy · do it</p><h2>mine a settlement.<br><em>the hash is the accounting.</em></h2>
  <p class="lead">no synthetic puzzle. the nonce you grind is the index of one random ordering of contributors; the marginal it produces is one Shapley sample. securing the chain and computing who deserves credit are one act.</p>
  <p class="mute">four questions, one computation read four times: the state is the fixed point; a contribution is a marginal of the same iteration; fair division samples that marginal over random orderings; honesty is a KL divergence the iteration already holds. nothing is bolted on — no committee, no separate oracle, no gas model.</p></div>
  <div class="panel"><label>cluster · 4 contributors (you are one)</label><div class="hash" id="mine-hash">press sample</div><div class="bars" id="mine-bars"></div><div class="tickets" id="mine-tickets"></div><div class="row"><button class="btn" id="mine-go">sample ×100</button><button class="btn ghost" id="mine-auto">auto</button><span class="fine" id="mine-stat">0 samples · 0 tickets ${M('k')}</span></div><div class="out" id="mine-out">a ticket is a sample whose hash clears the difficulty. each ticket carries its own proof and folds into one accumulator per cluster.</div></div>` },
{ ch: 4, cls: 'ink', bridge: 'a mint this precise must never move under you. freeze it.', html: `
  <div><p class="kick">05 · syntropy · what inflation buys</p><h2>truth. fairness.<br><em>order.</em></h2>
  <p class="lead">three kinds of work no single party can fake for the others. one budget pays all three; the split between work and stake is read off the chain — a thermostat, not a calendar.</p>
  <div class="steps"><div class="step"><b>stakers → truth</b><span>the serum pays the surprising answer, charges the predictable one</span></div><div class="step"><b>settlement miners → fairness</b><span>every hash is a Shapley sample; credit is computed, not bargained</span></div><div class="step"><b>folders and the beacon → order in time</b><span>one history, one proof, one clock</span></div></div></div>
  <div class="panel"><label>active staking ratio · r = <span id="r-val">35</span>%</label><input type="range" id="r-range" min="0" max="100" value="35"><div class="split"><i class="w" id="split-w" style="width:65%"></i><i class="s" id="split-s" style="width:35%"></i></div><div class="split-l"><span>work · settlement miners</span><span>stake · active risk</span></div><div class="out" id="r-out"></div><p class="fine">emission goes to work and risk only. idle capital buys rank and nothing else, so wealth cannot compound by sitting ${M('k')}</p></div>` },

/* 06 · frozen */
{ ch: 5, cls: 'ink', html: `
  <div><p class="kick">06 · frozen · i · the arithmetic</p><h2>frozen.<br><em>a protocol does not upgrade.</em></h2>
  <p class="lead">one field, one hash, one proof path, through every layer. the base is small enough to read, specified three times — in prose, in trident, in rust — and cross-verified. what changes is the graph; what never changes is the arithmetic the graph is judged by. three things never move: the arithmetic, the will, the secrecy.</p></div>
  <div class="steps"><div class="step"><b>kelvin versioning · nox at 0 K</b><span>every layer has a temperature that only drops; a frozen layer takes no proposals. nox: 18 instructions, one field, one hash, one reduction — frozen forever. a program written today runs and proves the same way in fifty years</span></div><div class="step"><b>one field · deterministic · no halting problem</b><span>Goldilocks p = 2⁶⁴ − 2³² + 1 for hashing, proving, the kernel and the mint. loops bounded, no recursion, no heap, no floats, no clocks: byte-identical results on any two machines — bostrom's 2,949,732 links rebuilt from events, identical ${M('m')}</span></div><div class="step"><b>compact · 275 thousand lines</b><span>against 3.2 million lines before LLVM and 39 million with it ${M('m')}. understanding comes first; the freeze is what you earn after</span></div><div class="step"><b>written three times</b><span>every primitive in prose, in rust, in trident on nox — and the three must agree; a disagreement is a bug whichever side is wrong. the compiler proving it compiled itself: target ${M('t')}</span></div></div>` },
{ ch: 5, html: `
  <div><p class="kick">06 · frozen · ii · the will</p><h2>the protocol is<br><em>itself a neuron.</em></h2>
  <p class="lead">a key, a balance, will, karma. its treasury address is derived from the genesis block — no private key held by any party. twelve tunables, three metabolic signals, one deterministic optimization loop: it retunes itself from the graph's own φ⋆ and every node agrees on the tuning. no founder steers it alone — what makes this a superintelligence and not another ranked ledger.</p>
  <p class="mute">below: a claim about the protocol itself. stake on it. any stake moves rank; only a bet with valence — right or wrong, at risk — earns from the truth market. passive capital buys rank and nothing else.</p></div>
  <div class="panel"><label>claim</label><div id="stake-claim" style="font-size:15px;line-height:1.5">“proof of work” belongs with “Shapley sampling”</div><label>valence</label><div class="dial" id="stake-dial"><button data-v="-1">−1 · wrong</button><button data-v="0" class="on">0 · passive</button><button data-v="1">+1 · right</button></div><label>amount · <span id="stake-amt">10</span> tokens</label><input type="range" id="stake-range" min="1" max="100" value="10"><div class="row"><button class="btn" id="stake-go">stake</button><span class="fine" id="stake-hint"></span></div><div class="out" id="stake-out"></div></div>` },
{ ch: 5, cls: 'ink', bridge: 'frozen rules, a living economy: the money of information, and what it buys.', html: `
  <div><p class="kick">06 · frozen · iii · the secrecy</p><h2>speak<br><em>without fear.</em></h2>
  <p class="lead">no one knows who said what, and yet everyone knows what everyone said. three invariants, all at genesis.</p></div>
  <div class="steps"><div class="step"><b>the cyberlink is public, the author is private</b><span>the network sees that p links q and the aggregates φ⋆ runs on; never which neuron signed</span></div><div class="step"><b>balances and transfers are private</b><span>stealth addresses and veil</span></div><div class="step"><b>settlement reveals nothing</b><span>a miner's share leaks nothing of its neighbourhood beyond public aggregates</span></div><div class="step"><b>the third leg of truth</b><span>privacy removes the coordination channel through which a false consensus is manufactured</span></div></div>` },

/* 07 · network of moneys */
{ ch: 6, cls: 'ink', html: `
  <div><p class="kick">07 · the money of information</p><h2>$CYB.<br><em>the currency of cyberia.</em></h2>
  <p class="lead">the money of information: born when the network learns — a unit exists because a cyberlink shifted collective focus and the shift was proven — and spent on access to it: inference from the compiled model, storage, bandwidth, on open markets. intelligence has a price; this is the unit it is priced in.</p>
  <p class="mute">cyber is a network of moneys and CYB is the root. every other token is born as its own household chain, registered here by name, its value never leaving home; trading is conditions under proofs, so the bridge exploit class is made unexpressible.</p></div>
  <div class="nums"><div class="n"><big>Δφ⁺</big><small>what mints it · proven focus shift, nothing else</small></div><div class="n"><big>inference</big><small>what it buys · answers from the compiled mind, storage, bandwidth</small></div><div class="n"><big>18,446,744</big><small>cap · p / 10¹² · arithmetic, not a vote ${M('t')}</small></div><div class="n"><big>187,416</big><small>genesis · ≈ 1% · prior stake continues ${M('t')}</small></div><div class="n"><big>1 · ∞</big><small>one root · every token its own household chain · proposal ${M('t')}</small></div><div class="n"><big>0</big><small>bridges · value never leaves its home chain ${M('t')}</small></div></div>` },
{ ch: 6, cls: 'one wash', html: `
  <div><p class="kick">07 · the money of information · growth</p><h2>a graph, like a gas, <em>crosses a threshold.</em></h2>
  <p class="mute">with the incentive in place the graph grows, and past a threshold individual links stop mattering: only the field describes the system — the graph's own Avogadro number. five years in, nine orders of magnitude early. nobody is late. on 2026-11-05, five years to the day after the bostrom genesis, $BOOT and $PUSSY relaunch as the first two households in the network — same keys, same balances, the burial snapshots as genesis. CYB, the root, follows in phase 3.</p>
  ${scaleFig()}<div class="row" style="justify-content:space-between"><span class="fine">self · crystal · bootloader ${M('m')} measured — cognition · thermodynamic · planetary ${M('t')} derived — Avogadro: a physical constant</span><span class="count-down" id="cd" style="font-size:22px">—</span></div><p class="fine">countdown to 2026-11-05 13:22:42 UTC — the bostrom genesis block, to the second</p></div>` },
{ ch: 6, cls: 'ink', html: `
  <div><p class="kick">07 · the money of information · you</p><h2>you read the deck.<br><em>you also mined.</em></h2>
  <div class="neuron" id="neuron-card"></div><div class="tally"><div><b id="t-links">0</b>links in the graph<a href="#5" data-k="links">↩ link something</a></div><div><b id="t-samples">0</b>settlement samples<a href="#13" data-k="samples">↩ mine</a></div><div><b id="t-stake">0</b>tokens at risk<a href="#16" data-k="stake">↩ stake</a></div><div><b id="t-invites">0</b>mentor links<a href="#20" data-k="invites">↩ bring a mind</a></div></div>
  <p class="lead">everything here ran in your browser against a toy graph. on 2026-11-05 the same acts run on bostrom and space pussy, with proofs, for real — and every link you make there is a weight in the next compile.</p>
  <div class="ctas"><a class="btn" href="https://cyb.ai" target="_blank" rel="noopener">get the robot</a><a class="btn ghost" href="https://cyber.page/litepaper/">read the litepaper</a><a class="btn ghost" href="https://cyber.page/whitepaper/">the whitepaper</a><a class="btn ghost" href="https://cyber.page/launch/">the launch tracker</a><a class="btn ghost" href="#0" id="restart">start again</a></div>
  <p class="fine" style="margin-top:18px">${M('m')} measured · ${M('t')} target · ${M('k')} mock — every number marked. <a href="https://cyber.page/audit/game-theory/">the game-theory audit</a> · <a href="https://cyber.page/physical-analogies/">the physics audit</a> · <a href="https://soft3.org">soft3.org</a></p></div>
  <div class="panel"><label>bring a mind · your name</label><input type="text" id="ref-name" placeholder="name" value=""><div class="row"><button class="btn" id="ref-go">make my link</button><span class="fine">mentor's share · 10% of the student's token at birth + fees for life · proposed ${M('t')}</span></div><div class="out" id="ref-out">every neuron that joins through your link is your student. teaching is paid here, for once.</div></div>` },
];


/* ------------------------------------------------------------------ sound · synthesized, no files */
const sfx = (() => {
  let ac = null, master = null, on = localStorage.getItem('cyber.sound') !== 'off', amb = null;
  const ensure = () => { if (ac) return true; try { ac = new (window.AudioContext || window.webkitAudioContext)(); master = ac.createGain(); master.gain.value = .22; master.connect(ac.destination); } catch { return false; } return true; };
  const resume = () => { if (ac && ac.state === 'suspended') ac.resume(); };
  const tone = (f0, f1, dur, type = 'sine', g = .5, at = 0) => { if (!on || !ensure()) return; resume(); const t = ac.currentTime + at; const o = ac.createOscillator(), v = ac.createGain(); o.type = type; o.frequency.setValueAtTime(f0, t); if (f1) o.frequency.exponentialRampToValueAtTime(f1, t + dur); v.gain.setValueAtTime(0, t); v.gain.linearRampToValueAtTime(g, t + .006); v.gain.exponentialRampToValueAtTime(.0005, t + dur); o.connect(v); v.connect(master); o.start(t); o.stop(t + dur + .02); };
  const noise = (dur, g = .3, hp = 2000) => { if (!on || !ensure()) return; resume(); const t = ac.currentTime; const n = ac.sampleRate * dur | 0, b = ac.createBuffer(1, n, ac.sampleRate), d = b.getChannelData(0); for (let i = 0; i < n; i++) d[i] = (Math.random() * 2 - 1) * (1 - i / n); const src = ac.createBufferSource(); src.buffer = b; const f = ac.createBiquadFilter(); f.type = 'highpass'; f.frequency.value = hp; const v = ac.createGain(); v.gain.value = g; src.connect(f); f.connect(v); v.connect(master); src.start(t); };
  const ambient = (start) => { if (!ensure()) return; if (start && !amb && on) { const v = ac.createGain(); v.gain.value = 0; v.connect(master); const f = ac.createBiquadFilter(); f.type = 'lowpass'; f.frequency.value = 220; f.connect(v); const os = [55, 55.4, 82.5].map((fr) => { const o = ac.createOscillator(); o.type = 'triangle'; o.frequency.value = fr; o.connect(f); o.start(); return o; }); v.gain.linearRampToValueAtTime(.06, ac.currentTime + 4); amb = { v, os }; } if (!start && amb) { amb.v.gain.linearRampToValueAtTime(0, ac.currentTime + 1); const a = amb; amb = null; setTimeout(() => a.os.forEach((o) => o.stop()), 1200); } };
  const NOTES = [220, 246.9, 277.2, 329.6, 370, 440, 493.9];
  return {
    get on() { return on; },
    toggle() { on = !on; localStorage.setItem('cyber.sound', on ? 'on' : 'off'); if (on) { ensure(); resume(); ambient(true); tone(660, 990, .12, 'sine', .4); } else ambient(false); return on; },
    wake() { if (on && ensure()) { resume(); ambient(true); } },
    tick() { noise(.02, .25, 3000); tone(1800, 1200, .03, 'square', .08); },
    move(dir, ch) { noise(.03, .2, 1500); tone(dir > 0 ? 440 : 520, dir > 0 ? 560 : 400, .08, 'sine', .25); if (ch != null) { const r = NOTES[ch]; tone(r, null, .5, 'sine', .18, .04); tone(r * 1.5, null, .45, 'sine', .1, .08); } },
    link() { tone(880, 1320, .16, 'sine', .35); tone(1320, 1760, .22, 'triangle', .18, .1); },
    hash() { noise(.012, .12, 4000); },
    ticket() { tone(1760, 1750, .35, 'sine', .3); tone(2640, 2630, .25, 'sine', .12, .02); },
    converge(d) { const f = 220 + 1200 * Math.min(1, Math.max(0, (-Math.log10(Math.max(d, 1e-9)) - 1) / 7)); tone(f, f * 1.02, .07, 'triangle', .22); },
    stake(v) { tone(v ? 160 : 110, 60, .25, 'sine', .45); noise(.05, .15, 800); },
    mentor() { [0, 2, 4, 6].forEach((k, i) => tone(NOTES[k] * 2, null, .3, 'sine', .22, i * .09)); },
    chord(ch) { const r = NOTES[ch]; [1, 1.25, 1.5, 2].forEach((m, i) => tone(r * m, null, .9, 'sine', .14, i * .05)); },
  };
})();
/* count-up for integer figures */
const countUp = (slide) => { slide.querySelectorAll('.n big').forEach((b) => { if (b.dataset.done) return; const raw = b.firstChild && b.firstChild.nodeType === 3 ? b.firstChild.nodeValue.trim() : ''; if (!/^[\d,]+$/.test(raw)) return; const target = +raw.replace(/,/g, ''); if (!target || target < 100) return; b.dataset.done = 1; const t0 = performance.now(); const step = (t) => { const k = Math.min(1, (t - t0) / 900), e = 1 - Math.pow(1 - k, 3); b.firstChild.nodeValue = Math.round(target * e).toLocaleString('en-US'); if (k < 1) requestAnimationFrame(step); }; requestAnimationFrame(step); }); };

/* ------------------------------------------------------------------ build */
const deck = $('#deck'), segs = $('#segs');
S.forEach((s, i) => {
  const d = el(`<section class="slide ch${s.ch} ${s.cls || ''}" data-i="${i}" data-ch="${String(s.ch + 1).padStart(2, '0')}" style="--c:${CH[s.ch].c}"><div class="body">${s.html}</div>${s.bridge ? `<p class="bridge">→ ${s.bridge}</p>` : ''}</section>`);
  deck.appendChild(d);
});
CH.forEach((c, i) => {
  const n = S.filter((s) => s.ch === i).length;
  const d = el(`<div class="seg" style="--sc:${c.c}" data-ch="${i}"><div class="fill"></div><span class="lab">${String(i + 1).padStart(2, '0')} ${c.name}</span></div>`);
  d.addEventListener('click', () => go(S.findIndex((s) => s.ch === i)));
  d.dataset.n = n; segs.appendChild(d);
});

let cur = -1, auto = null;
const go = (i) => {
  i = Math.max(0, Math.min(S.length - 1, i));
  if (i === cur) return;
  const dir = i > cur ? 1 : -1, prevCh = cur >= 0 ? S[cur].ch : -1;
  cur = i;
  deck.dataset.dir = dir;
  document.querySelectorAll('.slide').forEach((s, k) => { s.classList.toggle('on', k === i); s.classList.toggle('left', k < i); });
  sfx.move(dir, S[i].ch !== prevCh ? S[i].ch : null);
  if (navigator.vibrate) navigator.vibrate(8);
  graph.pulse();
  countUp(document.querySelectorAll('.slide')[i]);
  $('#pos').textContent = `${String(i + 1).padStart(2, '0')} / ${S.length}`;
  document.body.classList.toggle('ink', (S[i].cls || '').includes('ink'));
  document.body.className = document.body.className.replace(/\bch\d\b/g, '').trim() + ' ch' + S[i].ch;
  if (auto) $('#mine-auto').click();
  const ch = S[i].ch, idx = S.filter((s, k) => s.ch === ch && k <= i).length, n = S.filter((s) => s.ch === ch).length;
  document.querySelectorAll('.seg').forEach((sg, k) => { sg.classList.toggle('cur', k === ch); sg.classList.toggle('done', k < ch); $('.fill', sg).style.width = k < ch ? '100%' : k === ch ? (idx / n * 100) + '%' : '0'; });
  history.replaceState(null, '', '#' + i);
  graph.chapter(ch);
  onEnter(i);
};
window.deckGo = go; window.deckCount = () => S.length;
$('#prev').onclick = () => go(cur - 1); $('#next').onclick = () => go(cur + 1);
document.addEventListener('keydown', (e) => {
  if (e.target.tagName === 'INPUT' || (e.target.tagName === 'BUTTON' && [' ', 'Enter'].includes(e.key))) return;
  if (['ArrowRight', ' ', 'PageDown', 'ArrowDown'].includes(e.key)) { e.preventDefault(); go(cur + 1); }
  if (['ArrowLeft', 'PageUp', 'ArrowUp'].includes(e.key)) { e.preventDefault(); go(cur - 1); }
  if (e.key === 'Home') go(0); if (e.key === 'End') go(S.length - 1);
});
let wheelLock = 0;
document.addEventListener('wheel', (e) => { if (e.target.closest('.panel,.body') && e.target.closest('.body')?.scrollHeight > e.target.closest('.body')?.clientHeight + 4) return; const t = Date.now(); if (t - wheelLock < 700 || Math.abs(e.deltaY) < 24) return; wheelLock = t; go(cur + (e.deltaY > 0 ? 1 : -1)); }, { passive: true });
let tx = null, ty = 0; document.addEventListener('touchstart', (e) => { tx = e.target.closest('input[type=range]') ? null : e.touches[0].clientX; ty = e.touches[0].clientY; }, { passive: true });
document.addEventListener('touchend', (e) => { if (tx == null) return; const dx = e.changedTouches[0].clientX - tx, dy = e.changedTouches[0].clientY - ty; if (Math.abs(dx) > 60 && Math.abs(dx) > Math.abs(dy)) go(cur + (dx < 0 ? 1 : -1)); tx = null; });
$('#restart')?.addEventListener('click', (e) => { e.preventDefault(); go(0); });
document.addEventListener('pointerdown', (e) => { sfx.wake(); if (e.target.closest('button, a, .chip, .seg, input[type=range]')) sfx.tick(); }, { passive: true });
document.addEventListener('keydown', () => sfx.wake(), { once: true });
const sndBtn = $('#snd'); const sndPaint = () => { sndBtn.textContent = sfx.on ? '◉ sound' : '○ sound'; sndBtn.classList.toggle('off', !sfx.on); }; sndBtn.onclick = () => { sfx.toggle(); sndPaint(); }; sndPaint();
/* cursor: a dot and a lagging ring on fine pointers */
if (matchMedia('(pointer:fine)').matches) { const dot = el('<div class="cur-dot"></div>'), ring = el('<div class="cur-ring"></div>'); document.body.append(dot, ring); let rx = 0, ry = 0, tx2 = 0, ty2 = 0; addEventListener('mousemove', (e) => { tx2 = e.clientX; ty2 = e.clientY; dot.style.transform = `translate(${tx2}px,${ty2}px)`; const hot = e.target.closest('button, a, .chip, .seg, input, canvas, .fig'); ring.classList.toggle('hot', !!hot); }, { passive: true }); const lag = () => { rx += (tx2 - rx) * .18; ry += (ty2 - ry) * .18; ring.style.transform = `translate(${rx}px,${ry}px)`; requestAnimationFrame(lag); }; lag(); document.body.classList.add('has-cursor'); }

/* ------------------------------------------------------------------ yield */
const Y = { links: 0, samples: 0, stake: 0, invites: 0 };
const bump = (k, v) => { Y[k] += v; const e = $('#y-' + k); e.textContent = fmt(Y[k]); e.parentElement.classList.remove('bump'); void e.offsetWidth; e.parentElement.classList.add('bump'); const t = $('#t-' + k); if (t) t.textContent = fmt(Y[k]); };

/* ------------------------------------------------------------------ graph canvas */
const graph = (() => {
  const cv = $('#graph'), ctx = cv.getContext('2d');
  let W = 0, H = 0, nodes = [], edges = [], mouse = { x: -1e9, y: -1e9 }, hue = '#00ff41', pulse = 0;
  const resize = () => { W = cv.width = innerWidth * devicePixelRatio; H = cv.height = innerHeight * devicePixelRatio; };
  addEventListener('resize', resize); resize();
  const rnd = (a, b) => a + Math.random() * (b - a);
  const add = (n) => { for (let i = 0; i < n; i++) { const p = { x: rnd(0, W), y: rnd(0, H), vx: rnd(-.15, .15), vy: rnd(-.15, .15), r: rnd(1, 2.4) * devicePixelRatio, phi: Math.random(), born: performance.now(), label: null }; nodes.push(p); if (nodes.length > 1 && Math.random() < .9) { const q = nodes[Math.floor(Math.random() * (nodes.length - 1))]; edges.push([p, q]); } } };
  add(70);
  addEventListener('mousemove', (e) => { mouse.x = e.clientX * devicePixelRatio; mouse.y = e.clientY * devicePixelRatio; });
  const colors = ['#ff1a00', '#ff7a00', '#ffc400', '#00ff41', '#00b8ff', '#1e00ff', '#7a00ff'];
  const tick = () => {
    ctx.clearRect(0, 0, W, H);
    const t = performance.now();
    for (const p of nodes) {
      p.x += p.vx; p.y += p.vy;
      if (p.x < 0 || p.x > W) p.vx *= -1; if (p.y < 0 || p.y > H) p.vy *= -1;
      const dx = p.x - mouse.x, dy = p.y - mouse.y, d2 = dx * dx + dy * dy;
      if (d2 < 160 * 160 * devicePixelRatio) { const d = Math.sqrt(d2) + 1; p.x += dx / d * .8; p.y += dy / d * .8; }
    }
    ctx.lineWidth = 1 * devicePixelRatio;
    for (const [a, b] of edges) {
      const age = Math.min(1, (t - Math.max(a.born, b.born)) / 1200);
      ctx.strokeStyle = hue; ctx.globalAlpha = .12 + .5 * (1 - age) * (a.label || b.label ? 1 : 0) + .08 * Math.max(a.phi, b.phi);
      ctx.beginPath(); ctx.moveTo(a.x, a.y); ctx.lineTo(b.x, b.y); ctx.stroke();
    }
    for (const p of nodes) {
      const age = Math.min(1, (t - p.born) / 900);
      const r = p.r * (1 + 2.5 * (1 - age)) * (1 + .6 * p.phi) + (pulse > 0 ? pulse * 2 * p.phi : 0);
      ctx.globalAlpha = .55 + .45 * p.phi; ctx.fillStyle = p.label ? '#fff' : hue;
      ctx.beginPath(); ctx.arc(p.x, p.y, r, 0, Math.PI * 2); ctx.fill();
      if (p.label) { ctx.globalAlpha = .9; ctx.fillStyle = '#fff'; ctx.font = `${12 * devicePixelRatio}px JetBrains Mono, monospace`; ctx.fillText(p.label, p.x + 8 * devicePixelRatio, p.y - 8 * devicePixelRatio); }
    }
    ctx.globalAlpha = 1; pulse *= .95;
    requestAnimationFrame(tick);
  };
  tick();
  return {
    chapter(c) { hue = colors[c]; },
    link(a, b) {
      const mk = (label) => { const p = { x: rnd(W * .3, W * .7), y: rnd(H * .3, H * .7), vx: rnd(-.1, .1), vy: rnd(-.1, .1), r: 3 * devicePixelRatio, phi: 1, born: performance.now(), label }; nodes.push(p); return p; };
      const pa = nodes.find((n) => n.label === a) || mk(a), pb = nodes.find((n) => n.label === b) || mk(b);
      edges.push([pa, pb]); for (let i = 0; i < 3; i++) { const q = nodes[Math.floor(Math.random() * nodes.length)]; edges.push([Math.random() < .5 ? pa : pb, q]); }
      pulse = 1;
    },
    iterate() { for (const p of nodes) { let s = 0, n = 0; for (const [a, b] of edges) { if (a === p) { s += b.phi; n++; } if (b === p) { s += a.phi; n++; } } p.next = n ? .15 * p.phi + .85 * s / n : p.phi * .97; } let d = 0; for (const p of nodes) { d += Math.abs(p.next - p.phi); p.phi = p.next; } const m = Math.max(...nodes.map((p) => p.phi)) || 1; for (const p of nodes) p.phi = p.phi / m; return d / nodes.length; },
    grow(n) { add(n); pulse = .6; },
    pulse() { pulse = Math.max(pulse, .8); },
  };
})();

/* ------------------------------------------------------------------ interactives */
const CH_A = ['proof of work', 'a wrong claim', 'light-minutes', 'my grandmother\'s recipe', 'the Landauer limit'];
const CH_B = ['Shapley sampling', 'costs the staker', 'local finality', 'a particle, forever', '3×10⁻²¹ J per bit'];
const chips = (id, list, input) => { const c = $(id); list.forEach((t) => { const b = el(`<span class="chip">${t}</span>`); b.onclick = () => { $(input).value = t; }; c.appendChild(b); }); };
chips('#chips-a', CH_A, '#link-a'); chips('#chips-b', CH_B, '#link-b');
$('#link-go').onclick = () => {
  const a = esc($('#link-a').value.trim().slice(0, 80)), b = esc($('#link-b').value.trim().slice(0, 80)); if (!a || !b) return;
  if (a.toLowerCase() === b.toLowerCase()) { $('#link-out').innerHTML = 'a thing linked to itself moves focus by <b>0.0000</b> bits. copying earns nothing.'; return; }
  graph.link(a, b); bump('links', 1); sfx.link();
  const d = (0.0007 + Math.random() * 0.004).toFixed(4), s = (40 + Math.random() * 55).toFixed(0);
  sha(a + ' → ' + b).then((h) => { const o = $('#link-out'); o.innerHTML = o.innerHTML.replace('particle …', `particle <b>${h}</b>`); });
  $('#link-out').innerHTML = `linked <b>${a}</b> ⟶ <b>${b}</b> · particle … (sha-256 here, hemera on mainnet)\nΔφ⋆ <b>+${d} bits</b> · surprise <b>${s}%</b> against the crowd's prediction · your share of the shift: <b>100%</b> until someone else links it ${M('k')}\n${Y.links === 1 ? 'that is what the mint pays for. nothing else.' : 'the graph behind you grew. keep going.'}`;
  $('#stake-claim').textContent = `“${a}” belongs with “${b}”`;
};

const cbar = (chart, v, col) => { const b = el(`<i title="${v.toExponential(2)}"></i>`); b.dataset.v = v; if (col) b.style.background = col; chart.appendChild(b); while (chart.children.length > 36) chart.firstChild.remove(); const mx = Math.max(...[...chart.children].map((x) => +x.dataset.v)) || 1; [...chart.children].forEach((x) => { x.style.height = Math.max(2, 100 * (+x.dataset.v) / mx) + '%'; }); };
let kIter = 0; $('#kernel-run').onclick = () => { let n = 0; const chart = $('#kernel-chart'); const run = () => { const real = cyberGraph.iterate(); const d = real ? real.d : graph.iterate(); graph.iterate(); kIter++; n++; cbar(chart, d); sfx.converge(d); $('#kernel-iter').textContent = fmt(kIter); $('#kernel-delta').textContent = `‖Δφ‖ = ${d.toExponential(2)}${real && real.kap ? ` · κ̂ ≈ ${Math.min(real.kap, .999).toFixed(3)}` : ''}`; if (n < 12 && d > 1e-7) requestAnimationFrame(run); else if (d <= 1e-7) $('#kernel-delta').textContent += ' · fixed point'; }; run(); };
$('#unique-run').onclick = () => { const chart = $('#unique-chart'); chart.innerHTML = ''; const A = cyberGraph.run('random'), B = cyberGraph.run('random'); let n = 0; const step = () => { const ra = A.step(), rb = B.step(); if (!ra) { $('#unique-out').textContent = 'sample still loading — open the cybergraph slide first'; return; } let dist = 0; const pa = A.phi, pb = B.phi; for (let i = 0; i < pa.length; i++) dist += Math.abs(pa[i] - pb[i]); n++; cbar(chart, dist, n % 2 ? COLS[4] : '#fff'); if (n % 3 === 0) sfx.converge(dist); $('#unique-out').textContent = `step ${n} · ‖φ_A − φ_B‖₁ = ${dist.toExponential(2)}`; if (n < 80 && dist > 1e-6) requestAnimationFrame(step); else { $('#unique-out').textContent += dist <= 1e-6 ? ' · one focus' : ''; if (dist <= 1e-6) sfx.chord(2); } }; step(); };

const NAMES = ['you', 'archivist', 'neuron 0x3f…', 'sensor'];
const weights = [0, 0, 0, 0]; let samples = 0, tickets = 0;
const mineBars = $('#mine-bars'); NAMES.forEach((n, i) => mineBars.appendChild(el(`<div class="bar ${i === 0 ? 'me' : ''}"><span>${n}</span><i style="transform:scaleX(0)"></i><span>0%</span></div>`)));
const hex = () => Array.from({ length: 24 }, () => '0123456789abcdef'[Math.random() * 16 | 0]).join('');
const sample = (k) => {
  for (let i = 0; i < k; i++) {
    const order = [0, 1, 2, 3].sort(() => Math.random() - .5); let acc = 0;
    for (const j of order) { const v = acc + [1.0, 1.6, .7, .4][j] * (0.6 + Math.random() * .8); weights[j] += v - acc; acc = v; }
    samples++; if (Math.random() < .03) { tickets++; $('#mine-tickets').appendChild(el('<i></i>')); sfx.ticket(); }
  }
  const tot = weights.reduce((a, b) => a + b, 0) || 1;
  [...mineBars.children].forEach((b, i) => { const f = weights[i] / tot; $('i', b).style.transform = `scaleX(${f.toFixed(3)})`; b.lastElementChild.textContent = (f * 100).toFixed(1) + '%'; });
  $('#mine-hash').textContent = hex() + ' · nonce ' + samples;
  $('#mine-stat').innerHTML = `${fmt(samples)} samples · ${tickets} tickets ${M('k')}`;
  if (samples % 500 === 0 || samples <= 100) $('#mine-out').innerHTML = `your share converges as orderings accumulate. with <b>${fmt(samples)}</b> samples the division is already within a few percent of the exact Shapley value — that is the whole trick: the proof-of-work <b>is</b> the accounting. ${M('k')}`;
  bump('samples', k); graph.grow(Math.ceil(k / 25));
};
$('#mine-go').onclick = () => { sfx.hash(); sample(100); };
$('#mine-auto').onclick = (e) => { if (auto) { clearInterval(auto); auto = null; e.target.textContent = 'auto'; } else { auto = setInterval(() => sample(20), 120); e.target.textContent = 'stop'; } };

let valence = 0; $('#stake-dial').addEventListener('click', (e) => { const b = e.target.closest('button'); if (!b) return; valence = +b.dataset.v; [...$('#stake-dial').children].forEach((x) => x.classList.toggle('on', x === b)); hint(); });
const hint = () => { const amt = +$('#stake-range').value; $('#stake-amt').textContent = amt; $('#stake-hint').innerHTML = valence === 0 ? `moves rank · earns nothing ${M('k')}` : `at risk · pays if the picture moves ${valence > 0 ? 'with' : 'against'} the claim ${M('k')}`; };
$('#stake-range').oninput = hint; hint();
$('#stake-go').onclick = () => { const amt = +$('#stake-range').value; sfx.stake(valence); if (valence !== 0) bump('stake', amt); const ev = valence === 0 ? 0 : (amt * (0.08 + Math.random() * .12)).toFixed(2); $('#stake-out').innerHTML = valence === 0 ? `staked <b>${amt} tokens</b> passively. the claim ranks higher. you earn <b>0</b> — idle capital cannot compound here. ${M('k')}` : `staked <b>${amt} tokens</b> at valence <b>${valence > 0 ? '+1' : '−1'}</b>. if the serum confirms you, expected yield ≈ <b>${ev}</b> from the liars' side of the market; if not, that is what you lose ${M('k')}`; };

$('#ref-go').onclick = () => { const n = ($('#ref-name').value.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '') || 'anon'); const url = `${location.origin}${location.pathname}?m=${n}#0`; sfx.mentor(); if (!Y.invites) bump('invites', 1); $('#ref-out').innerHTML = `<b>${url}</b>\nsend it to one person who knows something. when they link, you teach; when they mint, you earn the mentor's share.`; navigator.clipboard?.writeText(url).then(() => { $('#ref-out').innerHTML += '\n<b>copied</b>'; }).catch(() => {}); };

/* two ledgers */
const ledV = $('#led-v'); const hx = (n) => Array.from({ length: n }, () => '0123456789abcdef'[Math.random() * 16 | 0]).join('');
const ledTick = () => { if (cur !== 2 || !ledV) return; const r = el(`<div class="row-l"><span>tx ${hx(12)}…</span><span class="ok">settled · ${(2 + Math.random() * 4).toFixed(1)} s</span></div>`); ledV.prepend(r); while (ledV.children.length > 7) ledV.lastChild.remove(); };
setInterval(ledTick, 900);

/* one budget */
const rOut = () => { const r = +$('#r-range').value; $('#r-val').textContent = r; $('#split-w').style.width = (100 - r) + '%'; $('#split-s').style.width = r + '%'; $('#r-out').innerHTML = r < 15 ? `stake is scarce → the subsidy leans on <b>work</b>: a phone with nothing can still earn by sampling.` : r > 70 ? `most capital is at risk → the budget leans on <b>stake</b>; work still gets its floor, so the door never closes.` : `a thermostat: <b>${100 - r}%</b> of emission to settlement work, <b>${r}%</b> to active risk, recalibrated from what the chain observes.`; };
$('#r-range').oninput = rOut; rOut();

/* your neuron */
const KEY = crypto.getRandomValues(new Uint8Array(32));
const NEURON = Array.from(KEY.slice(0, 6), (b) => b.toString(16).padStart(2, '0')).join('');
const identicon = (key, size = 7) => { const cols = ['#ff1a00', '#ff7a00', '#ffc400', '#00ff41', '#00b8ff', '#1e00ff', '#7a00ff']; let o = `<svg viewBox="0 0 ${size} ${size}" shape-rendering="crispEdges">`; for (let y = 0; y < size; y++) for (let x = 0; x < Math.ceil(size / 2); x++) { const b = key[(y * 4 + x) % key.length]; if (b % 3 === 0) continue; const c = cols[b % 7]; o += `<rect x="${x}" y="${y}" width="1" height="1" fill="${c}"/>`; if (x !== size - 1 - x) o += `<rect x="${size - 1 - x}" y="${y}" width="1" height="1" fill="${c}"/>`; } return o + '</svg>'; };
const sha = async (t) => { const d = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(t)); return Array.from(new Uint8Array(d).slice(0, 8), (b) => b.toString(16).padStart(2, '0')).join(''); };
$('#neuron-card').innerHTML = `${identicon(KEY)}<div><b>neuron 0x${NEURON}…</b>a throwaway keypair made in your browser for this visit ${M('k')} · on mainnet, mudra keys</div>`;
document.querySelectorAll('.tally a').forEach((a) => { a.addEventListener('click', (e) => { e.preventDefault(); go(+a.getAttribute('href').slice(1)); }); });

const cd = () => { const t = Date.parse('2026-11-05T13:22:42Z') - Date.now(); if (t < 0) { $('#cd').textContent = 'live'; return; } const d = Math.floor(t / 864e5), h = Math.floor(t % 864e5 / 36e5), m = Math.floor(t % 36e5 / 6e4), s = Math.floor(t % 6e4 / 1e3); $('#cd').textContent = `${d}d ${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`; };
setInterval(cd, 1000); cd();

const onEnter = (i) => { if (i === 2) cyberGraph.start(); else cyberGraph.stop(); if (i === 6 || i === 7) cyberGraph.load(); if (i === S.length - 1) ['links', 'samples', 'stake', 'invites'].forEach((k) => { const t = $('#t-' + k); if (t) t.textContent = fmt(Y[k]); const a = $(`.tally a[data-k=${k}]`); if (a) a.style.display = Y[k] ? 'none' : 'block'; }); };

/* mentor param → greet */
const m = new URLSearchParams(location.search).get('m'); if (m) { $('.kick', deck).textContent = `cyber · ${m} sent you · a frozen protocol for collective lifeforms`; $('#link-out').textContent = `${m} sent you here. link something they do not know yet.`; }

addEventListener('hashchange', () => go(parseInt(location.hash.slice(1)) || 0));
go(parseInt(location.hash.slice(1)) || 0);
})();
