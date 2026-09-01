---
title: restructure
tags: cyber, core, plan
crystal-type: pattern
crystal-domain: cyber
status: plan
---
# restructure — crystallizing cyb · cyber · cyberia

a plan first, movements second. nothing in this document moves a file; every section is a contract for one commit series, link-fixes included, so the site is never broken in between. every page named below is a live link — click through and judge it. the three ledgers carry a verdict for **all 501 pages** (238 cyber · 83 cyb · 180 cyberia): stays / promotes / merges / moves / folds / extracts / dies — with the destination and the reason. zero pages undecided.

## the rule

membership is decided by one question — **who is the subject of this page?** — so the split is idempotent: run the rule twice, nothing moves the second time.

| the subject is… | it lives in |
|---|---|
| the protocol — the law of the one mind | **cyber** |
| the robot — the body a user touches | **cyb** |
| the nation — land, people, state | **cyberia** |
| one soft3 component's mechanism | that component's repo |
| the world at large | [[crystal]] / [[cybics]] |
| a dead vehicle | [[bootloader]] |

corollary: **cyber is not [[soft3]].** soft3 owns mechanisms; cyber owns what must be true regardless of mechanism. a page explaining *how* something is computed belongs to the component that computes it; a page stating *what must hold* belongs to cyber.

---

# 1 · cyber — target structure

the protocol crystal: **7 modules, 133 pages** (+ chronicle). every page listed; *italics* = to be written or renamed in this plan.

**index** (4): [[README]] · [[blog]] · [[research]] · [[concepts]]

**core — the phenomenon** (42 atoms):
[[cybergraph]] · [[particle]] · [[neuron]] · [[cyberlink]] · [[token]] · [[focus]] · [[rank]] *(absorbs [[cyberank]])* · [[karma]] · [[attention]] · [[staking]] · [[mining]] · [[rewards]] · [[impulse]] · [[intent]] · [[inception]] · [[sealing]] · [[cascade]] · [[cell]] · [[view]] · [[prob]] · [[rational neuron]] · [[fixed point]] · [[tri-kernel]] · [[tru]] · [[knowledge]] · [[hash]] · [[superintelligence]] · [[interplanetary superintelligence]] · [[self]] · [[happiness]] · [[personality]] · [[landauer limit]] · [[crystal]] · [[core]] · [[subgraphs]] · [[license]] · [[install]] · [[contribute]] · [[context packing]] · [[cyberculture]] · [[cybersophy]] · [[cybersphere]] + [[cyberverse]]

**contract — invariants over soft3** (6): [[cyber/cybergraph]] · [[cyber/$CYB|$CYB]] · [[cyber/launch]] · *cyber/epistemology* (promotion of [[epistemology]]) · *cyber/identity* (promotion of [[identity]]) · *cyber/proofs* (to write — 5 inbound refs already wait for it)

**nomics — the money** (15): [[cybernomics]] · [[nomics]] · [[netics]] · [[cap]] · [[delegation]] · [[automated market maker]] · [[cyberlink market protocol]] · [[tokens]] · [[tokens/plumb|plumb]] · [[tokens/coin|coin]] · [[tokens/badge|badge]] · [[tokens/basic token operations|basic token operations]] · [[specs/adaptive hybrid economics|adaptive hybrid economics]] · [[specs/light-money|light money]] · [[specs/money-loop|money loop]]

**cips — deep protocol articles** (9): [[3c]] · [[channel]] · [[communication]] · [[network]] · [[hierarchy]] · [[light]] · [[gravity]] · [[luminosity]] · [[security]]

**papers** (2 + 20 research): [[litepaper]] · [[whitepaper]] ·
[[research/physical analogies|physical analogies]] · [[research/algorithmic essence of superintelligence|algorithmic essence of superintelligence]] · [[research/knowledge capacity|knowledge capacity]] · [[research/link production|link production]] · [[research/universal law|universal law]] · [[research/egregore properties|egregore properties]] · [[research/future of computation|future of computation]] · [[research/neuroscience principles for machine mind|neuroscience principles for machine mind]] · [[research/unified mining|unified mining]] · [[research/knowledge economy|knowledge economy]] · [[research/theoretical foundations|theoretical foundations]] · [[research/bootstrap|bootstrap]] · [[research/32-byte tokens|32-byte tokens]] · [[research/energy market|energy market]] · [[research/gradient descent|gradient descent]] · [[research/cybergraph model architecture|cybergraph model architecture]] · [[research/focus flow computation|focus flow computation]] · [[research/adaptive hybrid economics|adaptive hybrid economics (research)]] · [[analizer]] *(→ research/)* · [[seer]] *(→ research/)*

**specs — engineering contracts** (5): [[specs/README|specs index]] · [[specs/component-ownership|component ownership]] · [[specs/full-flow-claims|full-flow claims]] · [[specs/node-modes|node modes]] · [[specs/rewards-completeness|rewards completeness]]

**self — the protocol as a neuron** (4): [[self/dmn|dmn]] · [[self/linking|linking]] · [[self/parametrization|parametrization]] · [[self/sigma|sigma]]

**chronicle**: blog/ keeps every real entry; the dated one-liners are log lines, not drafts. they split by subject (valley entries → cyberia's chronicle), the two empty ones die ([[blog/2024_09_29]], [[blog/2025_02_15]]).

# 2 · cyb — target structure

the robot: **31 pages**, one flat module set. the repo's `root/` directory dissolves — pages live flat like every other crystal. current pages linked at today's paths.

**spine** (8): [[cyb/root/README|robot (README)]] · [[cyb/root/product|product]] · [[cyb/root/philosophy|philosophy]] · [[cyb/root/story|story]] · [[cyb/root/spec|spec]] · [[cyb/root/roadmap|roadmap]] · [[cyb/root/team|team]] · [[cyb/root/releases/v0.1.0|releases]]

**reference** (17): [[cyb/root/rust-architecture|architecture]] *(absorbs [[cyb/root/architecture|the JS-era architecture]])* · [[cyb/root/rendering|rendering]] · [[cyb/root/routing|routing]] · [[cyb/root/terminal|terminal]] · [[cyb/root/cells|cells]] · [[cyb/root/apps|apps]] · [[cyb/root/avatar|avatar]] · [[cyb/root/format|format]] · [[cyb/root/cyb-registry|cyb-registry]] · [[cyb/root/cyb-model|cyb-model]] · [[cyb/root/cyb-vocab|cyb-vocab]] · [[cyb/root/fs|fs]] · [[cyb/root/scripting|scripting]] · [[cyb/root/truth|truth]] · [[cyb/root/features|features]] · [[cyb/root/android|android]] · [[cyb/root/release-process|release-process]]

**ops** (4): [[cyb/root/dev|dev]] · [[cyb/root/benchmarks|benchmarks]] · [[cyb/root/ward|ward]] · *legacy/README* (one page explaining the JS era, written at fold time)

**incoming** (2): [[security audit private key import]] (from cyber) · plans stay in `.claude/plans/`

# 3 · cyberia — target structure

the nation: **six shelves**, each a directory with a contract README.

**foundation** (9, stays as is): [[cyberia/foundation/vision|vision]] · [[cyberia/foundation/manifesto|manifesto]] · [[cyberia/foundation/architecture|architecture]] · [[cyberia/foundation/strategy|strategy]] · [[cyberia/foundation/whitepaper|whitepaper]] · [[cyberia/foundation/space doctrine|space doctrine]] · [[cyberia/foundation/org|org]] · [[cyberia/foundation/cyberian|cyberian]] · [[cyberia/foundation/README|README]]

**protocol** (14): [[cyberia/protocol/README|README]] · [[cyberia/protocol/century-index|century index]] · [[cyberia/protocol/bank-above-banks|bank above banks]] · [[cyberia/protocol/location proof|location proof]] · [[cyberia/protocol/space-accounting|space accounting]] · [[cyberia/protocol/services|services]] · [[cyberia/protocol/system|system]] · [[cyberia/protocol/marketplace|marketplace]] · [[cyberia/protocol/marketplace-spec|marketplace spec]] · [[cyberia/protocol/ladder|ladder]] · [[cyberia/protocol/orgs|orgs]] · [[cyberia/protocol/ephemeris|ephemeris]] · [[cyberia/protocol/dyson sphere|dyson sphere]] · [[cyberia/protocol/maps|maps]] *(absorbs the 7 one-line maps/\* pages as a table)*

**land** (new shelf — the operating estate, promoted from research/cyb-land): visit · stay · activities · attractions · community — the ~45 pages with content; the 15 empty attraction/stay stubs die or gain their sentence at move time. incoming: [[landscape]] + [[nitrogener]] as land/flora.

**community** (grows): [[cyberia/community|community]] · talents · blog (chronicle; receives the ~20 valley log entries from cyber's blog)

**courses** (stays): [[cyberia/courses/README|README]] + edge-city-patagonia-2025 set

**research** (workbench, explicitly unfinished): [[cyberia/research/README|README]] · mimi · migration-market · genome-protocol · oxytocin · cyber-sheep · cyberia-my · tech *(the 40 seed pages fold into one indexed table; the ~12 with ≥50 words keep their pages)* · incoming: [[team speed competition]] · [[smart capital]]


## the ledger — cyber (238 pages, 238 verdicts, 0 undecided)

| page | words | verdict | why |
|---|---|---|---|
| [[blog/2019_12_01|2019_12_01]] | 13 | stays · chronicle | protocol log entry |
| [[blog/2024_04_12|2024_04_12]] | 13 | stays · chronicle | protocol log entry |
| [[blog/2024_06_27|2024_06_27]] | 13 | stays · chronicle | protocol log entry |
| [[blog/2024_07_06|2024_07_06]] | 22 | stays · chronicle | protocol log entry |
| [[blog/2024_07_19|2024_07_19]] | 6 | stays · chronicle | protocol log entry |
| [[blog/2024_07_29|2024_07_29]] | 43 | stays · chronicle | protocol post |
| [[blog/2024_07_31|2024_07_31]] | 39 | stays · chronicle | protocol log entry |
| [[blog/2024_08_02|2024_08_02]] | 30 | stays · chronicle | protocol log entry |
| [[blog/2024_08_03|2024_08_03]] | 19 | stays · chronicle | protocol log entry |
| [[blog/2024_08_04|2024_08_04]] | 18 | stays · chronicle | protocol log entry |
| [[blog/2024_08_05|2024_08_05]] | 6 | stays · chronicle | protocol log entry |
| [[blog/2024_08_06|2024_08_06]] | 11 | stays · chronicle | protocol log entry |
| [[blog/2024_08_09|2024_08_09]] | 9 | stays · chronicle | protocol log entry |
| [[blog/2024_08_10|2024_08_10]] | 19 | stays · chronicle | protocol log entry |
| [[blog/2024_08_11|2024_08_11]] | 12 | stays · chronicle | protocol log entry |
| [[blog/2024_08_16|2024_08_16]] | 57 | stays · chronicle | protocol post |
| [[blog/2024_08_22|2024_08_22]] | 12 | stays · chronicle | protocol log entry |
| [[blog/2024_08_24|2024_08_24]] | 20 | stays · chronicle | protocol log entry |
| [[blog/2024_08_26|2024_08_26]] | 3 | stays · chronicle | protocol log entry |
| [[blog/2024_09_01|2024_09_01]] | 58 | stays · chronicle | protocol post |
| [[blog/2024_09_07|2024_09_07]] | 5 | stays · chronicle | protocol log entry |
| [[blog/2024_09_08|2024_09_08]] | 50 | stays · chronicle | protocol post |
| [[blog/2024_09_17|2024_09_17]] | 17 | stays · chronicle | protocol log entry |
| [[blog/2024_09_20|2024_09_20]] | 22 | stays · chronicle | protocol log entry |
| [[blog/2024_09_27|2024_09_27]] | 35 | stays · chronicle | protocol log entry |
| [[blog/2024_10_01|2024_10_01]] | 86 | stays · chronicle | protocol post |
| [[blog/2024_10_03|2024_10_03]] | 13 | stays · chronicle | protocol log entry |
| [[blog/2024_11_04|2024_11_04]] | 12 | stays · chronicle | protocol log entry |
| [[blog/2024_11_05|2024_11_05]] | 3 | stays · chronicle | protocol log entry |
| [[blog/2024_11_15|2024_11_15]] | 6 | stays · chronicle | protocol log entry |
| [[blog/2024_12_04|2024_12_04]] | 50 | stays · chronicle | protocol post |
| [[blog/2026_01_13|2026_01_13]] | 4 | stays · chronicle | protocol log entry |
| [[blog/2026_01_27|2026_01_27]] | 9 | stays · chronicle | protocol log entry |
| [[blog/2026_02_26|2026_02_26]] | 17 | stays · chronicle | protocol log entry |
| [[blog/2026_03_01|2026_03_01]] | 321 | stays · chronicle | protocol post |
| [[blog/2026_03_05|2026_03_05]] | 64 | stays · chronicle | protocol post |
| [[blog/2026_03_14|2026_03_14]] | 56 | stays · chronicle | protocol post |
| [[blog/2026_03_16|2026_03_16]] | 89 | stays · chronicle | protocol post |
| [[blog/2026_03_23|2026_03_23]] | 804 | stays · chronicle | protocol post |
| [[blog/2026_03_24|2026_03_24]] | 324 | stays · chronicle | protocol post |
| [[blog/2026_03_25|2026_03_25]] | 586 | stays · chronicle | protocol post |
| [[blog/2026_03_26|2026_03_26]] | 298 | stays · chronicle | protocol post |
| [[blog/2026_03_27|2026_03_27]] | 529 | stays · chronicle | protocol post |
| [[blog/2026_04_25|2026_04_25]] | 275 | stays · chronicle | protocol post |
| [[blog/2026_04_26|2026_04_26]] | 191 | stays · chronicle | protocol post |
| [[blog/2026_04_30|2026_04_30]] | 632 | stays · chronicle | protocol post |
| [[blog/2026_05_12|2026_05_12]] | 736 | stays · chronicle | protocol post |
| [[blog/2026_05_21|2026_05_21]] | 736 | stays · chronicle | protocol post |
| [[blog/2026_08_28|2026_08_28]] | 929 | stays · chronicle | protocol post |
| [[blog/2026_08_29|2026_08_29]] | 1007 | stays · chronicle | protocol post |
| [[3c|3c]] | 1144 | stays · cips | deep protocol article |
| [[channel|channel]] | 1859 | stays · cips | deep protocol article |
| [[communication|communication]] | 1023 | stays · cips | deep protocol article |
| [[gravity|gravity]] | 491 | stays · cips | deep protocol article |
| [[hierarchy|hierarchy]] | 1814 | stays · cips | deep protocol article |
| [[light|light]] | 1047 | stays · cips | deep protocol article |
| [[luminosity|luminosity]] | 485 | stays · cips | deep protocol article |
| [[network|network]] | 2362 | stays · cips | deep protocol article |
| [[security|security]] | 363 | stays · cips | deep protocol article |
| [[cyber/$CYB|$CYB]] | 5512 | stays · contract | protocol invariants |
| [[cyber/cybergraph|cybergraph]] | 515 | stays · contract | protocol invariants |
| [[cyber/launch|launch]] | 3381 | stays · contract | protocol invariants |
| [[attention|attention]] | 19 | stays · core | atom of the phenomenon |
| [[cascade|cascade]] | 194 | stays · core | atom of the phenomenon |
| [[cell|cell]] | 600 | stays · core | atom of the phenomenon |
| [[concepts|concepts]] | 483 | stays · core | atom of the phenomenon |
| [[context packing|context packing]] | 347 | stays · core | how the graph enters an LLM window |
| [[contribute|contribute]] | 333 | stays · core | atom of the phenomenon |
| [[core|core]] | 92 | stays · core | atom of the phenomenon |
| [[crystal|crystal]] | 551 | stays · core | atom of the phenomenon |
| [[cyberculture|cyberculture]] | 150 | stays · core | atom of the phenomenon |
| [[cybergraph|cybergraph]] | 151 | stays · core | atom of the phenomenon |
| [[cyberlink|cyberlink]] | 74 | stays · core | atom of the phenomenon |
| [[cybersophy|cybersophy]] | 2222 | stays · core | atom of the phenomenon |
| [[cybersphere|cybersphere]] | 124 | stays · core | atom of the phenomenon |
| [[cyberverse|cyberverse]] | 145 | stays · core | atom of the phenomenon |
| [[fixed point|fixed point]] | 161 | stays · core | atom of the phenomenon |
| [[focus|focus]] | 67 | stays · core | atom of the phenomenon |
| [[happiness|happiness]] | 53 | stays · core | atom of the phenomenon |
| [[hash|hash]] | 212 | stays · core | atom of the phenomenon |
| [[impulse|impulse]] | 28 | stays · core | atom of the phenomenon |
| [[inception|inception]] | 108 | stays · core | atom of the phenomenon |
| [[install|install]] | 322 | stays · core | atom of the phenomenon |
| [[intent|intent]] | 143 | stays · core | atom of the phenomenon |
| [[interplanetary superintelligence|interplanetary superintelligence]] | 497 | stays · core | atom of the phenomenon |
| [[karma|karma]] | 24 | stays · core | atom of the phenomenon |
| [[knowledge|knowledge]] | 35 | stays · core | atom of the phenomenon |
| [[landauer limit|landauer limit]] | 71 | stays · core | atom of the phenomenon |
| [[license|license]] | 1887 | stays · core | atom of the phenomenon |
| [[mining|mining]] | 77 | stays · core | atom of the phenomenon |
| [[neuron|neuron]] | 29 | stays · core | atom of the phenomenon |
| [[particle|particle]] | 34 | stays · core | atom of the phenomenon |
| [[personality|personality]] | 791 | stays · core | atom of the phenomenon |
| [[privacy|privacy]] | 628 | stays · core | the protocol privacy hub |
| [[prob|prob]] | 115 | stays · core | atom of the phenomenon |
| [[rank|rank]] | 88 | stays · core | atom of the phenomenon |
| [[rational neuron|rational neuron]] | 378 | stays · core | atom of the phenomenon |
| [[rewards|rewards]] | 184 | stays · core | atom of the phenomenon |
| [[sealing|sealing]] | 121 | stays · core | atom of the phenomenon |
| [[self|self]] | 374 | stays · core | atom of the phenomenon |
| [[staking|staking]] | 113 | stays · core | atom of the phenomenon |
| [[subgraphs|subgraphs]] | 272 | stays · core | atom of the phenomenon |
| [[superintelligence|superintelligence]] | 128 | stays · core | atom of the phenomenon |
| [[token|token]] | 73 | stays · core | atom of the phenomenon |
| [[tri-kernel|tri-kernel]] | 33 | stays · core | atom of the phenomenon |
| [[tru|tru]] | 26 | stays · core | atom of the phenomenon |
| [[view|view]] | 51 | stays · core | atom of the phenomenon |
| `CLAUDE.md` | 1167 | stays · index | index / infra |
| [[README|cyber README]] | 285 | stays · index | index / infra |
| [[blog|blog]] | 16 | stays · index | index / infra |
| [[research|research]] | 17 | stays · index | index / infra |
| [[tokens|tokens]] | 101 | stays · index | index / infra |
| [[automated market maker|automated market maker]] | 202 | stays · nomics | the money |
| [[cap|cap]] | 193 | stays · nomics | the money |
| [[cyberlink market protocol|cyberlink market protocol]] | 155 | stays · nomics | the money |
| [[cybernomics|cybernomics]] | 269 | stays · nomics | the money |
| [[delegation|delegation]] | 183 | stays · nomics | the money |
| [[netics|netics]] | 922 | stays · nomics | the money |
| [[nomics|nomics]] | 709 | stays · nomics | the money |
| [[specs/adaptive hybrid economics|adaptive hybrid economics]] | 883 | stays · nomics | the money |
| [[specs/light-money|light-money]] | 908 | stays · nomics | the money |
| [[specs/money-loop|money-loop]] | 1133 | stays · nomics | the money |
| [[tokens/badge|badge]] | 23 | stays · nomics | the money |
| [[tokens/basic token operations|basic token operations]] | 41 | stays · nomics | the money |
| [[tokens/coin|coin]] | 27 | stays · nomics | the money |
| [[tokens/plumb|plumb]] | 147 | stays · nomics | the money |
| [[litepaper|litepaper]] | 2794 | stays · papers | the two papers |
| [[whitepaper|whitepaper]] | 22639 | stays · papers | the two papers |
| [[research/32-byte tokens|32-byte tokens]] | 1203 | stays · research | protocol-level research |
| [[research/adaptive hybrid economics|adaptive hybrid economics]] | 32 | stays · research | protocol-level research |
| [[research/algorithmic essence of superintelligence|algorithmic essence of superintelligence]] | 3149 | stays · research | protocol-level research |
| [[research/bootstrap|bootstrap]] | 2440 | stays · research | protocol-level research |
| [[research/cybergraph model architecture|cybergraph model architecture]] | 1677 | stays · research | protocol-level research |
| [[research/egregore properties|egregore properties]] | 1992 | stays · research | protocol-level research |
| [[research/energy market|energy market]] | 1299 | stays · research | protocol-level research |
| [[research/focus flow computation|focus flow computation]] | 1208 | stays · research | protocol-level research |
| [[research/future of computation|future of computation]] | 3111 | stays · research | protocol-level research |
| [[research/gradient descent|gradient descent]] | 1593 | stays · research | protocol-level research |
| [[research/knowledge capacity|knowledge capacity]] | 2027 | stays · research | protocol-level research |
| [[research/knowledge economy|knowledge economy]] | 942 | stays · research | protocol-level research |
| [[research/link production|link production]] | 2205 | stays · research | protocol-level research |
| [[research/neuroscience principles for machine mind|neuroscience principles for machine mind]] | 6858 | stays · research | protocol-level research |
| [[research/physical analogies|physical analogies]] | 3352 | stays · research | protocol-level research |
| [[research/privacy trilateral|privacy trilateral]] | 3867 | stays · research | ZK+FHE+MPC — protocol-level survey |
| [[research/theoretical foundations|theoretical foundations]] | 898 | stays · research | protocol-level research |
| [[research/unified mining|unified mining]] | 1317 | stays · research | protocol-level research |
| [[research/universal law|universal law]] | 1506 | stays · research | protocol-level research |
| [[self/dmn|dmn]] | 758 | stays · self | protocol as neuron |
| [[self/linking|linking]] | 905 | stays · self | protocol as neuron |
| [[self/parametrization|parametrization]] | 1762 | stays · self | protocol as neuron |
| [[self/sigma|sigma]] | 1004 | stays · self | protocol as neuron |
| [[specs/README|specs/README README]] | 499 | stays · specs | engineering contract |
| [[specs/component-ownership|component-ownership]] | 635 | stays · specs | engineering contract |
| [[specs/full-flow-claims|full-flow-claims]] | 522 | stays · specs | engineering contract |
| [[specs/node-modes|node-modes]] | 577 | stays · specs | engineering contract |
| [[specs/rewards-completeness|rewards-completeness]] | 345 | stays · specs | engineering contract |
| [[epistemology|epistemology]] | 2770 | promotes → cyber/epistemology | contract family, demand measured |
| [[identity|identity]] | 567 | promotes → cyber/identity | contract family, demand measured |
| [[cyberank|cyberank]] | 34 | merges → rank | rank owns the alias and all 38 inbound refs |
| [[research/algebraic state commitments|algebraic state commitments]] | 1806 | moves → bbg | component research goes home |
| [[research/bbg|bbg]] | 1993 | moves → bbg | component research goes home |
| [[research/data availability strategy|data availability strategy]] | 3036 | moves → bbg | component research goes home |
| [[research/data structures for polynomial state|data structures for polynomial state]] | 1626 | moves → bbg | component research goes home |
| [[research/polynomial nouns|polynomial nouns]] | 2877 | moves → bbg | component research goes home |
| [[research/programmable state|programmable state]] | 1703 | moves → bbg | component research goes home |
| [[research/state model|state model]] | 789 | moves → bbg | component research goes home |
| [[research/storage proofs|storage proofs]] | 1390 | moves → bbg | component research goes home |
| [[cli|cli]] | 35 | moves → bootloader | vehicle tooling |
| [[congress|congress]] | 192 | moves → bootloader | the vehicle team |
| [[research/bostrom compilation report|bostrom compilation report]] | 1523 | moves → bootloader | component research goes home |
| [[54|54]] | 14 | moves → bootloader chronicle | per the rule |
| [[55|55]] | 722 | moves → bootloader chronicle | per the rule |
| [[blog/2024_08_15|2024_08_15]] | 6 | moves → bootloader chronicle | vehicle-era call log |
| [[roadmap|roadmap]] | 758 | moves → bootloader chronicle | 2019 plan |
| [[accumulator|accumulator]] | 196 | moves → crystal | cryptography atom |
| [[helix|helix]] | 363 | moves → crystal | mathematics atom |
| [[history|history]] | 129 | moves → crystal | history of computing |
| [[skyrmion|skyrmion]] | 382 | moves → crystal | physics atom |
| [[time-crystal|time-crystal]] | 375 | moves → crystal | physics atom |
| [[topoisomerase|topoisomerase]] | 303 | moves → crystal | biology atom |
| [[topological-invariant|topological-invariant]] | 415 | moves → crystal | mathematics atom |
| [[security audit private key import|security audit private key import]] | 202 | moves → cyb reference/ | audits the robot |
| [[research/cyberlink protocol structure|cyberlink protocol structure]] | 472 | moves → cybergraph | component research goes home |
| [[blog/2024_08_21|2024_08_21]] | 6 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_08_23|2024_08_23]] | 6 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_08_27|2024_08_27]] | 10 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_09_10|2024_09_10]] | 7 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_09_12|2024_09_12]] | 32 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_09_15|2024_09_15]] | 19 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_10_07|2024_10_07]] | 4 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_10_13|2024_10_13]] | 14 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_11_23|2024_11_23]] | 3 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_12_05|2024_12_05]] | 5 | moves → cyberia chronicle | valley log entry |
| [[blog/2024_12_22|2024_12_22]] | 4 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_04_04|2025_04_04]] | 12 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_04_07|2025_04_07]] | 19 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_04_10|2025_04_10]] | 7 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_04_15|2025_04_15]] | 28 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_04_23|2025_04_23]] | 7 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_09_01|2025_09_01]] | 11 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_09_06|2025_09_06]] | 4 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_09_08|2025_09_08]] | 4 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_09_15|2025_09_15]] | 5 | moves → cyberia chronicle | valley log entry |
| [[blog/2025_10_22|2025_10_22]] | 5 | moves → cyberia chronicle | valley log entry |
| [[blog/2026_01_24|2026_01_24]] | 7 | moves → cyberia chronicle | valley log entry |
| [[landscape|landscape]] | 25 | moves → cyberia land/flora | valley gardening |
| [[nitrogener|nitrogener]] | 93 | moves → cyberia land/flora | valley gardening |
| [[smart capital|smart capital]] | 15 | moves → cyberia research/ | per the rule |
| [[team speed competition|team speed competition]] | 55 | moves → cyberia research/ | per the rule |
| `SPEC.md` | 2603 | moves → cyberia/midao | workspace spec |
| [[leadership|leadership]] | 833 | moves → cyberia/midao | org canon |
| [[russian school of inscription|russian school of inscription]] | 1593 | moves → cybics/lang | culture essay |
| [[research/foculus|foculus]] | 520 | moves → foculus | component research goes home |
| [[research/provable consensus|provable consensus]] | 2066 | moves → foculus | component research goes home |
| [[research/structural-sync|structural-sync]] | 2736 | moves → foculus | component research goes home |
| [[research/vec formalization|vec formalization]] | 1799 | moves → foculus | component research goes home |
| [[research/hashing and confidentiality|hashing and confidentiality]] | 1151 | moves → hemera | component research goes home |
| [[subgraphs/kadek|kadek]] | 52 | moves → kadek repo (private card) | per the rule |
| [[subgraphs/lytics|lytics]] | 36 | moves → lytics repo docs | per the rule |
| [[research/256 symbols|256 symbols]] | 899 | moves → nox | component research goes home |
| [[research/nox - frozen provable computer|nox - frozen provable computer]] | 740 | moves → nox | component research goes home |
| [[analizer|analizer]] | 1009 | moves → research/ | protocol research misfiled at root |
| [[seer|seer]] | 1318 | moves → research/ | protocol research misfiled at root |
| [[subgraphs/conformance|conformance]] | 26 | moves → soft3 docs | per the rule |
| [[research/five algebras|five algebras]] | 3619 | moves → strata | component research goes home |
| [[subgraphs/tape|tape]] | 12 | moves → tape repo docs | per the rule |
| [[research/trinity meets polynomial state|trinity meets polynomial state]] | 1496 | moves → trident | component research goes home |
| [[research/gflownet focus flow|gflownet focus flow]] | 1766 | moves → tru | component research goes home |
| [[research/spectral gap from convergence|spectral gap from convergence]] | 1229 | moves → tru | component research goes home |
| [[research/tri-kernel architecture|tri-kernel architecture]] | 1943 | moves → tru | component research goes home |
| [[research/polynomial proof system|polynomial proof system]] | 1862 | moves → zheng | component research goes home |
| [[research/recursive brakedown|recursive brakedown]] | 1949 | moves → zheng | component research goes home |
| [[research/zheng vs starks|zheng vs starks]] | 14 | moves → zheng | component research goes home |
| [[blog/2024_09_29|2024_09_29]] | 0 | dies | empty entry |
| [[blog/2025_02_15|2025_02_15]] | 0 | dies | empty entry |
| [[research/programming model|programming model]] | 11 | dies | empty / superseded |
| [[sparks|sparks]] | 0 | dies | empty / superseded |
| [[style|style]] | 11 | dies | empty / superseded |

## the ledger — cyb (83 pages, 83 verdicts, 0 undecided)

| page | words | verdict | why |
|---|---|---|---|
| `CLAUDE.md` | 279 | stays · index | repo face / infra |
| [[cyb/README|cyb README]] | 153 | stays · index | repo face / infra |
| `.claude/plans/android-support.md` | 858 | stays · plans | workbench |
| `.claude/plans/live-cell-runtime.md` | 1263 | stays · plans | workbench |
| `.claude/plans/live-cybergraph.md` | 814 | stays · plans | workbench |
| `.claude/plans/portable-backends.md` | 2193 | stays · plans | workbench |
| [[cyb/root/android|android]] | 593 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/apps|apps]] | 136 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/avatar|avatar]] | 28 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/benchmarks|benchmarks]] | 162 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/cells|cells]] | 721 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/cyb-model|cyb-model]] | 4 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/cyb-registry|cyb-registry]] | 416 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/cyb-vocab|cyb-vocab]] | 4 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/dev|dev]] | 207 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/features|features]] | 939 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/format|format]] | 446 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/fs|fs]] | 195 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/particle|particle]] | 337 | stays · reference (flattens to /) | how the robot handles particles |
| [[cyb/root/release-process|release-process]] | 966 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/rendering|rendering]] | 2122 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/routing|routing]] | 2526 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/rust-architecture|rust-architecture]] | 1455 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/scripting|scripting]] | 1061 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/terminal|terminal]] | 947 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/truth|truth]] | 234 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/ward|ward]] | 1653 | stays · reference (flattens to /) | how the robot works |
| [[cyb/root/README|root/README README]] | 673 | stays · spine (flattens to /) | the product story |
| [[cyb/root/philosophy|philosophy]] | 597 | stays · spine (flattens to /) | the product story |
| [[cyb/root/product|product]] | 2471 | stays · spine (flattens to /) | the product story |
| [[cyb/root/releases/v0.1.0|v0.1.0]] | 506 | stays · spine (flattens to /) | the product story |
| [[cyb/root/roadmap|roadmap]] | 577 | stays · spine (flattens to /) | the product story |
| [[cyb/root/spec|spec]] | 630 | stays · spine (flattens to /) | the product story |
| [[cyb/root/story|story]] | 99 | stays · spine (flattens to /) | the product story |
| [[cyb/root/team|team]] | 113 | stays · spine (flattens to /) | the product story |
| [[cyb/root/whitepaper|whitepaper]] | 4362 | merges → product; paper to legacy/ | one product story |
| [[cyb/root/architecture|architecture]] | 2291 | merges → rust-architecture | JS-era architecture, absorbed |
| [[cyb/root/wire|wire]] | 745 | moves → cybergraph docs | component matter |
| [[cyb/root/hardware|hardware]] | 442 | moves → honeycrisp docs | component matter |
| [[cyb/root/runtime|runtime]] | 612 | moves → honeycrisp docs | component matter |
| [[cyb/root/order|order]] | 179 | moves → nox docs | component matter |
| [[cyb/root/os|os]] | 1465 | moves → soft3 docs | component matter |
| [[cyb/root/pipeline|pipeline]] | 279 | moves → soft3 docs | component matter |
| [[cyb/root/compile|compile]] | 561 | moves → tru docs | component matter |
| [[cyb/root/context|context]] | 821 | moves → tru docs | component matter |
| [[cyb/root/model-lifecycle|model-lifecycle]] | 1109 | moves → tru docs | component matter |
| [[cyb/root/multiproof|multiproof]] | 2756 | moves → zheng docs | component matter |
| [[cyb/root/backend|backend]] | 663 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/brain/learn|learn]] | 20 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/brain/list|list]] | 272 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/brain/particle|particle]] | 16 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/brain/root|root]] | 15 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/core|core]] | 234 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/main|main]] | 9 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/mvp-screens|mvp-screens]] | 402 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/offline|offline]] | 32 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle|oracle]] | 118 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle/ask|ask]] | 21 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle/learn|learn]] | 2 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle/neurons|neurons]] | 14 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle/product|product]] | 133 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle/raw|raw]] | 7 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle/search|search]] | 18 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/oracle/views|views]] | 13 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/portal|portal]] | 87 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/portal/my avatars/api|api]] | 8 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/portal/my avatars/legacy|legacy]] | 54 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/portal/my spells/api|api]] | 4 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/portal/my spells/practice|practice]] | 101 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/robot|robot]] | 1648 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/robot/psycho|psycho]] | 9 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/robot/trainer|trainer]] | 6 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/root|root]] | 8 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/signer|signer]] | 3109 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/studio|studio]] | 4 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/sync|sync]] | 344 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/virus|virus]] | 24 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/wasm|wasm]] | 5024 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/wgpu|wgpu]] | 83 | folds → legacy/ | retired JS-era product (old.cyb.ai) |
| [[cyb/root/portal/avatars|avatars]] | 0 | dies | zero words |
| [[cyb/root/portal/neurons|neurons]] | 0 | dies | zero words |
| [[cyb/root/portal/skills|skills]] | 0 | dies | zero words |
| [[cyb/root/portal/spells|spells]] | 0 | dies | zero words |

## the ledger — cyberia (180 pages, 180 verdicts, 0 undecided)

| page | words | verdict | why |
|---|---|---|---|
| [[cyberia/courses/README|courses/README README]] | 51 | stays · courses | teaching |
| [[cyberia/courses/edge-city-patagonia-2025|edge-city-patagonia-2025]] | 373 | stays · courses | teaching |
| [[cyberia/courses/edge-city-patagonia-2025/biome engineering|biome engineering]] | 886 | stays · courses | teaching |
| [[cyberia/courses/edge-city-patagonia-2025/bostrom story|bostrom story]] | 294 | stays · courses | teaching |
| [[cyberia/courses/edge-city-patagonia-2025/collective intelligence|collective intelligence]] | 239 | stays · courses | teaching |
| [[cyberia/courses/edge-city-patagonia-2025/cyber valley story|cyber valley story]] | 813 | stays · courses | teaching |
| [[cyberia/courses/edge-city-patagonia-2025/energy and water system|energy and water system]] | 893 | stays · courses | teaching |
| [[cyberia/courses/edge-city-patagonia-2025/soil, heat and carbon|soil, heat and carbon]] | 450 | stays · courses | teaching |
| [[cyberia/foundation/README|foundation/README README]] | 52 | stays · foundation | why the nation |
| [[cyberia/foundation/architecture|architecture]] | 3083 | stays · foundation | why the nation |
| [[cyberia/foundation/cyberian|cyberian]] | 480 | stays · foundation | why the nation |
| [[cyberia/foundation/manifesto|manifesto]] | 329 | stays · foundation | why the nation |
| [[cyberia/foundation/org|org]] | 475 | stays · foundation | why the nation |
| [[cyberia/foundation/space doctrine|space doctrine]] | 1252 | stays · foundation | why the nation |
| [[cyberia/foundation/strategy|strategy]] | 771 | stays · foundation | why the nation |
| [[cyberia/foundation/vision|vision]] | 3287 | stays · foundation | why the nation |
| [[cyberia/foundation/whitepaper|whitepaper]] | 1860 | stays · foundation | why the nation |
| [[cyberia/README|cyberia README]] | 93 | stays · index | the nation face |
| [[cyberia/community|community]] | 36 | stays · index | the nation face |
| [[cyberia/protocol/README|protocol/README README]] | 874 | stays · protocol | how the state runs |
| [[cyberia/protocol/bank-above-banks|bank-above-banks]] | 5553 | stays · protocol | how the state runs |
| [[cyberia/protocol/century-index|century-index]] | 1824 | stays · protocol | how the state runs |
| [[cyberia/protocol/dyson sphere|dyson sphere]] | 340 | stays · protocol | how the state runs |
| [[cyberia/protocol/ephemeris|ephemeris]] | 350 | stays · protocol | how the state runs |
| [[cyberia/protocol/ladder|ladder]] | 255 | stays · protocol | how the state runs |
| [[cyberia/protocol/location proof|location proof]] | 2785 | stays · protocol | how the state runs |
| [[cyberia/protocol/maps|maps]] | 8 | stays · protocol | how the state runs |
| [[cyberia/protocol/marketplace-spec|marketplace-spec]] | 2654 | stays · protocol | how the state runs |
| [[cyberia/protocol/marketplace|marketplace]] | 2562 | stays · protocol | how the state runs |
| [[cyberia/protocol/orgs|orgs]] | 507 | stays · protocol | how the state runs |
| [[cyberia/protocol/services|services]] | 2928 | stays · protocol | how the state runs |
| [[cyberia/protocol/space-accounting|space-accounting]] | 1837 | stays · protocol | how the state runs |
| [[cyberia/protocol/system|system]] | 1616 | stays · protocol | how the state runs |
| [[cyberia/research/README|research/README README]] | 100 | stays · research | workbench index |
| [[cyberia/research/cyber-sheep/README|research/cyber-sheep/README README]] | 28 | stays · research | workbench project |
| [[cyberia/research/cyber-sheep/cyber-sheep|cyber-sheep]] | 3316 | stays · research | workbench project |
| [[cyberia/research/cyberia-my/README|research/cyberia-my/README README]] | 140 | stays · research | workbench project |
| [[cyberia/research/genome-protocol/README|research/genome-protocol/README README]] | 30 | stays · research | workbench project |
| [[cyberia/research/genome-protocol/attested genome protocol|attested genome protocol]] | 1509 | stays · research | workbench project |
| [[cyberia/research/migration-market/README|research/migration-market/README README]] | 52 | stays · research | workbench project |
| [[cyberia/research/migration-market/migration market funnel|migration market funnel]] | 389 | stays · research | workbench project |
| [[cyberia/research/migration-market/migration market model|migration market model]] | 2461 | stays · research | workbench project |
| [[cyberia/research/mimi/CLAUDE|CLAUDE]] | 851 | stays · research | project infra |
| [[cyberia/research/mimi/README|research/mimi/README README]] | 483 | stays · research | workbench project |
| [[cyberia/research/oxytocin/README|research/oxytocin/README README]] | 272 | stays · research | workbench project |
| [[cyberia/research/oxytocin/about|about]] | 618 | stays · research | workbench project |
| [[cyberia/research/oxytocin/economics|economics]] | 518 | stays · research | workbench project |
| [[cyberia/research/oxytocin/membership|membership]] | 264 | stays · research | workbench project |
| [[cyberia/research/oxytocin/roadmap|roadmap]] | 763 | stays · research | workbench project |
| [[cyberia/research/oxytocin/services/README|research/oxytocin/services/README README]] | 372 | stays · research | workbench project |
| [[cyberia/research/oxytocin/services/thermal-circuit|thermal-circuit]] | 248 | stays · research | workbench project |
| [[cyberia/research/oxytocin/services/universal-chair-zone|universal-chair-zone]] | 248 | stays · research | workbench project |
| [[cyberia/research/tech/3d printing/README|research/tech/3d printing/README README]] | 107 | stays · research/tech | seed with content |
| [[cyberia/research/tech/adhesive/README|research/tech/adhesive/README README]] | 335 | stays · research/tech | seed with content |
| [[cyberia/research/tech/antenna/README|research/tech/antenna/README README]] | 92 | stays · research/tech | seed with content |
| [[cyberia/research/tech/battery/README|research/tech/battery/README README]] | 99 | stays · research/tech | seed with content |
| [[cyberia/research/tech/cube/README|research/tech/cube/README README]] | 142 | stays · research/tech | seed with content |
| [[cyberia/research/tech/energy autonomy/README|research/tech/energy autonomy/README README]] | 38 | stays · research/tech | seed with content |
| [[cyberia/research/tech/engine/README|research/tech/engine/README README]] | 97 | stays · research/tech | seed with content |
| [[cyberia/research/tech/insulation/README|research/tech/insulation/README README]] | 244 | stays · research/tech | seed with content |
| [[cyberia/research/tech/magic forest/README|research/tech/magic forest/README README]] | 317 | stays · research/tech | seed with content |
| [[cyberia/research/tech/multigrid/README|research/tech/multigrid/README README]] | 26 | stays · research/tech | seed with content |
| [[cyberia/research/tech/pump/README|research/tech/pump/README README]] | 96 | stays · research/tech | seed with content |
| [[cyberia/research/tech/roman concrete/README|research/tech/roman concrete/README README]] | 255 | stays · research/tech | seed with content |
| [[cyberia/research/tech/soil battery/README|research/tech/soil battery/README README]] | 397 | stays · research/tech | seed with content |
| [[cyberia/research/tech/water purification/README|research/tech/water purification/README README]] | 857 | stays · research/tech | seed with content |
| [[cyberia/research/tech/wheel/README|research/tech/wheel/README README]] | 70 | stays · research/tech | seed with content |
| [[cyberia/explanation/your share of the sun|your share of the sun]] | 957 | moves → foundation/ | an explanation essay, foundation shelf |
| [[cyberia/research/cyb-land/README|research/cyb-land/README README]] | 75 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities|activities]] | 34 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/camp|camp]] | 44 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/eat|eat]] | 165 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/heal|heal]] | 135 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/heal/banya|banya]] | 68 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/heal/meditation|meditation]] | 46 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/hike|hike]] | 171 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/hike/sacred path|sacred path]] | 149 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/learn|learn]] | 71 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/learn/autonomy tour|autonomy tour]] | 145 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/learn/host|host]] | 199 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/activities/ride|ride]] | 55 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/attractions|attractions]] | 168 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/attractions/animals|animals]] | 67 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/attractions/batuka|batuka]] | 34 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/attractions/lolok gunung|lolok gunung]] | 20 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/attractions/sanghyang|sanghyang]] | 31 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/attractions/sinwood|sinwood]] | 131 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/attractions/sunset|sunset]] | 143 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/cart|cart]] | 92 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/community|community]] | 34 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/community/blog|blog]] | 22 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/community/invest|invest]] | 248 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/community/nomads|nomads]] | 185 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/community/talents|talents]] | 48 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/cyb.land|cyb.land]] | 175 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/events|events]] | 150 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/launch event|launch event]] | 164 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/marketing|marketing]] | 1253 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/stay|stay]] | 211 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/stay/around|around]] | 49 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/stay/twin peaks|twin peaks]] | 26 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit|visit]] | 236 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit/cyberlink|cyberlink]] | 21 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit/daypass|daypass]] | 184 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit/nightpass|nightpass]] | 186 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit/parking|parking]] | 30 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit/recycling|recycling]] | 137 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit/visas|visas]] | 136 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/research/cyb-land/visit/wilderness|wilderness]] | 45 | moves → land/ | the operating estate, promoted shelf |
| [[cyberia/protocol/maps/bed|bed]] | 113 | folds → protocol/maps | one-line scales become a table |
| [[cyberia/protocol/maps/block|block]] | 9 | folds → protocol/maps | one-line scales become a table |
| [[cyberia/protocol/maps/district|district]] | 10 | folds → protocol/maps | one-line scales become a table |
| [[cyberia/protocol/maps/region|region]] | 9 | folds → protocol/maps | one-line scales become a table |
| [[cyberia/protocol/maps/sector|sector]] | 3 | folds → protocol/maps | one-line scales become a table |
| [[cyberia/protocol/maps/trail|trail]] | 2 | folds → protocol/maps | one-line scales become a table |
| [[cyberia/protocol/maps/wall|wall]] | 205 | folds → protocol/maps | one-line scales become a table |
| [[cyberia/research/tech/README|research/tech/README README]] | 255 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/anaerobic composting/README|research/tech/anaerobic composting/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/bamtex/README|research/tech/bamtex/README README]] | 1 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/batch rocket stove/README|research/tech/batch rocket stove/README README]] | 3 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/biochar/README|research/tech/biochar/README README]] | 10 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/biofilter/README|research/tech/biofilter/README README]] | 1 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/energy and water system/README|research/tech/energy and water system/README README]] | 20 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/gas generator/README|research/tech/gas generator/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/heat collectors/README|research/tech/heat collectors/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/heat exchanger/README|research/tech/heat exchanger/README README]] | 5 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/heat pump/README|research/tech/heat pump/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/inverter/README|research/tech/inverter/README README]] | 1 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/lime paste/README|research/tech/lime paste/README README]] | 5 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/lithium-ion battery/README|research/tech/lithium-ion battery/README README]] | 3 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/manure urine miner/README|research/tech/manure urine miner/README README]] | 3 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/photobioreactor/README|research/tech/photobioreactor/README README]] | 1 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/photovoltaic panel/README|research/tech/photovoltaic panel/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/rain water collection/README|research/tech/rain water collection/README README]] | 3 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/solar chimney/README|research/tech/solar chimney/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/stirling engine/README|research/tech/stirling engine/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/superwood/README|research/tech/superwood/README README]] | 1 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/thermoelectric generator/README|research/tech/thermoelectric generator/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/water battery/README|research/tech/water battery/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/water storage maximization/README|research/tech/water storage maximization/README README]] | 15 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/water system/README|research/tech/water system/README README]] | 16 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/tech/wind turbine/README|research/tech/wind turbine/README README]] | 2 | folds → tech index | one-line seed, folds to a table row |
| [[cyberia/research/events/AGENTS|AGENTS]] | 2143 | extracts → events repo | vendored software product |
| [[cyberia/research/events/README|research/events/README README]] | 5749 | extracts → events repo | vendored software product |
| [[cyberia/research/events/backend/INDEXER_TESTS_STATUS|INDEXER_TESTS_STATUS]] | 273 | extracts → events repo | vendored software product |
| [[cyberia/research/events/backend/README|research/events/backend/README README]] | 0 | extracts → events repo | vendored software product |
| [[cyberia/research/events/backend/cyber_valley/indexer/service/events/README|research/events/backend/cyber_valley/indexer/service/events/README README]] | 279 | extracts → events repo | vendored software product |
| [[cyberia/research/events/backend/cyber_valley/indexer/service/snapshots/README|research/events/backend/cyber_valley/indexer/service/snapshots/README README]] | 629 | extracts → events repo | vendored software product |
| [[cyberia/research/events/client/README|research/events/client/README README]] | 288 | extracts → events repo | vendored software product |
| [[cyberia/research/events/deploy/README|research/events/deploy/README README]] | 153 | extracts → events repo | vendored software product |
| [[cyberia/research/events/docs/10-25-deliverables|10-25-deliverables]] | 219 | extracts → events repo | vendored software product |
| [[cyberia/research/events/docs/branch-milestone-summary-2026-02|branch-milestone-summary-2026-02]] | 535 | extracts → events repo | vendored software product |
| [[cyberia/research/events/docs/extending|extending]] | 1426 | extracts → events repo | vendored software product |
| [[cyberia/research/events/docs/google-cloud-maps-api|google-cloud-maps-api]] | 2241 | extracts → events repo | vendored software product |
| [[cyberia/research/events/docs/qris-integration|qris-integration]] | 667 | extracts → events repo | vendored software product |
| [[cyberia/research/events/docs/scaling-plan|scaling-plan]] | 1190 | extracts → events repo | vendored software product |
| [[cyberia/research/events/ethereum/TEST_REPORT|TEST_REPORT]] | 1435 | extracts → events repo | vendored software product |
| [[cyberia/research/cyb-land/activities/heal/massage|massage]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/activities/hike/sanghyang black|sanghyang black]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/andara|andara]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/batukaru|batukaru]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/coffee plantation|coffee plantation]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/firefly canyon|firefly canyon]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/full moon|full moon]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/lesung|lesung]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/new moon|new moon]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/pucuk|pucuk]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/robots|robots]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/stargazing|stargazing]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/sunrise hiking|sunrise hiking]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/attractions/travers|travers]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/community/invest/company|company]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/community/invest/land|land]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/stay/glamping|glamping]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/stay/tent rent|tent rent]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/visit/lighting|lighting]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/visit/security|security]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/visit/sound|sound]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/cyb-land/visit/trails|trails]] | 0 | dies | empty stub (topic kept as a line in parent) |
| [[cyberia/research/mimi/docs/error-analysis-2026-01-05|error-analysis-2026-01-05]] | 2112 | dies | ops artifact |

---

# 4 · order of movements

1. **deletions that free names** (1c + empty cyb stubs + empty cyberia stubs) — zero inbound by measurement, zero breakage
2. **within-repo folds** — [[rank]]⊕[[cyberank]] · maps table · tech index · cyb-land → land/ · cyb root/ flattening
3. **cyber → outward** (1b) — ~24 pages with ~40 link fixes
4. **research redistribution** (1a) — 25 pages home; [[research]] index rewritten as the map of what stayed and where the rest went
5. **cyb legacy fold** (2a + 2b)
6. **cyberia boundaries** — events extraction, incoming placements
7. **contract promotions** — *cyber/epistemology*, *cyber/identity*, *cyber/proofs* written from [[epistemology]], [[identity]], [[security]] material

# 5 · acceptance

- link integrity ≥ 99% per subgraph (measured 94.5% in cyber on 2026-09-01)
- zero pages under 12 words outside indexes (was 38)
- zero phantom targets with ≥ 3 inbound refs (was 7)
- every page passes the rule in one step; arguable pages are named in this document with their tiebreak
- crystallinity ≥ 95% (was 88%) — same script, number published in each phase's commit message

*plan approved → phases execute as separate commit series. this page then becomes the record of what moved where, and why.*
