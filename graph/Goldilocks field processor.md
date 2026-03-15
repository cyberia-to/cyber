---
tags: trident, cyber, article
alias: GFP, Goldilocks Field Processor, AURUM, gfp spec, gfp-spec
crystal-type: article
crystal-domain: cyber
stake: 9519611796818916
---
# The Goldilocks Field Processor

## Hardware Specification, Proof of Useful Work, and Unified Economics

*"The miner IS the prover. The puzzle IS the workload. The chip IS the product."*

---

## The Core Idea in 30 Seconds

Every useful operation in nox — block proving, focus computation, private transactions, FHE bootstrapping, neural inference — reduces to four primitives over one field. A chip optimized for these four primitives accelerates everything simultaneously. The Proof of Work puzzle requires producing stark proofs using exactly these primitives. Therefore: the optimal mining hardware IS the optimal utility hardware. Mining rewards bootstrap chip development. Chip development accelerates the network. The network generates fees. Fees replace mining rewards. The flywheel self-sustains.

```
┌─────────────────────────────────────────────────────────┐
│                    THE FLYWHEEL                          │
│                                                         │
│   Mining rewards → Fund GFP development                 │
│        ↑                    ↓                           │
│   Network grows    GFP accelerates proving              │
│        ↑                    ↓                           │
│   Users pay fees ← Proving serves users                 │
│                                                         │
│   Same chip. Same operations. Two revenue streams.      │
└─────────────────────────────────────────────────────────┘
```

---

# Part I: The Four Primitives

## 1. Why Four and Only Four

Every computation in the nox stack reduces to a small set of operations over the [[Goldilocks field]] $p = 2^{64} - 2^{32} + 1$. By profiling every workload — [[stark]] proving, BBG authentication, [[tri-kernel]] ranking, private transfers, FHE bootstrapping, neural inference — we find four primitive families that account for >95% of all cycles:

| Primitive | Symbol | What it computes | % of typical workload |
|-----------|--------|-----------------|----------------------|
| Field MAC | `fma` | $c \leftarrow c + a \times b \bmod p$ | ~40% |
| [[NTT]] butterfly | `ntt` | Paired multiply-add with twiddle factor | ~35% |
| [[Poseidon2]] round | `p2r` | Full-state permutation (MDS + S-box) | ~15% |
| Table lookup | `lut` | $y \leftarrow T[x]$ with authentication | ~10% |

These are not design choices — they are what survives when you ask "what operations does every workload need?" The answer is always: modular arithmetic, polynomial transforms, algebraic hashing, and nonlinear function evaluation.

### 1.1 Who Needs What

```
                          fma    ntt    p2r    lut
                         ─────  ─────  ─────  ─────
stark proving (WHIR)       ██     ███    ██     █
BBG authentication         █      █      ███    
Tri-kernel focus           ███    ██     █      
Private transfer (ZK)      ██     █      ███    █
FHE bootstrapping (PBS)    ██     ███    █      ██
Neural network inference   ███    ██            ██
Quantum simulation         ██     ███    
Block production           ██     ██     ███    █

█ = light use   ██ = medium   ███ = dominant
```

Every workload uses at least three of four primitives. No workload uses only one. A chip optimized for all four accelerates everything; a chip missing any one primitive bottlenecks critical workloads.

### 1.2 Why Not Just GPUs

GPUs optimize for IEEE 754 floating point. Goldilocks field arithmetic wastes GPU transistors:

- Float mantissa logic: 52-bit mantissa handling is irrelevant for 64-bit modular arithmetic. ~30% wasted silicon.
- Denormal/NaN/Inf handling: Entire circuits for edge cases that never occur in field arithmetic. ~5% wasted.
- Exponent processing: 11-bit exponent path unused. ~10% wasted.
- Rounding modes: 4 IEEE rounding modes, none applicable. ~3% wasted.

Net result: a GPU is ~50% efficient for Goldilocks work. A purpose-built GFP is 100% efficient — same transistor budget, 2× throughput, lower power.

Additionally, GPUs lack native support for the Goldilocks reduction trick: since $p = 2^{64} - 2^{32} + 1$, modular reduction is `a_lo - a_hi × (2³² - 1)` — two 64-bit ops instead of division. This can be hardwired into a GFP as a single-cycle operation; on GPU it's 4-6 instructions.

### 1.3 Why Not FPGAs

FPGAs are the right prototyping platform but wrong production target:

- 10-50× less energy-efficient than ASICs for fixed operations
- The operation set is provably stable (see §1.4) — no need for reconfigurability
- Cost per unit 100-1000× higher than mass-produced ASICs

Recommendation: FPGA for GFP v0 prototyping, ASIC for GFP v1 production.

### 1.4 Stability Proof

Why won't the instruction set change?

The four primitives are mathematically necessary:

1. fma: Field arithmetic IS the computation model. nox's 16 patterns reduce to field ops. This cannot change without changing the field — which would break every existing proof, commitment, and hash. The field is a genesis parameter.

2. ntt: [[NTT]] is the fast path for polynomial multiplication in $R_p$. Polynomial multiplication is required by [[stark]] ([[WHIR]]), FHE (CMUX), convolution (AI), and QFT (quantum). The Cooley-Tukey butterfly is the optimal algorithm for power-of-2 NTT since 1965. This cannot improve asymptotically.

3. p2r: Algebraic hashing over $\mathbb{F}_p$ requires a permutation with high algebraic degree. [[Poseidon2]] MDS matrix + $x^7$ S-box is the current optimal choice. Even if the hash function changes (Poseidon3, Griffin, Anemoi), the hardware primitive is the same: full-width permutation over $\mathbb{F}_p^t$ with a power-map nonlinearity. The round function hardware is parametrizable.

4. lut: Lookup tables are required for any non-polynomial function: neural network activations, cryptographic S-boxes, FHE test polynomials, comparison operations. The lookup mechanism is universal — only the table contents change. Hardware stores table values; software selects which table.

Conclusion: The four primitives will remain correct for any field-first computation over Goldilocks for as long as:
- The Goldilocks field remains secure (lattice/factoring hardness: decades)
- starks remain the proof system family (hash-based: quantum-resistant)
- Polynomial operations remain O(n log n) via NTT (information-theoretic lower bound)

This is sufficient stability to justify ASIC investment.

---

# Part II: GFP Architecture

## 2. Hardware Specification

### 2.1 Top-Level Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    GOLDILOCKS FIELD PROCESSOR                     │
│                         GFP-1 (codename: AURUM)                  │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                  FMA ARRAY (256 units)                      │  │
│  │                                                            │  │
│  │  Unit: c ← c + a × b mod p    Latency: 1 cycle            │  │
│  │  Reduction: hardwired sparse   Throughput: 256 ops/cycle   │  │
│  │  Grouping: 16 clusters × 16   Local register file: 32 F_p │  │
│  │                                                            │  │
│  │  Modes:                                                    │  │
│  │    F_p:    standard field MAC                              │  │
│  │    F_p²:   complex MAC (2 units cooperate)                 │  │
│  │    batch:  SIMD across 16 independent lanes                │  │
│  └─────────────────────────┬──────────────────────────────────┘  │
│                            │ crossbar                             │
│  ┌──────────────┐  ┌──────┴──────┐  ┌────────────────────────┐  │
│  │  NTT ENGINE  │  │  POSEIDON2  │  │    LOOKUP ENGINE       │  │
│  │              │  │  PIPELINE   │  │                        │  │
│  │  Butterfly:  │  │             │  │  Tables: 4 × 64K      │  │
│  │   2^15 pt    │  │  Width: 12  │  │  (configurable)       │  │
│  │   in-place   │  │  S-box: x^7 │  │                        │  │
│  │              │  │  Rounds: 22 │  │  Modes:                │  │
│  │  Twiddle     │  │  (8 full +  │  │   direct: y = T[x]    │  │
│  │  ROM:        │  │   14 partial│  │   authed: y = T[x]    │  │
│  │   precomputed│  │  )          │  │     + LogUp accum     │  │
│  │   roots of   │  │             │  │   batch: vectorized   │  │
│  │   unity      │  │  Throughput:│  │     across clusters   │  │
│  │              │  │  1 perm/    │  │                        │  │
│  │  Throughput: │  │  22 cycles  │  │  Throughput:           │  │
│  │   full NTT   │  │  = ~12M/s  │  │   256 lookups/cycle   │  │
│  │   in ~32K    │  │             │  │                        │  │
│  │   cycles     │  │  Pipeline:  │  │  LogUp accumulator:   │  │
│  │              │  │   4-deep    │  │   hardware running sum │  │
│  └──────────────┘  └─────────────┘  └────────────────────────┘  │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    MEMORY HIERARCHY                         │  │
│  │                                                            │  │
│  │  L0 (on-die SRAM):                                        │  │
│  │    Twiddle factor ROM: 256 KB (precomputed ω^k for NTT)   │  │
│  │    Lookup tables: 4 × 512 KB = 2 MB (active tables)       │  │
│  │    FMA register file: 16 clusters × 32 × 8B = 4 KB        │  │
│  │                                                            │  │
│  │  L1 (on-die SRAM): 8 MB                                   │  │
│  │    NTT workspace (2^15 elements = 256 KB per transform)    │  │
│  │    Poseidon2 state buffer                                  │  │
│  │    Merkle path cache (hot BBG paths)                       │  │
│  │                                                            │  │
│  │  L2 (HBM interface): 8-16 GB                              │  │
│  │    Full execution trace buffer                             │  │
│  │    Polynomial commitment workspace                         │  │
│  │    Graph adjacency (hot partition)                          │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    CONTROL                                 │  │
│  │                                                            │  │
│  │  Instruction decoder: 4 opcodes (fma, ntt, p2r, lut)      │  │
│  │  + memory ops (load, store, fence)                         │  │
│  │  + control flow (branch, call, halt)                       │  │
│  │  Scheduler: out-of-order within cluster, in-order across   │  │
│  │  DMA: streaming load/store for trace data                  │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

### 2.2 Instruction Set: GFP-ISA

Exactly 10 instructions. Nothing more.

```
FIELD ARITHMETIC (4 instructions)
──────────────────────────────────
FMA   rd, ra, rb, rc    │ rd ← rc + ra × rb mod p     │ 1 cycle
FRED  rd, ra            │ rd ← reduce(ra)  (128→64)   │ 1 cycle  
FINV  rd, ra            │ rd ← ra^(p-2) mod p         │ ~62 cycles (Fermat chain)
FCMP  rd, ra, rb        │ rd ← (ra < rb) ? 1 : 0      │ 1 cycle

TRANSFORM (2 instructions)
──────────────────────────────────
NTT   base, log_n, dir  │ In-place NTT at base addr    │ ~N/2·log(N) cycles
NTTU  base, log_n       │ NTT + pointwise multiply     │ Fused NTT-mul-iNTT

HASH (1 instruction)
──────────────────────────────────
P2R   base, count       │ Poseidon2 permutation(s)     │ 22 cycles / permutation

LOOKUP (1 instruction)
──────────────────────────────────
LUT   rd, ra, table_id  │ rd ← T[ra], accumulate LogUp │ 1 cycle

MEMORY (2 instructions)
──────────────────────────────────
LD    rd, [addr]        │ Load F_p from memory          │ 1-N cycles (cache dependent)
ST    [addr], rs        │ Store F_p to memory           │ 1-N cycles
```

Design principles:
- Every instruction operates on $\mathbb{F}_p$ elements, not bytes
- No integer arithmetic — everything is modular
- No float — no IEEE 754 logic whatsoever
- `FMA` is the universal primitive — multiplication is always fused with addition
- `NTT` is a block instruction (like GPU warp ops) — triggers the butterfly network
- `P2R` is pipelined — multiple permutations overlap in the Poseidon2 pipeline
- `LUT` accumulates LogUp authentication automatically in hardware — every lookup is proof-ready

### 2.3 Key Parameters

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| FMA units | 256 | 16 clusters × 16 units. Matches typical stark trace width |
| Clock target | 1-2 GHz | Conservative for 7nm/5nm process |
| NTT max size | $2^{15}$ in-place | Covers TFHE (N=2048), WHIR layer sizes |
| Poseidon2 width | 12 F_p elements | Standard Poseidon2 state (t=12) |
| Poseidon2 throughput | ~12M perms/sec | At 1.5 GHz: 1.5G/22 cycles × pipeline depth 4 |
| Lookup tables | 4 active, 64K entries each | ReLU, sigmoid, S-box, custom — hot-swappable |
| L1 SRAM | 8 MB | Holds full NTT workspace + Merkle cache |
| HBM | 8-16 GB | Full execution trace for large proofs |
| TDP target | 75-150W | PCIe card form factor |
| Die size target | ~200mm² | 7nm, competitive with mid-range GPU |

### 2.4 Performance Estimates

Based on 256 FMA units at 1.5 GHz:

| Workload | CPU (Ryzen 9) | GPU (RTX 4090) | GFP-1 | Speedup vs CPU |
|----------|---------------|----------------|-------|----------------|
| [[stark]] prove (1M constraints) | ~10 sec | ~2 sec | ~0.2 sec | 50× |
| [[Poseidon2]] hash (1M inputs) | ~15 ms | ~3 ms | ~0.08 ms | 180× |
| [[NTT]] $2^{20}$ | ~50 ms | ~5 ms | ~0.7 ms | 70× |
| [[TFHE]] bootstrap (PBS) | ~20 ms | ~4 ms | ~0.4 ms | 50× |
| Neural inference (MNIST enc) | ~60 sec | ~10 sec | ~1 sec | 60× |
| [[tri-kernel]] [[focus]] (10K nodes) | ~100 ms | ~15 ms | ~1.5 ms | 65× |

These are conservative estimates assuming 50% utilization. Real workloads with tuned scheduling should achieve 70-80% utilization.

### 2.5 Form Factors

```
GFP-1 PCIe    │ Full card, 150W TDP, 16 GB HBM    │ Validator / Prover node
GFP-1 M.2     │ M.2 2280 form factor, 25W TDP      │ Desktop / Laptop miner
GFP-1 SoC     │ ARM core + GFP on same die, 10W    │ Mobile / IoT node
GFP-1 USB     │ USB-C dongle, 5W                    │ Light client accelerator
```

Multiple form factors enable the participation spectrum from phone miners to datacenter provers — crucial for decentralization (§4).

---

# Part III: Proof of Useful Work

## 3. The PoUW Scheme

### 3.1 The Central Insight

Traditional PoW: the puzzle is unrelated to useful computation (SHA-256 partial preimage). Energy is wasted. Hardware is single-purpose.

nox PoUW: the puzzle IS a [[stark]] proof. stark proving requires exactly the four GFP primitives (fma, ntt, p2r, lut) in exactly the proportions of real workloads. Therefore:

- Optimizing for mining = optimizing for utility
- Mining hardware = proving hardware
- Mining energy = proving energy (not wasted)

The trick is designing the puzzle so that:
1. It cannot be solved without exercising all four primitives
2. The primitive ratios match real workload ratios
3. Solutions are quickly verifiable
4. The puzzle is progress-free (memoryless) for fair mining
5. Solutions are not reusable (no proof recycling)

### 3.2 The Benchmark Circuit

The PoUW puzzle requires producing a valid stark proof of a specific benchmark circuit $\mathcal{B}$. The circuit is designed to exercise all four GFP primitives in production-representative proportions.

```
BENCHMARK CIRCUIT B(challenge, nonce) → digest
═══════════════════════════════════════════════

INPUT:
  challenge : 4 × F_p    (from block header, public)
  nonce     : 2 × F_p    (miner's search variable)

PHASE 1: FIELD ARITHMETIC (40% of constraints)
────────────────────────────────────────────────
  // Matrix-vector product simulating tri-kernel focus step
  // Uses same dimensions as real focus computation
  
  state ← challenge
  for round in 0..R_fma:
    state ← M × state + bias          // 12×12 matrix over F_p
    state[0] ← state[0] + nonce[0]    // nonce injection
  
  // This exercises FMA units in the exact pattern of
  // tri-kernel diffusion computation

PHASE 2: NTT POLYNOMIAL OPERATIONS (35% of constraints)
────────────────────────────────────────────────────────
  // Polynomial multiplication simulating WHIR folding
  
  poly_a ← encode_as_polynomial(state, degree=N)
  poly_b ← encode_as_polynomial(state ⊕ challenge, degree=N)
  poly_c ← NTT_multiply(poly_a, poly_b)    // Forward NTT, pointwise, inverse NTT
  
  // WHIR-style folding
  for layer in 0..log(N):
    poly_c ← fri_fold(poly_c, challenge_hash(layer))
  
  // This exercises NTT engine in the exact pattern of
  // stark WHIR commitment + FHE polynomial multiply

PHASE 3: POSEIDON2 HASHING (15% of constraints)
────────────────────────────────────────────────
  // Merkle tree construction simulating BBG authentication
  
  leaves ← [poly_c[i] for i in 0..TREE_SIZE]
  root ← build_merkle_tree(leaves, hash=Poseidon2)
  
  // Chain hash for final mixing
  digest ← Poseidon2(root || state || nonce)
  
  // This exercises Poseidon2 pipeline in the exact pattern of
  // BBG Merkle tree construction + proof hashing

PHASE 4: LOOKUP TABLE (10% of constraints)
──────────────────────────────────────────
  // Table evaluations simulating NN activation + FHE PBS
  
  for i in 0..R_lut:
    state[i % 12] ← T_relu[state[i % 12]]     // ReLU table
    state[(i+1) % 12] ← T_sbox[state[(i+1) % 12]]  // S-box table
  
  // Mix into digest
  digest ← Poseidon2(digest || state)
  
  // This exercises lookup engine in the exact pattern of
  // neural network activation + Poseidon2 S-box

OUTPUT:
  digest : 4 × F_p

PUZZLE CONDITION:
  digest < target    (standard partial preimage)
```

Why each phase is necessary:
- Remove Phase 1 → chip without FMA array. Cannot do matrix operations → useless for tri-kernel, neural nets.
- Remove Phase 2 → chip without NTT. Cannot do polynomial ops → useless for stark proving, FHE.
- Remove Phase 3 → chip without Poseidon2. Cannot hash → useless for any authentication.
- Remove Phase 4 → chip without lookup. Cannot do activations → useless for AI and FHE bootstrapping.

A chip that solves the puzzle efficiently MUST have all four units in roughly the right proportions. There is no shortcut that skips any phase because the phases are data-dependent — Phase 2's input depends on Phase 1's output, Phase 3 depends on Phase 2, Phase 4 depends on Phase 3, and the final digest depends on all four.

### 3.3 The Proof-of-Proof Structure

The miner doesn't just find a nonce where digest < target. The miner produces a stark proof that the benchmark circuit was evaluated correctly.

```
MINING STEP:
  1. Receive challenge from latest block header
  2. Try nonce values until digest < target
  3. For the winning nonce, generate stark proof π:
       π proves "B(challenge, nonce) = digest AND digest < target"
  4. Submit (nonce, π) as proof of work

VERIFICATION (by any node):
  1. Check π is a valid stark proof (O(log n) time, ~100K constraints)
  2. Check public inputs match (challenge from block header, digest < target)
  3. Done. No re-execution of B needed.
```

Why proof-of-proof, not just proof-of-evaluation:

The stark proof π itself requires producing an execution trace, committing it via WHIR (NTT-heavy), hashing with Poseidon2, and verifying lookup arguments. The proof generation process exercises the same four primitives AGAIN, amplifying the useful-work requirement.

Verification is O(log n) — any light client can verify in milliseconds. This satisfies compute-verify symmetry.

### 3.4 Difficulty Adjustment

```
DIFFICULTY PARAMETERS:
  target           : F_p threshold (lower = harder)
  R_fma            : Number of FMA rounds (scales Phase 1 cost)
  N                : NTT degree (scales Phase 2 cost)
  TREE_SIZE        : Merkle tree leaves (scales Phase 3 cost)
  R_lut            : Lookup rounds (scales Phase 4 cost)

ADJUSTMENT RULE (per epoch = 720 blocks ≈ 12 hours):
  Adjust target to maintain constant block time (10 sec target).
  
  Additionally, every 10 epochs (~5 days):
    Measure actual primitive utilization ratios from on-chain proofs.
    Adjust R_fma, N, TREE_SIZE, R_lut to keep ratios at 40:35:15:10.
    
  This prevents miners from building chips that over-provision one unit
  and under-provision others — the puzzle adapts to match utility ratios.

RATIO ENFORCEMENT:
  If miners collectively shift toward NTT-heavy solutions:
    → increase R_fma (more field arithmetic needed)
    → decrease N (less NTT headroom)
  Effect: rebalances toward utility-representative ratios
  
  The network's puzzle mirrors the network's actual workload distribution.
```

### 3.5 Progress-Freedom and Fairness

Progress-freedom: The puzzle is memoryless — each nonce attempt has identical probability of success regardless of previous attempts. This ensures small miners earn proportionally to their hashrate (no pool requirement for variance reduction).

Proof: The final digest is $\text{Poseidon2}(\ldots || \text{nonce})$. Poseidon2 is a pseudorandom permutation. For uniformly random nonce, the digest is uniformly distributed in $\mathbb{F}_p^4$. The probability $\text{digest} < \text{target}$ is $\text{target}/p^4$, independent of all previous attempts. QED.

Non-reusability: Each proof is bound to a specific block challenge (derived from the previous block hash). A proof generated for block $n$ cannot be submitted for block $n+1$ because the challenge changes. No proof stockpiling.

### 3.6 Anti-Gaming Analysis

| Attack | Defense |
|--------|---------|
| Skip Phase 1 (no FMA) | Phase 2 input depends on Phase 1 output. Invalid trace → invalid stark |
| Skip Phase 2 (no NTT) | Phase 3 input depends on Phase 2 output. Plus: stark proof itself requires NTT |
| Precompute tables | Tables are parameterized by challenge — change every block |
| Outsource proof generation | Proof is bound to miner's identity (coinbase). Outsourcing = giving away rewards |
| Recycle old proofs | Challenge includes prev_block_hash. Every block requires fresh proof |
| Shortcut stark proof | stark soundness: forging a proof requires breaking collision resistance of Poseidon2 |
| Unbalanced chip (all NTT, no FMA) | Ratio adjustment (§3.4) penalizes imbalanced architectures |
| FPGA/GPU competition | GFP has 2× efficiency advantage (§1.2). FPGA/GPU can participate but earn less per watt |

---

# Part IV: Unified Economics

## 4. Two Revenue Streams, One Chip

### 4.1 Supply Side: Mining

Miners produce stark proofs of the benchmark circuit. Valid proofs earn block rewards.

```
BLOCK STRUCTURE:
┌──────────────────────────────────────┐
│ Block Header                         │
│   prev_hash      : H(prev_block)     │
│   state_root     : BBG root          │
│   timestamp      : unix time         │
│   pow_challenge  : H(prev_hash||h)   │
│   pow_nonce      : F_p × 2           │
│   pow_proof      : stark proof       │
│   pow_digest     : 4 × F_p           │
│   difficulty     : target threshold  │
│   miner          : [[neuron]] address│
│                                      │
│ Body                                 │
│   transactions   : [cyberlink, ...]  │
│   focus_updates  : [Δπ, ...]        │
│   fee_proofs     : [stark, ...]      │
└──────────────────────────────────────┘

REWARD:
  block_reward = base_emission(epoch) + Σ(transaction_fees)
  
  base_emission follows halving schedule:
    Year 1-2:  1000 FOCUS / block
    Year 3-4:   500 FOCUS / block
    Year 5-8:   250 FOCUS / block
    Year 9+:    fees only (pure utility)
```

### 4.2 Demand Side: Proving-as-a-Service

The same GFP that mines also serves users by proving their transactions.

```
USER TRANSACTION FLOW:
  1. User creates cyberlink/transfer/query
  2. User broadcasts unsigned transaction to mempool
  3. Prover node picks up transaction
  4. GFP generates stark proof of transaction validity
  5. Prover includes proven transaction in block
  6. User pays fee → prover earns fee

PROVING COSTS (GFP-1 estimates):
  Cyberlink (1 edge):           ~12K constraints  →  ~2 ms  →  ~0.001 FOCUS fee
  Private transfer (4-in-4-out): ~50K constraints  →  ~8 ms  →  ~0.005 FOCUS fee
  Focus update (local):         ~10K constraints   →  ~1.5 ms → ~0.001 FOCUS fee
  FHE bootstrap (1 PBS):        ~500K constraints  →  ~80 ms →  ~0.05 FOCUS fee
  Neural inference (MNIST):     ~5M constraints    →  ~800 ms → ~0.5 FOCUS fee
```

### 4.3 The Economics of Dual Revenue

A GFP operator earns from both streams simultaneously:

```
MINER ECONOMICS (per GFP-1 card, Year 1):
  
  Mining revenue:
    Hashrate share: depends on network size
    Expected block reward: proportional to hashrate
    
  Proving revenue:
    Transactions proved: ~500/sec capacity
    Average fee: ~0.005 FOCUS
    Revenue: ~2.5 FOCUS/sec = ~216,000 FOCUS/day
    
  Total: mining_reward + proving_fees
  
  Cost:
    Hardware: $X (amortized over 3 years)
    Electricity: 150W × 24h × $0.05/kWh = $0.18/day
    Bandwidth: ~$0.50/day
    
  The chip pays for itself through utility even if mining rewards → 0.
  This is the key economic difference from Bitcoin ASICs.
```

Why this works: Bitcoin ASICs have zero utility beyond mining. When block rewards halve, miners' revenue halves and hardware becomes unprofitable. GFP hardware has perpetual utility — as long as the network has users, provers earn fees. Mining rewards bootstrap adoption; proving fees sustain it.

### 4.4 Participation Tiers

```
TIER 1: LIGHT CLIENT (phone, USB dongle)
  Hardware: GFP-1 USB (5W) or ARM SoC
  Role: Verify proofs, participate in DAS
  Revenue: None (consumer)
  Cost: ~$20-50 for USB dongle
  
TIER 2: HOME MINER (desktop, M.2 card)
  Hardware: GFP-1 M.2 (25W)
  Role: Mine blocks + prove personal transactions
  Revenue: Small mining rewards + self-service proving
  Cost: ~$100-200 for M.2 card
  Benefit: Don't pay proving fees to others
  
TIER 3: VALIDATOR (server, PCIe card)
  Hardware: GFP-1 PCIe (150W)
  Role: Mine blocks + prove transactions for others + validate
  Revenue: Mining rewards + proving fees + validation rewards
  Cost: ~$500-2000 for PCIe card
  
TIER 4: PROVING FARM (datacenter, multiple cards)
  Hardware: Multiple GFP-1 PCIe
  Role: High-throughput proving service
  Revenue: Primarily proving fees (scale advantage)
  Cost: Standard datacenter economics
```

### 4.5 Fee Market Dynamics

```
PROVING FEE EQUILIBRIUM:

  Supply: Aggregate GFP capacity (proofs/second)
  Demand: Transaction volume (transactions/second)
  
  When demand > supply:
    Fees rise → more miners → more GFP hardware sold → supply increases
    
  When supply > demand:
    Fees fall → marginal miners turn off → supply decreases
    Surviving miners still earn mining rewards as floor
    
  Equilibrium: fee ≈ electricity cost of proving + amortized hardware
  
  Over time, as hardware improves:
    Cost per proof decreases → fees decrease → more transactions affordable
    → larger network → more total fee revenue (volume effect)
    
  This is the same dynamic as bandwidth markets:
    cheaper per-unit → more units consumed → larger total market
```

---

# Part V: The Proof-of-Work ↔ Utility Isomorphism

## 5. Why This Is Not Just "Useful PoW"

Previous "useful PoW" proposals (Primecoin, Gridcoin, AI PoW) bolt useful computation onto mining as an afterthought. The useful work and the puzzle are separate — the puzzle provides security, the useful work provides PR.

nox's PoUW is fundamentally different: the puzzle and the utility are algebraically identical.

### 5.1 The Isomorphism

```
MINING OPERATION              ↔    UTILITY OPERATION
═══════════════                    ═══════════════════
Phase 1: Matrix-vector FMA    ↔    Tri-kernel focus step
Phase 2: NTT polynomial mul   ↔    WHIR commitment / FHE CMUX
Phase 3: Poseidon2 Merkle     ↔    BBG state authentication
Phase 4: Lookup evaluation    ↔    NN activation / PBS test poly
stark proof generation        ↔    Transaction proving
Difficulty adjustment         ↔    Workload-proportional scaling
```

Every mining operation has a direct utility analog. The hardware path is identical. The only difference is the input: mining uses a random challenge; utility uses a user transaction. Same chip, same code path, same power consumption.

### 5.2 Formal Statement

Theorem (PoUW-Utility Isomorphism): Let $\mathcal{H}_{\text{mine}}$ be the optimal hardware for minimizing PoUW puzzle solution time, and $\mathcal{H}_{\text{prove}}$ be the optimal hardware for minimizing stark proof generation time for nox transactions. Then $\mathcal{H}_{\text{mine}} = \mathcal{H}_{\text{prove}}$.

Proof sketch:
1. The PoUW puzzle requires producing a stark proof of the benchmark circuit $\mathcal{B}$.
2. $\mathcal{B}$ exercises the four primitives (fma, ntt, p2r, lut) in ratios matching real nox workloads.
3. stark proof generation for any circuit over $\mathbb{F}_p$ requires the same four primitives (trace computation uses fma/ntt/lut; proof commitment uses ntt; Fiat-Shamir uses p2r; lookup arguments use lut).
4. Optimizing for $\mathcal{B}$-proof-speed = optimizing for general stark-proof-speed over $\mathbb{F}_p$.
5. The ratio adjustment mechanism (§3.4) ensures the puzzle's primitive ratios track actual workload ratios.
6. Therefore the optimal puzzle-solving hardware is optimal utility hardware. QED.

### 5.3 What This Enables

Bootstrapping: Early network has few users → few fees. Mining rewards justify GFP development. As hardware is developed, proving capability increases. Increased capability attracts users. Users generate fees.

No stranded assets: Unlike Bitcoin ASICs that become e-waste when mining is unprofitable, GFP hardware retains value as proving infrastructure indefinitely.

Hardware market alignment: GFP manufacturers earn revenue from both miners (who want hashrate) and enterprises (who want proving throughput). Larger addressable market → more R&D investment → faster improvement.

Decentralization via utility: Home miners (Tier 2) can earn by proving their own transactions even when mining rewards are negligible. As long as they use the network, the hardware earns its keep.

---

# Part VI: Integration with nox

## 6. Block Production Flow

```
FULL BLOCK PRODUCTION CYCLE:
═══════════════════════════

  1. CHALLENGE DERIVATION
     challenge = Poseidon2(prev_block_hash || block_height || timestamp)
     // Deterministic, unpredictable

  2. TRANSACTION COLLECTION
     mempool_txs = collect_pending_transactions()
     // Prioritize by fee/proof-size ratio

  3. TRANSACTION PROVING (GFP utility workload)
     for tx in mempool_txs:
       proof_tx = GFP.prove(tx.circuit, tx.witness)
       // Each proof exercises all four GFP primitives
       // This IS useful work — it proves real transactions

  4. FOCUS COMPUTATION (GFP utility workload)
     Δπ = tri_kernel_step(current_graph, new_edges)
     proof_focus = GFP.prove(tri_kernel_circuit, Δπ)
     // Focus update is also proven via stark

  5. STATE COMMITMENT
     new_bbg_root = update_bbg(proven_txs, Δπ)
     // NMT updates, MMR appends, polynomial recommitments

  6. POW PUZZLE (GFP mining workload)
     loop:
       nonce = random()
       digest = B(challenge, nonce)    // Benchmark circuit
       if digest < target:
         pow_proof = GFP.prove(B_circuit, (challenge, nonce))
         break

  7. BLOCK ASSEMBLY
     block = {
       header: { prev_hash, new_bbg_root, timestamp,
                 challenge, nonce, pow_proof, digest, difficulty, miner },
       body: { proven_txs, proof_focus }
     }

  8. BROADCAST
     broadcast(block)
     // Any node verifies in O(log n) by checking pow_proof + tx proofs
```

Observation: Steps 3-4 produce useful proofs (transaction validity, [[focus]] correctness). Step 6 produces the PoW proof. ALL steps use the same GFP. The GFP is never idle — when not solving the PoW puzzle, it's proving transactions. When not proving transactions, it's solving the puzzle. The scheduler interleaves both workloads on the same hardware.

### 6.1 Interleaved Scheduling

```
GFP TIME ALLOCATION (typical validator):
═══════════════════════════════════════

  ┌─────────┬──────────┬─────────┬──────────┬─────────┐
  │  Prove  │  PoW     │  Prove  │  Focus   │  PoW    │  ...
  │  tx #1  │  attempt │  tx #2  │  update  │  attempt│
  │  8ms    │  12ms    │  5ms    │  2ms     │  12ms   │
  └─────────┴──────────┴─────────┴──────────┴─────────┘
  
  Transaction proving: ~40% of GFP time (earns fees)
  PoW attempts: ~50% of GFP time (earns block rewards)
  Focus computation: ~10% of GFP time (network obligation)
  
  Operator can tune allocation:
    High-fee environment → more time on transaction proving
    Low-fee environment → more time on PoW
    Both use the same hardware at 100% utilization
```

## 7. Relationship to Existing PoS

nox currently uses Tendermint PoS (via Bostrom). The GFP PoUW can integrate as a hybrid:

```
HYBRID PoS + PoUW:
═══════════════════

  PoS provides:
    - Fast finality (Tendermint BFT)
    - Validator set management
    - Slashing for misbehavior
    
  PoUW provides:
    - Fair token distribution (permissionless entry)
    - Hardware development incentive
    - Sybil resistance via physical cost
    - Proving capacity growth
    
  Integration:
    Validators are selected by stake (PoS).
    Validators must include PoUW proofs in blocks they produce.
    PoUW difficulty scales to maintain target proof rate.
    Block rewards split: X% to validator (PoS), Y% to prover (PoUW).
    
  Over time, as network matures:
    PoS handles consensus (who proposes blocks).
    PoUW handles resource commitment (who has proving capacity).
    Fees go to whichever layer does the work.
```

---

# Part VII: Development Roadmap

## 8. From Theory to Silicon

### Phase 0: Software Emulation (Now → 6 months)

```
Deliverables:
  - GFP-ISA emulator (Rust)
  - Benchmark circuit B implementation
  - PoUW puzzle solver (software, CPU)
  - Difficulty adjustment simulator
  - Economic model simulation

Purpose:
  - Validate ISA completeness
  - Tune benchmark circuit parameters
  - Test difficulty adjustment dynamics
  - Establish performance baselines
  
Cost: ~$50K-100K (engineering time)
```

### Phase 1: FPGA Prototype (6-18 months)

```
Deliverables:
  - GFP core on Xilinx Alveo U280 or similar
  - 16-32 FMA units (1/8 to 1/16 of full design)
  - NTT engine (2^12 in-place)
  - Poseidon2 pipeline (1 deep)
  - Lookup engine (1 table, 16K entries)
  - Performance benchmarks vs CPU/GPU

Purpose:
  - Validate architectural decisions
  - Identify bottlenecks
  - Produce real PoUW proofs
  - Enable early testnet mining
  
Hardware: ~$5K per FPGA board
Cost: ~$200K-500K (FPGA dev + engineering)
```

### Phase 2: ASIC Tape-Out (18-36 months)

```
Deliverables:
  - GFP-1 ASIC (7nm or 5nm)
  - Full 256 FMA array
  - Full NTT engine (2^15)
  - Full Poseidon2 pipeline (4 deep)
  - Full lookup engine (4 tables, 64K each)
  - PCIe card reference design
  
Purpose:
  - Production mining and proving hardware
  - Enable mainnet PoUW
  
Cost: ~$5M-15M (tape-out + initial production run)
Revenue: Hardware sales + operational mining/proving
```

### Phase 3: Optimization (36+ months)

```
Targets:
  - GFP-2: 2× FMA density, 3nm process
  - GFP-SoC: ARM + GFP on single die (mobile)
  - GFP-USB: Minimal proving dongle
  - Multi-chip module for datacenter provers
```

---

# Part VIII: Specification Summary

## 9. One Page

```
╔══════════════════════════════════════════════════════════════════════╗
║                    GOLDILOCKS FIELD PROCESSOR                       ║
║                    Specification Summary v1.0                       ║
╠══════════════════════════════════════════════════════════════════════╣
║                                                                     ║
║  FIELD:     p = 2^64 - 2^32 + 1 (Goldilocks)                      ║
║  ISA:       10 instructions (4 field + 2 transform + 1 hash +      ║
║             1 lookup + 2 memory)                                    ║
║  PRIMITIVES:  fma (40%) · ntt (35%) · p2r (15%) · lut (10%)       ║
║                                                                     ║
║  HARDWARE (GFP-1):                                                  ║
║    FMA array:     256 units, 1 cycle/op                            ║
║    NTT engine:    2^15 in-place butterfly                          ║
║    Poseidon2:     t=12, 22-cycle pipeline, 4-deep                  ║
║    Lookup:        4 tables × 64K entries, authenticated             ║
║    Memory:        8 MB L1 SRAM + 8-16 GB HBM                      ║
║    TDP:           75-150W (PCIe) / 25W (M.2) / 5W (USB)           ║
║                                                                     ║
║  PROOF OF USEFUL WORK:                                              ║
║    Puzzle:        stark proof of benchmark circuit B                ║
║    Primitives:    Same four as utility (fma, ntt, p2r, lut)        ║
║    Ratios:        Match real workload (40:35:15:10)                 ║
║    Verification:  O(log n) — any light client                      ║
║    Adjustment:    Per-epoch target + periodic ratio rebalancing      ║
║    Progress-free: Each nonce independent (no pool required)         ║
║                                                                     ║
║  ECONOMICS:                                                         ║
║    Supply side:   Mine blocks → earn rewards                       ║
║    Demand side:   Prove transactions → earn fees                   ║
║    Same chip:     GFP serves both simultaneously                    ║
║    Tiers:         USB ($20) → M.2 ($200) → PCIe ($1K) → Farm      ║
║                                                                     ║
║  KEY PROPERTY:                                                      ║
║    Optimal mining hardware = Optimal utility hardware               ║
║    (PoUW-Utility Isomorphism, §5.2)                                ║
║                                                                     ║
║  INTEGRATION:                                                       ║
║    Hybrid PoS (consensus) + PoUW (resource commitment)              ║
║    Interleaved scheduling: mine + prove on same chip                ║
║    Block = PoW proof + proven transactions + focus update           ║
║                                                                     ║
║  ROADMAP:                                                           ║
║    Phase 0: Software emulation (now)                                ║
║    Phase 1: FPGA prototype (6-18 months)                            ║
║    Phase 2: ASIC tape-out (18-36 months)                            ║
║    Phase 3: Optimization + form factors (36+ months)                ║
║                                                                     ║
╚══════════════════════════════════════════════════════════════════════╝

    The miner IS the prover. The puzzle IS the workload.
    The chip IS the product. The network IS the customer.
```

---

*purpose. link. energy.*
*prove. mine. serve.*

---

## Cross-references

- See [[cyber/launch]] for how GFP fits into the development roadmap
- See [[rosetta stone]] for why these four primitives unify all domains
- See [[Goldilocks homomorphic encryption]] for [[TFHE]] over the [[Goldilocks field]]
- See [[trinity]] for the three-pillar architecture
- See [[privacy trilateral]] for the full privacy stack
