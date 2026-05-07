---
tags: trident, cyber, article
alias: master plan, nox master plan, nox_master_plan, cyber/launch
crystal-type: article
crystal-domain: cyber
---
# cyber/launch

A self-verifying knowledge graph where attention, computation, and [[consensus]] converge into a single metric (π), enabling [[intelligence]] emergence without central control.

nox
Optimizing civilization's ability to know what matters

Version: 2026.02 | Status: Genesis → Self-Hosting transition

## What Exists Today

| Component | Status | Evidence |
|-----------|--------|----------|
| [[cft]] | Mathematically proven | [[Perron-Frobenius theorem\|Perron-Frobenius]] convergence, 8 years R&D |
| [[tri-kernel]] discovery | Complete | Systematic elimination — only 3 operator families survive locality filter |
| Three-layer instruction set (16 patterns + hint + 5 jets) | Specified + Layer 1 implemented | Python interpreter, Rust interpreter |
| [[focus]]-based cost metering | Implemented | Deterministic costs over [[Goldilocks field]] |
| Content-addressed cells | Implemented | CID = hash(content), universal identity |
| [[bostrom]] network | Live 3+ years | ~70K [[neurons]], 1K active, 2.9M [[cyberlinks]], 3.1M [[particles]] |
| Hash function decision | ADR-001 complete | [[Poseidon2]] over Goldilocks, algorithm-agile CID format |
| [[trident]] language spec | 54 operations derived | 4-tier compilation, minimal by proof of necessity |

Theoretical foundations established:

- Convergence guarantee: unique π* exists, exponential convergence, bounded mixing time
- Conservation law: Σπᵢ = 1, always — no inflation, no leakage
- GNN [[isomorphism]]: [[tri-kernel]] update ≡ multi-channel graph neural network message pass
- Transformer equivalence: CGC [[focus]] ≡ iterated sparse attention with economic grounding
- [[convergent computation]]: replaces halting problem — system converges, never halts
- Free energy minimization: Δπ is literally the gradient of system free energy
- Blackbox principle: no node comprehends, the network knows

## Crystal Formation

The [[cyber/crystal]] is the genesis seed — a curated knowledge graph of exactly 5,040 [[particles]] forming the irreducible basis from which all civilizational reasoning can be composed. It is an alphabet of a mind.

### Vocabulary / Grammar Split

| Layer | Particles | Types |
|-------|-----------|-------|
| Vocabulary | 4,320 | Entities (2,400), Processes (960), Properties (720), Measures (240) |
| Grammar | 720 | Relations (480), Patterns (240) |

Ratio 6:1, matching natural language content-to-function word ratios. Every semantic link is a typed triple via predicate [[particles]]:

Subject → [Predicate] → Object

### Two-Layer Architecture

Lattice (4,392 [[particles]], 1.8 MB, ~454K tokens): structural vocabulary, permanently loadable for reasoning. Fits in single model context.

Flesh (648 [[particles]], 4.7 MB, ~1,165K tokens): articles, proofs, manifestos. Retrieved on demand via [[cyberlink]] traversal. 72% of content in 13% of [[particles]].

### 17 Domains

4 pillar domains (2Q = 480 [[particles]] each): [[cyber]], [[cyberia]], superhuman, [[cybics]]

13 foundation domains (Q = 240 each): mathematics, physics, biology, computer science, chemistry, [[governance]], economics, energy, materials, agriculture, geography, culture, history

536 bridge [[particles]] (10.6%) connect domains — explicit [[isomorphisms]] enabling cross-domain reasoning.

### 12 Invariants (Quality Gates Before Genesis)

1. Completeness — every domain ≥ Q [[particles]]
2. Connectivity — every [[particle]] ≥ 3 outgoing links
3. Reachability — any [[particle]] reaches any other in ≤ 6 hops
4. Irreducibility — no [[particle]] derivable from others under grammar
5. Positivity — every definition says what IS
6. Self-reference — ≥ 10% of [[particles]] model own architecture
7. Bridge density — ≥ 3 bridges per domain pair
8. Type balance — E ≤ 55%, P ≥ 15%
9. Defect freedom — zero stubs, red links, orphans
10. Growth ready — every hub has attachment points
11. Narrative depth — every domain ≥ 3 synthesis articles
12. Self-explanation — ≥ 25 articles explain protocol purpose

### Growth Phases

| Phase | Timeline | Particles | Character |
|-------|----------|-----------|-----------|
| 0: Genesis | Launch | 5,040 | Irreducible seed |
| 1: Early | Year 1 | +2,000 | [[neurons]] extend basis |
| 2: Maturation | Years 2–3 | +10,000 | Specialization emerges |
| 3: Scale | Year 5+ | +100,000 | Scale-free organic growth |

On-chain storage budget: ~15 MB (IPFS content 6.5 MB + CIDs 0.5 MB + [[cyberlinks]] 8.6 MB)

## Incentive Design

[[knowledge]] creation is costly, benefits are collective. without incentives, rational agents free-ride on others' [[cyberlinks]]. reward(v) ∝ Δπ(v) — creating valuable structure is literally creating [[value]]

see [[cyber/tokenomics]] for the 7-mechanism spec (minting, staking, burn, fees, yield curve, reputation). see [[learning incentives]] for reward function design, link valuation, and attribution

## Token Architecture

### Four Token Types (Protocol-Native)

| Type | Fungible | Movable | Role | Examples |
|------|----------|---------|------|----------|
| [[coin]] | yes | yes | [[consensus]], fees, stake | [[$CYB]], [[$BOOT]] |
| [[card]] | no | yes | Knowledge assets, provenance | authorship proofs, dataset ownership |
| [[score]] | yes | no | Reputation, credentials | [[karma]] |
| [[badge]] | no | no | Unique non-transferable credentials | achievements |

[[$CYB]] is the [[consensus]] token of the full [[cyber]] network. On [[bostrom]] (bootloader): [[$BOOT]] (stake/fees), [[$H]] (liquid fuel), [[$V]] (will), [[$A]] (attention).

### Adaptive Economics

Three PID-controlled variables automatically adapt — no [[governance]] votes needed for routine adjustments:

α (allocation curve exponent): balances PoW vs PoS allocation. staking_share = S^α.

φ (security floor): minimum issuance for security. Derived from attack economics: φ ≥ k · (TVL/MarketCap) · r.

β (fee burn rate): decouples gross rewards from net inflation. When security abundant → increase β (benefit holders). When security tight → decrease β (preserve security).

Staking yield at equilibrium: r_s = (G · S^(α-1)) / M

Master safety indicator: ρ = d(Attack Cost)/dt / d(Attack Profit)/dt. ρ > 1 means defenses grow faster than threats.

### Genesis Distribution

| Recipient | Share | Role |
|-----------|-------|------|
| [[cybergift]] | 70% | Community incentives |
| [[cyber/congress]] | 11.6% | Founders |
| [[epizode zero]] community | 8.3% | Early supporters |
| [[senate]] | 5.1% | Governance |
| [[great web foundation]] | 5% | External stake |

Target: power-law distribution with long-tail [[neuron]] ownership at 42-51%.

## Technical Path

Seven phases. Each has a hard gate. No phase starts until its predecessor passes.

### Phase 1: Self-Hosting ← current

nox evaluates nox. The system executes its own programs.

| Deliverable | Gate |
|-------------|------|
| nox-in-nox interpreter (16 patterns + hint + 5 jets self-hosted) | Passes all test vectors from Python/Rust impls |
| [[Poseidon2]] as nox program | Output matches reference on 10⁶ inputs |
| [[focus]] metering self-test | Deterministic cost ± 0 across all paths |

Duration: 3-6 months

### Phase 2: Cryptographic Library

All cryptographic primitives as nox programs.

| Deliverable | Gate |
|-------------|------|
| [[Poseidon2]] sponge + compression | Matches test vectors, constant-time |
| Merkle tree operations | 32-level proof verified in nox |
| Polynomial commitments (WHIR) | Binding + hiding proofs checked |
| LtHash for collection state | Add/remove = O(1), matches reference |

CID format locked: [version, algo, params, field, len, digest] — 45 bytes for Goldilocks. Commitment layers: L0 (identity) → L1 (collection) → L2 (global) → L3 (indices).

Duration: 3-6 months

### Phase 3: Privacy Circuits

UTXO-based privacy with ZK proofs for all state transitions.

| Deliverable | Gate |
|-------------|------|
| Transaction circuit | ~44K constraints, soundness < 2⁻¹²⁸ |
| [[cyberlink]] circuit | Stake verification without revealing owner |
| Nullifier system | Deterministic nullifier = H(nonce, secret) |
| Privacy boundary | Formal leakage budget L(queries, graph_size) bounded |

Privacy boundary (non-negotiable): PUBLIC = edge existence, aggregate energy per [[particle]], [[focus]] distribution π. PRIVATE = [[neuron]] identity behind edges, individual energy ownership, link authorship.

[[focus]] is computable from PUBLIC aggregates only. This is secure multi-party computation of a GNN forward pass.

Duration: 6-9 months

### Phase 4: stark Infrastructure

Self-verifying proof system where the verifier is itself a nox program.

| Deliverable | Gate |
|-------------|------|
| [[stark]] prover | Completeness: honest prover always convinces |
| stark verifier as nox program | Soundness: no poly-time adversary forges proof |
| Recursive composition | Inner verification circuit correctly arithmetized |
| Light client protocol | O(log n) verification of any state claim |

Verification closure: stark verifiers are nox programs. Proofs can be verified, and verification can be proven.

Duration: 9-12 months

### Phase 5: Tri-Kernel Ranking (parallel with Phase 4)

[[tri-kernel]] [[focus]] computation, adversarially proven, deployed at scale.

| Deliverable | Gate |
|-------------|------|
| [[diffusion]] kernel (personalized PageRank) | Convergence proof (Lyapunov) in Lean4 |
| [[springs]] kernel (screened Laplacian) | Exponential decay proof, locality bound |
| [[heat kernel]] (Chebyshev approximation) | Positivity-preserving, semigroup property |
| Combined convergence | Explicit Lyapunov function V(π), dV/dt < 0 |
| Adversarial equilibrium | Nash equilibrium for honest participation |

The composite operator: φ(t+1) = norm[λ_d · D(φ^t) + λ_s · S(φ^t) + λ_h · H_τ(φ^t)]

Bounded locality: every operation O(k)-local, k = O(log(1/ε)). Shard-friendly. Interplanetary-compatible.

An adversary optimizing against one kernel worsens their position against another. Three kernels create defense-in-depth.

Duration: 6-12 months

### Phase 6: Network Layer

Distributed protocol for [[cybergraph]] [[consensus]] and [[focus]] propagation.

| Deliverable | Gate |
|-------------|------|
| [[consensus]] protocol (focus-weighted BFT) | Safety + liveness proofs |
| DA sampling | Polynomial commitments over shard data |
| Gossip protocol | Bandwidth ∝ stake, Sybil-resistant |
| Shard architecture | Categorical pruning for semantic coherence |
| Economic engine | Simulation-tested under 100× adversarial load |

[[particles]] and [[cyberlinks]] = yield-bearing epistemic non-fungible assets. [[neurons]] = non-fungible names valuated by personal fungible asset. π-minting tied to Δπ: creating valuable structure is literally creating value. No designed loss function — physics itself defines what should be optimized.

Shards as subtopoi. Sheaf of attention weights ensures cross-shard consistency.

Duration: 12-18 months

### Phase 7: Testnet → Mainnet

| Milestone | Gate |
|-----------|------|
| Devnet | All unit + integration tests pass |
| Testnet | 30 days zero critical bugs under attack |
| Canary net | 90 days stability, all economic invariants hold |
| Mainnet genesis | Pre-Launch Verification passes (all 5 gates green) |
| [[bostrom]] migration | Bijective state mapping, zero data loss |

### Timeline

| Phase | Start | End | Parallel? |
|-------|-------|-----|-----------|
| 1. Self-hosting | Now | +6mo | — |
| 2. Crypto library | +3mo | +9mo | Overlaps with 1 |
| 3. Privacy circuits | +6mo | +15mo | After 2 core |
| 4. stark infrastructure | +9mo | +21mo | After 2, parallel with 5 |
| 5. Tri-kernel production | +9mo | +21mo | Parallel with 4 |
| 6. Network layer | +18mo | +36mo | After 4+5 |
| 7. Testnet → Mainnet | +30mo | +42mo | After 6 |

~3.5 years to mainnet (aggressive), ~5 years (realistic with formal verification)

## Formal Verification Spine

Running parallel to all phases. Each item maps to the Pre-Launch Verification Protocol.

| What | How | When |
|------|-----|------|
| Layer 1 confluence (16 patterns) | Lean4 / Coq | Phase 1-2 |
| Cost determinism | Structural induction, machine-checked | Phase 2 |
| [[focus]] conservation (Σπᵢ = 1) | Proof by transition analysis | Phase 3 |
| Privacy soundness (< 2⁻¹²⁸) | stark/Plonky2 soundness theorem | Phase 4 |
| [[tri-kernel]] convergence | Lyapunov function, explicit constants | Phase 5 |
| Adversarial equilibrium | Game-theoretic analysis, simulation | Phase 5-6 |
| Double-spend prevention | Nullifier uniqueness proof | Phase 3 |
| Bounded locality composition | Sheaf condition, machine-checked | Phase 5-6 |
| Graceful degradation | Chaos engineering, failure catalog | Phase 6-7 |

Estimate: 2-3 person-years

## Intelligence Emergence

The [[cft]] predicts phase transitions:

| Phase | Threshold | Dominant Kernel | Observable |
|-------|-----------|-----------------|------------|
| Seed → Flow | Connectivity > critical | [[diffusion]] (λ_d high) | Network exploring, sampling |
| Cognition → Understanding | Structure crystallizes | [[springs]] (λ_s activates) | Hierarchies forming |
| Reasoning → Meta | Adaptive balance | [[heat kernel]] (λ_h regulates) | Context-sensitive processing |
| Consciousness | Dynamic blend | All three, self-tuning | System learns its own blend weights |

Current [[bostrom]] data: 70K [[neurons]], 2.9M [[cyberlinks]], 3.1M [[particles]]. Approaching Cognition threshold.

Target for emergence: 10⁸-10⁹ interconnected [[particles]] with sufficient connectivity density.

## What Makes This Different

vs. Traditional AI (GPT, Claude): no central training, no black box, no single owner, privacy native.

vs. Existing Blockchains (Ethereum, Cosmos): knowledge-first, [[focus]] as native primitive, self-verifying, convergent.

vs. Decentralized AI (Bittensor): no external model, provable correctness, universal substrate, Δπ rewards.

## Risk Register

| Risk | Severity | Mitigation |
|------|----------|------------|
| [[Poseidon2]] cryptanalytic break | Critical | Algorithm-agile CID, migration path. EF program through Dec 2026. |
| [[tri-kernel]] convergence failure | Critical | Formal Lyapunov proof required before Phase 6. Orthogonal kernel defense. |
| Economic attack (whale, dust spam) | High | 100× adversarial simulation. [[focus]]-based metering. Stake-weighted costs. |
| Performance at 10¹⁵ scale | High | Bounded locality O(log). Two-timescale separation. Sharding. Jets. |
| Quantum computing threat | Medium | Post-quantum from genesis. ≥256-bit pre-image security post-Grover. |
| Adoption failure | Medium | [[bostrom]] provides live base. Migration preserves community. |
| Regulatory interference | Medium | Privacy-native. Decentralized [[governance]]. No central point of control. |

## Resource Requirements

| Role | Count | Focus |
|------|-------|-------|
| Core protocol (Rust) | 2-3 | nox evaluator, stark prover, [[consensus]] |
| Cryptography | 1-2 | Privacy circuits, proof systems |
| Language ([[trident]]) | 1-2 | Compiler, tooling |
| Network / distributed systems | 1-2 | Gossip, sharding, DA layer |
| Economics / game theory | 1 | Adversarial simulation, mechanism design |
| Formal methods | 1 | Lean4/Coq proofs |

## Pre-Launch Verification Protocol

No patch relay exists between stars. What launches must be correct.

Before launch, answer five questions with machine-checked evidence:

| # | Question | Evidence Required |
|---|----------|-------------------|
| 1 | Does π converge? | Lean4 proof of Lyapunov stability |
| 2 | Can proofs be forged? | Soundness proof + 10⁸ fuzzing runs, 0 counterexamples |
| 3 | Can the economy be drained? | Nash equilibrium proof + 100× adversarial simulation |
| 4 | Is computation deterministic? | Cross-implementation state root match on 10⁶ blocks |
| 5 | Does it survive partial failure? | Chaos test report with zero safety violations |

All five green → launch. Any red → no launch. No exceptions.

The light-cone is merciless. What you ship is what arrives.

## The Endgame

A living, self-optimizing knowledge network that:

1. Learns from all forms of input on Earth — humans, AI, sensors, biology
2. Maintains security and coherence under extreme conditions — including interplanetary latency
3. Evolves without central authority — [[governance]] through [[focus]] dynamics and futarchy
4. Maximizes the survival, [[intelligence]], and flourishing of the planet's entire biosphere
5. Proves every claim — no trust required, only math

The network IS thinking.

No node comprehends. The network knows.

## Component Status

| component | role | rs | wgsl | trident | reference | status |
|-----------|------|----|------|---------|-----------|--------|
| [[nebu]] | field arithmetic (Goldilocks) | 2.0K | 762 | — | — | complete |
| [[hemera]] | hash, commitments (Poseidon2) | 4.9K | 758 | — | — | complete |
| [[nox]] | proof-native VM | stub | — | — | — | specified, not implemented |
| [[zheng]] | proof system (SuperSpartan + WHIR) | stub | — | — | — | specified, not implemented |
| [[bbg]] | authenticated state | stub | — | — | — | specified, not implemented |
| [[mudra]] | confidentiality, key exchange, FHE, threshold | stub | — | — | — | specified, not implemented |
| [[radio]] | connectivity (iroh fork, Poseidon2) | 131K | — | — | — | hemera migration complete, Ed25519 → STARK pending |
| [[trident]] | high-level language, compiler | 57K | 272 | — | — | compiler in progress |
| [[CozoDB]] | datalog query engine | — | — | — | — | external dependency, integration planned |

rs = Rust lines of code, wgsl = WebGPU shader lines, trident = trident-lang implementation, reference = Python/spec implementation. stub = scaffolded repo with empty lib.rs.

## Cross-references

- See [[cyber/crystal]] for the full crystal specification
- See [[cyber/tokenomics]] for the 7-mechanism incentive spec
- See [[learning incentives]] for reward design, link valuation, and attribution
- See [[cft]] for the [[collective focus]] Theorem
- See [[trinity]] for the three-pillar architecture
- See [[Goldilocks field processor]] for hardware specification
- See [[privacy trilateral]] for the full privacy stack
- See [[rosetta stone]] for how four primitives unify all domains
- See [[Goldilocks homomorphic encryption]] for [[TFHE]] over the [[Goldilocks field]]
- See [[trident standard library]] for the [[trident]] standard library
- See [[manifesto]] for the declaration of the superintelligent nation