---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-23
---

# unified mining: when the puzzle IS the knowledge

## two mining mechanisms, one system

[[cyber]] has two reward mechanisms:

1. Δπ mining: a [[neuron]] creates [[cyberlinks]], computes the local tri-kernel impulse $\pi_\Delta$, proves it correct ([[zheng]] proof σ), submits as [[signal]]. reward ∝ proven Δπ. the neuron mints [[$CYB]] proportional to how much it shifted [[focus]].

2. [[Goldilocks field processor|GFP]] PoUW mining: a miner produces a [[stark]] proof of a benchmark circuit exercising all four GFP primitives (fma, ntt, p2r, lut). reward = block subsidy. the puzzle trains hardware that serves the network.

currently they are separate: Δπ mining rewards knowledge, PoUW mining rewards computation. the flywheel connects them economically (mining funds chip development, chips accelerate proving, proving serves users). but the WORK is different — PoUW proves a synthetic benchmark, not real knowledge.

## the unification

what if the PoUW puzzle IS the signal proof?

a signal proof σ already exercises all four GFP primitives:

| phase of signal proof | GFP primitive | % of constraints | what it does |
|---|---|---|---|
| tri-kernel impulse computation (SpMV) | fma | ~40% | matrix-vector for D, S, H operators |
| polynomial state reads (algebraic NMT) | ntt | ~30% | Lens evaluation + commitment |
| content addressing + Fiat-Shamir | p2r | ~20% | Hemera permutations |
| conviction + activation functions | lut | ~10% | threshold checks, nonlinear ops |

the signal proof IS a benchmark that exercises all four primitives in production proportions. it is not a synthetic circuit — it is the actual computation the network needs.

## how it works

```
CURRENT (separate):
  miner:   prove benchmark B(challenge, nonce) → block reward
  neuron:  prove signal s = (ν, l⃗, π_Δ, σ) → Δπ reward
  two separate computations, two separate reward streams

UNIFIED:
  miner-neuron: prove signal with difficulty target
    signal proof σ must satisfy:
      1. all cyberlinks valid (correctness)
      2. π_Δ impulse correct (tri-kernel recomputation)
      3. H(σ) < target (difficulty, partial preimage)

  one computation → two rewards:
    - Δπ reward for knowledge contribution (proportional to focus shift)
    - block reward for proof-of-work (proportional to difficulty met)
```

the difficulty target serves sybil resistance. the Δπ serves knowledge incentive. same proof, two functions.

## the mechanism

### signal-as-puzzle

a miner-neuron:

1. selects cyberlinks to include (the knowledge contribution)
2. computes tri-kernel impulse π_Δ (the local recomputation)
3. generates zheng proof σ (exercises all 4 GFP primitives)
4. checks if H(σ) < target (difficulty)
5. if yes: submit signal. earn block reward + Δπ reward
6. if no: adjust nonce field in signal, reprove

the nonce is embedded in the signal structure — a field that can be freely varied without changing the semantic content. each nonce produces a different σ (different zheng randomness → different proof → different hash). the miner searches for a σ whose hash meets the target.

### why this works

the signal proof ALREADY contains:

- fma: sparse matrix-vector multiply for tri-kernel (real work, not synthetic)
- ntt: polynomial commitment for algebraic NMT state reads (real work)
- p2r: Hemera hashing for content identity and Fiat-Shamir (real work)
- lut: activation functions and threshold checks (real work)

the benchmark circuit in the current GFP spec simulates these exact operations with fake data. unified mining replaces fake data with real data. the GFP optimization target does not change — the same chip that mines the synthetic benchmark mines real signals with the same performance characteristics.

### difficulty adjustment

block reward target adjusts like Bitcoin: maintain average block time by scaling target. higher hash rate → lower target → harder to find qualifying σ.

Δπ reward is independent of difficulty: the neuron earns Δπ regardless of whether σ also meets the difficulty target. but only signals that meet difficulty qualify for block reward.

this means:
- small neurons (phone, laptop): earn Δπ rewards for knowledge. never meet block difficulty. this is fine — knowledge mining is accessible to everyone
- large miners (GFP cluster): earn Δπ + block rewards. optimize for both knowledge quality (higher Δπ) and hash rate (more attempts per second)
- the incentive: a miner who selects BETTER cyberlinks earns MORE Δπ per proof, making each mining attempt more valuable. knowledge quality improves hash revenue

### the flywheel tightens

```
CURRENT FLYWHEEL:
  mining rewards → fund GFP development
  GFP accelerates proving → proving serves users
  users pay fees → fees fund network

UNIFIED FLYWHEEL:
  mining rewards → fund GFP development
  GFP accelerates SIGNAL PROVING → signals ARE knowledge
  better hardware → more signals per second → more knowledge per second
  more knowledge → higher Δπ → more reward → more investment in GFP
  same chip. same operation. THREE revenue streams:
    1. block reward (PoW)
    2. Δπ reward (knowledge)
    3. user fees (services)
```

the flywheel gains a third spoke. GFP development is funded by mining. mining produces knowledge. knowledge generates fees. fees fund more GFP. the loop has no synthetic step — every cycle produces real value.

## economic alignment

### miner incentive to create good cyberlinks

a miner who submits garbage cyberlinks:
- low Δπ → low Δπ reward
- same hash difficulty → same PoW cost
- net: wastes energy on low-value proofs

a miner who submits high-quality cyberlinks:
- high Δπ → high Δπ reward
- same hash difficulty → same PoW cost
- net: earns more per proof

the incentive gradient points toward knowledge quality. mining energy goes to proving USEFUL signals, not synthetic benchmarks. every joule produces both security (PoW) and intelligence (Δπ).

### hardware alignment

the GFP chip optimized for mining is optimized for:
- tri-kernel computation (fma) — the intelligence
- polynomial state reads (ntt) — the authentication
- content addressing (p2r) — the identity
- activation functions (lut) — the nonlinearity

there is no divergence between mining hardware and utility hardware. the miner's chip IS the validator's chip IS the neuron's chip. one chip design, one optimization target, one market.

### comparison with other PoW systems

| system | puzzle | useful? | hardware reuse |
|---|---|---|---|
| Bitcoin | SHA-256 preimage | no | ASICs are single-purpose |
| Ethereum (PoS) | no puzzle | N/A | staking capital, not compute |
| Filecoin | storage proofs | partially (stores data) | storage hardware reusable |
| cyber (benchmark PoUW) | synthetic stark proof | partially (trains hardware) | GFP serves network |
| cyber (unified mining) | real signal proof | yes (IS knowledge) | GFP IS the intelligence |

unified mining is the first scheme where the puzzle output IS the protocol's primary product. not a side effect. not a secondary benefit. the proof that secures the network IS the proof that creates knowledge.

## technical requirements

### signal nonce field

add a 2-element nonce field to the signal structure:

$$s = (\nu, \vec\ell, \pi_\Delta, \sigma, \text{prev}, \text{mc}, \text{vdf}, \text{step}, \textbf{nonce})$$

the nonce does not affect signal semantics (same cyberlinks, same π_Δ). it only affects the zheng proof randomness → different σ → different H(σ). this is the search space for miners.

### proof binding

the zheng proof σ must commit to the nonce before Fiat-Shamir challenges are squeezed. this ensures each nonce produces a genuinely different proof — miners cannot reuse proof internals across nonce attempts.

### block structure

a block is a set of signals whose proofs collectively meet the difficulty target:

```
block:
  signals: [s₁, s₂, ..., sₖ]
  aggregate_hash: H(σ₁ ‖ σ₂ ‖ ... ‖ σₖ) < target

  validity: each sᵢ has valid zheng proof σᵢ
  difficulty: aggregate hash below target
  reward: block_subsidy + Σ Δπ(sᵢ)
```

multiple signals per block means miners can aggregate knowledge from multiple neurons. a miner-pool collects signals from many neurons, proves them all, and splits rewards.

## what changes

the GFP page describes the benchmark circuit as four phases mimicking real workloads. unified mining removes the mimicry. the phases ARE the workloads:

| GFP benchmark phase | unified mining equivalent |
|---|---|
| Phase 1: matrix-vector (fma) | tri-kernel impulse SpMV |
| Phase 2: NTT polynomial (ntt) | algebraic NMT Lens openings |
| Phase 3: Poseidon2 hashing (p2r) | Hemera content addressing + Fiat-Shamir |
| Phase 4: lookup table (lut) | activation + threshold checks |

the chip specification does not change. the economic model changes: every hash cycle produces real knowledge instead of synthetic proof-of-capability.

see [[Goldilocks field processor]] for chip specification and flywheel economics. see [[cyber/nomics]] for reward mechanics. see [[foculus]] for how φ* determines finality. see [[cyber/research/provable consensus]] for how the global tri-kernel fits in zheng. see [[cyber/research/algorithmic essence of superintelligence]] for the full 16-component architecture
