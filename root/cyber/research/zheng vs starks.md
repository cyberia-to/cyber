---
tags: cyber, research, article
crystal-type: article
crystal-domain: cyber
date: 2026-03-23
---

# zheng vs STARKs: honest comparison

zheng-2 claims 30-150× smaller proofs, 20-100× faster verification, 2,300× cheaper recursion. are these real? what is the tradeoff? this document compares zheng-2 targets against production STARK systems with real numbers.

## the landscape (early 2026)

| system | field | IOP | PCS | hash | proof size | verify | prover | status |
|---|---|---|---|---|---|---|---|---|
| Stwo (StarkWare) | M31 (31-bit) | FRI | FRI+Merkle | Poseidon2/M31 | ~200 KiB | 10-50 ms | O(N log N) | production (Starknet) |
| Plonky3 (Polygon) | [[Goldilocks field\|Goldilocks]] (64-bit) | FRI | FRI+Merkle | Poseidon2/Gold | 100-200 KiB | 10-50 ms | O(N log N) | production (Polygon zkEVM) |
| SP1 (Succinct) | BabyBear (31-bit) | FRI | FRI | Poseidon2/BB | 100-200 KiB | 10-50 ms | O(N log N) | production (zkVM) |
| RISC Zero | BabyBear (31-bit) | FRI | FRI+Merkle | SHA-256 | 100-200 KiB | 10-50 ms | O(N log N) | production (Bonsai) |
| Binius (Irreducible) | F₂ tower | sumcheck | binary RS | external | workload-dep | workload-dep | O(N) | research/early |
| zheng-1 (Whirlaway) | Goldilocks | SuperSpartan | [[WHIR]] | [[Hemera]] | 157 KiB | 1.0 ms | O(N log N) | spec complete |
| zheng-2 (target) | Goldilocks + F₂ | SuperSpartan + HyperNova | WHIR/Brakedown + Binius | Hemera | 1-5 KiB | 10-50 μs | O(N) streaming | spec draft |

## where zheng-2 claims advantage

### 1. proof size: 30-150× smaller

```
FRI-based (Stwo, Plonky3, SP1):  100-200 KiB
WHIR (zheng-1):                   157 KiB
zheng-2 phase 1 (algebraic extraction):  5-12 KiB
zheng-2 phase 6 (Brakedown):     1-5 KiB
```

the improvement comes from two mechanisms:
- algebraic extraction: replace Merkle paths (log n × 32 bytes) with PCS openings (~200 bytes). this is a real, well-understood optimization. the tradeoff: PCS must be sound (computational assumption)
- Brakedown: expander-graph codes eliminate Merkle trees entirely. proof size is O(√N) field elements

honest assessment: 5-12 KiB is achievable with algebraic extraction on WHIR. 1-5 KiB requires Brakedown, which is newer and less battle-tested. FRI systems could also adopt algebraic extraction but haven't — their architecture is committed to Merkle-based FRI.

### 2. verification: 20-100× faster

```
FRI verify:    10-50 ms (walk Merkle paths, check polynomial evaluations)
WHIR verify:   1.0 ms (fewer rounds, hemera-native Merkle)
zheng-2:       10-50 μs (gravity-weighted: verify important particles first)
```

gravity-commitment: verification cost scales with π (importance). a light client verifying the top-100 particles spends most work on the highest-focus ones and O(1) on peripheral particles. this is a novel idea — no other system has importance-weighted verification.

honest assessment: the 10-50 μs target requires gravity-commitment + Brakedown + optimized Hemera. each is individually reasonable. the composition is unproven. 100-500 μs is more realistic near-term.

### 3. prover: O(N) vs O(N log N)

```
FRI-based: O(N log N) — FFT/NTT is the bottleneck
zheng-2:   O(N) streaming — sumcheck doesn't need FFT
```

SuperSpartan's sumcheck is inherently O(N): bookkeeping tables halve each round, total work = N + N/2 + N/4 + ... = 2N. no FFT. the prover streams through the trace once.

honest assessment: this is real. sumcheck IS linear. the constant factor matters — zheng-2's O(N) may be slower than Stwo's O(N log N) with highly optimized FFT in practice. but asymptotically, sumcheck wins. at N > 2³⁰, the gap is significant.

### 4. recursion: 2,300× cheaper

```
FRI recursion:  verify FRI in-circuit → ~200K-500K constraints per step
zheng-1:        verify WHIR in-circuit → ~70K constraints
zheng-2:        HyperNova folding → ~30 field operations per step
```

HyperNova folding is fundamentally different from recursive verification. instead of proving "the verifier accepted," you fold: accumulate the claim into a running accumulator with O(1) field operations. verification happens only at the END — one final proof covers all folded steps.

honest assessment: HyperNova folding at 30 field ops is real (published, peer-reviewed). the 2,300× over zheng-1 is correct math. the comparison to FRI recursion is even more dramatic — 500K constraints vs 30 ops. this is the single biggest advantage.

### 5. memory: O(√N) vs O(N)

```
FRI prover:  O(N) memory — must hold full trace + FFT butterfly
zheng-2:     O(√N) via tensor compression — stream and discard
```

tensor compression (accepted prop): the prover processes √N-sized blocks, streaming the trace in passes. total work remains O(N) but peak memory is O(√N).

honest assessment: this enables mobile proving (phone with 4 GB RAM can prove traces that need 64 GB on standard provers). real advantage for cyber's use case (neurons on phones).

### 6. dual algebra: 32-64× for binary workloads

```
AND gate in Goldilocks:  ~32 constraints (range decomposition)
AND gate in F₂:          1 constraint (native)
XOR gate:                same ratio
```

zheng-2 uses Binius for F₂ workloads (bitwise ops, quantized neural nets, hash evaluation). cross-algebra boundary: ~766 F_p constraints per crossing.

honest assessment: Binius advantage for binary is real and well-understood. Stwo/SP1 on 31-bit fields are better than Goldilocks for binary (smaller field = less waste) but still 16-32× worse than native F₂. zheng-2's dual approach is architecturally correct.

## where zheng-2 is at disadvantage

### 1. maturity

| system | production since | audits | bugs found/fixed |
|---|---|---|---|
| Stwo | 2024 (Starknet) | multiple | hundreds |
| Plonky3 | 2023 (Polygon) | multiple | hundreds |
| SP1 | 2024 | multiple | dozens |
| zheng-1 | spec only | 0 | 0 |
| zheng-2 | draft | 0 | 0 |

zheng has zero production hours. zero audits. zero bugs found because zero code. the first production bug in zheng could be catastrophic. Stwo/Plonky3 have years of hardening.

this is the biggest real risk. not the algorithms — the implementation.

### 2. hash cost

[[Hemera]] has 16 elements state width (vs Plonky3's 12) and 64 partial rounds (vs 22). this is 3.2× more expensive per hash. the reason: 256-bit capacity for permanent-grade security (content addressing must last centuries, not years).

| hash | state width | partial rounds | constraints per perm | security margin |
|---|---|---|---|---|
| Poseidon2/Plonky3 | 12 | 22 | ~230 | standard (128-bit) |
| Poseidon2/Stwo | 16 | 14 | ~200 | standard (128-bit) |
| Hemera | 16 | 64 | ~736 | permanent (256-bit, margin 2^918) |

zheng pays 3.2× per hash for permanent security. every Merkle path, every Fiat-Shamir squeeze, every content address costs more. this is a deliberate tradeoff — not a weakness, but it is a cost.

### 3. field size

Goldilocks (64-bit) vs M31/BabyBear (31-bit). smaller fields = faster multiplication (31-bit fits in hardware multiply natively on ARM). for pure hashing throughput:

```
Stwo/M31:     ~620K hashes/sec (M3 Pro)
Plonky3/Gold: ~1.7M hashes/sec (M3 Max)
Hemera/Gold:  ~62 MB/s throughput
```

31-bit fields are 2-4× faster for raw field ops on commodity hardware. zheng-2 compensates with [[Goldilocks field processor|GFP]] — custom silicon where the 64-bit field is native. without GFP, zheng is slower per-field-op than 31-bit systems.

### 4. single implementation

Stwo has multiple independent implementations (StarkWare, Polygon forks). Plonky3 is open-source with community forks. SP1 and RISC Zero have independent reimplementations.

zheng has one planned implementation. single point of failure. a consensus bug in zheng has no second implementation to compare against.

## the honest summary

| dimension | zheng-2 vs SOTA | real? | risk |
|---|---|---|---|
| proof size | 30-150× smaller | achievable with algebraic extraction + Brakedown | Brakedown maturity |
| verification | 20-100× faster | gravity-commitment is novel, unproven at scale | composition complexity |
| prover time | O(N) vs O(N log N) | real (sumcheck is linear) | constant factor may offset |
| recursion | 2,300× cheaper | real (HyperNova published) | cross-algebra boundary cost |
| memory | O(√N) vs O(N) | real (tensor compression) | streaming overhead |
| binary | 32-64× native | real (Binius) | dual-PCS complexity |
| maturity | 0 vs years | not a claim — a fact | highest risk |
| hash cost | 3.2× more | deliberate (permanent security) | GFP compensates |
| ecosystem | 1 impl vs many | not a claim — a fact | single point of failure |

## what makes zheng-2 structurally different

the existing STARK landscape shares one architecture: FRI-based univariate polynomial commitment with Merkle trees. Stwo, Plonky3, SP1, RISC Zero — all variations on the same theme. they differ in field choice, hash function, and engineering quality. but the core is identical: FFT + FRI + Merkle.

zheng-2 is a different architecture:

- multilinear (not univariate) — SuperSpartan operates on multilinear extensions, not univariate polynomials. no FFT needed
- sumcheck (not FRI) — the IOP is sumcheck protocol, inherently O(N). FRI is O(N log N)
- folding (not recursive verification) — HyperNova accumulates obligations without proving them until the end. recursion cost drops from O(verifier_circuit) to O(1)
- dual PCS — two backends (WHIR/Brakedown for Goldilocks, Binius for F₂) instead of one
- unified hash — Hemera serves ALL functions (Merkle, Fiat-Shamir, content addressing). other systems use different hashes for different purposes

this is not "a better STARK." it is a different proof architecture that happens to achieve the same goal (transparent, post-quantum, scalable verification). the comparison is like comparing a jet engine to a propeller — both fly, but the mechanisms differ.

## what this enables that FRI cannot

1. [[provable consensus]]: tri-kernel computation (1.42B constraints) fits in zheng at 33% capacity. FRI systems would need ~4× larger proof (O(N log N) vs O(N)), making the same circuit overflow

2. [[algebraic state commitments]]: polynomial state reads are O(1) field ops in zheng's sumcheck. in FRI, polynomial state reads still require Merkle paths

3. mobile proving: O(√N) memory means a phone can prove what a server proves. FRI's O(N) memory requires the full trace in RAM

4. zero-latency recursion: HyperNova folding during execution means the proof is complete when the program finishes. FRI recursion adds a separate proving step after execution

5. [[unified mining]]: signal proof exercises the same primitives as the consensus computation. FRI's FFT-heavy prover would bias mining hardware toward NTT — zheng's sumcheck-based prover is balanced across all four GFP primitives

see [[stark]] for the proof taxonomy. see [[zheng]] for the full specification. see [[Hemera]] for the hash function. see [[cyber/research/provable consensus]] for the consensus circuit. see [[Goldilocks field processor]] for custom hardware. see [[cyber/research/algebraic state commitments]] for polynomial state. see [[cyber/research/unified mining]] for proof-as-mining
