---
tags: trident, cyber, article
alias: master plan, nox master plan, nox_master_plan, cyber/launch
crystal-type: article
crystal-domain: cyber
---
# cyber/launch

A self-verifying knowledge graph where attention, computation, and [[consensus]] converge into a single metric (φ*), enabling [[intelligence]] emergence without central control.

nox
Optimizing civilization's ability to know what matters

Version: 2026.06 | Status: foundations complete → integration spine

## How to read this plan

Three anchors fix the whole path:

- MVP testnet — the first network where two independent nodes agree on one [[soft3/cybergraph|cybergraph]], every signal carrying its own proof.
- Mainnet — the launch that must be correct on the first try, because no patch relay exists between stars.
- The structural milestones between them — each one a capability the network gains, each one a hard gate that opens only when its test passes.

A milestone is code that passes its gate, never a date on a calendar. Durations below are work estimates in sessions, not deadlines. The order is a dependency order: a milestone starts when its predecessors pass.

## Where we are — 2026.06

The stack was built bottom-up, so it inverts the textbook order. The hard cryptographic floor — field, hash, commitments, the proof-native VM, the proof system, authenticated state — is implemented and tested. The frontier is the integration spine above it: the [[consensus]] that lets nodes agree, the runtime that drives the loop, and the single binary that wires them into a node.

| component | role | rust | tests | status |
|-----------|------|------|-------|--------|
| [[nebu]] | Goldilocks field (in [[strata]]) | — | — | complete |
| [[strata]] | 5 algebras × 4 tiers | 15.9K | 453 | complete |
| [[hemera]] | [[Poseidon2]] hash, [[cybics/crystal/particle|particle]] identity | 9.5K | 304 | complete, unaudited |
| [[lens]] | 5 commitment backends | 2.7K | 86 | complete; Ikat / Porphyry lightly tested |
| [[soft3/nox|nox]] | proof-native VM (18 patterns + jets) | 8.5K | 166 | complete; parallel reduction draft |
| [[zheng]] | [[SuperSpartan]] + Brakedown proofs | 4.5K | 87 | complete; look-argument soundness gap open |
| [[soft3/bbg|bbg]] | authenticated state | 3.5K | 53 | core complete; private scanning open |
| [[radio]] | iroh fork, QUIC + Poseidon2 | 130K | 478 | transport complete; Ed25519 → [[stark]] pending |
| [[sync]] | ordering, chain, VDF, DAS, erasure | 5.8K | 103 | core complete; push gossip + DAS verifier open |
| [[trident]] | .tri compiler | 76K | 981 | compiler in progress |
| [[glia]] | universal .model runtime | 24.8K | 145 | runs (CPU correct); backend parity ongoing |
| [[soft3/cybergraph\|cybergraph]] | signal processor | 2.0K | 49 | local-first; network + seal binding open |
| [[tru]] | φ*, [[tri-kernel]] | 1.0K | 7 | field converges; CT-0 model compile open |
| [[tape]] | particle framing | 1.0K | 24 | partial; wire format draft |
| [[tok]] | value layer (coin + card) | spec | — | specified |
| [[mudra]] | identity, crypto | scaffold | — | specified; build broken (path deps) |
| [[foculus]] | consensus by convergence | spec | — | specified, no code |
| [[soft3/soma|soma]] | runtime / the mind | spec | — | specified, no code |

The [[bostrom]] network has run 3+ years as the bootloader — ~70K [[cybics/crystal/neuron|neurons]], 2.9M [[cyberlinks]], 3.1M [[cybics/crystal/particle|particles]] — and is the migration source, not the soft3 network. The soft3-native node does not exist yet; building it is the spine of this plan.

Theoretical foundations established:

- Convergence guarantee: unique φ* exists, exponential convergence, bounded mixing time
- Conservation law: Σφ*ᵢ = 1, always — no inflation, no leakage
- GNN [[isomorphism]]: [[tri-kernel]] update ≡ multi-channel graph neural network message pass
- Transformer equivalence: CGC [[soft3/tru/specs/focus|focus]] ≡ iterated sparse attention with economic grounding
- [[convergent computation]]: replaces the halting problem — the system converges, never halts
- Free energy minimization: Δφ* is literally the gradient of system free energy

## The milestone ladder

| stage | milestone | gains | gate |
|-------|-----------|-------|------|
| M0 | Foundations | field, hash, commitments, VM, proofs, state | complete |
| S1 | Wire & framing | particles travel the network | [[tape]] round-trips, schema frozen |
| S2 | Identity | [[cybics/crystal/neuron|neurons]] sign and are addressed | [[mudra]] builds, keys + signatures verify |
| S3 | Proven processor | one node executes + proves a signal end-to-end | seal binding σ ⊢ scope enforced |
| S4 | Networking | nodes exchange signals | push gossip propagates, transport wired |
| S5 | Consensus v0 | nodes agree on finality | [[foculus]] φ*ᵢ > τ on a multi-node run |
| S6 | Node + genesis | one binary boots a network | soft3-node + genesis tooling |
| ★ | MVP testnet | the network agrees | 30 days, zero critical bugs under attack |
| P1 | Privacy | links public, owners private | nullifier soundness, leakage budget bounded |
| P2 | Economics | value + rewards live | conservation enforced, equilibrium simulated |
| P3 | Ranking at scale | φ* over the live graph | [[tru]] CT-0, Lyapunov proof |
| P4 | Recursive proof | light clients, verifier-as-nox | O(log n) state verification |
| P5 | Sharding & DA | planet-scale graph | DAS verifier, cross-shard sheaf consistency |
| P6 | Formal spine | machine-checked safety | the five Pre-Launch gates green |
| ★ | Mainnet | the launch that arrives correct | Pre-Launch Verification passes |

## M0 — Foundations (complete)

The floor every other milestone stands on. Implemented and tested today:

- [[strata]] — the five algebras (Goldilocks, F₂¹²⁸, R_q, tropical, isogeny) under one trait stack
- [[hemera]] — [[Poseidon2]] over Goldilocks; [[cybics/crystal/particle|particle]] identity, the [[soft3/nox|nox]] hash jet. Audit pending before any address is frozen.
- [[lens]] — Brakedown, Binius, Ikat, Assayer, Porphyry, one commitment per algebra
- [[soft3/nox|nox]] — the proof-native VM; 18 reduction patterns (16 compute + call + look) plus jets, every run a [[stark]] by construction
- [[zheng]] — [[SuperSpartan]] + Brakedown + [[sumcheck]]; prove and verify wired
- [[soft3/bbg|bbg]] — authenticated state, the mutator set, query proofs
- [[radio]] — QUIC transport with Poseidon2 verified streaming, gossip, and a docs store

Open inside this floor, carried as work into later milestones: [[hemera]] audit, [[zheng]] look-argument soundness, [[soft3/nox|nox]] parallel reduction, [[soft3/bbg|bbg]] private record scanning.

## The road to MVP testnet

Goal: two or more independent nodes converge on one [[soft3/cybergraph|cybergraph]] — signals gossip, order into per-[[cybics/crystal/neuron|neuron]] chains, apply their [[cyberlinks]] to state, and reach φ*ᵢ > τ finality, with every signal carrying a proof the [[cybics/crystal/neuron|neuron]] did what it declared.

In scope: signal lifecycle, proof at seal, gossip, minimal [[consensus]], a node binary.
Deferred past the testnet (see P-milestones): privacy circuits, the value layer and rewards, φ*-derived models, recursive proofs, sharding, formal verification. The testnet proves the architecture agrees; it does not yet carry value or hide owners.

### S1 — Wire & framing

[[tape]] is the typed frame every [[cybics/crystal/particle|particle]] and [[soft3/cybergraph/specs/signal|signal]] travels in. The encoder/decoder is partial and the wire format is a draft.

| deliverable | gate |
|-------------|------|
| [[tape]] encode/decode for particle, cyberlink, signal | round-trips on the conformance vectors |
| wire schema frozen | one schema, all SDKs read it |

Estimate: 2-3 sessions.

### S2 — Identity

[[mudra]] gives a [[cybics/crystal/neuron|neuron]] its keys and signatures. The crate is a scaffold and its build is broken on path dependencies. The testnet needs signing and addressing only; FHE, stealth, and threshold are P-milestone work.

| deliverable | gate |
|-------------|------|
| fix [[mudra]] path deps, crate builds | green build in the workspace |
| signing + verification (Ed25519 now, [[stark]] path later) | a signal's signature verifies at order |

Estimate: 1-2 sessions for the testnet subset.

### S3 — Proven processor (single node)

[[soft3/cybergraph\|cybergraph]] runs local-first today: intend / seal / link apply [[cyberlinks]] to [[soft3/bbg|bbg]], query runs [[inf]]. The missing property is the alignment guarantee — `seal(i, s)` accepted iff σ(s) ⊢ scope_hash(i). This closes the [[zheng]] look-argument soundness gap at the same time.

| deliverable | gate |
|-------------|------|
| [[zheng]] proves a [[soft3/nox|nox]] run against a [[soft3/bbg|bbg]] root | look argument verified, no soundness gap |
| seal binding enforced at the commit port | unproven signal rejected; proven signal applied |

Estimate: 3-5 sessions.

### S4 — Networking

[[radio]] moves bytes; [[sync]] orders them. Today [[sync]] is pull-only — peers fetch on demand, signals do not propagate. A network needs a mempool membrane that pushes a new signal to peers.

| deliverable | gate |
|-------------|------|
| push gossip for new signals | a signal reaches all peers before finality |
| [[sync]] ordering over the wire | per-neuron chains agree across nodes, equivocation rejected |

Estimate: 3-4 sessions.

### S5 — Consensus v0

[[foculus]] is a complete specification with zero code — the largest single gap. A [[cybics/crystal/particle|particle]] is final when φ*ᵢ > τ. The testnet needs the minimal core: compute φ* over committed state, apply the fork-choice rule, declare finality.

| deliverable | gate |
|-------------|------|
| φ* finality check (φ*ᵢ > τ) | matches [[tru]] field computation on the same graph |
| fork choice + safety on a multi-node run | no two nodes finalize conflicting state |

Estimate: 6-10 sessions. This is the critical path.

### S6 — Node + genesis

No binary wires the spine together, and there is no genesis. [[soft3/soma|soma]] is the runtime that drives the fetch → execute → prove → commit loop; the testnet needs its minimal drive-loop, not the full cognitive architecture.

| deliverable | gate |
|-------------|------|
| soft3-node binary (cybergraph + bbg + sync + radio + foculus) | boots, joins a peer, processes signals |
| minimal [[soft3/soma|soma]] drive-loop | one signal flows fetch → execute → prove → commit unattended |
| genesis + bootstrap tooling | a fresh network starts from a genesis root |

Estimate: 5-8 sessions.

## MVP testnet — the gate

| milestone | gate |
|-----------|------|
| Devnet | all unit + integration tests pass; signal flows end-to-end on one machine |
| Testnet | multi-node, public; 30 days with zero critical bugs under attack |

The testnet is the proof that the architecture converges: the dumb processor ([[soft3/cybergraph\|cybergraph]]) and the smart runtime ([[soft3/soma|soma]]) drive one network to agreement. It does not yet carry private value — that is the work from here to mainnet.

## Structural milestones: testnet → mainnet

Each one a capability the testnet lacks, ordered by dependency.

### P1 — Privacy circuits

UTXO-style privacy with ZK proofs for every state transition. Public: edge existence, aggregate energy per [[cybics/crystal/particle|particle]], the φ* distribution. Private: [[cybics/crystal/neuron|neuron]] identity behind an edge, individual energy ownership, link authorship. φ* stays computable from public aggregates alone — secure multi-party computation of a GNN forward pass. Depends on [[soft3/bbg|bbg]] private scanning and [[mudra]] stealth.

Gate: transaction + cyberlink circuits, nullifier soundness, leakage budget L(queries, graph) bounded.

### P2 — Economics

[[tok]] carries value: [[coin]] (fungible) and [[card]] (non-fungible), all change a [[plumb]] mutation under conservation. Rewards mint on Δφ* — creating valuable structure is creating [[value]] — Sybil-resistant via stake-weighting. Both [[tok]] and the reward spec are specified, not built.

Gate: conservation enforced by proof, reward equilibrium simulation-tested under 100× adversarial load.

### P3 — Ranking at scale

[[tru]] computes the field today; the CT-0 model-compilation passes bail. This milestone runs the [[tri-kernel]] over the live graph, deriving models from φ*, adversarially proven.

Gate: φ* over the production graph, explicit Lyapunov function with dV/dt < 0, bounded locality k = O(log(1/ε)).

### P4 — Recursive proof

The [[stark]] verifier is itself a [[soft3/nox|nox]] program: proofs can be verified, and verification can be proven. Unlocks O(log n) light clients.

Gate: inner verification circuit arithmetized, light-client verification of any state claim.

### P5 — Sharding & data availability

Shards as subtopoi, a sheaf of attention weights keeping cross-shard [[soft3/tru/specs/focus|focus]] consistent. [[sync]] has erasure coding and DAS commitments; the sample verifier is missing.

Gate: DAS verifier complete, cross-shard consistency proven, gossip bandwidth ∝ stake.

### P6 — Formal verification spine

Runs parallel to every milestone, converging here. Each line below is one of the Pre-Launch gates.

| what | how |
|------|-----|
| Layer 1 confluence (16 patterns) | Lean4 / Coq |
| cost determinism | structural induction, machine-checked |
| [[soft3/tru/specs/focus|focus]] conservation (Σφ*ᵢ = 1) | transition analysis |
| privacy soundness (< 2⁻¹²⁸) | [[stark]] soundness theorem |
| [[tri-kernel]] convergence | Lyapunov function, explicit constants |
| adversarial equilibrium | game theory + simulation |
| double-spend prevention | nullifier uniqueness proof |

## Mainnet — the gate

No patch relay exists between stars. What launches must be correct. Before launch, answer five questions with machine-checked evidence:

| # | question | evidence |
|---|----------|----------|
| 1 | Does φ* converge? | Lean4 proof of Lyapunov stability |
| 2 | Can proofs be forged? | soundness proof + 10⁸ fuzzing runs, 0 counterexamples |
| 3 | Can the economy be drained? | Nash equilibrium proof + 100× adversarial simulation |
| 4 | Is computation deterministic? | cross-implementation state-root match on 10⁶ blocks |
| 5 | Does it survive partial failure? | chaos test, zero safety violations |

All five green → launch. Any red → no launch. No exceptions.

| milestone | gate |
|-----------|------|
| Canary net | 90 days stability, all economic invariants hold |
| Mainnet genesis | Pre-Launch Verification passes (all 5 gates green) |
| [[bostrom]] migration | bijective state mapping, zero data loss |

The light-cone is merciless. What you ship is what arrives.

## Token Architecture

### Four Token Types (Protocol-Native)

| Type | Fungible | Movable | Role | Examples |
|------|----------|---------|------|----------|
| [[coin]] | yes | yes | [[consensus]], fees, stake | [[$CYB]], [[$BOOT]] |
| [[card]] | no | yes | knowledge assets, provenance | authorship proofs, dataset ownership |
| [[score]] | yes | no | reputation, credentials | [[cybics/crystal/karma|karma]] |
| [[badge]] | no | no | unique non-transferable credentials | achievements |

[[$CYB]] is the [[consensus]] token of the full [[cyber]] network. On [[bostrom]] (bootloader): [[$BOOT]] (stake/fees), [[$H]] (liquid fuel), [[$V]] (will), [[$A]] (attention).

### Adaptive Economics

Three PID-controlled variables adapt automatically — no [[governance]] vote needed for routine adjustment:

α (allocation curve exponent): balances PoW vs PoS allocation. staking_share = S^α.

φ (security floor): minimum issuance for security. φ ≥ k · (TVL/MarketCap) · r.

β (fee burn rate): decouples gross rewards from net inflation.

Staking yield at equilibrium: r_s = (G · S^(α-1)) / M. Master safety indicator: ρ = d(Attack Cost)/dt ÷ d(Attack Profit)/dt; ρ > 1 means defenses grow faster than threats.

### Supply and emission

Total supply is the [[nebu|Goldilocks field]] order (p = 2⁶⁴ − 2³² + 1). Genesis seeds a 281,405,532,467,645 [[$CYB]] starting balance for TOCYB holders (0.001526% of total); the rest emits over decades along a seven-halving curve. Full supply cap, genesis, and the emission curve live on [[$CYB]].

## The genesis seed

The [[cybics/crystal|crystal]] is the genesis content — exactly 5,040 [[cybics/crystal/particle|particles]] forming the irreducible basis from which all civilizational reasoning composes. An alphabet of a mind.

| Layer | Particles | Types |
|-------|-----------|-------|
| Vocabulary | 4,320 | Entities (2,400), Processes (960), Properties (720), Measures (240) |
| Grammar | 720 | Relations (480), Patterns (240) |

Two-layer load: a lattice (4,392 [[cybics/crystal/particle|particles]], ~454K tokens) of structural vocabulary that fits one model context, and flesh (648 [[cybics/crystal/particle|particles]], ~1,165K tokens) of articles and proofs retrieved on demand via [[cyberlink]] traversal. The 12 genesis invariants (completeness, connectivity, reachability ≤ 6 hops, irreducibility, positivity, self-reference, bridge density, type balance, defect freedom, growth-readiness, narrative depth, self-explanation) are the quality gates before the crystal ships. See [[cybics/crystal|crystal]] for the full specification.

## The endgame

A living, self-optimizing knowledge network that:

1. Learns from every form of input on Earth — humans, AI, sensors, biology
2. Holds coherence under extreme conditions, including interplanetary latency
3. Evolves without central authority — [[governance]] through [[soft3/tru/specs/focus|focus]] dynamics and futarchy
4. Maximizes the survival, [[intelligence]], and flourishing of the planet's biosphere
5. Proves every claim — no trust required, only math

The network IS thinking. No node comprehends. The network knows.

## Cross-references

- [[cybics/crystal|crystal]] — the genesis seed specification
- [[cyber/tokenomics]] — the 7-mechanism incentive spec
- [[learning incentives]] — reward design, link valuation, attribution
- [[cft]] — the [[collective focus]] theorem
- [[soft3]] — the stack these milestones build on
- [[soft3/cybergraph\|cybergraph]] and [[soft3/soma|soma]] — the dumb processor and the mind that drives it
