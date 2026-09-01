---
title: restructure
tags: cyber, core, plan
crystal-type: pattern
crystal-domain: cyber
status: plan
---
# restructure — crystallizing cyb · cyber · cyberia

a plan first, movements second. nothing in this document moves a file; every section below is a contract for a move batch that happens as its own commit, link-fixes included, so the site is never broken in between.

## why

measured on 2026-09-01: the three subject graphs are dumps with crystals inside. cyber/ holds 238 pages of which 53 are empty blog stubs, ~25 are component research that belongs to [[soft3]] repos, ~10 are encyclopedia atoms that belong to the [[crystal]], and a dozen are operational leftovers of the dead JS era. cyb/ holds 85 pages of which half document a product that was retired to old.cyb.ai. cyberia/ holds 145 pages including a vendored software product, forty seed stubs, and fifteen empty attraction pages. link integrity across cyber/ is 94.5%; crystallinity scored 88%.

the goal is not tidiness. the goal is that each subgraph becomes a **finished, idempotent, homoiconic system**:

- **finished** — no empty pages, no phantom link targets, no drafts pretending to be canon. every page either carries its weight or does not exist.
- **idempotent** — membership is decided by a rule, not by history. run the rule twice, nothing moves the second time. the rule is one question: **who is the subject of this page?**
- **homoiconic** — the site is itself a cybergraph: subgraphs are neurons' coherent claims, READMEs are their contracts, links resolve by name — never by guessed mount paths (the `[[cyber/…]]`, `[[inf/…]]`, `[[tokens/…]]` phantom classes are already dead; this plan keeps them dead).

## the rule

| the subject is… | it lives in |
|---|---|
| the protocol — the law of the one mind | **cyber** |
| the robot — the body a user touches | **cyb** |
| the nation — land, people, state | **cyberia** |
| one soft3 component's mechanism | that component's repo ([[tru]], [[bbg]], [[zheng]], [[nox]], [[foculus]], [[cybergraph spec|cybergraph]], [[hemera]], [[strata]], [[neural]], [[mudra]]…) |
| the world at large — physics, biology, history | [[crystal]] / [[cybics]] |
| a dead vehicle — bostrom-era artifacts | [[bootloader]] |

corollary: **cyber is not soft3.** soft3 owns mechanisms; cyber owns what must be true regardless of mechanism — the invariants ([[cyber/cybergraph|contract]]), the money ([[cybernomics]]), the meaning (papers), and the self (the protocol's own agency). a page explaining *how* something is computed belongs to the component that computes it; a page stating *what must hold* belongs to cyber.

---

## 1 · cyber — the protocol

target shape: a flat crystal of ~120 pages in seven modules. flat stays flat — modules are tag-families and hub pages, not new directories; the only directories are the corpora (research/, specs/, self/, blog/).

| module | what it is | anchor pages |
|---|---|---|
| **core** | the ~45 atoms that define the phenomenon | [[cybergraph]], [[particle]], [[neuron]], [[cyberlink]], [[focus]], [[cyberank]], [[karma]], [[staking]], [[mining]], [[impulse]], [[tri-kernel]], [[superintelligence]], [[fixed point]], [[self]] |
| **contract** | protocol invariants over soft3 | [[cyber/cybergraph]], and the family it anchors: cyber/proofs, cyber/identity, cyber/epistemology (to be written — the demand exists) |
| **nomics** | the money | [[cybernomics]], [[nomics]], [[netics]], [[cap]], [[token]], [[cyber/$CYB\|$CYB]], [[cyberlink market protocol]], tokens/{plumb, coin, badge, basic token operations} |
| **papers** | litepaper · whitepaper · protocol-level research | see the research split below |
| **cips** | the deep protocol articles tagged cip | [[3c]], [[channel]], [[communication]], [[network]], [[hierarchy]], [[identity]], [[light]], [[epistemology]], [[gravity]], [[luminosity]], [[security]] |
| **self** | the protocol as a neuron | self/{dmn, linking, parametrization, sigma} |
| **chronicle** | the blog — the protocol's own history | ~40 real posts |

### 1a · research/ splits by subject

component research goes home. every page below is about one component's mechanism and moves to that repo's docs/, with a redirect link left in the research index:

| page | → repo | page | → repo |
|---|---|---|---|
| bbg | [[bbg]] | recursive brakedown | [[zheng]] |
| data availability strategy | [[bbg]] | polynomial proof system | [[zheng]] |
| storage proofs | [[bbg]] | zheng vs starks | [[zheng]] |
| algebraic state commitments | [[bbg]] | 256 symbols | [[nox]] |
| data structures for polynomial state | [[bbg]] | nox — frozen provable computer | [[nox]] |
| programmable state | [[bbg]] | foculus | [[foculus]] |
| state model | [[bbg]] | provable consensus | [[foculus]] |
| polynomial nouns | [[bbg]] | vec formalization | [[foculus]] |
| hashing and confidentiality | [[hemera]] | structural-sync | [[foculus]] |
| five algebras | [[strata]] | spectral gap from convergence | [[tru]] |
| trinity meets polynomial state | [[trident]] | gflownet focus flow | [[tru]] |
| cyberlink protocol structure | [[cybergraph spec\|cybergraph]] | tri-kernel architecture | [[tru]] |

protocol-level research stays: physical analogies, algorithmic essence of superintelligence, knowledge capacity, link production, universal law, egregore properties, future of computation, neuroscience principles for machine mind, unified mining, knowledge economy, theoretical foundations, bootstrap, 32-byte tokens, energy market, gradient descent, cybergraph model architecture, focus flow computation, adaptive hybrid economics. **bostrom compilation report** → [[bootloader]] (it is a report about a vehicle).

### 1b · leaves cyber

| group | pages | → destination |
|---|---|---|
| world atoms | skyrmion, time-crystal, topoisomerase, topological-invariant, helix, happiness, history, landscape, nitrogener, russian school of inscription | [[crystal]] |
| bootloader era | cli (go-cyber), congress, 54, 55, analizer, roadmap (2019 post) | [[bootloader]] |
| cyberia matters | leadership, team speed competition, smart capital, seer, subgraphs/kadek | [[cyberia]] |
| component stubs | subgraphs/{lytics, tape, conformance} | their own repos ([[lytics]], tape, soft3) |
| org process | SPEC.md (workspace/sync spec) | [[cyberia/midao\|midao]] |

### 1c · dies in cyber

- 53 empty blog stubs (2019–2026) — delete; the chronicle keeps only real posts
- [[cyberank]] vs [[rank]] — one page; rank.md holds the alias and the traffic, cyberank.md folds into it
- style, sparks — delete
- research/programming model (11 words) — delete, superseded by [[nox]]

---

## 2 · cyb — the robot

target shape: ~35 pages. the subject is the product a user runs today: the Bevy binary, four worlds, local mind, sovereign transport.

| module | pages |
|---|---|
| **spine** | README (robot), product, philosophy, story, spec, roadmap, team |
| **reference** | architecture (rust), rendering, terminal, cells, apps, format, cyb-registry, fs, scripting, truth, features, releases/ |
| **plans** | .claude/plans/ stays as the workbench |

### leaves cyb

| group | pages | → destination |
|---|---|---|
| JS-era product (retired to old.cyb.ai) | backend, main, virus, sync (legacy), mvp-screens, wasm (CyberWasm), signer, wgpu, oracle/*, portal/*, brain/*, robot/{psycho, trainer}, offline, dev | **cyb/legacy/** — one directory, one README explaining the era; candidates for deletion after a season |
| component matter | compile, context (→ [[tru]] ct0 docs) · runtime, hardware (→ [[honeycrisp]]) · order (→ [[nox]]) · multiproof (→ [[zheng]]) · model-lifecycle (→ [[tru]]) · wire (→ [[cybergraph spec\|cybergraph]]) · os, root, pipeline (→ [[soft3]]) | each component's docs/ |
| duplicate whitepaper | root/whitepaper (cyb: the immortal robot) | merge its living claims into product.md; the paper archives to legacy/ |

empty portal/oracle/brain stubs (avatars, neurons, skills, spells, learn…) die with the legacy move.

---

## 3 · cyberia — the nation

target shape: foundation (why) · protocol (how the state runs) · land (where) · community (who) · courses (teaching) · research (workbench). the repo is closest to coherent already; its problem is boundaries and stubs.

| move | why |
|---|---|
| research/events/ (backend, client, ethereum, deploy — a vendored software product) | own repo `cyberia-to/events`; cyberia keeps a one-page product card |
| research/cyb-land/ → land/ (top-level) | it is not research — it is the operating estate; 15 empty attraction/stay/visit pages die or gain a sentence |
| research/tech/ 40 seed pages | keep the ~12 with content ≥50 words; the one-line stubs (stirling engine, inverter, biofilter…) fold into a single [[cyberia/research/tech\|tech index]] with one line each, pages deleted until someone writes them |
| research/mimi error-analysis, events test reports | ops artifacts — delete or move into the product repo |
| incoming from cyber | leadership, team speed competition, smart capital, seer, kadek |
| protocol/maps/* one-liners (bed, block, district, region, sector, trail) | fold into [[cyberia/protocol/maps\|maps]] as a table |

---

## 4 · order of movements

each phase is one commit series, link-fixes in the same commit, site verified green before the next:

1. **deletions that free names** — blog stubs, style, sparks, empty attraction pages, dup indexes (no inbound links by measurement — zero breakage)
2. **within-repo folds** — rank⊕cyberank, maps table, tech index, cyb-land → land/
3. **cyber → outward** — world atoms to crystal; bootloader era to bootloader; cyberia matters to cyberia; component stubs out (≈25 pages, ~40 link fixes)
4. **research redistribution** — 24 pages to component repos, research index rewritten as a map of both what stayed and where the rest went
5. **cyb legacy fold** — legacy/ directory, component matter out, whitepaper merge
6. **cyberia boundaries** — events extraction, incoming pages placed
7. **the missing contract pages** — write cyber/proofs, cyber/identity, cyber/epistemology from the existing epistemology/identity/security material (promotion, not creation from nothing)

## 5 · acceptance

the job is done when, measured again:

- link integrity ≥ 99% in each of the three subgraphs (was 94.5%)
- zero pages under 12 words outside indexes (was 38 in cyber alone)
- zero phantom targets with ≥3 inbound refs (was 7)
- every page passes the rule in one step — no page whose subject is arguable between two subgraphs without this document naming the tiebreak
- crystallinity score ≥ 95% (was 88%)

the measurement scripts live in the session log; the score recomputes in one pass. run it after each phase, publish the number in the commit message.

---

*plan approved → phases execute as separate commit series. this page then becomes the record of what moved where, and why.*
