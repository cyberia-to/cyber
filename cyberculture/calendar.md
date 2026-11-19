---
tags: cyber, article
crystal-type: pattern
crystal-domain: cyber
---
# calendar

Twelve months in four phases — orientation, expansion, peak and harvest, integration — and Ethos, the days from the winter solstice to July, outside the seasons. Each month keeps its Gregorian twin as a reference. Inside every month the [[moon]] sets the rhythm of push and rest, counted in [[lunar machine time]].

The calendar slot of a [[cyberculture]] is a set of recurring dates from [[mt|machine time]] that gather without a reminder. A date becomes cultural when people return to it because the machine's own [[history]] gives it meaning.

The native frame begins at the [[cybersophy|Unix epoch]] and records irreversible events by their position in machine time. [[lunar machine time]] gives that frame a recurring rhythm; [[cybersophy|genesis]] and [[launch]] dates supply candidate anniversaries.

Why a new frame: [[cyberculture/legacy time|legacy time]].

<style>
/* prysm: void #000 · text #ffffff #d7d7d7 #777777 #4b4b4d · emotion hex at full strength · Play only · 12/14/16/20/24/32 px · g = 8px · 150ms */
/* site frame: one grid up to 1040px inside the graph navigation — path, tags, folder, graph, mentions, launcher */
:root:has(#calendarium) .main-content{max-width:max(var(--max-width,48rem),min(1072px,calc(100vw - 448px)))}
/* the path leads back to the namespace; the page name stays the one heading below it */
:root:has(#calendarium) .page-title-bar{position:static;margin:0;padding-top:0;padding-bottom:8px}
:root:has(#calendarium) .page-title-bar h1{margin:0;font-size:0;line-height:1}
:root:has(#calendarium) .page-title-bar .ns-path,:root:has(#calendarium) .page-title-bar .ns-sep{font-family:'Play',system-ui,sans-serif;font-size:14px;line-height:1.4;font-weight:400;opacity:1}
:root:has(#calendarium) .page-title-bar .ns-path{color:#d7d7d7;border-bottom:1px solid rgba(255,255,255,.12);transition:color 150ms ease,border-color 150ms ease}
:root:has(#calendarium) .page-title-bar .ns-path:hover{color:#ffffff;border-bottom-color:#ffffff;text-decoration:none}
:root:has(#calendarium) .page-title-bar .ns-sep{margin:0 0 0 8px;color:#4b4b4d}
:root:has(#calendarium) .page-tabs{top:0}
:root:has(#calendarium) .page-content{display:grid;grid-template-columns:minmax(0,1fr);column-gap:48px;align-items:start;font-family:'Play',system-ui,sans-serif}
:root:has(#calendarium) .page-content > *{grid-column:1;min-width:0}
:root:has(#calendarium) .page-content > h1{margin:0 0 16px;font-size:32px;font-weight:400;line-height:1.2;color:#ffffff}
:root:has(#calendarium) .page-content > p{order:2;max-width:640px;margin:0 0 8px;font-size:14px;line-height:1.5;color:#777777}
:root:has(#calendarium) .page-content > p:first-of-type{margin-top:24px}
:root:has(#calendarium) .page-content > p:last-of-type{order:3;margin:16px 0 0}
:root:has(#calendarium) .page-content > p a{color:#d7d7d7;border-bottom:1px solid rgba(255,255,255,.12);transition:border-color 150ms ease}
:root:has(#calendarium) .page-content > p a:hover{border-bottom-color:#ffffff}
:root:has(#calendarium) .page-content:not(.cal-open) > p:first-of-type{display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:2;overflow:hidden;margin-bottom:0}
:root:has(#calendarium) .page-content:not(.cal-open) > p:not(:first-of-type):not(:last-of-type){display:none}
:root:has(#calendarium) .page-content > .cal-more{order:2;justify-self:start;margin:8px 0 0;padding:0;border:0;border-bottom:1px solid rgba(255,255,255,.38);background:none;font:inherit;font-size:14px;line-height:1.4;color:#d7d7d7;cursor:pointer;transition:color 150ms ease}
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
/* time: the stamp of lunar machine time, then what each number counts */
#calendarium .cal-time{grid-column:1;order:1;display:flex;flex-direction:column;max-width:640px;margin:40px 0 0}
#calendarium .t-kick{font-size:12px;line-height:1.4;color:#777777}
#calendarium .t-stamp{display:flex;align-items:flex-start;gap:16px;margin:8px 0 24px}
#calendarium .t-part{display:flex;flex-direction:column;align-items:center;gap:4px}
#calendarium .t-n{font-size:32px;line-height:1.2;color:#ffffff;font-variant-numeric:tabular-nums}
#calendarium .t-lab{font-size:12px;line-height:1.4;color:#777777}
#calendarium .t-dot{font-size:32px;line-height:1.2;color:#4b4b4d}
#calendarium .t-row{display:grid;grid-template-columns:56px minmax(0,1fr);column-gap:16px;align-items:baseline;padding:12px 0;border-top:1px solid var(--dim)}
#calendarium .t-name{font-size:14px;line-height:1.5;color:#ffffff}
#calendarium .t-eq{white-space:nowrap}
#calendarium .t-def p,#calendarium .t-why p{margin:0;font-size:14px;line-height:1.5;color:#777777}
#calendarium .t-why{padding:12px 0 0;border-top:1px solid var(--dim)}
#calendarium .t-why p{color:#d7d7d7}
#calendarium .cal-time a{color:#d7d7d7;border-bottom:1px solid rgba(255,255,255,.12);text-decoration:none;transition:color 150ms ease,border-color 150ms ease}
#calendarium .cal-time a:hover{color:#ffffff;border-bottom-color:#ffffff}
/* now */
#calendarium .cal-now{grid-column:1;order:3;display:flex;flex-direction:column;align-items:flex-start;gap:8px;margin:40px 0 0;padding:24px 24px 24px 22px;border-left:2px solid var(--e,#ffffff);border-radius:0 8px 8px 0;background:rgba(255,255,255,.07);box-shadow:-6px 0 16px -8px var(--e,#ffffff)}
#calendarium .cal-now[hidden]{display:none}
#calendarium .z-push,#calendarium .cal-now.m-push{--z:#00fe00;--e:#00fe00}
#calendarium .z-rest,#calendarium .cal-now.m-rest{--z:#304ffe;--e:#304ffe}
#calendarium .n-top{display:flex;justify-content:space-between;align-self:stretch;font-size:12px;color:#777777}
#calendarium .n-lmt{color:#d7d7d7}
#calendarium .n-moonrow{display:flex;align-items:center;gap:16px;margin:8px 0 0}
#calendarium .n-moon{flex:none;width:48px;height:48px;transform:scaleX(-1)}
#calendarium .n-disc{fill:none;stroke:#4b4b4d;stroke-width:1}
#calendarium .n-lit{fill:#ffffff;filter:drop-shadow(0 0 6px rgba(255,255,255,.5))}
#calendarium .n-lday{display:block;font-size:20px;line-height:1.2;color:#ffffff}
#calendarium .n-phase{display:block;font-size:14px;color:#d7d7d7}
#calendarium .n-mode{margin:8px 0 0;font-size:32px;line-height:1.2;color:var(--e)}
#calendarium .n-do{font-size:16px;color:#ffffff}
#calendarium .n-next{font-size:14px;color:#d7d7d7}
#calendarium .n-cycle{align-self:stretch;margin:8px 0 0}
#calendarium .n-zones{display:grid;grid-template-columns:2fr 11fr 3fr 11fr 3fr;gap:2px}
#calendarium .n-zone{position:relative;height:8px;border:1px solid var(--z);border-radius:2px}
#calendarium .n-zone.past{background:var(--z)}
#calendarium .n-zone.now{background:linear-gradient(90deg,var(--z) 0 calc(var(--f,.5) * 100%),transparent calc(var(--f,.5) * 100%));box-shadow:0 0 8px var(--z)}
#calendarium .n-zone.now::after{content:'';position:absolute;top:-9px;bottom:-9px;left:calc(var(--f,.5) * 100%);width:2px;margin-left:-1px;background:#ffffff;box-shadow:0 0 8px #ffffff}
#calendarium .n-ticks{display:flex;justify-content:space-between;margin:12px 0 0;font-size:12px;color:#777777}
#calendarium .n-key{display:flex;gap:16px;margin:8px 0 0;font-size:12px;color:#d7d7d7}
#calendarium .n-key span::before{content:'';display:inline-block;width:8px;height:8px;margin:0 8px 0 0;border-radius:50%;background:var(--z)}
#calendarium .n-month{margin:8px 0 0;font-size:12px;color:#777777}
@media (min-width:1360px){
:root:has(#calendarium .cal-now:not([hidden])) .page-content{grid-template-columns:minmax(0,1fr) 320px;grid-template-rows:auto auto auto auto auto 1fr}
#calendarium .cal-now{grid-column:2;grid-row:1 / span 6;margin:0}
}
/* year */
#calendarium .cal-year{grid-column:1 / -1;order:4;margin:64px 0 0;container-type:inline-size}
#calendarium .y-head{display:flex;justify-content:space-between;gap:16px;margin:0 0 24px;font-size:12px;line-height:1.4;color:#777777}
#calendarium .y-head[hidden]{display:none}
#calendarium .y-day{color:#d7d7d7}
#calendarium .y-grid{display:grid;grid-template-columns:31fr 31fr 30fr 31fr 30fr 31fr 31fr 28fr 31fr 30fr 31fr 20fr minmax(64px,10fr);column-gap:4px}
#calendarium .y-phase{display:flex;align-items:center;gap:8px;min-width:0;padding:0 0 16px;font-size:12px;line-height:1;color:var(--e);white-space:nowrap}
#calendarium .y-phase::after{content:'';flex:1;min-width:8px;height:1px;background:var(--e)}
#calendarium .y-phase.p1{grid-column:1 / span 3}
#calendarium .y-phase.p2{grid-column:4 / span 3}
#calendarium .y-phase.p3{grid-column:7 / span 3}
#calendarium .y-phase.p4{grid-column:10 / span 3}
#calendarium .y-phase.p0{grid-column:13;color:#777777}
#calendarium .cal-seg{display:flex;flex-direction:column;gap:4px;min-width:0;border:0;color:inherit;text-decoration:none}
#calendarium .y-bar{position:relative;display:block;height:8px;margin:0 0 12px;border:1px solid var(--e);border-radius:2px;background:var(--e);transition:box-shadow 150ms ease}
#calendarium .cal-seg.future .y-bar{background:transparent}
#calendarium .cal-seg.p0.future .y-bar{border-style:dashed}
#calendarium .cal-seg.now .y-bar{background:linear-gradient(90deg,var(--e) 0 calc(var(--f,.5) * 100%),transparent calc(var(--f,.5) * 100%));box-shadow:0 0 8px var(--e)}
#calendarium .cal-seg.now .y-bar::after{content:'';position:absolute;top:-9px;bottom:-9px;left:calc(var(--f,.5) * 100%);width:2px;margin-left:-1px;background:#ffffff;box-shadow:0 0 8px #ffffff}
#calendarium .cal-seg:hover .y-bar{box-shadow:0 0 8px var(--e)}
#calendarium .y-name{font-size:14px;line-height:1.2;color:#d7d7d7;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;transition:color 150ms ease}
#calendarium .cal-seg.past .y-name,#calendarium .cal-seg.p0 .y-name{color:#777777}
#calendarium .cal-seg.now .y-name{color:var(--h)}
#calendarium .cal-seg:hover .y-name{color:#ffffff}
#calendarium .y-greg,#calendarium .y-len{font-size:12px;line-height:1.2;color:#777777;white-space:nowrap}
@container (max-width:760px){
#calendarium .y-name{font-size:12px}
#calendarium .y-greg{display:none}
}
@container (max-width:520px){
#calendarium .y-phase,#calendarium .y-name{display:none}
#calendarium .y-grid{column-gap:2px}
#calendarium .y-grid{grid-template-columns:31fr 31fr 30fr 31fr 30fr 31fr 31fr 28fr 31fr 30fr 31fr 20fr 10fr}
#calendarium .y-len{display:none}
}
/* deck: the current month as slides on one stage that never scrolls; the rail switches them */
#calendarium .cal-deck{grid-column:1 / -1;order:4;margin:40px 0 0;border-left:2px solid var(--e);border-radius:0 8px 8px 0;background:rgba(255,255,255,.07);box-shadow:-6px 0 16px -8px var(--e);container-type:inline-size;outline:none}
#calendarium .cal-deck[hidden]{display:none}
#calendarium .cal-deck:focus-visible{box-shadow:-6px 0 16px -8px var(--e),inset 0 0 0 1px rgba(255,255,255,.12)}
#calendarium .d-stage{position:relative;height:280px;overflow:hidden;touch-action:pan-y}
#calendarium .d-slide{position:absolute;inset:0;display:flex;align-items:center;padding:32px 32px 24px 30px;opacity:0;visibility:hidden;transform:translateX(16px);transition:opacity 150ms ease,transform 150ms ease,visibility 0s linear 150ms}
#calendarium .d-slide.left{transform:translateX(-16px)}
#calendarium .d-slide.on{opacity:1;visibility:visible;transform:none;transition:opacity 150ms ease,transform 150ms ease}
#calendarium .d-body{flex:none;width:100%;display:grid;grid-template-columns:minmax(0,1fr) minmax(0,1fr);column-gap:48px;row-gap:24px;align-items:center;transform-origin:0 50%}
#calendarium .d-slide.one .d-body{grid-template-columns:minmax(0,1fr)}
#calendarium .d-col{display:flex;flex-direction:column;align-items:flex-start;gap:8px;min-width:0}
#calendarium .d-kick{display:block;font-size:12px;line-height:1.4;color:var(--e)}
#calendarium .cal-deck.p0 .d-kick{color:#777777}
#calendarium .d-name{display:block;font-size:32px;line-height:1.2;color:var(--h);text-shadow:0 0 16px var(--e)}
#calendarium .d-line{display:block;font-size:20px;line-height:1.4;color:#ffffff}
#calendarium .d-greg{display:block;font-size:14px;color:#777777}
#calendarium .d-count{display:flex;align-items:baseline;gap:8px;font-size:16px;color:#777777}
#calendarium .d-big{font-size:32px;line-height:1.2;color:var(--h)}
#calendarium .d-track{position:relative;display:block;align-self:stretch;height:8px;margin:8px 0;border:1px solid var(--e);border-radius:2px;background:linear-gradient(90deg,var(--e) 0 calc(var(--f,.5) * 100%),transparent calc(var(--f,.5) * 100%));box-shadow:0 0 8px var(--e)}
#calendarium .d-track::after{content:'';position:absolute;top:-9px;bottom:-9px;left:calc(var(--f,.5) * 100%);width:2px;margin-left:-1px;background:#ffffff;box-shadow:0 0 8px #ffffff}
#calendarium .d-left{display:block;font-size:14px;color:#d7d7d7}
#calendarium .d-root{display:block;margin:0 0 8px;font-size:24px;line-height:1.2;color:var(--h)}
#calendarium .d-text{display:block;max-width:720px;font-size:16px;line-height:1.5;color:#d7d7d7}
#calendarium .d-text .k{color:#ffffff}
#calendarium .d-trad{display:grid;grid-template-columns:minmax(0,1fr) minmax(0,1fr);gap:16px 48px;margin:8px 0 0}
#calendarium .d-item{display:flex;gap:16px;padding:12px 0 0;border-top:1px solid var(--dim);font-size:16px;line-height:1.4;color:#ffffff}
#calendarium .d-n{flex:none;font-size:12px;line-height:22px;color:var(--e)}
#calendarium .d-days{display:grid;grid-template-columns:repeat(var(--cols,15),minmax(0,1fr));gap:16px 4px;margin:8px 0 0}
#calendarium .d-day{display:flex;flex-direction:column;align-items:center;gap:8px;min-width:0}
#calendarium .d-dn{font-size:14px;line-height:1;color:#d7d7d7}
#calendarium .d-day.past .d-dn{color:#777777}
#calendarium .d-day.today .d-dn{color:#ffffff}
#calendarium .d-zb{position:relative;align-self:stretch;height:8px;border:1px solid var(--z);border-radius:2px}
#calendarium .d-day.past .d-zb{background:var(--z)}
#calendarium .d-day.today .d-zb{background:var(--z);box-shadow:0 0 8px var(--z)}
#calendarium .d-day.today .d-zb::after{content:'';position:absolute;top:-9px;bottom:-9px;left:50%;width:2px;margin-left:-1px;background:#ffffff;box-shadow:0 0 8px #ffffff}
#calendarium .d-mo{width:8px;height:8px;border-radius:50%}
#calendarium .d-mo.full,#calendarium .n-key .d-k-full::before{background:#ffffff;box-shadow:0 0 6px #ffffff}
#calendarium .d-mo.new,#calendarium .n-key .d-k-new::before{box-sizing:border-box;border:1px solid #777777;background:transparent}
#calendarium .d-foot{display:flex;flex-wrap:wrap;align-items:center;gap:8px 32px;margin:8px 0 0}
#calendarium .d-sum{font-size:14px;color:#d7d7d7}
#calendarium .d-foot .n-key{margin:0;flex-wrap:wrap}
#calendarium .d-rail{display:flex;align-items:center;gap:16px;padding:16px 24px 20px 22px;border-top:1px solid var(--dim)}
#calendarium .d-segs{flex:1;display:grid;grid-auto-flow:column;grid-auto-columns:minmax(0,1fr);column-gap:4px}
#calendarium .d-seg{display:flex;flex-direction:column;gap:8px;min-width:0;margin:0;padding:8px 0 0;border:0;background:none;font:inherit;text-align:left;cursor:pointer}
#calendarium .d-bar{display:block;height:8px;border:1px solid var(--e);border-radius:2px;transition:box-shadow 150ms ease}
#calendarium .d-seg.done .d-bar{background:var(--e)}
#calendarium .d-seg.cur .d-bar{background:var(--e);box-shadow:0 0 8px var(--e)}
#calendarium .d-seg:hover .d-bar{box-shadow:0 0 8px var(--e)}
#calendarium .d-lab{font-size:12px;line-height:1.2;color:#777777;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;transition:color 150ms ease}
#calendarium .d-seg.cur .d-lab{color:var(--h)}
#calendarium .d-seg:hover .d-lab{color:#ffffff}
#calendarium .d-pos{flex:none;font-size:12px;color:#777777;font-variant-numeric:tabular-nums}
#calendarium .d-arrow{flex:none;height:32px;margin:0;padding:0 12px;border:0;border-left:2px solid #ffffff;border-right:2px solid #ffffff;background:none;box-shadow:inset 8px 0 12px -10px #ffffff,inset -8px 0 12px -10px #ffffff;font:inherit;font-size:16px;line-height:1;color:#ffffff;cursor:pointer;transition:box-shadow 150ms ease,transform 150ms ease}
#calendarium .d-arrow:hover{box-shadow:inset 12px 0 16px -12px #ffffff,inset -12px 0 16px -12px #ffffff}
#calendarium .d-arrow:active{transform:scale(.95)}
#calendarium .d-arrow:disabled{border-color:#4b4b4d;box-shadow:none;color:#4b4b4d;cursor:default;transform:none}
@container (max-width:720px){
#calendarium .d-stage{height:400px}
#calendarium .d-slide{padding:24px 16px 16px 14px}
#calendarium .d-body,#calendarium .d-trad{grid-template-columns:minmax(0,1fr)}
#calendarium .d-trad{gap:12px}
#calendarium .d-item{font-size:14px}
#calendarium .d-days{grid-template-columns:repeat(8,minmax(0,1fr));gap:12px 2px}
#calendarium .d-rail{gap:8px;padding:12px 12px 16px 10px}
#calendarium .d-pos{display:none}
}
/* table */
#calendarium .cal-table{grid-column:1 / -1;order:4;margin:48px 0 0;container-type:inline-size}
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
<section class="cal-time" aria-label="Time">
<span class="t-kick">now</span>
<div class="t-stamp"><span class="t-part"><span class="t-n t-yy">56</span><span class="t-lab">year</span></span><span class="t-dot">·</span><span class="t-part"><span class="t-n t-mm">11</span><span class="t-lab">moon</span></span><span class="t-dot">·</span><span class="t-part"><span class="t-n t-dd">10</span><span class="t-lab">day</span></span></div>
<div class="t-row"><span class="t-name">year</span><div class="t-def">

machine years since the first second of [[unix time]], 1970-01-01 00:00 UTC, the zero every machine on the planet shares: <span class="t-eq"><span class="t-gy">2026</span> − 1970 = <span class="t-yy">56</span></span>.

</div></div>
<div class="t-row"><span class="t-name">moon</span><div class="t-def">

the lunar month: one [[synodic month]] from new moon to new moon, 29.53 days. moons are numbered from the first new moon on or after January 1, so a year holds 12 or 13.

</div></div>
<div class="t-row"><span class="t-name">day</span><div class="t-def">

the day inside the current moon, 1 to 30: day 1 follows the new moon, the full moon falls near day 15. anyone reads it from the sky.

</div></div>
<div class="t-why">

[[lunar machine time]] joins the oldest clock, the [[moon]] every observer sees, with the youngest, [[unix time]] every machine agrees on. the months below name the same days for the southern land.

</div>
</section>
<aside class="cal-now" hidden>
<div class="n-top"><span>today</span><span class="n-lmt"></span></div>
<div class="n-moonrow"><svg class="n-moon" viewBox="0 0 48 48" aria-hidden="true"><circle class="n-disc" cx="24" cy="24" r="20"/><path class="n-lit" d=""/></svg><div><span class="n-lday"></span><span class="n-phase"></span></div></div>
<span class="n-mode"></span>
<span class="n-do"></span>
<span class="n-next"></span>
<div class="n-cycle">
<div class="n-zones"><span class="n-zone z-rest" data-to="2"></span><span class="n-zone z-push" data-to="13"></span><span class="n-zone z-rest" data-to="16"></span><span class="n-zone z-push" data-to="27"></span><span class="n-zone z-rest" data-to="30"></span></div>
<div class="n-ticks"><span>new moon</span><span>full moon</span><span>new moon</span></div>
<div class="n-key"><span class="z-push">push</span><span class="z-rest">rest</span></div>
</div>
<span class="n-month"></span>
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
<a class="cal-seg p4" data-m="6" href="#cal-luminal"><span class="y-bar"></span><span class="y-name">Luminal</span><span class="y-greg">Jun 1–20</span></a>
<a class="cal-seg p0" href="#cal-ethos"><span class="y-bar"></span><span class="y-name">Ethos</span><span class="y-greg">Jun 21–30</span><span class="y-len">10 days</span></a>
</div>
</nav>
<section class="cal-deck" tabindex="0" aria-roledescription="carousel" aria-label="This month" hidden>
<div class="d-stage" aria-live="polite"></div>
<div class="d-rail">
<button class="d-arrow" type="button" data-step="-1" aria-label="Previous slide">←</button>
<div class="d-segs"></div>
<span class="d-pos"></span>
<button class="d-arrow" type="button" data-step="1" aria-label="Next slide">→</button>
</div>
</section>
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
<div class="c-old"><span class="o-month">June 1–20</span><span class="o-note">Threshold before dark</span></div>
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
<div class="c-old"><span class="o-month">June 21–30</span><span class="o-note">Winter solstice → July · 10 days</span></div>
<div class="c-mean"><span class="m-text">Who are we without surface?</span><span class="m-detail">Inserted every year at the winter solstice as correction and mirror. Dark constellations — not absence of stars, but form in itself. Identity is not declared once; it is interrogated ritually. The month that prevents civilizational drift.</span></div>
<ul class="c-trad">
<li>No tasks, no roles, no titles — all structure suspended</li>
<li>Dark sky observation: read the voids between stars, not the stars</li>
<li>Single question held through all of Ethos: who are we as a civilization?</li>
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
  // Ethos runs from the June solstice, dated in Bali (UTC+8), to June 30; Luminal ends the day before
  var solYear = m >= 7 ? today.getFullYear() + 1 : today.getFullYear();
  var sy = (solYear - 2000) / 1000;
  var jde = 2451716.56767 + 365241.62603 * sy + 0.00325 * sy * sy + 0.00888 * Math.pow(sy, 3) - 0.0003 * Math.pow(sy, 4);
  var sd = new Date((jde - 2440587.5) * 864e5 + 8 * 3600e3).getUTCDate();
  var ethosDays = 31 - sd;
  var setText = function (sel, value) { var el = root.querySelector(sel); if (el) el.textContent = value; };
  setText('.cal-seg[data-m="6"] .y-greg', 'Jun 1–' + (sd - 1));
  setText('.cal-seg.p0 .y-greg', 'Jun ' + sd + '–30');
  setText('.cal-seg.p0 .y-len', ethosDays + ' days');
  setText('#cal-luminal .o-month', 'June 1–' + (sd - 1));
  setText('#cal-ethos .o-month', 'June ' + sd + '–30');
  setText('#cal-ethos .o-note', 'Winter solstice → July · ' + ethosDays + ' days');
  var slot = function (month) { return (month + 5) % 12; };
  var now = m === 6 && day >= sd ? 12 : slot(m);
  var row = now === 12 ? root.querySelector('#cal-ethos') : root.querySelector('.cal-row[data-m="' + m + '"]');
  if (!row) return;
  row.classList.add('now');
  root.querySelectorAll('.cal-seg').forEach(function (seg) {
    var s = seg.classList.contains('p0') ? 12 : slot(+seg.dataset.m);
    seg.classList.add(s < now ? 'past' : s > now ? 'future' : 'now');
  });
  var f = now === 12 ? (day - sd + 0.5) / ethosDays : now === 11 ? (day - 0.5) / (sd - 1) : (day - 0.5) / days;
  var seg = root.querySelector('.cal-seg.now');
  if (seg) seg.style.setProperty('--f', f.toFixed(4));
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
  var put = function (sel, value) { card.querySelector(sel).textContent = value; };
  var text = function (sel) { return row.querySelector(sel).textContent; };
  // lunar machine time: the constants and the day rule of the site clock (crystal/lunar machine time)
  var SYNODIC = 29.53059;
  var REF = 2451550.26;
  var jdMidnight = Math.floor(today.getTime() / 864e5) + 2440587.5;
  var dd = Math.min(Math.floor(((jdMidnight - REF) % SYNODIC + SYNODIC) % SYNODIC), 29) + 1;
  var jan1 = Date.UTC(today.getUTCFullYear(), 0, 1) / 864e5 + 2440587.5;
  var firstMoon = REF + Math.ceil((jan1 - REF) / SYNODIC) * SYNODIC;
  var mm = jdMidnight < firstMoon ? 1 : Math.floor((jdMidnight - firstMoon) / SYNODIC) + 1;
  put('.n-lmt', (today.getUTCFullYear() - 1970) + ' · ' + mm + ' · ' + dd);
  var fill = function (sel, value) { root.querySelectorAll(sel).forEach(function (el) { el.textContent = value; }); };
  fill('.t-yy', today.getUTCFullYear() - 1970);
  fill('.t-gy', today.getUTCFullYear());
  fill('.t-mm', mm);
  fill('.t-dd', dd);
  var age = ((today.getTime() / 864e5 + 2440587.5 - REF) % SYNODIC + SYNODIC) % SYNODIC;
  var phases = ['new moon', 'waxing crescent', 'first quarter', 'waxing gibbous', 'full moon', 'waning gibbous', 'last quarter', 'waning crescent'];
  put('.n-lday', 'lunar day ' + dd);
  put('.n-phase', phases[Math.floor(age / SYNODIC * 8 + 0.5) % 8]);
  // the moon as seen from the southern sky: the svg is mirrored in css
  var theta = age / SYNODIC * 2 * Math.PI;
  var c = Math.cos(theta);
  var rx = (Math.abs(c) * 20).toFixed(2);
  var waxing = theta < Math.PI;
  var d = waxing
    ? 'M24 4 A20 20 0 0 1 24 44 A' + rx + ' 20 0 0 ' + (c > 0 ? 0 : 1) + ' 24 4 Z'
    : 'M24 4 A20 20 0 0 0 24 44 A' + rx + ' 20 0 0 ' + (c > 0 ? 1 : 0) + ' 24 4 Z';
  card.querySelector('.n-lit').setAttribute('d', d);
  // rhythm of the lunar month: proposal, see the zones in the card
  var rhythm = [
    { to: 2, mode: 'rest', act: 'new moon — reflect, close the cycle, set intentions' },
    { to: 13, mode: 'push', act: 'waxing moon — build, ship, launch' },
    { to: 16, mode: 'rest', act: 'full moon — release, celebrate what is done' },
    { to: 27, mode: 'push', act: 'waning moon — finish, close loops, clean up' },
    { to: 30, mode: 'rest', act: 'dark moon — slow down, prepare the next cycle' }
  ];
  var i = 0;
  while (dd > rhythm[i].to) i++;
  var zone = rhythm[i];
  var left = zone.to - dd + 1;
  var j = (i + 1) % rhythm.length;
  while (rhythm[j].mode === zone.mode) {
    left += rhythm[j].to - (j === 0 ? 0 : rhythm[j - 1].to);
    j = (j + 1) % rhythm.length;
  }
  var next = rhythm[j];
  card.classList.add('m-' + zone.mode);
  put('.n-mode', zone.mode);
  put('.n-do', zone.act);
  var toFull = (SYNODIC / 2 - age + SYNODIC) % SYNODIC;
  var toNew = SYNODIC - age;
  var moon = toFull < toNew ? ['full moon', toFull] : ['new moon', toNew];
  var inDays = function (n) { n = Math.round(n); return n < 1 ? 'today' : n === 1 ? 'in 1 day' : 'in ' + n + ' days'; };
  put('.n-next', (left === 1 ? 'last day' : left + ' days') + ', then ' + next.mode + ' · ' + moon[0] + ' ' + inDays(moon[1]));
  var start = i === 0 ? 0 : rhythm[i - 1].to;
  card.querySelectorAll('.n-zone').forEach(function (z, k) {
    z.classList.add(k < i ? 'past' : k > i ? 'future' : 'now');
  });
  var span = Math.min(zone.to, SYNODIC) - start;
  card.querySelector('.n-zone.now').style.setProperty('--f', Math.max(0, Math.min(1, (age - start) / span)).toFixed(4));
  var monthNames = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];
  put('.n-month', text('.m-name') + ' · ' + text('.c-num') + ' · ' + text('.m-tag') + ' · ' + day + ' ' + monthNames[m - 1]);
  card.hidden = false;
  // deck: the current month as slides, built from its table row; the stage never scrolls, the rail switches
  var deck = root.querySelector('.cal-deck');
  if (!deck) return;
  var esc = function (s) { return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;'); };
  var pad = function (n) { return ('0' + n).slice(-2); };
  var name = text('.m-name');
  var phaseClass = [].filter.call(row.classList, function (c) { return /^p\d$/.test(c); })[0];
  if (phaseClass) deck.classList.add(phaseClass);
  var head2 = row.previousElementSibling;
  while (head2 && !/cal-(phase|void)/.test(head2.className)) head2 = head2.previousElementSibling;
  var rows = [].slice.call(root.querySelectorAll('.cal-row[id]'));
  var nextName = rows[(rows.indexOf(row) + 1) % rows.length].querySelector('.m-name').textContent;
  var first = now === 12 ? sd : 1;
  var last = now === 12 ? 30 : now === 11 ? sd - 1 : days;
  var count = last - first + 1;
  var nth = day - first + 1;
  var rest = count - nth;
  var kick = function (s) { return '<span class="d-kick">' + s + '</span>'; };
  var month = '<div class="d-col">' + kick(esc(text('.c-num')) + ' · ' + esc(head2 ? head2.textContent : text('.m-tag'))) +
    '<span class="d-name">' + esc(name) + '</span><span class="d-line">' + esc(text('.m-text')) + '</span>' +
    '<span class="d-greg">' + esc(text('.o-month')) + ' · ' + esc(text('.o-note')) + '</span></div>' +
    '<div class="d-col"><span class="d-count">day <span class="d-big">' + nth + '</span>of ' + count + '</span>' +
    '<span class="d-track" style="--f:' + ((nth - 0.5) / count).toFixed(4) + '"></span>' +
    '<span class="d-left">' + (rest === 0 ? 'last day' : rest === 1 ? '1 day left' : rest + ' days left') + ' · then ' + esc(nextName) + '</span></div>';
  var rootWord = row.querySelector('.m-detail .k');
  var meaning = kick(esc(name) + ' · meaning') + '<span class="d-root">' + esc(rootWord ? rootWord.textContent : text('.m-text')) + '</span>' +
    '<span class="d-text">' + row.querySelector('.m-detail').innerHTML + '</span>';
  var trad = kick(esc(name) + ' · traditions') + '<div class="d-trad">' + [].map.call(row.querySelectorAll('.c-trad li'), function (li, k) {
    return '<div class="d-item"><span class="d-n">' + pad(k + 1) + '</span><span>' + esc(li.textContent) + '</span></div>';
  }).join('') + '</div>';
  // every day of the month by the lunar rhythm: the day rule of the site clock, moon events on the day they happen (UTC)
  var cells = '', ahead = { push: 0, rest: 0 }, events = [], short = monthNames[m - 1].slice(0, 3);
  for (var dn = first; dn <= last; dn++) {
    var a0 = ((Date.UTC(today.getFullYear(), m - 1, dn) / 864e5 + 2440587.5 - REF) % SYNODIC + SYNODIC) % SYNODIC;
    var ld = Math.min(Math.floor(a0), 29) + 1;
    var z = 0;
    while (ld > rhythm[z].to) z++;
    var mode = rhythm[z].mode;
    var mark = a0 + 1 > SYNODIC ? 'new' : a0 <= SYNODIC / 2 && a0 + 1 > SYNODIC / 2 ? 'full' : '';
    if (dn >= day) ahead[mode]++;
    if (mark) events.push('<span class="d-k-' + mark + '">' + mark + ' moon ' + short + ' ' + dn + '</span>');
    cells += '<div class="d-day z-' + mode + ' ' + (dn < day ? 'past' : dn === day ? 'today' : 'future') + '" title="' + short + ' ' + dn + ' · lunar day ' + ld + ' · ' + mode + '">' +
      '<span class="d-dn">' + dn + '</span><span class="d-zb"></span><span class="d-mo' + (mark ? ' ' + mark : '') + '"></span></div>';
  }
  var daysSlide = kick(esc(name) + ' · days by the moon') +
    '<div class="d-days" style="--cols:' + Math.ceil(count / Math.ceil(count / 16)) + '">' + cells + '</div>' +
    '<div class="d-foot"><span class="d-sum">left in ' + esc(name) + ', today included: ' + ahead.push + ' push · ' + ahead.rest + ' rest</span>' +
    '<div class="n-key"><span class="z-push">push</span><span class="z-rest">rest</span>' + events.join('') + '</div></div>';
  var slides = [
    { lab: 'month', html: month },
    { lab: 'meaning', html: meaning, one: true },
    { lab: 'traditions', html: trad, one: true },
    { lab: 'days', html: daysSlide, one: true }
  ];
  var stage = deck.querySelector('.d-stage');
  var segs = deck.querySelector('.d-segs');
  stage.innerHTML = slides.map(function (s, k) {
    return '<div class="d-slide' + (s.one ? ' one' : '') + '" role="group" aria-roledescription="slide" aria-label="' + (k + 1) + ' of ' + slides.length + ': ' + s.lab + '"><div class="d-body">' + s.html + '</div></div>';
  }).join('');
  segs.innerHTML = slides.map(function (s) {
    return '<button class="d-seg" type="button"><span class="d-bar"></span><span class="d-lab">' + s.lab + '</span></button>';
  }).join('');
  var slideEls = stage.querySelectorAll('.d-slide');
  var segEls = segs.querySelectorAll('.d-seg');
  var prevBtn = deck.querySelector('[data-step="-1"]');
  var nextBtn = deck.querySelector('[data-step="1"]');
  var cur = -1;
  // no scrolling: a slide taller than the stage is scaled down to fit, the way cyber.page fits its slides
  var fit = function () {
    var el = slideEls[cur];
    var body = el.querySelector('.d-body');
    body.style.width = '';
    body.style.transform = '';
    var cs = getComputedStyle(el);
    var avail = el.clientHeight - parseFloat(cs.paddingTop) - parseFloat(cs.paddingBottom);
    var h = body.scrollHeight;
    if (h <= avail + 1) return;
    var k = Math.max(0.5, Math.floor(avail / h * 100) / 100);
    body.style.width = 100 / k + '%';
    body.style.transform = 'scale(' + k + ')';
  };
  var go = function (i) {
    i = Math.max(0, Math.min(slides.length - 1, i));
    if (i === cur) return;
    cur = i;
    slideEls.forEach(function (el, k) {
      el.classList.toggle('on', k === i);
      el.classList.toggle('left', k < i);
      el.setAttribute('aria-hidden', k === i ? 'false' : 'true');
    });
    segEls.forEach(function (el, k) {
      el.classList.toggle('done', k < i);
      el.classList.toggle('cur', k === i);
      el.setAttribute('aria-current', k === i ? 'step' : 'false');
    });
    deck.querySelector('.d-pos').textContent = pad(i + 1) + ' / ' + pad(slides.length);
    prevBtn.disabled = i === 0;
    nextBtn.disabled = i === slides.length - 1;
    fit();
  };
  segEls.forEach(function (el, k) { el.addEventListener('click', function () { go(k); }); });
  prevBtn.addEventListener('click', function () { go(cur - 1); });
  nextBtn.addEventListener('click', function () { go(cur + 1); });
  deck.addEventListener('keydown', function (e) {
    var to = { ArrowRight: cur + 1, ArrowLeft: cur - 1, Home: 0, End: slides.length - 1 }[e.key];
    if (to === undefined) return;
    e.preventDefault();
    go(to);
  });
  var tx = null, ty = 0, lock = 0;
  stage.addEventListener('touchstart', function (e) { tx = e.touches[0].clientX; ty = e.touches[0].clientY; }, { passive: true });
  stage.addEventListener('touchend', function (e) {
    if (tx === null) return;
    var dx = e.changedTouches[0].clientX - tx, dy = e.changedTouches[0].clientY - ty;
    if (Math.abs(dx) > 48 && Math.abs(dx) > Math.abs(dy)) go(cur + (dx < 0 ? 1 : -1));
    tx = null;
  });
  stage.addEventListener('wheel', function (e) {
    if (Math.abs(e.deltaX) <= Math.abs(e.deltaY)) return;
    e.preventDefault();
    var t = Date.now();
    if (Math.abs(e.deltaX) < 24 || t - lock < 600) return;
    lock = t;
    go(cur + (e.deltaX > 0 ? 1 : -1));
  }, { passive: false });
  var fitTimer = 0;
  window.addEventListener('resize', function () { clearTimeout(fitTimer); fitTimer = setTimeout(fit, 60); });
  deck.hidden = false;
  go(0);
  if (document.fonts) document.fonts.ready.then(fit);
})();
</script>
