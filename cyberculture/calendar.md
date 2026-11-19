---
tags: cyber, article
crystal-type: pattern
crystal-domain: cyber
---
# calendar

Twelve months in four phases — orientation, expansion, peak and harvest, integration — and Ethos, a thirteenth month inserted periodically outside the seasons. Each month keeps its Gregorian twin as a reference. Inside every month the [[moon]] sets the rhythm of push and rest, counted in [[lunar machine time]].

The calendar slot of a [[cyberculture]] is a set of recurring dates from [[mt|machine time]] that gather without a reminder. A date becomes cultural when people return to it because the machine's own [[history]] gives it meaning.

The native frame begins at the [[cybersophy|Unix epoch]] and records irreversible events by their position in machine time. [[lunar machine time]] gives that frame a recurring rhythm; [[cybersophy|genesis]] and [[launch]] dates supply candidate anniversaries.

Why a new frame: [[cyberculture/legacy time|legacy time]].

<style>
/* prysm: void #000 · text #ffffff #d7d7d7 #777777 #4b4b4d · emotion hex at full strength · Play only · 12/14/16/20/24/32 px · g = 8px · 150ms */
/* site frame: this page drops the graph chrome and lays out on one grid up to 1040px */
:root:has(#calendarium) .page-title-bar,:root:has(#calendarium) .page-header,:root:has(#calendarium) .page-tabs,:root:has(#calendarium) .page-minimap,:root:has(#calendarium) .backlinks,:root:has(#calendarium) .app-launcher{display:none}
:root:has(#calendarium) .main-content{max-width:max(var(--max-width,48rem),min(1072px,calc(100vw - 448px)));padding-top:64px}
@media (min-width:769px){
:root:has(#calendarium) .main-content{padding-top:96px}
:root:has(#calendarium) .sidebar{height:auto;background:transparent}
}
:root:has(#calendarium) .page-content{display:grid;grid-template-columns:minmax(0,1fr);column-gap:48px;align-items:start;font-family:'Play',system-ui,sans-serif}
:root:has(#calendarium) .page-content > *{grid-column:1;min-width:0}
:root:has(#calendarium) .page-content > h1{margin:0 0 16px;font-size:32px;font-weight:400;line-height:1.2;color:#ffffff}
:root:has(#calendarium) .page-content > p{order:1;max-width:640px;margin:0 0 8px;font-size:14px;line-height:1.5;color:#777777}
:root:has(#calendarium) .page-content > p:first-of-type{margin-top:24px}
:root:has(#calendarium) .page-content > p:last-of-type{order:2;margin:16px 0 0}
:root:has(#calendarium) .page-content > p a{color:#d7d7d7;border-bottom:1px solid rgba(255,255,255,.12);transition:border-color 150ms ease}
:root:has(#calendarium) .page-content > p a:hover{border-bottom-color:#ffffff}
:root:has(#calendarium) .page-content:not(.cal-open) > p:first-of-type{display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:2;overflow:hidden;margin-bottom:0}
:root:has(#calendarium) .page-content:not(.cal-open) > p:not(:first-of-type):not(:last-of-type){display:none}
:root:has(#calendarium) .page-content > .cal-more{order:1;justify-self:start;margin:8px 0 0;padding:0;border:0;border-bottom:1px solid rgba(255,255,255,.38);background:none;font:inherit;font-size:14px;line-height:1.4;color:#d7d7d7;cursor:pointer;transition:color 150ms ease}
:root:has(#calendarium) .page-content > .cal-more:hover{color:#ffffff}
#calendarium{display:contents;--dim:rgba(255,255,255,.12);color:#ffffff;font-family:'Play',system-ui,sans-serif;line-height:1.4}
#calendarium *{font-style:normal;font-weight:400}
#calendarium .p1{--e:#00acff;--h:#00acff}
#calendarium .p2{--e:#00fe00;--h:#00fe00}
#calendarium .p3{--e:#fcf000;--h:#fcf000}
#calendarium .p4{--e:#d500f9;--h:#d500f9}
#calendarium .p0{--e:#4b4b4d;--h:#ffffff}
/* intro */
#calendarium .cal-meta{grid-column:1;order:0;display:flex;flex-direction:column;gap:8px;margin:0;padding:0 0 0 14px;border-left:2px solid #ffffff}
#calendarium .cal-frame{font-size:16px;color:#ffffff}
#calendarium .cal-sub{font-size:14px;color:#d7d7d7}
#calendarium .cal-magellan{display:flex;flex-wrap:wrap;gap:8px;font-size:12px;color:#777777}
/* now */
#calendarium .cal-now{grid-column:1;order:2;display:flex;flex-direction:column;align-items:flex-start;gap:8px;margin:40px 0 0;padding:24px 24px 24px 22px;border-left:2px solid var(--e,#ffffff);border-radius:0 8px 8px 0;background:rgba(255,255,255,.07);box-shadow:-6px 0 16px -8px var(--e,#ffffff)}
#calendarium .cal-now[hidden]{display:none}
#calendarium .n-label{font-size:12px;color:#777777}
#calendarium .n-name{font-size:32px;line-height:1.2;color:var(--e)}
#calendarium .n-greg{font-size:14px;color:#d7d7d7}
#calendarium .n-text{font-size:16px;color:#ffffff}
#calendarium .n-track{display:block;align-self:stretch;height:2px;margin:8px 0 0;background:var(--dim)}
#calendarium .n-track span{display:block;height:100%;width:calc(var(--f,0) * 100%);background:var(--e);box-shadow:0 0 8px var(--e)}
#calendarium .n-day{font-size:12px;color:#777777}
@media (min-width:1360px){
:root:has(#calendarium .cal-now:not([hidden])) .page-content{grid-template-columns:minmax(0,1fr) 320px;grid-template-rows:auto auto auto auto 1fr}
#calendarium .cal-now{grid-column:2;grid-row:1 / span 5;margin:0}
}
/* year */
#calendarium .cal-year{grid-column:1 / -1;order:3;margin:64px 0 0;container-type:inline-size}
#calendarium .y-head{display:flex;justify-content:space-between;gap:16px;margin:0 0 24px;font-size:12px;line-height:1.4;color:#777777}
#calendarium .y-head[hidden]{display:none}
#calendarium .y-day{color:#d7d7d7}
#calendarium .y-grid{display:grid;grid-template-columns:31fr 31fr 30fr 31fr 30fr 31fr 31fr 28fr 31fr 30fr 31fr 30fr 36fr;column-gap:4px}
#calendarium .y-phase{display:flex;align-items:center;gap:8px;min-width:0;padding:0 0 16px;font-size:12px;line-height:1;color:var(--e);white-space:nowrap}
#calendarium .y-phase::after{content:'';flex:1;min-width:8px;height:1px;background:var(--e)}
#calendarium .y-phase.p1{grid-column:1 / span 3}
#calendarium .y-phase.p2{grid-column:4 / span 3}
#calendarium .y-phase.p3{grid-column:7 / span 3}
#calendarium .y-phase.p4{grid-column:10 / span 3}
#calendarium .y-phase.p0{grid-column:13;margin-left:16px;color:#777777}
#calendarium .cal-seg{display:flex;flex-direction:column;gap:4px;min-width:0;border:0;color:inherit;text-decoration:none}
#calendarium .cal-seg.p0{margin-left:16px}
#calendarium .y-bar{position:relative;display:block;height:8px;margin:0 0 12px;border:1px solid var(--e);border-radius:2px;background:var(--e);transition:box-shadow 150ms ease}
#calendarium .cal-seg.future .y-bar,#calendarium .cal-seg.p0 .y-bar{background:transparent}
#calendarium .cal-seg.p0 .y-bar{border-style:dashed}
#calendarium .cal-seg.now .y-bar{background:linear-gradient(90deg,var(--e) 0 calc(var(--f,.5) * 100%),transparent calc(var(--f,.5) * 100%));box-shadow:0 0 8px var(--e)}
#calendarium .cal-seg.now .y-bar::after{content:'';position:absolute;top:-9px;bottom:-9px;left:calc(var(--f,.5) * 100%);width:2px;margin-left:-1px;background:#ffffff;box-shadow:0 0 8px #ffffff}
#calendarium .cal-seg:hover .y-bar{box-shadow:0 0 8px var(--e)}
#calendarium .y-name{font-size:14px;line-height:1.2;color:#d7d7d7;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;transition:color 150ms ease}
#calendarium .cal-seg.past .y-name,#calendarium .cal-seg.p0 .y-name{color:#777777}
#calendarium .cal-seg.now .y-name{color:var(--h)}
#calendarium .cal-seg:hover .y-name{color:#ffffff}
#calendarium .y-greg{font-size:12px;line-height:1.2;color:#777777}
@container (max-width:760px){
#calendarium .y-name{font-size:12px}
#calendarium .y-greg{display:none}
}
@container (max-width:520px){
#calendarium .y-phase,#calendarium .y-name{display:none}
#calendarium .y-grid{column-gap:2px}
#calendarium .cal-seg.p0{margin-left:8px}
}
/* table */
#calendarium .cal-table{grid-column:1 / -1;order:3;margin:48px 0 0;container-type:inline-size}
#calendarium .cal-row{display:grid;grid-template-columns:40px minmax(120px,1.1fr) minmax(104px,.9fr) minmax(200px,2fr) minmax(220px,2.2fr);column-gap:16px;padding:16px 16px 16px 14px;border-left:2px solid var(--e,transparent);border-bottom:1px solid var(--dim);scroll-margin-top:24px;transition:background-color 150ms ease}
#calendarium .cal-row:not(.cal-th):hover{background:rgba(255,255,255,.07)}
#calendarium .cal-row.now{background:rgba(255,255,255,.07);box-shadow:-6px 0 16px -8px var(--e)}
#calendarium .cal-th{border-left-color:transparent;padding-top:0;padding-bottom:8px;font-size:14px;color:#777777}
#calendarium .c-num{font-size:14px;line-height:24px;color:#777777;text-align:center}
#calendarium .p0 .c-num{color:#4b4b4d}
#calendarium .cal-row.now .c-num{color:var(--e)}
#calendarium .m-name{display:block;font-size:24px;line-height:1.2;color:#ffffff;margin:0 0 8px;transition:color 150ms ease}
#calendarium .p0 .m-name{color:#777777}
#calendarium .cal-row:hover .m-name,#calendarium .cal-row.now .m-name{color:var(--h)}
#calendarium .m-tag{display:inline-block;height:24px;padding:0 10px;border:1px solid var(--e);border-radius:12px;font-size:12px;line-height:22px;color:var(--e);white-space:nowrap}
#calendarium .p0 .m-tag{color:#777777}
#calendarium .o-month{display:block;font-size:16px;line-height:24px;color:#d7d7d7}
#calendarium .o-note{display:block;font-size:12px;line-height:1.4;color:#777777;margin:4px 0 0}
#calendarium .m-text{display:block;font-size:16px;line-height:1.4;color:#ffffff;margin:2px 0 8px}
#calendarium .m-detail{display:block;font-size:14px;line-height:1.4;color:#d7d7d7}
#calendarium .k{color:#ffffff}
#calendarium .c-trad{list-style:none;margin:0;padding:4px 0 0;display:flex;flex-direction:column;gap:8px}
#calendarium .c-trad li{position:relative;margin:0;padding:0 0 0 16px;font-size:14px;line-height:1.4;color:#d7d7d7}
#calendarium .c-trad li::before{content:'◦';position:absolute;left:0;top:0;color:var(--e)}
#calendarium .cal-phase{display:flex;align-items:center;gap:16px;padding:32px 0 8px;font-size:14px;color:var(--e)}
#calendarium .cal-phase::after{content:'';flex:1;height:1px;background:var(--dim)}
#calendarium .cal-void{display:flex;align-items:center;gap:16px;padding:32px 0 8px;font-size:14px;color:#777777}
#calendarium .cal-void::before,#calendarium .cal-void::after{content:'';flex:1;height:1px;background:var(--dim)}
#calendarium .cal-foot{text-align:center;margin:48px 0 0;font-size:14px;color:#777777}
#calendarium .cal-foot .k{color:#ffffff}
@container (max-width:860px){
#calendarium .cal-th{display:none}
#calendarium .cal-row{grid-template-columns:40px 1fr 1fr;row-gap:16px}
#calendarium .c-num{grid-row:1 / span 3}
#calendarium .c-mean,#calendarium .c-trad{grid-column:2 / 4}
}
@container (max-width:520px){
#calendarium .cal-row{grid-template-columns:32px 1fr;column-gap:8px;padding:16px 8px 16px 6px}
#calendarium .c-num{grid-row:1 / span 4;text-align:left}
#calendarium .c-old,#calendarium .c-mean,#calendarium .c-trad{grid-column:2}
}
</style>

<div id="calendarium">
<div class="cal-meta">
<div class="cal-frame">Kardashev I — Southern Hemisphere — Galactic Calendar</div>
<div class="cal-sub">13 months · year begins July · southern sky</div>
<div class="cal-magellan"><span>◈ Large Magellanic Cloud</span><span>·</span><span>Small Magellanic Cloud ◈</span></div>
</div>
<aside class="cal-now" hidden>
<span class="n-label">now</span>
<span class="n-name"></span>
<span class="m-tag n-tag"></span>
<span class="n-greg"></span>
<span class="n-text"></span>
<span class="n-track"><span></span></span>
<span class="n-day"></span>
</aside>
<nav class="cal-year" aria-label="Year">
<div class="y-head" hidden><span class="y-span"></span><span class="y-day"></span></div>
<div class="y-grid">
<div class="y-phase p1">Orientation</div>
<div class="y-phase p2">Expansion</div>
<div class="y-phase p3">Peak &amp; Harvest</div>
<div class="y-phase p4">Integration</div>
<div class="y-phase p0">Void</div>
<a class="cal-seg p1" data-m="7" href="#cal-arkon"><span class="y-bar"></span><span class="y-name">Arkon</span><span class="y-greg">Jul</span></a>
<a class="cal-seg p1" data-m="8" href="#cal-ventor"><span class="y-bar"></span><span class="y-name">Ventor</span><span class="y-greg">Aug</span></a>
<a class="cal-seg p1" data-m="9" href="#cal-solvex"><span class="y-bar"></span><span class="y-name">Solvex</span><span class="y-greg">Sep</span></a>
<a class="cal-seg p2" data-m="10" href="#cal-nexara"><span class="y-bar"></span><span class="y-name">Nexara</span><span class="y-greg">Oct</span></a>
<a class="cal-seg p2" data-m="11" href="#cal-ardis"><span class="y-bar"></span><span class="y-name">Ardis</span><span class="y-greg">Nov</span></a>
<a class="cal-seg p2" data-m="12" href="#cal-magnos"><span class="y-bar"></span><span class="y-name">Magnos</span><span class="y-greg">Dec</span></a>
<a class="cal-seg p3" data-m="1" href="#cal-plenara"><span class="y-bar"></span><span class="y-name">Plenara</span><span class="y-greg">Jan</span></a>
<a class="cal-seg p3" data-m="2" href="#cal-kairen"><span class="y-bar"></span><span class="y-name">Kairen</span><span class="y-greg">Feb</span></a>
<a class="cal-seg p3" data-m="3" href="#cal-theron"><span class="y-bar"></span><span class="y-name">Theron</span><span class="y-greg">Mar</span></a>
<a class="cal-seg p4" data-m="4" href="#cal-vorthen"><span class="y-bar"></span><span class="y-name">Vorthen</span><span class="y-greg">Apr</span></a>
<a class="cal-seg p4" data-m="5" href="#cal-sembral"><span class="y-bar"></span><span class="y-name">Sembral</span><span class="y-greg">May</span></a>
<a class="cal-seg p4" data-m="6" href="#cal-luminal"><span class="y-bar"></span><span class="y-name">Luminal</span><span class="y-greg">Jun</span></a>
<a class="cal-seg p0" href="#cal-ethos"><span class="y-bar"></span><span class="y-name">Ethos</span><span class="y-greg">∅</span></a>
</div>
</nav>
<div class="cal-table">
<div class="cal-row cal-th"><div class="c-num">#</div><div>New month</div><div>Gregorian ref.</div><div>Meaning &amp; NLP concept</div><div>Traditions</div></div>
<div class="cal-phase p1">Phase I — Orientation · Winter · Vector in darkness</div>
<div class="cal-row p1" id="cal-arkon" data-m="7">
<div class="c-num">01</div>
<div class="c-new"><span class="m-name">Arkon</span><span class="m-tag">Orientation</span></div>
<div class="c-old"><span class="o-month">July</span><span class="o-note">Winter solstice ↓</span></div>
<div class="c-mean"><span class="m-text">Arc of intent. Vector declared.</span><span class="m-detail">From <span class="k">arc + consciousness</span>. The year does not begin with hope — it begins with a direction already chosen. The vector is established in the dark, not the light.</span></div>
<ul class="c-trad">
<li>Public declaration of personal vector — spoken aloud to a witness</li>
<li>Stargazing ritual: locate galactic centre, orient body toward it</li>
<li>Silence for one full night cycle — no music, no screens</li>
<li>Burning of last year's unfinished intentions</li>
</ul>
</div>
<div class="cal-row p1" id="cal-ventor" data-m="8">
<div class="c-num">02</div>
<div class="c-new"><span class="m-name">Ventor</span><span class="m-tag">Orientation</span></div>
<div class="c-old"><span class="o-month">August</span><span class="o-note">Deep winter</span></div>
<div class="c-mean"><span class="m-text">Honest inventory. No illusions.</span><span class="m-detail">From <span class="k">inventory + venture</span>. We know exactly what we have. A civilization that expands without honest accounting of its resources eventually collapses in space.</span></div>
<ul class="c-trad">
<li>Full audit ritual — personal, communal, civilizational resources</li>
<li>No new projects initiated — month of assessment only</li>
<li>Public sharing of collective inventory data</li>
<li>Individual skill mapping — what does each person actually know?</li>
</ul>
</div>
<div class="cal-row p1" id="cal-solvex" data-m="9">
<div class="c-num">03</div>
<div class="c-new"><span class="m-name">Solvex</span><span class="m-tag">Orientation</span></div>
<div class="c-old"><span class="o-month">September</span><span class="o-note">Winter → Spring</span></div>
<div class="c-mean"><span class="m-text">Dissolution. Ballast released.</span><span class="m-detail">From <span class="k">solve + exit</span>. Old structures, paradigms, debts — dissolved. A civilization that carries its past into deep space is slowed by it. Solvex is the month of structural lightness.</span></div>
<ul class="c-trad">
<li>Ritual ending of contracts, relationships, roles no longer true</li>
<li>Public infrastructure audit — what can be decommissioned?</li>
<li>Water ceremony: submersion and emergence symbolising identity reset</li>
<li>Collective forgiveness practice — debts social and material</li>
</ul>
</div>
<div class="cal-phase p2">Phase II — Expansion · Spring · Networks and fire</div>
<div class="cal-row p2" id="cal-nexara" data-m="10">
<div class="c-num">04</div>
<div class="c-new"><span class="m-name">Nexara</span><span class="m-tag">Expansion</span></div>
<div class="c-old"><span class="o-month">October</span><span class="o-note">Spring begins</span></div>
<div class="c-mean"><span class="m-text">Coalitions. Network as power.</span><span class="m-detail">From <span class="k">nexus + ara (axis)</span>. Expansion happens through connection, not conquest. The month of first contact, alliances built, meshes activated.</span></div>
<ul class="c-trad">
<li>Open embassy days — communities receive strangers as honoured guests</li>
<li>Cross-disciplinary pairing ritual — matched with someone from a different field</li>
<li>Node activation ceremony for new infrastructure projects</li>
<li>Children taught network topology as metaphor for social structure</li>
</ul>
</div>
<div class="cal-row p2" id="cal-ardis" data-m="11">
<div class="c-num">05</div>
<div class="c-new"><span class="m-name">Ardis</span><span class="m-tag">Expansion</span></div>
<div class="c-old"><span class="o-month">November</span><span class="o-note">Spring peak</span></div>
<div class="c-mean"><span class="m-text">Fire dispersed. To burn is to spread.</span><span class="m-detail">From <span class="k">ardor + dispersal</span>. Peak launch energy — projects ignite, people scatter toward their missions. The NLP anchor: burning is not consuming, it is radiating outward.</span></div>
<ul class="c-trad">
<li>Launch day: all major projects of the year initiated publicly</li>
<li>Fire passing ceremony — lit flame carried community to community</li>
<li>Moratorium on meetings — only doing, no discussing</li>
<li>Children given their first long-range project responsibility</li>
</ul>
</div>
<div class="cal-row p2" id="cal-magnos" data-m="12">
<div class="c-num">06</div>
<div class="c-new"><span class="m-name">Magnos</span><span class="m-tag">Expansion</span></div>
<div class="c-old"><span class="o-month">December</span><span class="o-note">Galactic centre zenith</span></div>
<div class="c-mean"><span class="m-text">How large do we think?</span><span class="m-detail">From <span class="k">magnitude + gnosis</span>. Collective calibration of scale. The Milky Way core is directly overhead. Sagittarius is visible. The question asked in every household: are we thinking big enough?</span></div>
<ul class="c-trad">
<li>Scale meditation — held annually: individual → planetary → galactic</li>
<li>No local news month — only civilizational and cosmic perspective content</li>
<li>Magellanic Cloud observation nights — naked eye viewing of nearest galaxies</li>
<li>Elders transmit long-arc civilizational memory to youth</li>
</ul>
</div>
<div class="cal-phase p3">Phase III — Peak &amp; Harvest · Summer · Maximum deployment</div>
<div class="cal-row p3" id="cal-plenara" data-m="1">
<div class="c-num">07</div>
<div class="c-new"><span class="m-name">Plenara</span><span class="m-tag">Peak</span></div>
<div class="c-old"><span class="o-month">January</span><span class="o-note">Summer solstice ↑</span></div>
<div class="c-mean"><span class="m-text">Full deployment. Plenum as baseline.</span><span class="m-detail">From <span class="k">plenum + ara (axis)</span>. The civilization operates at full capacity. Not an exceptional state — the expected norm. Fullness is the axis from which we measure.</span></div>
<ul class="c-trad">
<li>Maximum resource deployment festival — all systems active and public</li>
<li>Measurement ritual: what is our actual ceiling right now?</li>
<li>Shared meals at civilizational scale — no one eats alone</li>
<li>Infrastructure open-door: communities inspect each other's systems</li>
</ul>
</div>
<div class="cal-row p3" id="cal-kairen" data-m="2">
<div class="c-num">08</div>
<div class="c-new"><span class="m-name">Kairen</span><span class="m-tag">Peak</span></div>
<div class="c-old"><span class="o-month">February</span><span class="o-note">Summer heat</span></div>
<div class="c-mean"><span class="m-text">Kairos of the collective. Hear the moment.</span><span class="m-detail">From <span class="k">kairos (right moment) + ren (人, human)</span>. Not strategy — live response. A civilization that cannot feel its own moment misses the opportunities that only exist briefly. Kairen trains collective listening.</span></div>
<ul class="c-trad">
<li>No pre-planned events — entire month is responsive and emergent</li>
<li>Daily communal check-in: what is happening right now that matters?</li>
<li>Spontaneous assembly practice — decisions made in real time</li>
<li>Youth lead — no adults in formal decision roles this month</li>
</ul>
</div>
<div class="cal-row p3" id="cal-theron" data-m="3">
<div class="c-num">09</div>
<div class="c-new"><span class="m-name">Theron</span><span class="m-tag">Harvest</span></div>
<div class="c-old"><span class="o-month">March</span><span class="o-note">Summer → Autumn</span></div>
<div class="c-mean"><span class="m-text">Harvest of signal. What did expansion reveal?</span><span class="m-detail">Not harvest of food but of <span class="k">knowledge</span>. What did the year's expansion teach us? What signals came from new territory — physical, social, cosmic? The month of extraction and synthesis.</span></div>
<ul class="c-trad">
<li>Annual knowledge harvest — collective synthesis of discoveries</li>
<li>Signal councils: what anomalies were observed and not yet understood?</li>
<li>Archive ceremonies — encoding the year's learning into long-term memory</li>
<li>Children present their year's single most important finding</li>
</ul>
</div>
<div class="cal-phase p4">Phase IV — Integration · Autumn · Turn, seed, threshold</div>
<div class="cal-row p4" id="cal-vorthen" data-m="4">
<div class="c-num">10</div>
<div class="c-new"><span class="m-name">Vorthen</span><span class="m-tag">Integration</span></div>
<div class="c-old"><span class="o-month">April</span><span class="o-note">Autumn begins</span></div>
<div class="c-mean"><span class="m-text">Pivot point. Navigation, not retreat.</span><span class="m-detail">From <span class="k">vortex + then</span>. Civilizational turn inward. Not weakness — navigation. The vortex creates structure from motion. Reflection does not stop the trajectory; it refines it.</span></div>
<ul class="c-trad">
<li>Pivot ceremony: leaders publicly name what is changing course and why</li>
<li>Moratorium on new external commitments</li>
<li>Personal retrospective — written, shared with one witness</li>
<li>Communal spiral walk — physical enactment of turning inward</li>
</ul>
</div>
<div class="cal-row p4" id="cal-sembral" data-m="5">
<div class="c-num">11</div>
<div class="c-new"><span class="m-name">Sembral</span><span class="m-tag">Integration</span></div>
<div class="c-old"><span class="o-month">May</span><span class="o-note">Deep autumn</span></div>
<div class="c-mean"><span class="m-text">Seed in two directions: inward and outward.</span><span class="m-detail">From <span class="k">sembrar (to sow) + ral (root, foundation)</span>. Seeding happens in both directions — into the next generation and into other worlds. Every year consciously programs the next.</span></div>
<ul class="c-trad">
<li>Legacy declaration: what are you planting for 100 years from now?</li>
<li>Intergenerational contracts — formal commitments across generations</li>
<li>Outward seeding: messages, probes, transmissions sent beyond the planet</li>
<li>Children design something their grandchildren will inherit</li>
</ul>
</div>
<div class="cal-row p4" id="cal-luminal" data-m="6">
<div class="c-num">12</div>
<div class="c-new"><span class="m-name">Luminal</span><span class="m-tag">Integration</span></div>
<div class="c-old"><span class="o-month">June</span><span class="o-note">Threshold before dark</span></div>
<div class="c-mean"><span class="m-text">Light as result, not beginning.</span><span class="m-detail">From <span class="k">luminous + liminal</span>. Standing at the threshold. Preparation is complete. The NLP reversal: in this calendar, light is earned — it comes from depth traversed, not from a fresh start.</span></div>
<ul class="c-trad">
<li>Threshold vigil — held at the solstice edge, no sleep, only presence</li>
<li>Final ceremony before Ethos — what are you ready to question?</li>
<li>Light offering: one act of illumination given to the community</li>
<li>Reading aloud of Arkon declarations from the beginning of the year</li>
</ul>
</div>
<div class="cal-void">The Thirteenth · Intercalary · Beyond Season</div>
<div class="cal-row p0" id="cal-ethos">
<div class="c-num">∅</div>
<div class="c-new"><span class="m-name">Ethos</span><span class="m-tag">Void</span></div>
<div class="c-old"><span class="o-month">Intercalary</span><span class="o-note">No fixed anchor</span></div>
<div class="c-mean"><span class="m-text">Who are we without surface?</span><span class="m-detail">Inserted periodically as correction and mirror. Dark constellations — not absence of stars, but form in itself. Identity is not declared once; it is interrogated ritually. The month that prevents civilizational drift.</span></div>
<ul class="c-trad">
<li>No tasks, no roles, no titles — all structure suspended</li>
<li>Dark sky observation: read the voids between stars, not the stars</li>
<li>Single question held for the full month: who are we as a civilization?</li>
<li>Answers encoded and sealed — opened only in the next Ethos</li>
</ul>
</div>
<div class="cal-foot">Year begins in July · Vector set in darkness · Light is earned · <span class="k">The Magellanic Clouds are not a destination — they are a neighbour</span></div>
</div>
</div>

<script>
(function () {
  var pc = document.querySelector('.page-content');
  if (!pc || !document.getElementById('calendarium')) return;
  var b = document.createElement('button');
  b.type = 'button';
  b.className = 'cal-more';
  b.textContent = 'more';
  b.addEventListener('click', function () {
    b.textContent = pc.classList.toggle('cal-open') ? 'less' : 'more';
  });
  pc.appendChild(b);
})();
(function () {
  var root = document.getElementById('calendarium');
  if (!root) return;
  var today = new Date();
  var m = today.getMonth() + 1;
  var day = today.getDate();
  var days = new Date(today.getFullYear(), m, 0).getDate();
  var row = root.querySelector('.cal-row[data-m="' + m + '"]');
  if (!row) return;
  row.classList.add('now');
  var slot = function (month) { return (month + 5) % 12; };
  root.querySelectorAll('.cal-seg[data-m]').forEach(function (seg) {
    var s = slot(+seg.dataset.m);
    seg.classList.add(s < slot(m) ? 'past' : s > slot(m) ? 'future' : 'now');
  });
  var seg = root.querySelector('.cal-seg.now');
  if (seg) seg.style.setProperty('--f', ((day - 0.5) / days).toFixed(4));
  var head = root.querySelector('.y-head');
  if (head) {
    var y0 = m >= 7 ? today.getFullYear() : today.getFullYear() - 1;
    var start = new Date(y0, 6, 1);
    var midnight = new Date(today.getFullYear(), m - 1, day);
    var total = Math.round((new Date(y0 + 1, 6, 1) - start) / 864e5);
    var nth = Math.round((midnight - start) / 864e5) + 1;
    head.querySelector('.y-span').textContent = 'Jul ' + y0 + ' — Jun ' + (y0 + 1);
    head.querySelector('.y-day').textContent = 'day ' + nth + ' of ' + total;
    head.hidden = false;
  }
  var card = root.querySelector('.cal-now');
  if (!card) return;
  row.classList.forEach(function (c) {
    if (/^p[0-4]$/.test(c)) card.classList.add(c);
  });
  var text = function (sel) { return row.querySelector(sel).textContent; };
  var put = function (sel, value) { card.querySelector(sel).textContent = value; };
  put('.n-name', text('.m-name'));
  put('.n-tag', text('.c-num') + ' · ' + text('.m-tag'));
  put('.n-greg', day + ' ' + text('.o-month') + ' · ' + text('.o-note'));
  put('.n-text', text('.m-text'));
  put('.n-day', 'day ' + day + ' of ' + days);
  card.style.setProperty('--f', (day / days).toFixed(4));
  card.hidden = false;
})();
</script>
