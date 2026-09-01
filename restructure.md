---
title: restructure
tags: cyber, core, plan
crystal-type: pattern
crystal-domain: cyber
status: plan
---
# restructure — crystallizing cyb · cyber · cyberia

a plan first, movements second. nothing in this document moves a file; every section is a contract for one commit series, link-fixes included, so the site is never broken in between. every page named below is a live link — click through and judge it.

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

### 1a · research redistribution — component research goes home

| page | → repo | page | → repo |
|---|---|---|---|
| [[research/bbg|bbg]] | [[bbg]] | [[research/recursive brakedown|recursive brakedown]] | [[zheng]] |
| [[research/data availability strategy|data availability strategy]] | [[bbg]] | [[research/polynomial proof system|polynomial proof system]] | [[zheng]] |
| [[research/storage proofs|storage proofs]] | [[bbg]] | [[research/zheng vs starks|zheng vs starks]] | [[zheng]] |
| [[research/algebraic state commitments|algebraic state commitments]] | [[bbg]] | [[research/256 symbols|256 symbols]] | [[nox]] |
| [[research/data structures for polynomial state|data structures for polynomial state]] | [[bbg]] | [[research/nox - frozen provable computer|nox — frozen provable computer]] | [[nox]] |
| [[research/programmable state|programmable state]] | [[bbg]] | [[research/foculus|foculus]] | [[foculus]] |
| [[research/state model|state model]] | [[bbg]] | [[research/provable consensus|provable consensus]] | [[foculus]] |
| [[research/polynomial nouns|polynomial nouns]] | [[bbg]] | [[research/vec formalization|vec formalization]] | [[foculus]] |
| [[research/hashing and confidentiality|hashing and confidentiality]] | [[hemera]] | [[research/structural-sync|structural-sync]] | [[foculus]] |
| [[research/five algebras|five algebras]] | [[strata]] | [[research/spectral gap from convergence|spectral gap from convergence]] | [[tru]] |
| [[research/trinity meets polynomial state|trinity meets polynomial state]] | [[trident]] | [[research/gflownet focus flow|gflownet focus flow]] | [[tru]] |
| [[research/cyberlink protocol structure|cyberlink protocol structure]] | [[cybergraph spec|cybergraph]] | [[research/tri-kernel architecture|tri-kernel architecture]] | [[tru]] |
| [[research/bostrom compilation report|bostrom compilation report]] | [[bootloader]] | | |

### 1b · leaves cyber — page by page

| page | → lands at | why there |
|---|---|---|
| [[skyrmion]] | crystal/skyrmion.md | physics atom |
| [[time-crystal]] | crystal/time-crystal.md | physics atom |
| [[topoisomerase]] | crystal/topoisomerase.md | biology atom |
| [[topological-invariant]] | crystal/topological-invariant.md | mathematics atom |
| [[helix]] | crystal/helix.md | mathematics atom |
| [[history]] | crystal/history of computing.md | computing history atom |
| [[accumulator]] | crystal/accumulator.md | cryptography atom |
| [[russian school of inscription]] | cybics/lang/ | culture essay |
| [[landscape]] | cyberia land/ (flora) | valley gardening |
| [[nitrogener]] | cyberia land/ (flora) | valley gardening |
| [[cli]] | bootloader/cli.md | dead vehicle's tooling |
| [[congress]] | bootloader/congress.md | the vehicle's team |
| [[54]] | bootloader chronicle | vehicle-era report |
| [[55]] | bootloader chronicle | vehicle-era letter |
| [[roadmap]] | bootloader chronicle | 2019 plan, historical |
| [[leadership]] | cyberia/midao — leadership.md | org practice canon |
| SPEC.md | cyberia/midao — workspace.md | org process spec |
| [[team speed competition]] | cyberia research/ | valley idea |
| [[smart capital]] | cyberia research/ | citadel capital idea |
| [[security audit private key import]] | cyb (reference/) | audit of the robot's code |
| [[subgraphs/kadek|kadek]] | kadek repo README | component's home (note: card says private) |
| [[subgraphs/lytics|lytics]] | [[lytics]] repo docs | component's home |
| [[subgraphs/tape|tape]] | tape repo docs | component's home |
| [[subgraphs/conformance|conformance]] | soft3 docs | stack-level concern |

### 1c · dies in cyber

[[style]] (11 words) · [[sparks]] (0) · [[research/programming model|programming model]] (11, superseded by [[nox]]) · [[cyberank]] as a file (its alias and traffic already live in [[rank]]) · [[blog/2024_09_29]] · [[blog/2025_02_15]]

---

# 2 · cyb — target structure

the robot: **31 pages**, one flat module set. the repo's `root/` directory dissolves — pages live flat like every other crystal. current pages linked at today's paths.

**spine** (8): [[cyb/root/README|robot (README)]] · [[cyb/root/product|product]] · [[cyb/root/philosophy|philosophy]] · [[cyb/root/story|story]] · [[cyb/root/spec|spec]] · [[cyb/root/roadmap|roadmap]] · [[cyb/root/team|team]] · [[cyb/root/releases/v0.1.0|releases]]

**reference** (17): [[cyb/root/rust-architecture|architecture]] *(absorbs [[cyb/root/architecture|the JS-era architecture]])* · [[cyb/root/rendering|rendering]] · [[cyb/root/routing|routing]] · [[cyb/root/terminal|terminal]] · [[cyb/root/cells|cells]] · [[cyb/root/apps|apps]] · [[cyb/root/avatar|avatar]] · [[cyb/root/format|format]] · [[cyb/root/cyb-registry|cyb-registry]] · [[cyb/root/cyb-model|cyb-model]] · [[cyb/root/cyb-vocab|cyb-vocab]] · [[cyb/root/fs|fs]] · [[cyb/root/scripting|scripting]] · [[cyb/root/truth|truth]] · [[cyb/root/features|features]] · [[cyb/root/android|android]] · [[cyb/root/release-process|release-process]]

**ops** (4): [[cyb/root/dev|dev]] · [[cyb/root/benchmarks|benchmarks]] · [[cyb/root/ward|ward]] · *legacy/README* (one page explaining the JS era, written at fold time)

**incoming** (2): [[security audit private key import]] (from cyber) · plans stay in `.claude/plans/`

### 2a · leaves cyb

| page | → lands at | why |
|---|---|---|
| [[cyb/root/compile|compile]] · [[cyb/root/context|context]] · [[cyb/root/model-lifecycle|model-lifecycle]] | [[tru]] docs | ct0 / model matter |
| [[cyb/root/runtime|runtime]] · [[cyb/root/hardware|hardware]] | [[honeycrisp]] docs | inference stack matter |
| [[cyb/root/order|order]] | [[nox]] docs | execution unit |
| [[cyb/root/multiproof|multiproof]] | [[zheng]] docs | proof architecture |
| [[cyb/root/wire|wire]] | [[cybergraph spec|cybergraph]] docs | transport frames |
| [[cyb/root/os|os]] · [[cyb/root/pipeline|pipeline]] · [[cyb/root/root|root]] | [[soft3]] docs | stack-level vision |
| [[cyb/root/whitepaper|cyb whitepaper]] | merge living claims → [[cyb/root/product|product]]; paper → legacy/ | one product story |

### 2b · folds into cyb legacy/

the retired JS product (old.cyb.ai), one directory with one README: [[cyb/root/backend|backend]] · [[cyb/root/main|main]] · [[cyb/root/virus|virus]] · [[cyb/root/sync|sync]] · [[cyb/root/mvp-screens|mvp-screens]] · [[cyb/root/wasm|wasm]] · [[cyb/root/signer|signer]] · [[cyb/root/wgpu|wgpu]] · [[cyb/root/offline|offline]] · [[cyb/root/oracle|oracle]]+6 children · portal/+6 · brain/+4 · robot/{psycho, trainer} · [[cyb/root/studio|studio]] · [[cyb/root/core|core]] · [[cyb/root/main|main]]. the 9 zero-word stubs among them die instead of moving.

---

# 3 · cyberia — target structure

the nation: **six shelves**, each a directory with a contract README.

**foundation** (9, stays as is): [[cyberia/foundation/vision|vision]] · [[cyberia/foundation/manifesto|manifesto]] · [[cyberia/foundation/architecture|architecture]] · [[cyberia/foundation/strategy|strategy]] · [[cyberia/foundation/whitepaper|whitepaper]] · [[cyberia/foundation/space doctrine|space doctrine]] · [[cyberia/foundation/org|org]] · [[cyberia/foundation/cyberian|cyberian]] · [[cyberia/foundation/README|README]]

**protocol** (14): [[cyberia/protocol/README|README]] · [[cyberia/protocol/century-index|century index]] · [[cyberia/protocol/bank-above-banks|bank above banks]] · [[cyberia/protocol/location proof|location proof]] · [[cyberia/protocol/space-accounting|space accounting]] · [[cyberia/protocol/services|services]] · [[cyberia/protocol/system|system]] · [[cyberia/protocol/marketplace|marketplace]] · [[cyberia/protocol/marketplace-spec|marketplace spec]] · [[cyberia/protocol/ladder|ladder]] · [[cyberia/protocol/orgs|orgs]] · [[cyberia/protocol/ephemeris|ephemeris]] · [[cyberia/protocol/dyson sphere|dyson sphere]] · [[cyberia/protocol/maps|maps]] *(absorbs the 7 one-line maps/\* pages as a table)*

**land** (new shelf — the operating estate, promoted from research/cyb-land): visit · stay · activities · attractions · community — the ~45 pages with content; the 15 empty attraction/stay stubs die or gain their sentence at move time. incoming: [[landscape]] + [[nitrogener]] as land/flora.

**community** (grows): [[cyberia/community|community]] · talents · blog (chronicle; receives the ~20 valley log entries from cyber's blog)

**courses** (stays): [[cyberia/courses/README|README]] + edge-city-patagonia-2025 set

**research** (workbench, explicitly unfinished): [[cyberia/research/README|README]] · mimi · migration-market · genome-protocol · oxytocin · cyber-sheep · cyberia-my · tech *(the 40 seed pages fold into one indexed table; the ~12 with ≥50 words keep their pages)* · incoming: [[team speed competition]] · [[smart capital]]

**leaves cyberia**: research/events (vendored software product — backend, client, ethereum, deploy) → its own repo `cyberia-to/events`, one product card stays; ops artifacts (mimi error-analysis, indexer test reports) → delete or move with the product.

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
