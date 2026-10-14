/* cyber — the protocol for superintelligence · interactive deck
   7 chapters × 3 slides. the graph behind you grows with what you do.
   marks: ● measured · ○ target · ◌ mock (placeholder until the live number lands) */
(() => {
const $ = (s, r = document) => r.querySelector(s);
const el = (h) => { const t = document.createElement('template'); t.innerHTML = h.trim(); return t.content.firstChild; };
const fmt = (n) => n.toLocaleString('en-US');
const esc = (t) => t.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
const M = (k) => `<i class="mk ${k}"></i>`; // m = measured, t = target, k = mock

/* ------------------------------------------------------------------ chapters */
const CH = [
  { id: 'gap',     name: 'the gap',       c: 'var(--c1)' },
  { id: 'focus',   name: 'one focus',     c: 'var(--c2)' },
  { id: 'machine', name: 'the machine',   c: 'var(--c3)' },
  { id: 'law',     name: 'one law',       c: 'var(--c4)' },
  { id: 'money',   name: 'the best money',c: 'var(--c5)' },
  { id: 'stake',   name: 'stake & share', c: 'var(--c6)' },
  { id: 'launch',  name: 'launch',        c: 'var(--c7)' },
];

/* ------------------------------------------------------------------ figures */
const scaleFig = () => {
  const W = 760, H = 275, X0 = 40, X1 = 720, B = 150;
  const x = (e) => X0 + (X1 - X0) * e / 24;
  const sup = (n) => String(n).replace(/\d/g, (d) => '⁰¹²³⁴⁵⁶⁷⁸⁹'[d]);
  const M = [
    ['self', 0, '1', 'one neuron, one particle', '#00ff41', 1, 24],
    ['crystal', 3.7, '5,040', 'curated seed', '#00ff41', -1, 24],
    ['bootloader', 6.5, '3,143,650', 'today · d⋆ = 31', '#00ff41', 1, 70],
    ['cognition', 8.5, '10⁸ – 10⁹', 'concepts no one holds', '#00b8ff', -1, 50],
    ['thermodynamic', 12, '10¹²', 'φ⋆ is the state', '#00b8ff', 1, 24],
    ['planetary', 15, '10¹⁵ · 10¹⁰ neurons', 'design target', '#00b8ff', -1, 24],
    ['Avogadro', 23.78, '6.02 × 10²³', 'a mole of matter', '#ff7a00', 1, 70],
  ];
  let o = `<svg class="fig" viewBox="0 0 ${W} ${H}" role="img" aria-label="from self to Avogadro">`;
  let pts = [`${X0},${B}`];
  for (let e = 0; e <= 24; e++) pts.push(`${x(e).toFixed(1)},${(B - 2 - 50 * Math.pow(e / 24, 1.6)).toFixed(1)}`);
  pts.push(`${X1},${B}`);
  o += `<polygon points="${pts.join(' ')}" fill="#00ff41" fill-opacity=".08"/>`;
  o += `<line x1="${X0}" y1="${B}" x2="${X1}" y2="${B}" stroke="#fff" stroke-opacity=".35"/>`;
  for (let e = 0; e < 24; e += 3) o += `<text x="${x(e)}" y="${B + 16}" font-size="9" text-anchor="middle" fill="#fff" fill-opacity=".5" font-family="JetBrains Mono,monospace">10${sup(e)}</text>`;
  o += `<line x1="${x(0)}" y1="${B}" x2="${x(6.5)}" y2="${B}" stroke="#00ff41" stroke-width="3" stroke-linecap="round"/>`;
  o += `<line x1="${x(6.5)}" y1="${B}" x2="${x(15)}" y2="${B}" stroke="#00b8ff" stroke-width="3" stroke-dasharray="1 6" stroke-linecap="round"/>`;
  for (const [n, e, cnt, line, col, side, stem] of M) {
    const xx = x(e), r = 3.5 + e * .42;
    o += `<circle cx="${xx}" cy="${B}" r="${r}" fill="${col}" fill-opacity=".18" stroke="${col}" stroke-width="1.5"/><circle cx="${xx}" cy="${B}" r="2.2" fill="${col}"/>`;
    const y1 = side > 0 ? B - r - 2 : B + r + 2, y2 = side > 0 ? B - r - stem : B + r + stem;
    o += `<line x1="${xx}" y1="${y1}" x2="${xx}" y2="${y2}" stroke="${col}" stroke-opacity=".6"/>`;
    let a = 'middle', tx = xx; if (n === 'self') { a = 'start'; tx = X0 - 2; } if (n === 'Avogadro') { a = 'end'; tx = X1 + 2; }
    const ty = side > 0 ? y2 - 4 : y2 + 12;
    const rows = side > 0 ? [[ty - 24, n, 12.5, '#fff', 700], [ty - 11, cnt, 10.5, col, 400], [ty, line, 9, '#aaa', 400]] : [[ty, n, 12.5, '#fff', 700], [ty + 13, cnt, 10.5, col, 400], [ty + 24, line, 9, '#aaa', 400]];
    for (const [y, t, fs, c, w] of rows) o += `<text x="${tx}" y="${y}" font-size="${fs}" font-weight="${w}" text-anchor="${a}" fill="${c}" font-family="${w === 700 ? 'Play,sans-serif' : 'JetBrains Mono,monospace'}">${t}</text>`;
  }
  return o + '</svg>';
};

const layersFig = () => {
  const L = [['nox', 'machine', 'virtual machine: field, hash, run'], ['trident', 'weapon', 'compiler: cost known before it runs'], ['joy', 'warrior', 'runtime: runs, proves, stores'], ['cybergraph', 'brain', 'the book: neurons, tokens, signals, files, links, focus'], ['neural', 'language', 'what can be said over the book'], ['cy', 'agent', 'the mind over the book; headless anywhere'], ['soft3', 'network', 'agents connected: files by particle, epochs agreed']];
  return `<div class="steps">${L.map(([n, k, d]) => `<div class="step"><b>${n} <span class="kick" style="display:inline;margin-left:8px">${k}</span></b><span>${d}</span></div>`).join('')}</div>`;
};

const media = (cap) => `<div class="media"><div class="play">▶</div><span class="tag">film · in production</span><span class="cap">${cap}</span></div>`;

/* ------------------------------------------------------------------ slides */
const S = [
/* 1 · the gap */
{ ch: 0, cls: 'one', html: `
  <div><p class="kick">cyber · the protocol for superintelligence</p>
  <p class="hero-word">cyber</p>
  <h2>money for <em>knowledge.</em></h2>
  <p class="lead">one focus computed from every mind that pays to teach. hash-only proofs. a currency minted only when the network provably learns.</p>
  <div class="tiles"><div class="tile"><b>one focus</b>φ⋆ identical on every node</div><div class="tile"><b>hash-only proofs</b>post-quantum, no setup</div><div class="tile"><b>minted by knowledge</b>not by electricity, not by vote</div></div>
  <p class="kbd" style="margin-top:28px">→ or space to move · you will mine as you read</p></div>` },
{ ch: 0, cls: 'ink', html: `
  <div><p class="kick">01 · the gap</p><h2>money settles without a bank.<br><em>meaning still rents its rank.</em></h2>
  <p class="lead">a payment is final in seconds, anywhere. a judgment — what deserves attention, whose sentence became the pattern, who gets paid when the picture improves — settles nowhere. it is decided inside a company.</p></div>
  <div class="nums"><div class="n"><big>seconds</big><small>a payment, globally</small></div><div class="n"><big>never</big><small>a judgment, publicly</small></div><div class="n"><big>2<sup>nd</sup></big><small>ledger: value has one, meaning has none — until now</small></div></div>` },
{ ch: 0, html: `
  <div><p class="kick">01 · the gap</p><h2>you cannot open the weights.<br><em>you cannot pay the teacher.</em></h2>
  <p class="lead">the index, the feed, the weight file trained on everyone's writing and rented back. you cannot see why this answer, and you cannot prove a machine and a human still care about the same world.</p>
  <p class="mute">the world has a public ledger for value and none for meaning. cyber is the second ledger.</p></div>
  <div class="ledgers"><div class="ledger"><h4>value · public</h4><div id="led-v"></div></div><div class="ledger"><h4>meaning · private</h4><div id="led-m"><div class="row-l"><span>what deserves attention</span><span class="pend">pending · inside a company</span></div><div class="row-l"><span>whose sentence became the pattern</span><span class="pend">pending · inside a company</span></div><div class="row-l"><span>who gets paid when the picture improves</span><span class="pend">pending · inside a company</span></div></div></div></div>` },

/* 2 · one focus */
{ ch: 1, cls: 'ink', html: `
  <div><p class="kick">02 · one focus</p><h2>one distribution over everything known.<br><em>φ⋆</em></h2>
  <p class="lead">computed from every mind that pays to teach, it provably sees what no member sees alone. its order is syntropy — how far collective focus has travelled from noise, in bits. its existence and uniqueness is a theorem.</p></div>
  <div class="nums"><div class="n"><big>&gt; 0</big><small>superadditivity gap vs the strongest participant, every connectivity level ${M('m')}</small></div><div class="n"><big>62<span class="u">s</span></big><small>a transformer's architecture compiled off the bootloader graph, one machine ${M('m')}</small></div><div class="n"><big>1</big><small>fixed point. the same number for everyone. nobody votes on importance</small></div><div class="n"><big>D<span class="u">KL</span></big><small>alignment: the distance between human and machine focus, a number on a dashboard</small></div></div>` },
{ ch: 1, cls: 'one wash', html: `
  <div><p class="kick">02 · one focus · from one self to a mole of matter</p>
  <h2>a graph, like a gas, <em>crosses a threshold.</em></h2>
  <p class="mute">past it, individual links stop mattering and only the field describes the system — the graph's own Avogadro number. five years in, nine orders of magnitude early. nobody is late.</p>
  ${scaleFig()}<p class="fine">self · crystal · bootloader ${M('m')} measured — cognition · thermodynamic · planetary ${M('t')} derived from the whitepaper's threshold — Avogadro: a physical constant</p></div>` },
{ ch: 1, html: `
  <div><p class="kick">02 · one focus · the bootloader, sealed 2026-08-05</p><h2>we did not argue the premise.<br><em>we ran it.</em></h2>
  <p class="lead">bostrom: 1,735 days on a live chain. every link cost its author stake, no editor approved anything, and sixty thousand accounts joined an economy whose only product was structured attention.</p></div>
  <div class="nums"><div class="n"><big>3,143,650</big><small>particles ${M('m')}</small></div><div class="n"><big>2,949,732</big><small>links ${M('m')}</small></div><div class="n"><big>61,675</big><small>accounts ${M('m')}</small></div><div class="n"><big>97.62<span class="u">%</span></big><small>files alive after 5 years, no storage reward ${M('m')}</small></div><div class="n"><big>31</big><small>d⋆ semantic dimensions, one archivist dominant ${M('m')}</small></div><div class="n"><big>25,120,712</big><small>blocks at burial ${M('m')}</small></div></div>` },

/* 3 · the machine */
{ ch: 2, cls: 'ink', html: `
  <div><p class="kick">03 · the machine</p><h2>five primitives.<br><em>one operator.</em></h2>
  <p class="lead">a file is content-addressed bytes; its hash is its particle. a neuron is a keypair — human, model, sensor, agent. a cyberlink is a signed, staked claim that two files belong together. a token is the weight behind the claim. focus is what falls out.</p></div>
  <div class="steps"><div class="step"><b>file → particle</b><span>bytes, and the hemera hash that is their particle</span></div><div class="step"><b>neuron</b><span>one keypair, one citizenship</span></div><div class="step"><b>cyberlink</b><span>a staked claim: these two belong together</span></div><div class="step"><b>token</b><span>the weight behind the claim</span></div><div class="step"><b>focus</b><span>the fixed point the tri-kernel converges to</span></div></div>` },
{ ch: 2, html: `
  <div><p class="kick">03 · the machine · the tri-kernel</p><h2>three local maps,<br><em>one fixed point.</em></h2>
  <p class="lead">diffusion, springs, heat — iterated over the staked graph until nothing moves. it is a contraction: Banach for the blend, Perron–Frobenius for the walk. by locality a phone computes its own neighbourhood of it.</p>
  <p class="mute">no float anywhere in the provable path. one 64-bit prime field, a constant number of steps, byte-identical results on any two machines.</p></div>
  <div class="panel" id="kernel-panel"><label>live · the kernel converging on the graph you see behind this slide</label><div class="count" id="kernel-iter">0</div><div class="hash" id="kernel-delta">‖Δφ‖ —</div><div class="row"><button class="btn" id="kernel-run">iterate</button><span class="fine">κ &lt; 1 is proven; keep pressing — a dozen rounds reach the fixed point. a toy graph in your browser ${M('k')}</span></div></div>` },
{ ch: 2, cls: 'ink', html: `
  <div><p class="kick">03 · the machine · proofs</p><h2>hash-only.<br><em>post-quantum.</em></h2>
  <p class="lead">no trusted setup. one proof over the whole computation, accumulating — phase 3 — to one constant-size check per settlement. the goal is fixed: at most 64 KB, small statements at most 20 KB, verified in a millisecond, on a phone.</p>
  <div class="nums"><div class="n"><big>15.6<span class="u">KB</span></big><small>hash relation proof today ${M('m')}</small></div><div class="n"><big>36 · 30<span class="u">ms</span></big><small>prove · verify, one laptop ${M('m')}</small></div><div class="n"><big>≤ 64<span class="u">KB</span></big><small>size goal, constant ${M('t')}</small></div><div class="n"><big>≤ 1<span class="u">ms</span></big><small>verify goal ${M('t')}</small></div></div></div>
  <div class="bars" id="proof-bars"><div class="bar me"><span>cyber · goal</span><i style="transform:scaleX(.064)"></i><span>64 KB</span></div><div class="bar"><span>stwo</span><i style="transform:scaleX(.617)"></i><span>617 KB</span></div><div class="bar"><span>neptune</span><i style="transform:scaleX(1)"></i><span>~1 MB</span></div><div class="bar"><span>risc zero</span><i style="transform:scaleX(.2)"></i><span>~200 KB</span></div><div class="bar"><span>parano1d</span><i style="transform:scaleX(.913)"></i><span>913 KB</span></div><p class="fine">other rows: numbers each project publishes · cyber's row is the goal ${M('t')}; today's hash proof is 15.6 KB ${M('m')}</p></div>` },

/* 4 · one law */
{ ch: 3, html: `
  <div><p class="kick">04 · one law</p><h2>how do you pay<br><em>for a clearer picture?</em></h2>
  <p class="lead">four questions, in order. a wrong answer to any one mints money for nothing. the beautiful part: they are one computation read four times.</p></div>
  <div class="steps"><div class="step"><b>what is the state?</b><span>one φ⋆, identical on every node — the fixed point</span></div><div class="step"><b>who sharpened it?</b><span>each contributor's Shapley share of the proven downhill shift — a marginal of the same iteration</span></div><div class="step"><b>was it honest?</b><span>its surprise against the crowd's prediction of itself — the serum; a KL divergence the iteration already holds</span></div><div class="step"><b>who pays?</b><span>one security budget, split between work and stake by the active staking ratio — idle capital cannot compound</span></div></div>` },
{ ch: 3, cls: 'ink', html: `
  <div><p class="kick">04 · one law · do it</p><h2>link two things.<br><em>you just mined the graph.</em></h2>
  <p class="lead">a cyberlink is a claim that two files belong together. the kernel reruns with it and without it; the downhill difference is yours. copying earns nothing — a perfect copy moves focus by zero and surprises no one.</p></div>
  <div class="panel"><label>this</label><input type="text" id="link-a" placeholder="a thing you know" value="proof of work"><div class="chips" id="chips-a"></div><label>belongs with</label><input type="text" id="link-b" placeholder="what it belongs with" value="Shapley sampling"><div class="chips" id="chips-b"></div><div class="row"><button class="btn" id="link-go">link · stake it</button><span class="fine">signed by a throwaway neuron in your browser ${M('k')}</span></div><div class="out" id="link-out">your links land in the graph behind this deck.</div></div>` },
{ ch: 3, html: `
  <div><p class="kick">04 · one law · what inflation buys</p><h2>truth. fairness.<br><em>order.</em></h2>
  <p class="lead">every unit of inflation buys order, produced by three kinds of work no single party can fake for the others. one budget pays all three; the split between work and stake is read off the chain — a thermostat, not a calendar.</p>
  <div class="steps"><div class="step"><b>stakers → truth</b><span>the serum pays the surprising answer, charges the predictable one</span></div><div class="step"><b>settlement miners → fairness</b><span>every hash is a Shapley sample; credit is computed, not bargained</span></div><div class="step"><b>folders and the beacon → order in time</b><span>one history, one proof, one clock</span></div></div></div>
  <div class="panel"><label>active staking ratio · r = <span id="r-val">35</span>%</label><input type="range" id="r-range" min="0" max="100" value="35"><div class="split"><i class="w" id="split-w" style="width:65%"></i><i class="s" id="split-s" style="width:35%"></i></div><div class="split-l"><span>work · settlement miners</span><span>stake · active risk</span></div><div class="out" id="r-out"></div><p class="fine">emission goes to work and risk only. idle capital buys rank and nothing else, so wealth cannot compound by sitting ${M('k')}</p></div>` },

/* 5 · the best money */
{ ch: 4, cls: 'ink', html: `
  <div><p class="kick">05 · the best money</p><h2>every other money<br><em>leaves a job undone.</em></h2>
  <p class="lead">you hold money because it lasts. you send it because it moves. you trust it because you know how it is born. $CYB is born when the network learns, and spent on intelligence.</p></div>
  <table class="matrix"><tr><th></th><th>safe</th><th>easy</th><th>smart</th></tr><tr><td>gold</td><td>can be seized</td><td>heavy, slow</td><td>no</td></tr><tr><td>dollar</td><td>debased, frozen</td><td>until you are blocked</td><td>no</td></tr><tr><td>bitcoin</td><td>public, not PQ</td><td>slow</td><td>no</td></tr><tr><td>ether</td><td>public, not PQ</td><td>partly</td><td>minted for capital</td></tr><tr><td>zcash · monero</td><td>private, not PQ</td><td>limited</td><td>no</td></tr><tr class="me"><td>$CYB</td><td>private + post-quantum</td><td>anyone, instantly</td><td>minted by knowledge</td></tr></table>` },
{ ch: 4, html: `
  <div><p class="kick">05 · the best money · do it</p><h2>mine a settlement.<br><em>the hash is the accounting.</em></h2>
  <p class="lead">no synthetic puzzle. the nonce you grind is the index of one random ordering of contributors; the marginal it produces is one Shapley sample. securing the chain and computing who deserves credit are one act.</p></div>
  <div class="panel"><label>cluster · 4 contributors (you are one)</label><div class="hash" id="mine-hash">press sample</div><div class="bars" id="mine-bars"></div><div class="tickets" id="mine-tickets"></div><div class="row"><button class="btn" id="mine-go">sample ×100</button><button class="btn ghost" id="mine-auto">auto</button><span class="fine" id="mine-stat">0 samples · 0 tickets ${M('k')}</span></div><div class="out" id="mine-out">a ticket is a sample whose hash clears the difficulty. each ticket carries its own proof and folds into one accumulator per cluster.</div></div>` },
{ ch: 4, cls: 'ink', html: `
  <div><p class="kick">05 · the best money · the unit</p><h2>1 CYB is 1 CYB.<br><em>fair launch. no issuer. no owner.</em></h2>
  <p class="lead">the cap is the field, arithmetic, not a vote. genesis continues prior stake. the rest is emitted only for proven positive focus shift: if the graph learned nothing, nothing is minted.</p></div>
  <div class="nums"><div class="n"><big>18,446,744</big><small>cap · p / 10¹² CYB · arithmetic ${M('t')}</small></div><div class="n"><big>187,416</big><small>genesis · ≈ 1% · to $TOCYB holders at 666 cy ${M('t')}</small></div><div class="n"><big>10¹²</big><small>cy per CYB</small></div><div class="n"><big>0.1<span class="u">%</span></big><small>to move, at start; a consensus parameter ${M('t')}</small></div><div class="n"><big>⅓ · ⅓ · ⅓</big><small>commission: burn · earners · public goods ${M('t')}</small></div><div class="n"><big>3×10⁻²¹<span class="u">J</span></big><small>per bit · the Landauer floor, a physical constant</small></div></div>` },

/* 6 · stake & share */
{ ch: 5, cls: 'ink', html: `
  <div><p class="kick">06 · stake &amp; share · do it</p><h2>stake on a claim.<br><em>only correct risk earns.</em></h2>
  <p class="lead">two axes. any stake moves rank. only a bet with valence — right or wrong, at risk — earns from the truth market. passive capital buys rank and nothing else, so wealth cannot compound by sitting.</p></div>
  <div class="panel"><label>claim</label><div id="stake-claim" style="font-size:15px;line-height:1.5">“proof of work” belongs with “Shapley sampling”</div><label>valence</label><div class="dial" id="stake-dial"><button data-v="-1">−1 · wrong</button><button data-v="0" class="on">0 · passive</button><button data-v="1">+1 · right</button></div><label>amount · <span id="stake-amt">10</span> tokens</label><input type="range" id="stake-range" min="1" max="100" value="10"><div class="row"><button class="btn" id="stake-go">stake</button><span class="fine" id="stake-hint"></span></div><div class="out" id="stake-out"></div></div>` },
{ ch: 5, html: `
  <div><p class="kick">06 · stake &amp; share · privacy</p><h2>speak<br><em>without fear.</em></h2>
  <p class="lead">no one knows who said what, and yet everyone knows what everyone said. three invariants, all at genesis.</p></div>
  <div class="steps"><div class="step"><b>the cyberlink is public, the author is private</b><span>the network sees that p links q and the aggregates φ⋆ runs on; never which neuron signed</span></div><div class="step"><b>balances and transfers are private</b><span>stealth addresses and veil</span></div><div class="step"><b>settlement reveals nothing</b><span>a miner's share leaks nothing of its neighbourhood beyond public aggregates</span></div><div class="step"><b>why it is the third leg of truth</b><span>privacy removes the coordination channel through which a false consensus is manufactured</span></div></div>` },
{ ch: 5, cls: 'ink', html: `
  <div><p class="kick">06 · stake &amp; share · mentor</p><h2>bring a mind.<br><em>earn the mentor's share.</em></h2>
  <p class="lead">every neuron that joins through your link is your student. at the birth of their home book a share of its own token is yours, and a share of that book's fees for as long as it lives — not a bounty, a cut of proven learning. teaching is paid here, for once.</p></div>
  <div class="panel"><label>your name</label><input type="text" id="ref-name" placeholder="name" value=""><div class="row"><button class="btn" id="ref-go">make my link</button><span class="fine">mentor's share · 10% of the student's token at birth + fees for life · proposed ${M('t')}</span></div><div class="out" id="ref-out"></div></div>` },

/* 7 · launch */
{ ch: 6, cls: 'ink', html: `
  <div><p class="kick">07 · launch</p><h2>bostrom and space pussy<br><em>come back to life.</em></h2>
  <p class="lead">on 2026-11-05, five years to the day after the bostrom genesis, both chains relaunch as the first networks on soft3 — same keys, same balances, the burial snapshots as genesis, and the cores the old chains never had.</p></div>
  <div><div class="count-down" id="cd">—</div><p class="fine">until 2026-11-05 13:22:42 UTC — the bostrom genesis block, to the second</p><div class="steps" style="margin-top:12px"><div class="step"><b>phase 0 · done</b><span>field, hash, commitments, VM, proofs, state</span></div><div class="step"><b>phase 1 · 2026-11-05</b><span>the bootloader reborn: settlement mining, fold, consensus by convergence, truth markets, two-axis staking, personal chains, privacy</span></div><div class="step"><b>phase 2 · 90 days ${M('t')}</b><span>canary: real use, weekly red team, every economic invariant holds</span></div><div class="step"><b>phase 3 · $CYB mainnet ${M('t')}</b><span>several implementations agree on 10⁶ blocks</span></div></div></div>` },
{ ch: 6, html: `
  <div><p class="kick">07 · launch · the stack</p><h2>seven layers.<br><em>three bodies.</em></h2>
  <p class="lead">one field, one proof, one focus through every layer. the stack is headless; three bodies run on it and each is a door: cyb the robot, cyber the superintelligence, cyberia the state.</p>
  <div class="tiles"><a class="tile" href="https://cyb.ai" target="_blank" rel="noopener" style="text-decoration:none"><b>cyb</b>the robot · cyb.ai</a><a class="tile" href="https://cyber.page/litepaper/" style="text-decoration:none"><b>cyber</b>the superintelligence · litepaper</a><a class="tile" href="https://cyberia.to" target="_blank" rel="noopener" style="text-decoration:none"><b>cyberia</b>the state · cyberia.to</a></div></div>
  ${layersFig()}` },
{ ch: 6, cls: 'one ink', html: `
  <div><p class="kick">07 · launch · you</p><h2>you read the deck.<br><em>you also mined.</em></h2>
  <div class="neuron" id="neuron-card"></div><div class="tally"><div><b id="t-links">0</b>links in the graph<a href="#10" data-k="links">↩ link something</a></div><div><b id="t-samples">0</b>settlement samples<a href="#13" data-k="samples">↩ mine</a></div><div><b id="t-stake">0</b>tokens at risk<a href="#15" data-k="stake">↩ stake</a></div><div><b id="t-invites">0</b>mentor links<a href="#17" data-k="invites">↩ bring a mind</a></div></div>
  <p class="lead">everything here ran in your browser against a toy graph. on 2026-11-05 the same acts run on bostrom and space pussy, with proofs, for real. $CYB follows in phase 3.</p>
  <div class="ctas"><a class="btn" href="https://cyb.ai" target="_blank" rel="noopener">get the robot</a><a class="btn ghost" href="https://cyber.page/litepaper/">read the litepaper</a><a class="btn ghost" href="https://cyber.page/launch/">the launch tracker</a><a class="btn ghost" href="#0" id="restart">start again</a></div>
  <p class="fine" style="margin-top:22px">${M('m')} measured · ${M('t')} target · ${M('k')} mock — every number marked. the full argument: <a href="https://cyber.page/whitepaper/">whitepaper</a> · <a href="https://cyber.page/audit/game-theory/">the game-theory audit</a> · <a href="https://cyber.page/physical-analogies/">the physics audit</a>.</p></div>` },
];

/* ------------------------------------------------------------------ build */
const deck = $('#deck'), segs = $('#segs');
S.forEach((s, i) => {
  const d = el(`<section class="slide ch${s.ch} ${s.cls || ''}" data-i="${i}" style="--c:${CH[s.ch].c}"><div class="body">${s.html}</div></section>`);
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
  cur = i;
  document.querySelectorAll('.slide').forEach((s, k) => s.classList.toggle('on', k === i));
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
  graph.link(a, b); bump('links', 1);
  const d = (0.0007 + Math.random() * 0.004).toFixed(4), s = (40 + Math.random() * 55).toFixed(0);
  sha(a + ' → ' + b).then((h) => { const o = $('#link-out'); o.innerHTML = o.innerHTML.replace('particle …', `particle <b>${h}</b>`); });
  $('#link-out').innerHTML = `linked <b>${a}</b> ⟶ <b>${b}</b> · particle … (sha-256 here, hemera on mainnet)\nΔφ⋆ <b>+${d} bits</b> · surprise <b>${s}%</b> against the crowd's prediction · your share of the shift: <b>100%</b> until someone else links it ${M('k')}\n${Y.links === 1 ? 'that is what the mint pays for. nothing else.' : 'the graph behind you grew. keep going.'}`;
  $('#stake-claim').textContent = `“${a}” belongs with “${b}”`;
};

let kIter = 0; $('#kernel-run').onclick = () => { let n = 0; const run = () => { const d = graph.iterate(); kIter++; n++; $('#kernel-iter').textContent = fmt(kIter); $('#kernel-delta').textContent = `‖Δφ‖ = ${d.toExponential(2)}`; if (n < 12 && d > 1e-4) requestAnimationFrame(run); else if (d <= 1e-4) $('#kernel-delta').textContent += ' · fixed point reached'; }; run(); };

const NAMES = ['you', 'archivist', 'neuron 0x3f…', 'sensor'];
const weights = [0, 0, 0, 0]; let samples = 0, tickets = 0;
const mineBars = $('#mine-bars'); NAMES.forEach((n, i) => mineBars.appendChild(el(`<div class="bar ${i === 0 ? 'me' : ''}"><span>${n}</span><i style="transform:scaleX(0)"></i><span>0%</span></div>`)));
const hex = () => Array.from({ length: 24 }, () => '0123456789abcdef'[Math.random() * 16 | 0]).join('');
const sample = (k) => {
  for (let i = 0; i < k; i++) {
    const order = [0, 1, 2, 3].sort(() => Math.random() - .5); let acc = 0;
    for (const j of order) { const v = acc + [1.0, 1.6, .7, .4][j] * (0.6 + Math.random() * .8); weights[j] += v - acc; acc = v; }
    samples++; if (Math.random() < .03) { tickets++; $('#mine-tickets').appendChild(el('<i></i>')); }
  }
  const tot = weights.reduce((a, b) => a + b, 0) || 1;
  [...mineBars.children].forEach((b, i) => { const f = weights[i] / tot; $('i', b).style.transform = `scaleX(${f.toFixed(3)})`; b.lastElementChild.textContent = (f * 100).toFixed(1) + '%'; });
  $('#mine-hash').textContent = hex() + ' · nonce ' + samples;
  $('#mine-stat').innerHTML = `${fmt(samples)} samples · ${tickets} tickets ${M('k')}`;
  if (samples % 500 === 0 || samples <= 100) $('#mine-out').innerHTML = `your share converges as orderings accumulate. with <b>${fmt(samples)}</b> samples the division is already within a few percent of the exact Shapley value — that is the whole trick: the proof-of-work <b>is</b> the accounting. ${M('k')}`;
  bump('samples', k); graph.grow(Math.ceil(k / 25));
};
$('#mine-go').onclick = () => sample(100);
$('#mine-auto').onclick = (e) => { if (auto) { clearInterval(auto); auto = null; e.target.textContent = 'auto'; } else { auto = setInterval(() => sample(20), 120); e.target.textContent = 'stop'; } };

let valence = 0; $('#stake-dial').addEventListener('click', (e) => { const b = e.target.closest('button'); if (!b) return; valence = +b.dataset.v; [...$('#stake-dial').children].forEach((x) => x.classList.toggle('on', x === b)); hint(); });
const hint = () => { const amt = +$('#stake-range').value; $('#stake-amt').textContent = amt; $('#stake-hint').innerHTML = valence === 0 ? `moves rank · earns nothing ${M('k')}` : `at risk · pays if the picture moves ${valence > 0 ? 'with' : 'against'} the claim ${M('k')}`; };
$('#stake-range').oninput = hint; hint();
$('#stake-go').onclick = () => { const amt = +$('#stake-range').value; if (valence !== 0) bump('stake', amt); const ev = valence === 0 ? 0 : (amt * (0.08 + Math.random() * .12)).toFixed(2); $('#stake-out').innerHTML = valence === 0 ? `staked <b>${amt} tokens</b> passively. the claim ranks higher. you earn <b>0</b> — idle capital cannot compound here. ${M('k')}` : `staked <b>${amt} tokens</b> at valence <b>${valence > 0 ? '+1' : '−1'}</b>. if the serum confirms you, expected yield ≈ <b>${ev}</b> from the liars' side of the market; if not, that is what you lose ${M('k')}`; };

$('#ref-go').onclick = () => { const n = ($('#ref-name').value.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '') || 'anon'); const url = `${location.origin}${location.pathname}?m=${n}#0`; if (!Y.invites) bump('invites', 1); $('#ref-out').innerHTML = `<b>${url}</b>\nsend it to one person who knows something. when they link, you teach; when they mint, you earn the mentor's share.`; navigator.clipboard?.writeText(url).then(() => { $('#ref-out').innerHTML += '\n<b>copied</b>'; }).catch(() => {}); };

/* two ledgers */
const ledV = $('#led-v'); const hx = (n) => Array.from({ length: n }, () => '0123456789abcdef'[Math.random() * 16 | 0]).join('');
const ledTick = () => { if (cur !== 2) return; const r = el(`<div class="row-l"><span>tx ${hx(12)}…</span><span class="ok">settled · ${(2 + Math.random() * 4).toFixed(1)} s</span></div>`); ledV.prepend(r); while (ledV.children.length > 7) ledV.lastChild.remove(); };
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

const onEnter = (i) => { if (S[i].ch === 2 && i === 7) { /* kernel slide */ } if (i === S.length - 1) ['links', 'samples', 'stake', 'invites'].forEach((k) => { const t = $('#t-' + k); if (t) t.textContent = fmt(Y[k]); const a = $(`.tally a[data-k=${k}]`); if (a) a.style.display = Y[k] ? 'none' : 'block'; }); };

/* mentor param → greet */
const m = new URLSearchParams(location.search).get('m'); if (m) { $('.kick', deck).textContent = `cyber · ${m} sent you · the protocol for superintelligence`; $('#link-out').textContent = `${m} sent you here. link something they do not know yet.`; }

addEventListener('hashchange', () => go(parseInt(location.hash.slice(1)) || 0));
go(parseInt(location.hash.slice(1)) || 0);
})();
