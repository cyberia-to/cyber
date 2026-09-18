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
   - P1 · a signal's content is visible to its participants; the network sees a commitment and a proof of validity.
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
| 3 | ticket proof cost < ticket reward | foculus, zheng | open | measure on real m(n) with the folded proof | 2026-09-25 |
| 4 | progress-freedom across clusters: difficulty schedule corrects per-cluster cost | foculus | open | simulation on million.rs | 2026-10-09 |
| 5 | fold is a commutative monoid, decide is O(1) | zheng, foculus | open | end-to-end on three nodes | 2026-10-09 |
| 6 | beacon unpredictable and unbiasable | foculus | open | VDF over finalized signals, attack_vectors.rs | 2026-10-16 |
| 7 | withholding bias bounded by compute share | foculus | open | priced forfeit, role separation, simulation | 2026-10-16 |
| 8 | no two nodes finalize conflicting state | foculus | open | network gate with conflicts and partition | 2026-10-16 |
| 9 | nodes converge from different starting states | foculus, cybergraph | open | network gate | 2026-10-02 |
| 10 | P1 signal privacy | mudra, zheng | open | commitment + validity proof, leakage bounded | 2026-10-16 |
| 11 | P2 stealth and veil | mudra, tok | open | stealth.md, veil.md, transfer vectors | 2026-10-16 |
| 12 | P3 miner ego-net hidden beyond aggregates | foculus | open | interface invariant 4, test | 2026-10-16 |
| 13 | conservation under mint and transfer | tok | open | proof at every mutation — mint, burn, transfer tested (plumb#1); lock remains | 2026-10-09 |
| 14 | genesis is bijective with the snapshot | bostrom, cybergraph | open | replayed root matches, zero loss | 2026-10-02 |
| 15 | claim by old key binds one neuron | mudra | measured | 13 bridge tests | closed |
| 16 | every referenced particle resolves in cyb | cyb | open | availability audit, missing list | 2026-10-23 |
| 17 | signed signal cannot be forged or replayed | mudra, cybergraph | measured | signal codec strict, consumer audit | closed |
| 18 | durable storage survives failure and restart | bbg | open | bbg P0 gates D1–D5 | 2026-10-09 |
| 19 | one particle definition, one `file::Particle` type across bbg, cybergraph, foculus, tru | file, bbg, cybergraph | open | decision on this page, then the type shared | 2026-09-25 |
| 20 | every bostrom particle re-addressed under hemera with its CID kept as a naming link; missing bytes have a defined identity | bostrom, cybergraph | open | re-hash run over the burial blockstore, count matches | 2026-10-02 |
| 21 | signals gossip between nodes over radio | radio, foculus, soft3 | open | three nodes, a signal reaches all peers before finality | 2026-10-02 |
| 22 | file bytes fetch by particle over radio blobs with verified streaming | radio, cyb | open | cyb opens a file it never had from a peer | 2026-10-09 |
| 23 | a file store answers by particle on the three network machines, seeded from the burial | cybernode, bbg | open | 3,069,134 files served, availability audit | 2026-10-16 |
| 24 | the serum is strictly proper in the meta-report and selects truth over coordinated consensus | tru | proven | strong-truthfulness.md; truth_scoring.rs 6 tests | implementation open, 2026-10-09 |
| 25 | ICBS market: cost function, inverse coupling, spam cost, per-link positions under conservation | tok, tru | open | market program on chain, honesty_loop.rs extended | 2026-10-09 |
| 26 | two axes: passive stake moves rank and earns nothing; only $v_\ell \neq 0$ risk earns | tok, tru | open | reward equation §11 implemented, test with idle and Sybil capital | 2026-10-16 |
| 27 | allocation curve $\theta^\alpha$ splits the security budget; floor derived from attack economics | tok | open | parameters fixed on this page, simulation | 2026-10-16 |
| 28 | the yield annuity pays foundational links; the discovery leak (§12) bounded or accepted | tru, foculus | open | per-epoch re-scoring on real graph; the leak is an accepted open frontier for phase 1 | 2026-10-23 |
| 29 | valence privacy hides individual positions and reports; only aggregates public | mudra, zheng | open | same proof profile as P1 | 2026-10-16 |
| 30 | any neuron roots a home book with its own token on the same binary; registration links name and state root into the graph | soft3, tok, cybergraph | open | a neuron on pussy-rc roots a book from cyb; the book settles at home | 2026-10-09 |
| 31 | domain finality per book: a book settles locally, cross-book conditions wait on evidence | foculus | open | oikos foundation 2 specified and exercised on two books | 2026-10-16 |
| 32 | referral: a share r of $ν at book birth goes to the referrer who linked the name; $ν holders receive the book's fees pro rata under conservation | tok, cybergraph | open | vectors: birth allocation, fee split, transfer | 2026-10-16 |
| 33 | referring an inactive or Sybil account yields zero | tok, tru | open | simulation with fake books | 2026-10-23 |

### calendar

| week | lane A · core | lane B · body | lane C · content |
|---|---|---|---|
| 1 · to 09-25 | registry agreed; privacy P1–P3 designed; property 3 measured | particle definition frozen (19); genesis pipeline, both chains, one node | availability map; radio wired into cyb for blob fetch |
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
- gates in CI: cyb fleet (three bodies) for the body; the network gate (three nodes, three machines, real genesis, conflicts, partition) for the core, run daily; the release train every friday by the trisha pattern: branches, six native builds, cross-platform verification, receipts.
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
| open | which particle definition wins: hemera(data) 32 bytes, or hemera over the lens commitment |
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

one pull request per hour from the launch worker (`scripts/launch-hour.sh`, sonnet, launchd `to.cyberia.launch-hour`), one row per PR. the owner merges; a merged row flips its registry property when the evidence is in.

| when (UTC) | property | repo | pull request | state |
|---|---|---|---|---|
| 2026-09-18T21:44:19Z | 13 | plumb (tok) | [launch #13: add transfer to close conservation's second mutation](https://github.com/cyberia-to/plumb/pull/1) | open |

## cross-references

[[rewards|rewards]] · [[foculus]] · [[tru]] · [[mudra]] · [[cybics/crystal|crystal]] · [[cyber/tokenomics]] · [[soft3]] · [[bootloader/bostrom|bostrom]] · [[bootloader/tokens/$PUSSY|space-pussy]] · [delivery roadmap](/cyber/roadmap/index)
