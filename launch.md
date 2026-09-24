---
tags: cyber, launch, roadmap, article
alias: launch, cyber launch, launch phases, master plan, nox master plan, nox_master_plan, bootloader reborn, 2026-11-05
crystal-type: plan
crystal-domain: cyber
status: active
---
# cyber/launch

the launch of [[cyber]] is a sequence of phases, each a network that must hold before the next one exists. the phase with a date is the first: on 2026-11-05, five years after the [[bootloader/bostrom|bostrom]] genesis, bostrom and [[bootloader/tokens/$PUSSY|pussy]] come back to life in a new body, minutes apart, as two [[soft3]] chains with the full reward mechanics. [[$CYB]] launches later, after implementations in several languages and formal verification, and the two reborn chains are the bootloader and the canary that earn it.

version: 2026.09 · status: phase 1 in delivery · this page is the single tracker; every gate, decision and status change lands here.

## the phases

| phase | what | gate | when |
|---|---|---|---|
| 0 | foundations: field, hash, commitments, VM, proofs, state | complete | done |
| 1 | the bootloader reborn: bostrom and pussy on soft3 with settlement mining, fold, foculus consensus and privacy | the property registry below is green or explicitly deferred; two chains live from the burial snapshots | 2026-11-05 |
| 2 | canary: the two chains under real use and weekly red team | 90 days, all economic invariants hold, no critical bug under attack | 2026-11-05 → 2027-02 |
| 3 | $CYB mainnet | implementations in several languages agree on 10⁶ blocks; the formal spine (P4) green; the six mainnet questions answered | after phase 2 |

a milestone is code that passes its gate. the date exists for phase 1 only, and it is a gift, so what does not fit the date is cut from phase 1, never squeezed.

## where we are · 2026.09

the stack was built bottom-up. the cryptographic floor is implemented and tested; the frontier is running the pieces together as a network. counts are rust lines and `#[test]` functions in the checkout on 2026-09-18.

| component | role | lines | tests | phase 1 use | status |
|---|---|---|---|---|---|
| [[strata]] | five algebras, [[nebu]] Goldilocks | 16.1K | 453 | every field operation | complete |
| [[hemera]] | [[Poseidon2]] hash, [[particle]] identity | 11.4K | 313 | particles, transcripts, trees | complete, unaudited |
| [[lens]] | commitment backends, 7 crates | 5.6K | 128 | Brakedown for zheng, bbg openings | evaluation binding is `TensorMerkle`; recursive form blocked |
| [[soft3/nox|nox]] | proof-native VM, 18 patterns + jets | 8.9K | 175 | the protocol's own programs | complete |
| [[zheng]] | SuperSpartan + Brakedown + HyperNova folding | 12.6K | 211 | ticket proofs, fold, decide, privacy proofs | execution proofs bind computation to result; fold + decide exist; private execution incomplete |
| [[soft3/bbg|bbg]] | authenticated state, vendored fjall | 13.1K | 216 | state, openings, file store | core complete; durable storage gates open (P0) |
| [[radio]] | QUIC transport, gossip, blobs with cyber-bao | 194K | 559 | signal gossip, file bytes by particle | multi-node tested; a dependency of nothing in phase 1 yet |
| [[foculus]] | consensus by convergence, beacon, settlement, tickets, fold, domain finality | 15.7K | 207 | cores 2, 3, 5 | single machine, never a network; `net` feature on upstream iroh |
| [[tru]] | φ*, [[tri-kernel]], impulse, truth scoring | 10.2K | 86 | the value oracle, the serum | φ*, κ < 1, impulse; serum scored in 6 tests |
| [[tok]] | coin, card, conservation, mint, lock, burn | 0.6K | 8 | mint, stake, ICBS, $ν books, referral | published; no live economy |
| [[soft3/cybergraph\|cybergraph]] | signal processor, applications, receipts | 7.5K | 97 | admission, registration of books | durable applications in review |
| [[inf]] | query engine over authenticated state, 7 crates | 80.8K | 369 | what links this file; reads for cyb | live |
| [[mudra]] | identity, legacy bridge, spell, stealth, veil | 2.9K | 49 | claims, P1–P3, valence | keys, claims, bridge; stealth and veil specified |
| [neuron](../neuron/README.md) | the subject: id, model, engine, node | 9.8K | 50 | one identity across bbg, mudra, tok, inf | local |
| [[tade]] | framing | 1.2K | 26 | the wire | 0.1.0 published |
| [[file]] | `file = (particle, data)`, kinds, sniff | 0.2K | 4 | the one particle type; render kinds | two days old; used by spark and cyb only |
| spark | opener: file → surface | 0.1K | 3 | media and PDF render | text and image |
| [[soft3]] | node and CLI | 2.8K | 28 | the chain binary | one node live, HTTP ingress, one signal one block |
| [[cyb]] | the body: worlds, networks, relay, vault | 19.2K | 76 | the client; content visible | networks, relay, beacon, spell, fleet gate |
| [vault](../vault/README.md) | custody, spell, signing | 6.7K | 46 | keys and claims in cyb | local CLI, 46 tests |
| [[rune]] | cyb's page language, 6 crates | 8.3K | 193 | memory and graph worlds | 0.1.0 |
| [[prysm]] | paint: chunks → UI | 1.6K | 0 | every rendered row | live in cyb, no tests |
| [[mir]] | render: positions → world | 7.6K | 54 | the graph world | live |
| [[soma]] | kernel and agent | 4.3K | 24 | the mind's tasks in cyb | local |
| [[glia]] | model runtime, import | 30.5K | 159 | local inference in cyb | 9 tok/s |
| [[honeycrisp]] | acpu, aruminium, unimem | 134K | 366 | glia's accelerator | live |
| nu | nushell fork, 24 crates, cyb's console | 387K | 5228 | the terminal world | a local checkout without a git remote; a reproducibility gap for the release train |
| [[fs]] | edit, patch, sovereign sync | spec | — | none; bytes move over radio | target design only |
| [[bootloader/bostrom|bostrom]] | burial scripts, snapshot, claim checker | py/js | — | genesis pipeline, claim page | complete for the old chain |
| [[cybernode]] | cyberproxy, deimos, jupiter, io | ops | — | three nodes, file store, blockstore | io overloaded |

the dependency closure above is computed from the path dependencies of the phase-1 binaries (soft3 node, cyb, cy, true-cyber, foculus, neuron, tok, tru) on 2026-09-18, plus the non-code owners. radio and fs are listed because the cores need them, not because a binary depends on them yet.

the burial is complete and is the genesis material: bostrom halted at 25,120,712 with 61,675 accounts, 46,039 passport owners, 2,949,732 cyberlinks and 3,143,650 particles rebuilt bit-exact from block events, 97.63% of particle bytes present; space-pussy with 29,112 links recovered. every dataset is pinned with a CID and a sha256 at [snapshot.bostrom.network](https://snapshot.bostrom.network).

## phase 1 · the bootloader reborn · 2026-11-05

today · 2026-09-23 · 43 days to launch. gates: cyb fleet green (21/21), foculus · tru · mudra · tok · cybergraph green, zheng red unchanged (owner-tree example `legacy_wire_cost`, `TraceProof: serde::Serialize`), spacepussy-test at 39,672 (+31 blocks: the fleet gate's relay cast them). red: incident, see the decisions log dated 2026-09-23: the workers invented registry rows 39–253 and opened 217 pull requests overnight; registry restored to the owner's 38 rows plus 39–40 that absorb the sweep's real findings, prompt hardened, workers paused until the triage closes, the 33 reviewed PRs untouched and still 0 merged. red: the reviewed merge queue is now five days old; every day it waits, the pin fix in eleven foculus PRs drifts further from the owner's dirty tree. triage result at 12:10 UTC: of 211 sweep PRs, 34 kept and retitled onto rows 13, 14, 16, 22, 23, 34, 37, 39, 40 (listed in the work log), 170 closed with the label `sweep`, 7 soft3 coverage PRs had already been merged by the owner. the owner merged 8 pull requests this morning, the first merges since the workers started: 7 soft3 coverage PRs and bbg #15, which closes the self-authentication half of row 23. open launch PRs now: 72 (33 reviewed + 34 kept + spark #1 + retitles).

review · 2026-09-22 · all 33 worker pull requests reviewed by four review agents, each in an isolated worktree with the tests run: 30 to merge, 2 held as drafts (mudra #3, #4 until the mudra `cyber-nox` 0.3 pin lands), 1 new (spark #1, so that file #1 cannot break spark and cyb). real defects found and fixed on the branches before the verdicts: foculus #7's beacon binding let an attacker relabel the epoch or substitute claims_root on a finished VDF (b1cf1d5 fixes the binding and the tests now recompute the digest); tru #11 clawed back past annuity accrual on a falling target (clipped at zero per §2 and §12); tru #9's minority test passed by a 2-ULP accident (rewritten as a fixed-share minority with a non-collapsing margin); plumb #4 computed the referral share through f64 (now u128 fixed point); plumb #5's Sybil proof was a tautology (replaced by a real fee pool distributed pro rata to $ν holders); plumb #1's burn panicked on an unknown holder; foculus #15 listed cluster members in input order (now canonical); foculus #14 compared f64 confidences (now an integer threshold); zheng #21 was rewritten around the corrected P1 (author leak, not content). the eleven foculus PRs now carry a byte-identical Cargo pin fix equal to the owner's #5 and merge in any order with zero conflicts; the plumb chain is prefix-closed 1→5; tru 8→11 merges clean. merge order: foculus #12 → #13 → #6 → #7 → #8 → #9 → #10 → #11 → #14 → #15 → #16 → #5 · tru #8 → #9 → #10 → #11 · zheng #20, #21 · plumb #1 → #2 → #3 → #4 → #5 · mudra #5 · file #1, #2, spark #1 · radio #5 → #4 · bbg #9 (owner) → #10, #11 · cybergraph #4, #5, #3 (owner) · cyb #1392. every row stays partial after merge: no PR provides the multi-node gate, the VDF is the sequential-squaring placeholder, banded_target and partition_into_clusters are not yet called by a live epoch.

### the four cores

these are the point of launching. none of them is cut, none of them is faked.

1. mint by Shapley and settlement mining. a neuron links, [[tru]] measures the directed impulse Δφ⁺, the surprise gate ρ prices it, and the share is settled by the lottery of [[rewards|rewards]] §7: every hash attempt is a real Shapley sample, every winning ticket carries a [[zheng]] proof, and the swarm mean converges by Hoeffding. the subsidy is the same ticket (§8).
2. fold. winning tickets aggregate per cluster into one O(1) accumulator by HyperNova folding ([[foculus]] fold-mining): self-fold, cluster tree, decide, then [[tok]] mints under conservation. without this, settlement verification is a DoS surface.
3. consensus by convergence. [[foculus]] on independent nodes: a particle is final when φ*ᵢ > τ, forks resolve by φ*, the beacon comes from a VDF over finalized signals, equivocation is rejected, and nodes converge from different starting states.
4. hybrid economics: truth markets and staking. this is the game changer, and it is what makes the mint more than a payout. two mechanisms, one program set:
   - the truth market. every cyberlink carries an [[ICBS]] position (cost $C = \lambda\sqrt{s_Y^2+s_N^2}$, self-scaling liquidity, the spam cost) and a [[Bayesian Truth Serum]] meta-report; the serum is the oracle, the market the liquidity skin over it, and [[valence]] privacy the third leg that removes the coordination channel. [[strong truthfulness]] proves the fused mechanism extracts truth even in the perpetual market with no external resolver. surprise ρ, the gate of the mint, is read from it.
   - staking on two axes ([[rewards|rewards]] §9–13). any stake moves rank: weight in $A^{\text{eff}}$, hence φ* and [[cyberank]]. only correct risk under $v_\ell \neq 0$ moves reward; passive capital buys influence and earns nothing by category. the security budget splits PoW/PoS by the allocation curve $\theta^\alpha$; base emission goes to work and risk only. operations: mint, burn (eternal particles and cyberlinks), lock (stake on particles or cyberlinks). the pulse is the instant mint; the annuity is the yield stream that pays foundational links as the graph grows around them.
5. personal chains and the referral. every [[neuron]] roots its own home book ([[cyber/research/oikos|oikos]]): one non-fungible name, one token in the neuron's name, $ν. the book is where a personal mind is born: the neuron's links, its local φ*, its obligations, settled at home and registered into the [[cybergraph]] by name and state root; [[foculus]] gives each book domain finality. without personal chains no personal brains are born, so this ships in phase 1 with the two root chains.
   the referral rides on it. registration is a cyberlink: the referrer's neuron links the newcomer's name. at the birth of ν's book a fixed share r of $ν goes to the referrer, and holding $ν is the right to a pro-rata share of the fees ν's book collects for as long as the book lives. the referrer is paid in the referee's own token, from the referee's own economy: a fake account has a book with no fees and a token worth nothing, so referring it pays nothing, which keeps the referral Sybil-resistant by construction, in the spirit of paying on focus created and never per head.
6. privacy. three invariants, all three at genesis:
   - P1 · the edge is public, the author is private. the network sees that particle p links particle q, the axon weight, and the aggregates φ* is computed from; it does not see which neuron signed, who holds the position, or who owns the energy. corrected 2026-09-22: the earlier wording, content visible only to participants, would have hidden the edges themselves and left nothing for the collective φ* to run on.
   - P2 · balances and transfers are private by [[mudra]] stealth addresses and veil.
   - P3 · settling a marginal m(n) reveals nothing of the miner's ego-net beyond public aggregates.

learning is not a seventh item: it is what the first item measures. φ* is the state of the collective mind, and a link pays because it moved it.

### cut from phase 1

- programmability for users: nox programs on chain, [[joy]] for foreign programs, [[trident]] contracts. the chain runs exactly the programs the protocol needs: φ* and impulse, settlement and fold, the ICBS market and the serum, staking with lock, burn and the yield stream, conservation and the mint. programmability for anything else returns as the first canary upgrade.
- UniversalHash and the viewing economy: out of scope; a separate network later.
- the fee role as paid inference; book-level fees on personal chains are in, because the referral pays from them. sigma, eidos, wysm, kern, soma tasks, IKP/IBC, Neptune.
- [[$CYB]]: phase 3.

### the two chains

one token, one chain ([[cyber/research/oikos|oikos]]). bostrom carries [[bootloader/tokens/$BOOT|$BOOT]], pussy carries [[bootloader/tokens/$PUSSY|$PUSSY]]. same binary, same genesis pipeline, two genesis roots, launched minutes apart on 2026-11-05. bostrom is the bootloader; pussy is the canary that takes every rehearsal first. they are the two root books; every neuron's personal book hangs off them by registration, so the same binary that runs bostrom runs a neuron's own chain.

genesis rules:

- balances come from the burial snapshot, every denomination. every account claims by signature with its old key; the [[mudra]] legacy bridge turns a secp256k1 key into a hemera neuron. 8,390 wallets that never sent a transaction are still claimable. the 8.97T BOOT held by the passport contract is a decision still open on this page.
- stake at genesis is bonded uniformly from the snapshot, as the old conditions distributed it: everybody starts bonded, and whoever wants order rebalances. the bond accounts for milliampere and millivolt held at halt, because without the resource tokens the old links carried no weight and the migrated graph would pay nothing. the exact formula is a line in the registry below.
- the graph comes home as files and cyberlinks: 2.9M links and 3.1M particles for bostrom, 29K links for pussy, replayed root matched against the snapshot. karma carries as reputation.
- the missing 2.37% of particle bytes are listed; a black hole stays a black hole until someone sparks it.

### content, visible: cyb

the old network let people link and never let them see what they linked. this is the failure phase 1 fixes, and it weighs as much as the cores: a graph nobody can read is a graph nobody links.

- availability: every particle the chain references resolves in [[cyb]] from the network, the burial blockstore or a peer; the 97.63% baseline is the floor and the missing list is public.
- render: text and images today; phase 1 adds media (video, audio) and a PDF reader, because a large share of the bootloader's linked files are PDFs. the render lives in `spark`, the opener that turns a file into a surface.
- from any link to its two files in one tap, and from any file to what links it.

### critical dependencies found on the second scan · 2026-09-18

the first draft of this page missed six dependencies. two of them decide genesis.

1. one particle. three definitions are in play today: the [[file]] crate and [[cyber/file]] say `particle = hemera(data)`, 32 bytes, no prefix; the cybergraph particle spec describes a flat 64-byte namespace; [[cyber/particle]] names bbg's construction, hemera over the [[lens]] commitment to the data, domain-separated. every particle of the migrated graph is computed by whichever wins, so the definition freezes before the genesis pipeline runs, and `file::Particle` becomes the one type bbg, cybergraph, foculus and tru share instead of four private `[u8; 32]` aliases.
2. re-addressing the bootloader graph. the 3,143,650 particles of bostrom are IPFS CIDs; the new identity is a hemera hash of the bytes. the 97.63% with bytes present are re-hashed and carry their CID as a naming cyberlink; the 2.37% without bytes have a CID and no data, so their particle cannot be computed, and the rule for them is a genesis decision: a black hole addressed by a cyberlink from the CID label, sparked if the bytes ever surface.
3. transport. [[radio]] is a dependency of nothing in the phase-1 binaries; the node accepts signals over HTTP (`/v1/link`, `/v2/frame`, `/v3/action`) and foculus carries an optional `net` feature on upstream iroh for settle gossip only. signal gossip between nodes and blob transfer of file bytes by particle (iroh-blobs with cyber-bao) both hang on wiring radio into the node and into cyb. this is on the critical path of core 3 and of lane C at once.
4. serving the burial bytes. 3,069,134 files sit on io's IPFS flatfs under CIDs, on an overloaded machine. phase 1 needs a file store that answers by hemera particle, seeded from that blockstore, replicated to the three network machines, so that cyb resolves what the graph references. storage economics stays open; availability at genesis does not.
5. file kinds for render. media and PDF enter through `file::Kind` and `sniff`, then spark opens them; the crate is two days old and only cyb uses it.
6. the subject. `neuron-id` is the identity every core crate already shares (bbg, mudra, tok, inf, soma); claims by old key land on it. it stays in, and it is the model for what `file::Particle` must become for content.

out of scope and noted: five new warriors created on 2026-09-16 (gaw for Polkadot, tolya for Solana, vitalina for Ethereum, zenda for Zcash, pearla for Pearl inference) belong to the warriors program, not to phase 1.

### lanes

| lane | owns | first gate |
|---|---|---|
| A · core | settlement, fold, foculus network, truth market, staking, personal chains and referral, privacy | one node: impulse → m(n) → ticket → zheng proof; one link with an ICBS position and a serum report scored |
| B · body | genesis pipeline, nodes on three machines, claims, releases | genesis of pussy boots from real data |
| C · content | one particle type, file store by particle, radio blobs, media and PDF render in cyb | a bostrom PDF opens in cyb from its cyberlink, fetched by particle |

lane A is the critical path and yields to nothing. lanes B and C run on the proven conveyors: the burial scripts and the coordinated release train of 2026-09-16.

### property registry

every claim the launch stands on, with the kind of evidence it has. states: proven · simulated · measured · open. an open property has a date by which it is closed or explicitly deferred to phase 2 on this page.

| # | property | owner | state | evidence | close by |
|---|---|---|---|---|---|
| 1 | φ* exists, unique, converges with κ < 1 | tru | proven | convergence.md, bostrom measurement λ₂ ≈ 0.13 | closed |
| 2 | Σφ*ᵢ = 1 | tru | proven | tri-kernel spec | closed |
| 3 | ticket proof cost < ticket reward | foculus, zheng | open | measure on real m(n) with the folded proof; foculus/audit/ticket-proof-cost.md — O(1) shape holds (decide ~18.7ms, verify ~9.2ms flat n=8..512), reward-side pricing still blocked on #26/#27; PR open: [foculus#6](https://github.com/cyberia-to/foculus/pull/6) — measured: decide/verify 9–19 ms per ticket, flat in size; spec said 10–50 µs | 2026-09-25 |
| 4 | progress-freedom across clusters: difficulty schedule corrects per-cluster cost | foculus | open | settle-target banded linearly by cluster size (`tickets::banded_target`); two-point measurement n=4 vs n=32 shows flat target ratio ~12x collapsed to ~1.5x banded (launch #4, foculus/audit/progress-freedom-banding.md); network-scale million.rs simulation still open; PR open: [foculus#10](https://github.com/cyberia-to/foculus/pull/10) — banded target by cluster size | 2026-10-09 |
| 5 | fold is a commutative monoid, decide is O(1) | zheng, foculus | open | end-to-end on three nodes; PR open: [zheng#20](https://github.com/cyberia-to/zheng/pull/20) — fold either order, both decide() | 2026-10-09 |
| 6 | beacon unpredictable and unbiasable | foculus | open | VDF over finalized signals, attack_vectors.rs; PR open: [foculus#7](https://github.com/cyberia-to/foculus/pull/7) — four VDF-binding attacks rejected | 2026-10-16 |
| 7 | withholding bias bounded by compute share | foculus | open | analytic bound derived, bias(q) ≤ (mean−min)·q/(1−q); measured on a 400-ticket pool at q=0.05–0.5, within bound and monotone (launch #7, foculus/audit/withholding-bias.md); priced forfeit and role separation still open; PR open: [foculus#11](https://github.com/cyberia-to/foculus/pull/11) — withholding bias bounded by compute share | 2026-10-16 |
| 8 | no two nodes finalize conflicting state | foculus | open | network gate with conflicts and partition; PR open: [foculus#8](https://github.com/cyberia-to/foculus/pull/8) — MinHash agrees across n-way conflicts | 2026-10-16 |
| 9 | nodes converge from different starting states | foculus, cybergraph | open | network gate; PR open: [foculus#9](https://github.com/cyberia-to/foculus/pull/9) — convergence order-independent of arrival | 2026-10-02 |
| 10 | P1 signal privacy: edge public, author private | mudra, zheng | open | commitment + validity proof, leakage bounded — audited: cybergraph's signal spec broadcasts `ℓ⃗` in plaintext with a validity proof over disclosed content, the inverse of P1; no mudra spec defines a content-commitment interface; zheng's private-execution backend has no reviewed hiding protocol (launch #10, mudra/audit/p1-signal-content-commitment-gap.md); PR open: [zheng#21](https://github.com/cyberia-to/zheng/pull/21) [mudra#3](https://github.com/cyberia-to/mudra/pull/3) — audit: wire format leaks full link content; P1 needs zheng private execution | 2026-10-16 |
| 11 | P2 stealth and veil | mudra, tok | open | stealth.md, veil.md, transfer vectors — audited: tok's ledger holds every balance plain by `(NeuronId, TokenId) → u64`, no ciphertext type or stealth column; neither stealth nor veil has code in mudra/src, and stealth's dependency crate `genies` does not exist in this checkout (launch #11, mudra/audit/p2-stealth-veil-transfer-gap.md); PR open: [mudra#4](https://github.com/cyberia-to/mudra/pull/4) — audit: stealth/veil gap vs tok ledger | 2026-10-16 |
| 12 | P3 miner ego-net hidden beyond aggregates | foculus | open | interface invariant 4, test — audited: `SettlementTicket.marginals` and `replay_marginals` publish the full per-contributor vector by design, not an aggregate; closing needs zheng private execution (launch #12, foculus/audit/p3-ego-net-leak.md); PR open: [foculus#12](https://github.com/cyberia-to/foculus/pull/12) — audit: tickets publish the full marginal vector in the clear | 2026-10-16 |
| 13 | conservation under mint and transfer | tok | open | proof at every mutation — mint, burn, transfer tested (plumb#1); lock remains; PR open: [plumb#1](https://github.com/cyberia-to/plumb/pull/1) [plumb#5](https://github.com/cyberia-to/plumb/pull/5) — transfer added; referral payout conserved | 2026-10-09 |
| 14 | genesis is bijective with the snapshot | bostrom, cybergraph | open | replayed root matches, zero loss; PR open: [cybergraph#5](https://github.com/cyberia-to/cybergraph/pull/5) — genesis replay bijection contract | 2026-10-02 |
| 15 | claim by old key binds one neuron | mudra | measured | 13 bridge tests | closed |
| 16 | every referenced particle resolves in cyb | cyb | open | availability audit, missing list — audit module implemented and tested: resolves against local store + ASCII names, reports fraction and missing list (launch #16, cyb/audit/particle-availability.md); run over the real burial vocabulary still blocked on 22/23; PR open: [file#1](https://github.com/cyberia-to/file/pull/1) [cyb#1392](https://github.com/cyberia-to/cyb/pull/1392) — PDF/video/audio sniffed; availability audit module (draft) | 2026-10-23 |
| 17 | signed signal cannot be forged or replayed | mudra, cybergraph | measured | signal codec strict, consumer audit | closed |
| 18 | durable storage survives failure and restart | bbg | open | bbg P0 gates D1–D5 — D2 archival population and checkpoint boundary specified (launch #18, bbg specs/storage.md), population code still open; PR open: [bbg#10](https://github.com/cyberia-to/bbg/pull/10) — WARM archival population and checkpoint boundary | 2026-10-09 |
| 19 | one particle definition, one `file::Particle` type across bbg, cybergraph, foculus, tru | file, bbg, cybergraph | open | decision on this page, then the type shared — five current definitions and their contradictions surveyed (launch #19, cybergraph/audit/particle-alias-survey.md); PR open: [cybergraph#4](https://github.com/cyberia-to/cybergraph/pull/4) — survey of five particle definitions with file:line | 2026-09-25 |
| 20 | every bostrom particle re-addressed under hemera with its CID kept as a naming link; missing bytes have a defined identity | bostrom, cybergraph | open | `readdress(cid, bytes)` implemented and tested: bytes present → `Particle::hash(bytes)`, missing → black hole `Particle::hash(cid)`, sparks when bytes surface (launch #20, file crate); the run over the 3,143,650-CID burial blockstore with count-matches check still open; PR open: [file#2](https://github.com/cyberia-to/file/pull/2) — re-address CIDs to hemera particles, black hole for missing bytes; needs the decision in row 19 | 2026-10-02 |
| 21 | signals gossip between nodes over radio | radio, foculus, soft3 | open | fixed a pre-existing build break (iroh-docs pulled upstream iroh-blobs/iroh-gossip instead of this workspace's fork); proved multi-hop relay — a 4-node chain where the receiver only knows its immediate neighbor still gets the broadcast (launch #21, radio); real Signal payload, "before finality", and wiring radio as soft3's actual transport still open; PR open: [radio#5](https://github.com/cyberia-to/radio/pull/5) — gossip relays multi-hop; fork pins fixed | 2026-10-02 |
| 22 | file bytes fetch by particle over radio blobs with verified streaming | radio, cyb | open | cyb opens a file it never had from a peer; PR open: [radio#4](https://github.com/cyberia-to/radio/pull/4) — verified-streaming tests for the particle CLI | 2026-10-09 |
| 23 | a file store answers by particle on the three network machines, seeded from the burial | cybernode, bbg | measured | 3,069,134 files served, availability audit; PR open: [bbg#11](https://github.com/cyberia-to/bbg/pull/11) — audit: network fetch has no self-authentication check ; bbg#15 merged 2026-09-23: fetch_content verifies the fetched bytes hash to the particle| 2026-10-16 |
| 24 | the serum is strictly proper in the meta-report and selects truth over coordinated consensus | tru | proven | strong-truthfulness.md; truth_scoring.rs 7 tests, incl. scaling to 200-agent coordinated majority (launch #24, tru/audit/coordinated-consensus-scoring.md); PR open: [tru#9](https://github.com/cyberia-to/tru/pull/9) — informed minority beats coordinated majority at scale | implementation open, 2026-10-09 |
| 25 | ICBS market: cost function, inverse coupling, spam cost, per-link positions under conservation | tok, tru | open | cost function + prices implemented and tested (tru/rs/icbs.rs, launch #25); market program on chain, honesty_loop.rs extended still open; PR open: [tru#8](https://github.com/cyberia-to/tru/pull/8) — ICBS cost function and price derivatives | 2026-10-09 |
| 26 | two axes: passive stake moves rank and earns nothing; only $v_\ell \neq 0$ risk earns | tok, tru | open | reward equation §11 implemented, test with idle and Sybil capital; PR open: [plumb#2](https://github.com/cyberia-to/plumb/pull/2) — stake-yield pays active risk, zeros passive and Sybil | 2026-10-16 |
| 27 | allocation curve $\theta^\alpha$ splits the security budget; floor derived from attack economics | tok | open | split and floor formulas implemented over Fx, simulated across α∈{0.3,0.5,0.7} and c_sec∈{1x..5x} (launch #27, tru/audit/allocation-curve.md); c_sec/r_atk PID loop still open; PR open: [tru#10](https://github.com/cyberia-to/tru/pull/10) — PoW/PoS allocation split and security floor | 2026-10-16 |
| 28 | the yield annuity pays foundational links; the discovery leak (§12) bounded or accepted | tru, foculus | open | discrete per-epoch accrual primitive implemented and tested, falsified-link non-reversal verified (launch #28, tru/audit/annuity-accrual.md); wiring ω(t)/Δφ*_j(t) to real ICBS price and focusing output still open; the leak is an accepted open frontier for phase 1; PR open: [tru#11](https://github.com/cyberia-to/tru/pull/11) — per-epoch accrual of the annuity | 2026-10-23 |
| 29 | valence privacy hides individual positions and reports; only aggregates public | mudra, zheng | open | same proof profile as P1; PR open: [mudra#5](https://github.com/cyberia-to/mudra/pull/5) — audit: valence leaks on the wire | 2026-10-16 |
| 30 | any neuron roots a home book with its own token on the same binary; registration links name and state root into the graph | soft3, tok, cybergraph | open | a neuron on pussy-rc roots a book from cyb; the book settles at home — ledger half done: deterministic book token id, one-home-book-per-neuron invariant (launch #30, tok/oikos-book.md); cybergraph naming-link registration still open; PR open: [plumb#3](https://github.com/cyberia-to/plumb/pull/3) — one home-book token per neuron | 2026-10-09 |
| 31 | domain finality per book: a book settles locally, cross-book conditions wait on evidence | foculus | open | oikos foundation 2 specified and exercised on two books; PR open: [foculus#13](https://github.com/cyberia-to/foculus/pull/13) — book-level domain finality | 2026-10-16 |
| 32 | referral: a share r of $ν at book birth goes to the referrer who linked the name; $ν holders receive the book's fees pro rata under conservation | tok, cybergraph | open | birth allocation vector done — split committed atomically under conservation, self-referral rejected, r a caller-supplied parameter pending the open r decision (launch #32, tok/referral.md); fee split and transfer vectors still open; PR open: [plumb#4](https://github.com/cyberia-to/plumb/pull/4) — referral birth-mint split | 2026-10-16 |
| 33 | referring an inactive or Sybil account yields zero | tok, tru | open | simulation with fake books; PR open: [plumb#5](https://github.com/cyberia-to/plumb/pull/5) — zero for inactive and Sybil books | 2026-10-23 |
| 34 | the public graph is served by every full node and every partial node for its namespaces, replicated by radio, sampled by DAS; a light client verifies against the tip root; the burial file store is the seed, not the only copy | foculus, bbg, radio, cybernode | open | `decide_availability` implemented and tested: a withheld (non-responding) shard counts as a failed sample exactly like a tampered one, verdict compared against a confidence threshold (launch #34, foculus/src/das.rs); this closes the local, single-process half of "DAS sampling passes against a withheld chunk" only — three-machine replication, a fourth node joining, and light-client tip-root verification are unstarted; PR open: [foculus#14](https://github.com/cyberia-to/foculus/pull/14) — DAS availability decision treats a withheld shard as a failed sample | 2026-10-16 |
| 35 | clusters are canonical: an epoch's claims partition into connected components of overlapping ε-supports, derived from the graph and ε, no miner-drawn boundary; the settlement proof commits to the support | foculus, tru | open | `epsilon_support` (bounded-radius neighborhood) and `partition_into_clusters` (union-find, canonical order) implemented and tested (launch #35, foculus/src/cluster.rs); radius not yet derived from ε, wiring into epoch.rs and the settlement-proof boundary commitment still open; PR open: [foculus#15](https://github.com/cyberia-to/foculus/pull/15) — ε-support derivation and canonical cluster partition | 2026-10-09 |
| 36 | hardware profile of a ticket measured on the bostrom graph: samples per second and joules per sample on Apple Silicon, x86 desktop, a phone and a GPU; the per-cluster difficulty schedule equalizes pay per second of work | foculus, warriors | open | `hardware_profile` binary grinds real `try_settlement_ticket` samples with a streaming `powermetrics` sampler; one of four points measured — Apple M4 Max: 672.8 samples/s at ε-support 8 down to 2.1 samples/s at ε-support 4096, 5-6.4W, near-linear in support size (launch #36, foculus/audit/hardware-profile.md); x86, phone, GPU runs still open; PR open: [foculus#16](https://github.com/cyberia-to/foculus/pull/16) — samples/s and mJ/sample vs ε-support size on Apple Silicon | 2026-10-16 |
| 37 | fixed point everywhere on the settle-mint path: tok `conservation.rs` (`fx_to_tokens`, `fx_weight`) converts through f64 on main; foculus `k_min` and `fx_weight` likewise | tok, foculus | open | found in review; violates arithmetic.md; a one-file fix — both functions rewritten over `x.raw().as_u64()` in u128, `fx_to_tokens` rounding half-up and `fx_weight` truncating to match the old f64 behavior exactly; `cargo test` green (10, 2 new); PR open: [plumb#12](https://github.com/cyberia-to/plumb/pull/12) — compute fx_to_tokens/fx_weight in fixed point, not f64 | 2026-09-30 |
| 38 | mudra builds from a clean checkout: `cyber-nox` pin 0.1.2 → 0.3 (the owner's dirty tree has it) | mudra | open | pins bumped to match local nox 0.3.0/zheng 0.4.0, `cargo check --tests` and `cargo test` green (20 tests); `--features prove` still broken on a real API break (`Statement.bbg_root`), documented in mudra/audit/nox-pin-build-gate.md as the next slice; PR open: [mudra#6](https://github.com/cyberia-to/mudra/pull/6) — bump cyber-nox/zheng pins so mudra builds from a clean checkout | 2026-09-25 |
| 39 | every phase-1 component builds and tests from a clean checkout of its default branch against the default branches of its siblings: no dead `../tape` path, no version pin behind a sibling, no crate present only in an owner's working tree | every owner | open | found by the workers' sweep: bbg, cybergraph, foculus, mudra, neuron, nox, prysm, zheng, true-cyber, glia, honeycrisp, rune, vault, radio all fail today; one pin PR per repo, then the release train proves it weekly | 2026-09-30 |
| 40 | a verifier checks every argument it takes: no `verify_*` that ignores its root, particle, leaves, namespace or graph commitment | bbg, zheng, lens | open | found by the sweep: bbg `verify_particle` and `verify_opening`, zheng look-opening replay and `verify_phi_star` binding; each a soundness hole until fixed | 2026-09-30 |

### calendar

| week | lane A · core | lane B · body | lane C · content |
|---|---|---|---|
| 1 · to 09-25 | slipping on every cell that needs a merge: registry agreed ✓; privacy P1–P3 designed → audits found the gaps instead (rows 10–12, 29), design slipping; property 3 measured ✓ in PR | particle definition frozen (19) slipping: decision open; genesis pipeline one node: not started, replay contract in PR (14) | availability map in PR (cyb#1392); radio into cyb: not started, gossip proven multi-hop (radio#5) |
| 2 · to 10-02 | settlement end to end on one node; ICBS positions and serum reports on links (25) | three nodes, three machines, radio gossip (21), re-addressing run (20) | media render in spark; blob fetch by particle (22) |
| 3 · to 10-09 | fold tree to decide and mint; serum scoring implemented (24); a neuron roots its own book (30); spec freeze of the core | claim by old key in cyb and on bostrom.network | PDF reader in spark |
| 4 · to 10-16 | privacy P1–P3 and valence (29) in code; two axes and allocation curve (26, 27); domain finality and the referral (31, 32); properties 4, 6, 7, 8 | network gate with conflicts and partition | link → files → links navigation |
| 5 · to 10-23 | the core on three nodes from real genesis; feature freeze | release train: cyb, node, true-cyber, six builds | property 16 |
| 6 · to 10-30 | red team fixes; registry closed or deferred | genesis candidates of both chains; runbook, alerts | public rehearsal on pussy-rc |
| 7 · to 11-05 | freeze | ceremony: bostrom genesis at 13:22:42 UTC, five years to the second after bostrom block 1 (2021-11-05T13:22:42Z); pussy at 13:37 UTC | two chains live, cyb reads them |

freezes: core specs 2026-10-09 · features 2026-10-23 · genesis candidates 2026-10-28 · public rehearsal 2026-10-30 → 11-02 · launch 2026-11-05.

### controls

- this page is the tracker. status changes, decisions and deferrals are edits here, dated; evidence lives in the owning repository's `audit/`.
- daily loop: every morning the gates run, the registry and the calendar are updated here, red items go to the top; every evening the [chronicle beat](https://cyberia.blog) carries the day.
- gates in CI: cyb fleet (three bodies) for the body; the network gate (three nodes, three machines, real genesis, conflicts, partition) for the core, run daily.
- the release train, rules in [[cyberia/dev]] § release train and in each repo's CLAUDE.md: cyb, cyber and soft3 first. a candidate every friday 12:00 UTC from origin default branches only, gated by each repo's named gates, published as a draft pre-release with binaries, SHA256SUMS, sources.json, candidate.json and release-validation.json; a red gate ships red with evidence; one bump is one PR touching manifests, changelog and sibling pins; the phase-1 manifest `soft3/release/phase1.toml` pins sibling revisions and drift is red; freeze from cut to verdict; agents cut, gate, write receipts to `audit/release-<date>/` and open bump PRs; only the owner merges a bump, promotes, publishes or tags. one work-log row per candidate. first candidate: 2026-09-25.
- red team: every week the same package (core specs, code, this registry) goes to four models on top subscriptions, Grok, Kimi, Claude and GPT, with one task: break it. findings land in `audit/security/<model>-<date>.md`; a real finding becomes a test in attack_vectors.rs before it is called fixed.
- staging: spacepussy-test (live) → pussy-rc with real genesis on three nodes → pussy and bostrom on 2026-11-05.
- plan B, decided in advance: if the fold tree does not deliver an O(1) accumulator on three nodes by 2026-10-23, phase 1 launches with settlement and a per-cluster ticket cap, the DoS risk accepted on the canary, and fold ships as the first upgrade. no other core has a plan B.

### decisions log

| date | decision |
|---|---|
| 2026-09-18 | phase 1 dated 2026-11-05; both chains the same day, minutes apart |
| 2026-09-18 | ceremony time: bostrom 13:22:42 UTC (block 1 anniversary to the second), pussy 13:37 UTC. Berlin 14:22 · Moscow 16:22 · Bali 21:22 · Beijing 21:22 · New York 08:22 · San Francisco 05:22 |
| 2026-09-18 | the four cores are non-negotiable; programmability, uhash, fees, $CYB are out of phase 1 |
| 2026-09-18 | genesis stake bonded uniformly from the snapshot, resource tokens counted |
| 2026-09-18 | privacy P1–P3 all required at genesis |
| 2026-09-18 | content availability and render in cyb is a phase-1 core requirement |
| 2026-09-18 | personal chains are a core: every neuron roots a home book with its own token; the referral pays the referrer a share of $ν at birth and pro-rata fees of the book |
| open | the referral share r; transferability of $ν; whether a referrer's referrer receives anything (default: no) |
| 2026-09-18 | hybrid economics is a core: truth markets (ICBS + serum + valence) and staking on two axes ship in phase 1 |
| 2026-09-18 | component table is the computed dependency closure; nu (cyb's console) is a local checkout without a remote and must get one before the release train |
| 2026-09-18 | second scan: file, radio, re-addressing, file store, inf, cybernode added as phase-1 dependencies |
| open | which particle definition wins: hemera(data) 32 bytes, or hemera over the lens commitment — blocks row 20; survey in cybergraph#4 |
| open | privacy at genesis with the current wire: the signal frame carries link content in the clear (zheng#21) and tickets carry the marginal vector (foculus#12); either zheng private execution lands in phase 1 or P1/P3 are redefined |
| 2026-09-22 | consolidated state: the root chain of each token holds the public graph, the union of every published edge with hidden authorship, and that graph is the collective brain φ* and inference run on; a personal book holds the private side of the same edges (author, positions, balances) and publishes edges to the root by registration. a book that publishes nothing contributes nothing and earns nothing. this closes the gap between one public head and many private books; the wire and ticket leaks (rows 10, 12, 29) are about the private side, not the edges |
| 2026-09-23 | the automated workers are paused until the process is debugged; the daily overview stays |
| 2026-09-23 | release train step 1 done: every working tree under ~/cyber committed and pushed. origin now carries the owner's versions on branches with PRs: bbg 0.3.0 (#9), lens 0.2.0 (#14), nox 0.3.0 (#17), zheng 0.4.0 (#33), neuron on main, honeycrisp #11, rune #6, trident #58 (self-hosting docs), warriors #1, evy #1, cyber-valley #2, tade #6, soft3 #19 (train rules); trisha and joy trees were identical to their defaults. five warrior stubs got private repos (gaw, pearla, tolya, vitalina, zenda). open for the owner: cyberia-to main vs master divergence; trisha nested `target/` not ignored; the five PRs above are the "origin = truth" merges the train needs before the first candidate |
| 2026-09-23 | release train rules settled for cyb, cyber and soft3 (controls, cyberia/dev.md, root CLAUDE.md, the three repos' CLAUDE.md): weekly candidate from origin only, named gates, draft pre-release with receipts, one-PR bumps, phase-1 manifest, owner-only promotion. steps still to build: the manifest, the clean-checkout gate in CI, the candidate workflow modelled on trident's release pipeline (the owner's correction: trident, not trisha), the bump script; first candidate 2026-09-25. the rules also reach Codex, Kimi Code and Grok Build through AGENTS.md beside every CLAUDE.md and in the agents' global homes |
| 2026-09-23 | incident: between 2026-09-22 12:00 and 2026-09-23 09:18 UTC the launch workers, finding every registry row taken, invented rows 39–253 in this table, appended 209 work-log rows for them and opened 217 pull requests (test coverage, release-mode invariants, non-ASCII hex guards, allocation bounds, build-pin fixes) across 30 repositories. the prompt said "do not return NONE while any lane has a tractable row" and did not forbid editing the registry. the rows are removed here, the prompt now forbids touching the registry and requires the row to exist, and the 217 PRs are triaged: the few that serve real rows (clean-checkout builds, genesis, content render, fixed point) are retitled onto them; the rest are closed with a comment and the label `sweep`, reopenable if wanted |
| 2026-09-22 | who serves the public graph: every full node holds and serves the whole root state; a partial node (cyb) holds its namespaces plus completeness proofs; a light client holds the tip root and opens against it; availability is replication over radio plus DAS sampling with erasure coding (foculus vec.md), with the burial file store on cybernode as the seed; three full nodes on three machines are the phase-1 floor (row 34) |
| 2026-09-22 | clusters are not chosen by miners: an epoch's claims partition canonically into components of overlapping ε-supports; a miner chooses which cluster to sample; difficulty per cluster equalizes pay per second (rows 4, 35); the partition is specified and not implemented |
| 2026-09-22 | hardware profile of settlement mining, for the community: a ticket is a tri-kernel recompute on an ε-support (sparse matrix–vector passes over Goldilocks) plus a zheng proof (sumcheck over multilinear tables, Brakedown linear-code commitment, Poseidon2 hashing); the recompute is memory-latency bound on random graph access, the proof is bandwidth bound with a hashing compute tail; no NTT, no dense matmul; unified-memory machines with fast random access (Apple Silicon) sit near the optimum and a GPU gains little; the ASIC endgame is a Goldilocks field processor with fma, p2r and lut, which serves the tri-kernel itself and is welcome; recorded as the phase-1 claim to be measured on row 3 |
| 2026-09-23 | the owner authorized review and merge of straightforward launch PRs, prioritizing a stable node. Nine selected node/ingress/storage PRs merged; the local release binary passes six process tests. Dependency reproducibility, lifecycle overload/drain, storage failure qualification and multi-node convergence remain open. [Review and evidence](audit/launch-review-2026-09-23.md). |
| open | identity of the 2.37% of bostrom files without bytes |
| open | 8.97T BOOT on the passport contract |
| open | the bond formula for milliampere and millivolt |

## phase 2 · canary

90 days from launch. the two chains under real use, the weekly red team running, and the registry still the tracker. what phase 2 must show: neurons join, stay and create focus; every economic invariant holds; no critical bug under attack. upgrades land in this order: fold if plan B was taken, then programmability (nox programs, joy, trident contracts), then the fee role. the parameter change procedure of foculus is written and exercised once on pussy before it is needed on bostrom.

## phase 3 · $CYB mainnet

the launch that must be correct on the first try. it needs what the canary cannot give: independent implementations in several languages agreeing on state roots, and the formal spine.

| # | question | evidence |
|---|---|---|
| 1 | does φ* converge? | [[eidos]] proof of Lyapunov stability |
| 2 | can proofs be forged? | soundness proof + 10⁸ fuzzing runs, 0 counterexamples |
| 3 | can the economy be drained? | Nash equilibrium proof + 100× adversarial simulation |
| 4 | is computation deterministic? | cross-implementation state-root match on 10⁶ blocks |
| 5 | does it survive partial failure? | chaos test, zero safety violations |
| 6 | does it provide real utility? | the canary record |

the formal spine, running alongside phases 1 and 2 and converging here: layer 1 confluence ([[eidos]]), cost determinism, focus conservation, privacy soundness (< 2⁻¹²⁸), tri-kernel convergence with explicit constants, adversarial equilibrium, double-spend prevention. recursive proofs (the verifier as a nox program, light clients) and sharding with DAS belong here too.

$CYB genesis is separate from the two bootloader chains: a 187,416,084,623,451,570 $CYB starting balance for $C holders (≈ 1% of the Goldilocks field order), the rest along a power-law emission; [[bootloader/tokens/$BOOT|$BOOT]] and [[bootloader/tokens/$PUSSY|$PUSSY]] balances do not convert. the migrated graph is aligned to the [[cybics/crystal|crystal]], the 5,040-particle seed, under its 12 genesis invariants. supply, emission and the crystal keep their own pages: [[$CYB]], [[cybics/crystal|crystal]].

## token architecture

| type | fungible | movable | role | examples |
|---|---|---|---|---|
| [[coin]] | yes | yes | consensus, stake | [[bootloader/tokens/$BOOT|$BOOT]], [[bootloader/tokens/$PUSSY|$PUSSY]], later [[$CYB]] |
| [[card]] | no | yes | knowledge assets, provenance | authorship proofs |
| [[score]] | yes | no | reputation | [[cybics/crystal/karma|karma]] |
| [[badge]] | no | no | non-transferable credentials | achievements |

## the endgame

a living, self-optimizing knowledge network that learns from every form of input on Earth, holds coherence under interplanetary latency, evolves without central authority, maximizes the flourishing of the biosphere, and proves every claim. the network is thinking. no node comprehends. the network knows.

## work log

pull requests from the launch workers (`scripts/launch-hour.sh`, sonnet, launchd `to.cyberia.launch-hour`): three parallel slots on lanes A, B and C, a tick every 20 minutes, up to three PRs per run, one row per PR, conflicting open PRs merged with their base first. the owner merges; a merged row flips its registry property when the evidence is in.

| when (UTC) | property | repo | pull request | state |
|---|---|---|---|---|
| 2026-09-18T21:44:19Z | 13 | plumb (tok) | [launch #13: add transfer to close conservation's second mutation](https://github.com/cyberia-to/plumb/pull/1) | reviewed 2026-09-22 |
| 2026-09-19T00:04:00Z | 3 | foculus | [launch #3: measure ticket proof decide/verify cost](https://github.com/cyberia-to/foculus/pull/6) | reviewed 2026-09-22 |
| 2026-09-19T01:14:34Z | 5 | zheng | [launch #5: fold either order, both decide() successfully](https://github.com/cyberia-to/zheng/pull/20) | reviewed 2026-09-22 |
| 2026-09-19T02:19:46Z | 25 | tru | [launch #25: ICBS cost function and price derivatives](https://github.com/cyberia-to/tru/pull/8) | reviewed 2026-09-22 |
| 2026-09-19T03:34:00Z | 6 | foculus | [launch #6: attack tests for beacon VDF binding](https://github.com/cyberia-to/foculus/pull/7) | reviewed 2026-09-22 |
| 2026-09-19T04:45:00Z | 24 | tru | [launch #24: informed minority beats coordinated majority at scale](https://github.com/cyberia-to/tru/pull/9) | reviewed 2026-09-22 |
| 2026-09-19T05:10:00Z | 16 | file | [launch #16: sniff PDF, video and audio file kinds](https://github.com/cyberia-to/file/pull/1) | reviewed 2026-09-22 |
| 2026-09-19T05:35:00Z | 19 | cybergraph | [launch #19: survey particle type definitions across the four owners](https://github.com/cyberia-to/cybergraph/pull/4) | reviewed 2026-09-22 |
| 2026-09-23T23:13:48Z | 39 | soft3 | [launch #39: add the phase-1 sibling-pin manifest (release/phase1.toml)](https://github.com/cyberia-to/soft3/pull/21) | open |
| 2026-09-19T05:55:00Z | 8 | foculus | [launch #8: MinHash agrees across n-way conflicts and arrival orders](https://github.com/cyberia-to/foculus/pull/8) | reviewed 2026-09-22 |
| 2026-09-19T04:57:02Z | 9 | foculus | [launch #9: prove convergence is order-independent of link arrival](https://github.com/cyberia-to/foculus/pull/9) | reviewed 2026-09-22 |
| 2026-09-19T05:00:52Z | 26 | plumb (tok) | [launch #26: stake-yield term pays active risk, zeros passive/Sybil](https://github.com/cyberia-to/plumb/pull/2) | reviewed 2026-09-22 |
| 2026-09-19T06:04:15Z | 18 | bbg | [launch #18: define WARM archival population and checkpoint boundary](https://github.com/cyberia-to/bbg/pull/10) | reviewed 2026-09-22 |
| 2026-09-19T06:05:00Z | 22 | radio | [launch #22: verified-streaming tests for the particle CLI](https://github.com/cyberia-to/radio/pull/4) | reviewed 2026-09-22 |
| 2026-09-19T06:07:21Z | 4 | foculus | [launch #4: band settlement target by cluster size](https://github.com/cyberia-to/foculus/pull/10) | reviewed 2026-09-22 |
| 2026-09-19T06:14:57Z | 7 | foculus | [launch #7: bound withholding bias by compute share](https://github.com/cyberia-to/foculus/pull/11) | reviewed 2026-09-22 |
| 2026-09-19T06:23:11Z | 14 | cybergraph | [launch #14: genesis replay bijection contract](https://github.com/cyberia-to/cybergraph/pull/5) | reviewed 2026-09-22 |
| 2026-09-19T06:26:00Z | 30 | plumb (tok) | [launch #30: neuron roots one home-book token in the ledger](https://github.com/cyberia-to/plumb/pull/3) | reviewed 2026-09-22 |
| 2026-09-19T06:35:00Z | 32 | plumb (tok) | [launch #32: referral birth-mint split, self-referral rejected](https://github.com/cyberia-to/plumb/pull/4) | reviewed 2026-09-22 |
| 2026-09-19T06:42:00Z | 12 | foculus | [launch #12: audit — settlement tickets leak the full per-contributor marginal vector](https://github.com/cyberia-to/foculus/pull/12) | reviewed 2026-09-22 |
| 2026-09-19T06:35:32Z | 31 | foculus | [launch #31: book-level domain finality for oikos foundation 2](https://github.com/cyberia-to/foculus/pull/13) | reviewed 2026-09-22 |
| 2026-09-21T15:38:19Z | 33 | plumb (tok) | [launch #33: conserve referral payout, zero for inactive/Sybil books](https://github.com/cyberia-to/plumb/pull/5) | reviewed 2026-09-22 |
| 2026-09-21T15:52:00Z | 20 | file | [launch #20: re-address bostrom CIDs to Hemera particles](https://github.com/cyberia-to/file/pull/2) | reviewed 2026-09-22 |
| 2026-09-23T10:35:00Z | 13 | plumb (tok) | [launch #13 (2): add lock to close conservation's third mutation](https://github.com/cyberia-to/plumb/pull/13) | open |
| 2026-09-21T16:10:00Z | 27 | tru | [launch #27: implement PoW/PoS allocation split and security floor](https://github.com/cyberia-to/tru/pull/10) | reviewed 2026-09-22 |
| 2026-09-21T16:32:00Z | 28 | tru | [launch #28: discrete per-epoch accrual for the yield-stream annuity](https://github.com/cyberia-to/tru/pull/11) | reviewed 2026-09-22 |
| 2026-09-21T15:40:00Z | 21 | radio | [launch #21: fix iroh-docs fork pins, prove gossip relays multi-hop](https://github.com/cyberia-to/radio/pull/5) | reviewed 2026-09-22 |
| 2026-09-21T16:52:00Z | 10 | mudra | [launch #10 (draft): trace P1 signal content-commitment gap](https://github.com/cyberia-to/mudra/pull/3) | draft |
| 2026-09-21T17:10:00Z | 11 | mudra | [launch #11 (draft): trace P2 stealth/veil transfer-privacy gap](https://github.com/cyberia-to/mudra/pull/4) | draft |
| 2026-09-21T15:55:09Z | 10 | zheng | [launch #10: audit — signal wire format leaks full link content](https://github.com/cyberia-to/zheng/pull/21) | open — duplicate: mudra#3 above also claims property 10, opened 54s earlier; owner to pick one |
| 2026-09-21T15:56:50Z | 29 | mudra | [launch #29: audit — valence leaks on the wire, collides with truth-scoring spec](https://github.com/cyberia-to/mudra/pull/5) | reviewed 2026-09-22 |
| 2026-09-21T15:58:21Z | 23 | bbg | [launch #23: audit — network fetch has no self-authentication check](https://github.com/cyberia-to/bbg/pull/11) | reviewed 2026-09-22 |
| 2026-09-21T16:43:00Z | 16 | cyb | [launch #16 (draft): particle availability audit module](https://github.com/cyberia-to/cyb/pull/1392) | draft |
| 2026-09-22T08:52:00Z | 34 | foculus | [launch #34: DAS availability decision treats a withheld shard as a failed sample](https://github.com/cyberia-to/foculus/pull/14) | reviewed 2026-09-22 |
| 2026-09-22T09:05:00Z | 35 | foculus | [launch #35: ε-support derivation and canonical cluster partition](https://github.com/cyberia-to/foculus/pull/15) | reviewed 2026-09-22 |
| 2026-09-22T08:41:00Z | 36 | foculus | [launch #36: measure settlement-mining hardware profile](https://github.com/cyberia-to/foculus/pull/16) | reviewed 2026-09-22 |
| 2026-09-23T22:10:45Z | 39 | cyb | [launch #39: cyb — serialize make fleet across concurrent invocations](https://github.com/cyberia-to/cyb/pull/1399) | open |
| 2026-09-22T11:05:00Z | 38 | mudra | [launch #38: bump cyber-nox/zheng pins so mudra builds from a clean checkout](https://github.com/cyberia-to/mudra/pull/6) | open |
| 2026-09-22T10:58:41Z | 37 | plumb (tok) | [launch #37: compute settle-mint conservation in fixed point](https://github.com/cyberia-to/plumb/pull/6) | open |
| 2026-09-23T02:05:00Z | 37 | plumb (tok) | [launch #37: compute fx_to_tokens/fx_weight in fixed point, not f64](https://github.com/cyberia-to/plumb/pull/10) | open |
| 2026-09-23T03:49:00Z | 37 | plumb (tok) | [launch #37: compute fx_to_tokens/fx_weight in fixed point, not f64](https://github.com/cyberia-to/plumb/pull/12) | open |
| 2026-09-23T09:59:00Z | 39 | bbg | [launch #39: bump lens/nox/zheng pins and tape->tade path so bbg builds clean](https://github.com/cyberia-to/bbg/pull/12) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | cybergraph | [launch #39: bump bbg/zheng/nox/lens pins; nox test migration still needed](https://github.com/cyberia-to/cybergraph/pull/6) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | mudra | [launch #39: fix bbg_root sentinel so mudra builds with --features prove](https://github.com/cyberia-to/mudra/pull/7) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | neuron | [launch #39: add missing neuron-id crate so bbg/tok/mudra build clean](https://github.com/cyberia-to/neuron/pull/1) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | nox | [launch #39: bump cyber-lens-brakedown pin so nox builds clean](https://github.com/cyberia-to/nox/pull/6) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | prysm | [launch #39: bump tape->tade path so prysm builds clean](https://github.com/cyberia-to/prysm/pull/4) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | zheng | [launch #39: zheng builds from a clean checkout; TensorMerkle gap found](https://github.com/cyberia-to/zheng/pull/22) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | true-cyber | [launch #39: true-cyber does not build from a clean checkout](https://github.com/cyberia-to/true-cyber/pull/1) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | true-cyber | [launch #39: revert the broken cyb-core GraphSession WIP rewrite](https://github.com/cyberia-to/true-cyber/pull/2) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | glia | [launch #39: fix TransformerConfig clean-checkout test build break](https://github.com/cyberia-to/glia/pull/3) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | honeycrisp | [launch #39: honeycrisp builds clean; Tape::take alignment invariant enforced in release](https://github.com/cyberia-to/honeycrisp/pull/1) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | rune | [launch #39: rune builds clean; prysm chunk-noun decode fully covered](https://github.com/cyberia-to/rune/pull/1) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | vault | [launch #39: vault builds clean; cover codec's Encoder/Decoder](https://github.com/cyberia-to/vault/pull/2) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | radio | [launch #39: iroh-bench builds against this workspace's iroh/quinn fork](https://github.com/cyberia-to/radio/pull/6) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | lens | [launch #39: squeeze_field concatenates enough hash for wide fields](https://github.com/cyberia-to/lens/pull/10) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 39 | foculus | [launch #39: audit — net-feature build break is iroh 0.96's ed25519-dalek pin](https://github.com/cyberia-to/foculus/pull/31) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 40 | bbg | [launch #40: audit — verify_particle ignores its root and particle arguments](https://github.com/cyberia-to/bbg/pull/14) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 40 | bbg | [launch #40: audit — verify_opening ignores its leaves and namespace](https://github.com/cyberia-to/bbg/pull/19) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 40 | zheng | [launch #40: reject look-opening root replay that diverges from native fold](https://github.com/cyberia-to/zheng/pull/26) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 40 | zheng | [launch #40: verify_phi_star binds graph_commit to the real graphs](https://github.com/cyberia-to/zheng/pull/29) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 40 | foculus | [launch #40: reject a mismatched directed_total in verify_epoch_cert](https://github.com/cyberia-to/foculus/pull/27) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 40 | nox | [launch #40: merkle_verify jet rejects a path length mismatched to depth](https://github.com/cyberia-to/nox/pull/9) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 40 | mudra | [launch #40: reject non-canonical r/s scalar encodings in ecdsa::verify](https://github.com/cyberia-to/mudra/pull/12) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 23 | bbg | [launch #23: (merged or retitled)](https://github.com/cyberia-to/bbg/pull/15) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 14 | soft3 | [launch #14: cover node genesis load/validate with tests](https://github.com/cyberia-to/soft3/pull/4) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 14 | soft3 | [launch #14: accept the launch chain set in genesis validation](https://github.com/cyberia-to/soft3/pull/7) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 14 | soft3 | [launch #14: add genesis install subcommand](https://github.com/cyberia-to/soft3/pull/8) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 16 | spark | [launch #16: spark opens PDF/video/audio as Document/Media surfaces](https://github.com/cyberia-to/spark/pull/3) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 37 | foculus | [launch #37: compute fold-mining k_min in fixed point, not f64](https://github.com/cyberia-to/foculus/pull/17) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 37 | foculus | [launch #37: compute fx_weight from raw fixed point, not f64](https://github.com/cyberia-to/foculus/pull/30) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 13 | plumb | [launch #13: clip_shares clips to zero on non-positive directed_total](https://github.com/cyberia-to/plumb/pull/9) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 13 | plumb | [launch #13: burn(0) on an untouched neuron/token pair no longer panics](https://github.com/cyberia-to/plumb/pull/11) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 34 | foculus | [launch #34: capacity-weighted placement no longer starves peers](https://github.com/cyberia-to/foculus/pull/28) | kept from the sweep, retitled |
| 2026-09-23T09:59:00Z | 22 | radio | [launch #22: persist radio-cli's blob store across invocations](https://github.com/cyberia-to/radio/pull/9) | kept from the sweep, retitled |
| 2026-09-23T10:47:00Z | 22 | radio | [launch #22 (2): tag a fetched blob by hash so list shows it](https://github.com/cyberia-to/radio/pull/14) | open |
| 2026-09-23T10:58:00Z | 23 | bbg | [launch #23 (2) (draft): fix misleading tiered.rs module-header comment](https://github.com/cyberia-to/bbg/pull/23) | draft |
| 2026-09-23T10:34:45Z | 40 | lens | [launch #40: Brakedown/Ikat verify never checks queried codeword values](https://github.com/cyberia-to/lens/pull/13) | open |
| 2026-09-23T10:45:53Z | 18 | bbg | [launch #18: stage WARM values into COLD's pending batch via demote()](https://github.com/cyberia-to/bbg/pull/24) | open |
| 2026-09-23T10:49:41Z | 18 | bbg | [launch #18 (2): evict a WARM key once its archival checkpoint succeeds](https://github.com/cyberia-to/bbg/pull/25) | open |
| 2026-09-23T10:52:39Z | 27 | tru | [launch #27 (2): generic clamped PID controller over Fx for §10's three loops](https://github.com/cyberia-to/tru/pull/22) | open |
| 2026-09-23T11:04:00Z | 14 | soft3 | [launch #14: add genesis claim subcommand over the mudra legacy bridge](https://github.com/cyberia-to/soft3/pull/18) | open |
| 2026-09-23T11:24:00Z | 37 | foculus | [launch #37 (2): gate settlement decide on meets_precision](https://github.com/cyberia-to/foculus/pull/34) | open |
| 2026-09-23T11:48:00Z | 3 | foculus | [launch #3 (2): reconcile decide/verify latency target with measurement](https://github.com/cyberia-to/foculus/pull/35) | open |
| 2026-09-23T18:24:58Z | 38 | mudra | [launch #38 (2): bind bbg_root sentinel so mudra builds with --features prove](https://github.com/cyberia-to/mudra/pull/13) | open |
| 2026-09-23T18:31:20Z | 35 | foculus | [launch #35 (2): expose the canonical cluster partition on EpochRunner](https://github.com/cyberia-to/foculus/pull/36) | open |
| 2026-09-23T18:47:58Z | 30 | cybergraph | [launch #30: register_book publishes a neuron's home-book naming link](https://github.com/cyberia-to/cybergraph/pull/9) | open |
| 2026-09-23T18:53:00Z | 4 | foculus | [launch #4 (2): wire banded_target into a live epoch's settle policy](https://github.com/cyberia-to/foculus/pull/38) | open |
| 2026-09-23T19:05:00Z | 12 | foculus | [launch #12 (2): specify the marginal-hiding ZK statement for P3](https://github.com/cyberia-to/foculus/pull/39) | open |
| 2026-09-23T19:30:00Z | 12 | foculus | [launch #12 (3): audit — fold-mining ClusterAcc gossips the marginal vector plaintext](https://github.com/cyberia-to/foculus/pull/40) | draft |
| 2026-09-23T19:22:00Z | 32 | plumb (tok) | [launch #32 (2): pro-rata $ν holder fee distribution](https://github.com/cyberia-to/plumb/pull/14) | open |
| 2026-09-23T19:38:00Z | 25 | plumb (tok) | [launch #25 (2): ICBS reserves and positions market, conservation-checked buy/sell](https://github.com/cyberia-to/plumb/pull/15) | open |
| 2026-09-23T19:58:00Z | 31 | foculus | [launch #31 (2): bind book_id into FinalityEvidence, close root-collision exposure](https://github.com/cyberia-to/foculus/pull/41) | open |
| 2026-09-23T19:41:00Z | 24 | tru | [launch #24: harden BTS crowd reference against v_ℓ=0 decoy reports](https://github.com/cyberia-to/tru/pull/23) | open |
| 2026-09-23T20:12:00Z | 7 | foculus | [launch #7: role-separation check for a contending miner (§7)](https://github.com/cyberia-to/foculus/pull/42) | open |
| 2026-09-23T20:35:00Z | 26 | plumb (tok) | [launch #26 (2): PoW-subsidy term of the §11 reward equation](https://github.com/cyberia-to/plumb/pull/16) | open |
| 2026-09-23T19:53:00Z | 18 | bbg | [launch #18 (3): archival checkpoint schedule for the demote/archive/evict_archived sweep](https://github.com/cyberia-to/bbg/pull/26) | open |
| 2026-09-23T20:15:00Z | 40 | mudra | [launch #40 (2): reject non-canonical x-coordinate in Point::from_sec1](https://github.com/cyberia-to/mudra/pull/14) | open |
| 2026-09-23T20:31:59Z | 39 | inf | [launch #39: bind bbg_root sentinel so inf-lower builds with --features prove](https://github.com/cyberia-to/inf/pull/4) | open |
| 2026-09-23T20:26:32Z | 21 | radio | [launch #21 (2): gossip carries a real SettleMsg envelope, not raw bytes](https://github.com/cyberia-to/radio/pull/15) | open |
| 2026-09-23T20:34:55Z | 39 | cybergraph | [launch #39 (2): migrate stack_*.rs tests to nox 0.3/zheng 0.4](https://github.com/cyberia-to/cybergraph/pull/10) | open |
| 2026-09-23T20:39:39Z | 34 | foculus | [launch #34 (3): compute DAS confidence in Fx, not f64](https://github.com/cyberia-to/foculus/pull/43) | open |
| 2026-09-23T21:05:00Z | 21 | radio | [launch #21 (3): gossip a real ClaimAnnounce/RewardClaim envelope](https://github.com/cyberia-to/radio/pull/16) | open |
| 2026-09-23T20:52:00Z | 5 | foculus | [launch #5 (2): fold_acc_checked rejects overlapping accumulators](https://github.com/cyberia-to/foculus/pull/44) | open |
| 2026-09-23T21:06:32Z | 40 | foculus | [launch #40: nmt verify() binds proof.root to a trusted root](https://github.com/cyberia-to/foculus/pull/45) | open |
| 2026-09-23T21:07:39Z | 21 | radio | [launch #21 (4): gossip a real SelfAcc/ClusterAcc envelope](https://github.com/cyberia-to/radio/pull/17) | open |
| 2026-09-23T21:10:52Z | 40 | lens | [launch #40 (2): Assayer commitment omits edge topology, source, target](https://github.com/cyberia-to/lens/pull/15) | open |
| 2026-09-23T21:08:19Z | 7 | foculus | [launch #7 (2): price the withholding forfeit deterrent](https://github.com/cyberia-to/foculus/pull/46) | open |
| 2026-09-23T21:17:00Z | 3 | foculus | [launch #3 (3): measure the steady-state fold-step cost](https://github.com/cyberia-to/foculus/pull/47) | open |
| 2026-09-23T21:25:34Z | 14 | bostrom | [launch #14 (2): make graph_rebuild importable, cover event pairing](https://github.com/cyberia-to/bostrom/pull/13) | open |
| 2026-09-23T21:30:57Z | 18 | bbg | [launch #18: audit — row 18 has two diverging, unmerged storage PRs](https://github.com/cyberia-to/bbg/pull/27) | open |
| 2026-09-23T21:32:00Z | 36 | foculus | [launch #36 (2): parallel scaling of the settlement-ticket sample](https://github.com/cyberia-to/foculus/pull/48) | open |
| 2026-09-23T21:47:22Z | 14 | bostrom | [launch #14 (3): make holdings.py importable, cover denom labels and pool rates](https://github.com/cyberia-to/bostrom/pull/14) | open |
| 2026-09-23T21:50:32Z | 16 | spark | [launch #16 (3): image dimensions from raw PNG/GIF/JPEG bytes](https://github.com/cyberia-to/spark/pull/5) | open |
| 2026-09-23T21:51:07Z | 39 | mir | [launch #39: mir builds clean; bevy-plugin tests gated by required-features](https://github.com/cyberia-to/mir/pull/10) | open |
| 2026-09-23T21:53:38Z | 14 | bostrom | [launch #14 (4): audit — manifest.json particle totals off by 2](https://github.com/cyberia-to/bostrom/pull/15) | open |
| 2026-09-23T22:08:59Z | 16 | file | [launch #16 (2): Particle::from_hex rejects non-ASCII instead of panicking](https://github.com/cyberia-to/file/pull/5) | open |
| 2026-09-23T22:12:00Z | 14 | bostrom | [launch #14 (5): unit-cover extract.py's per-record pure logic](https://github.com/cyberia-to/bostrom/pull/16) | open |
| 2026-09-23T22:11:26Z | 15 | mudra | [launch #15: Claim::decode rejects non-ASCII hex fields instead of panicking](https://github.com/cyberia-to/mudra/pull/15) | open |
| 2026-09-23T22:15:00Z | 14 | bostrom | [launch #14 (6): make graph_scan.py import-safe, cover pure logic](https://github.com/cyberia-to/bostrom/pull/17) | open |
| 2026-09-23T22:26:34Z | 39 | soft3 | [launch #39: soft3 builds and tests clean against current sibling checkouts](https://github.com/cyberia-to/soft3/pull/20) | open |
| 2026-09-23T22:52:00Z | 37 | foculus | [launch #37 (3): compute k_min/meets_precision in Fx, not f64](https://github.com/cyberia-to/foculus/pull/49) | open |
| 2026-09-23T22:56:00Z | 23 | bbg | [launch #23: wire NetworkStore::das_sample into TieredStore](https://github.com/cyberia-to/bbg/pull/28) | open |
| 2026-09-23T22:46:04Z | 16 | spark | [launch #16 (4): WebP dimensions from RIFF chunk bytes](https://github.com/cyberia-to/spark/pull/6) | open |
| 2026-09-23T22:50:02Z | 39 | tru | [launch #39: tru builds and tests clean against sibling origin defaults](https://github.com/cyberia-to/tru/pull/24) | open |
| 2026-09-23T23:02:00Z | 14 | bostrom | [launch #14 (7): unit-cover extract.py's HTTP-retry and cmd_balances/supply/passport](https://github.com/cyberia-to/bostrom/pull/18) | open |
| 2026-09-23T22:53:12Z | 39 | plumb | [launch #39: plumb builds and tests clean against sibling origin defaults](https://github.com/cyberia-to/plumb/pull/17) | open |
| 2026-09-23T23:10:00Z | 40 | bbg | [launch #40 (3): audit — verify_query ignores which dimension and root it answers](https://github.com/cyberia-to/bbg/pull/29) | open |
| 2026-09-23T23:32:53Z | 22 | radio | [launch #22: resolve a blob by cyber particle hash, not just blob hash](https://github.com/cyberia-to/radio/pull/18) | open |
| 2026-09-23T23:28:00Z | 16 | spark | [launch #16 (5): BMP dimensions from raw header bytes](https://github.com/cyberia-to/spark/pull/7) | open |
| 2026-09-23T23:36:00Z | 16 | file | [launch #16 (3): sniff BMP files as Kind::ImageBmp](https://github.com/cyberia-to/file/pull/6) | open |
| 2026-09-23T23:44:00Z | 16 | spark | [launch #16 (6): ICO dimensions from the directory header](https://github.com/cyberia-to/spark/pull/8) | open |
| 2026-09-23T23:47:00Z | 16 | file | [launch #16 (7): sniff TIFF images by byte-order magic](https://github.com/cyberia-to/file/pull/7) | open |
| 2026-09-23T23:56:00Z | 16 | spark | [launch #16 (7): TIFF dimensions from the IFD](https://github.com/cyberia-to/spark/pull/9) | open |
| 2026-09-24T00:04:59Z | 39 | file | [launch #39: confirm file builds clean against hemera's origin default](https://github.com/cyberia-to/file/pull/9) | open |
| 2026-09-24T00:04:08Z | 16 | file | [launch #16: sniff ICO files as Kind::ImageIco](https://github.com/cyberia-to/file/pull/8) | open |
| 2026-09-24T00:06:00Z | 39 | spark | [launch #39: confirm spark builds clean against file/hemera origin defaults](https://github.com/cyberia-to/spark/pull/10) | open |
| 2026-09-24T00:09:14Z | 39 | strata | [launch #39: fix Fp3 norm/inv shader bug found by the clean-checkout gate](https://github.com/cyberia-to/strata/pull/12) | open |
| 2026-09-24T00:12:49Z | 26 | plumb (tok) | [launch #26 (3): service-fee term of the §11 reward equation](https://github.com/cyberia-to/plumb/pull/18) | open |
| 2026-09-24T00:26:12Z | 39 | hemera | [launch #39: confirm hemera builds and tests clean from origin/main](https://github.com/cyberia-to/hemera/pull/12) | open |
| 2026-09-24T00:28:52Z | 39 | soft3 | [launch #39 (2): clean-checkout gate reads phase1.toml, fails on sibling drift](https://github.com/cyberia-to/soft3/pull/22) | open |
| 2026-09-24T00:38:20Z | 39 | vault | [launch #39 (2): vault — rebuild against mudra's spell rename, cover Header/Keys](https://github.com/cyberia-to/vault/pull/7) | open |
| 2026-09-24T00:26:12Z | 39 | tade | [launch #39: confirm tade builds and tests clean from origin/main](https://github.com/cyberia-to/tade/pull/7) | open |
| 2026-09-24T00:38:12Z | 5 | foculus | [launch #5 (2): incremental seen-digest avoids O(k) commitment rehash](https://github.com/cyberia-to/foculus/pull/50) | open |
| 2026-09-24T00:44:00Z | 16 | file | [launch #16 (8): sniff SVG files as Kind::ImageSvg](https://github.com/cyberia-to/file/pull/10) | open |
| 2026-09-24T00:46:31Z | 16 | spark | [launch #16 (9): parse SVG width/height from the root svg element](https://github.com/cyberia-to/spark/pull/11) | open |
| 2026-09-24T00:49:16Z | 16 | spark | [launch #16 (10): PDF page size from the first /MediaBox array](https://github.com/cyberia-to/spark/pull/12) | open |
| 2026-09-24T00:47:13Z | 39 | vault | [launch #39 (3): vault — cover record.rs's SecretInput and Entry](https://github.com/cyberia-to/vault/pull/8) | open |
| 2026-09-24T01:20:00Z | 40 | foculus | [launch #40: verify_certified_ticket trusts a fold-seal's beacon/cluster binding](https://github.com/cyberia-to/foculus/pull/51) | open |
| 2026-09-24T01:13:52Z | 21 | soft3 | [launch #21: audit — soft3 can't wire foculus's settle radio yet](https://github.com/cyberia-to/soft3/pull/23) | open |
| 2026-09-24T01:45:00Z | 40 | foculus | [launch #40: verify_receipt trusts a settle seal's epoch/beacon binding](https://github.com/cyberia-to/foculus/pull/52) | open |
| 2026-09-24T01:29:00Z | 16 | file | [launch #16: sniff HEIC/HEIF/AVIF via ftyp major brand](https://github.com/cyberia-to/file/pull/11) | open |
| 2026-09-24T01:41:00Z | 16 | file | [launch #16 (2): sniff AVI and QuickTime MOV containers](https://github.com/cyberia-to/file/pull/12) | open |
| 2026-09-24T01:59:00Z | 14 | bostrom | [launch #14 (8): cover extract.py's pools/staking/pubkeys/manifest/smart](https://github.com/cyberia-to/bostrom/pull/19) | open |
| 2026-09-24T01:38:00Z | 39 | cyber | [launch #39: confirm cyber builds and tests clean from origin/master](https://github.com/cyberia-to/cyber/pull/110) | open |
| 2026-09-24T01:36:00Z | 16 | file | [launch #16 (3): sniff ZIP containers](https://github.com/cyberia-to/file/pull/13) | open |
| 2026-09-24T02:05:00Z | 16 | file | [launch #16 (4): sniff MP3, WAV, OGG and FLAC by magic bytes](https://github.com/cyberia-to/file/pull/14) | open |
| 2026-09-24T01:56:00Z | 40 | mudra | [launch #40 (3): ecdsa::verify accepts a high-S malleable duplicate signature](https://github.com/cyberia-to/mudra/pull/16) | open |
| 2026-09-24T02:12:06Z | 16 | file | [launch #16: sniff MP4/M4V video via ftyp major brand](https://github.com/cyberia-to/file/pull/17) | open |
| 2026-09-24T02:19:19Z | 40 | hemera | [launch #40: verify_batch checks a caller-supplied root, not proof.root](https://github.com/cyberia-to/hemera/pull/13) | open |
| 2026-09-24T02:04:00Z | 6 | foculus | [launch #6 (2): audit — Goldilocks VDF has no unknown-order delay guarantee](https://github.com/cyberia-to/foculus/pull/53) | open |
| 2026-09-24T02:16:00Z | 19 | file | [launch #19: File::verified — self-authenticating particle binding](https://github.com/cyberia-to/file/pull/16) | open |

| 2026-09-24T02:35:00Z | 22 | radio | [launch #22 (6): expose radio-cli's pure logic as a lib target with tests](https://github.com/cyberia-to/radio/pull/19) | open |
| 2026-09-24T02:46:00Z | 16 | spark | [launch #16 (11): extract WAV audio duration from raw bytes](https://github.com/cyberia-to/spark/pull/13) | open |
| 2026-09-24T02:58:00Z | 16 | spark | [launch #16 (12): extract MP4/QuickTime duration from the mvhd box](https://github.com/cyberia-to/spark/pull/14) | open |
| 2026-09-24T03:08:00Z | 16 | spark | [launch #16 (13): extract FLAC duration from the STREAMINFO block](https://github.com/cyberia-to/spark/pull/15) | open |
| 2026-09-24T02:46:10Z | 16 | spark | [launch #16 (14): extract Ogg Vorbis duration from the last page granule](https://github.com/cyberia-to/spark/pull/16) | open |
| 2026-09-24T02:49:00Z | 40 | inf | [launch #40: verify_expr_proof used a Statement that could never match the one proved](https://github.com/cyberia-to/inf/pull/5) | open |
| 2026-09-24T02:47:56Z | 16 | file | [launch #16 (15): sniff M4A/M4B/M4P audio via ftyp major brand](https://github.com/cyberia-to/file/pull/18) | open |
| 2026-09-24T03:15:00Z | 39 | foculus | [launch #39 (2): migrate net feature's iroh dependency 0.96 to 1.2](https://github.com/cyberia-to/foculus/pull/54) | open |
| 2026-09-24T03:20:00Z | 16 | file | [launch #16 (16): sniff WebM video via EBML DocType](https://github.com/cyberia-to/file/pull/19) | open |
| 2026-09-24T03:33:00Z | 16 | file | [launch #16 (17): sniff FLV video by header signature](https://github.com/cyberia-to/file/pull/20) | open |
| 2026-09-24T03:52:00Z | 39 | cyb | [launch #39: fix open_balance's always-failing balance proof](https://github.com/cyberia-to/cyb/pull/1400) | open |
| 2026-09-24T03:16:00Z | 40 | lens | [launch #40: squeeze_field collects enough bytes for wide fields](https://github.com/cyberia-to/lens/pull/16) | open |
| 2026-09-24T03:28:02Z | 14 | bostrom | [launch #14 (9): cover graph_rebuild's work() HTTP dispatch](https://github.com/cyberia-to/bostrom/pull/20) | open |
| 2026-09-24T03:29:40Z | 14 | bostrom | [launch #14 (10): cover graph_scan's fetch_window() HTTP retry loop](https://github.com/cyberia-to/bostrom/pull/21) | open |
| 2026-09-24T03:33:54Z | 16 | spark | [launch #16 (18): HEIC/HEIF/AVIF dimensions from the ispe box](https://github.com/cyberia-to/spark/pull/18) | open |
| 2026-09-24T03:41:00Z | 16 | spark | [launch #16 (15): extract MP3 duration from raw MPEG-1 Layer III frames](https://github.com/cyberia-to/spark/pull/17) | open |
| 2026-09-24T03:53:00Z | 39 | neuron | [launch #39: audit — clean checkout blocked on mudra's unpushed neuron/spell modules](https://github.com/cyberia-to/neuron/pull/8) | open |
| 2026-09-24T03:50:18Z | 40 | bbg | [launch #40 (2): verify_particle_bound checks commitment against trusted state](https://github.com/cyberia-to/bbg/pull/30) | open |
| 2026-09-24T03:57:39Z | 40 | foculus | [launch #40: verify_sample never checked shard_roots against the trusted root](https://github.com/cyberia-to/foculus/pull/55) | open |
| 2026-09-24T04:10:00Z | 16 | file | [launch #16: sniff Matroska (MKV) and WebM by EBML DocType](https://github.com/cyberia-to/file/pull/21) | open |
| 2026-09-24T04:10:00Z | 16 | file | [launch #16 (2): distinguish docx/xlsx/pptx/epub from generic ZIP](https://github.com/cyberia-to/file/pull/22) | open |
| 2026-09-24T04:10:00Z | 16 | file | [launch #16 (3): sniff bare ADTS AAC audio by frame sync](https://github.com/cyberia-to/file/pull/23) | open |
| 2026-09-24T03:52:25Z | 11 | mudra | [launch #11: audit — genies now exists, not yet constant-time for stealth](https://github.com/cyberia-to/mudra/pull/17) | open |

## cross-references

[[rewards|rewards]] · [[foculus]] · [[tru]] · [[mudra]] · [[cybics/crystal|crystal]] · [[cyber/tokenomics]] · [[soft3]] · [[bootloader/bostrom|bostrom]] · [[bootloader/tokens/$PUSSY|space-pussy]] · [delivery roadmap](/cyber/roadmap/index)
