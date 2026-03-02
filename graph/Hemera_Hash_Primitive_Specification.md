---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: Hemera specification, Hemera spec
---
# Hemera: A Permanent Hash Primitive for Planetary-Scale Collective Intelligence

Version: 1.0
Status: Decision Record
Authors: mastercyb, Claude (Anthropic)
Date: February 2026

---

## Abstract

Hemera is a complete cryptographic hash primitive for permanent content addressing in [[cyber]], a knowledge graph designed to operate at planetary scale (10¹⁵ nodes) with immutable content identifiers. Hemera adopts the Poseidon2 permutation structure but diverges from the ecosystem in field selection ([[Goldilocks field]] rather than the dominant BabyBear/M31), state width (t=16), and round count (R_P=64), yielding parameters chosen for permanent-grade security: 256-bit classical collision resistance, 170-bit quantum collision resistance, and algebraic degree 7⁶⁴ ≈ 2¹⁸⁰ — far beyond any foreseeable attack capability. A Hemera hash is 64 raw bytes — no version prefix, no header, no escape hatch.

The name Hemera (Ἡμέρα, "Day") denotes the primordial Greek goddess who brings light from darkness — as the hash function brings clear, deterministic identity from arbitrary content. Hemera is a complete primitive over the [[Goldilocks field]]: the name specifies the prime, S-box, state width, round counts, rate, capacity, padding scheme, encoding rules, and output format. There is exactly one Hemera, and it has exactly one mode: sponge. No compression function, no qualifiers, no variants.

```
Hemera = Poseidon2(
    p  = 2⁶⁴ − 2³² + 1,   -- Goldilocks
    d  = 7,                 -- S-box: x → x⁷
    t  = 16,                -- state width
    Rꜰ = 8,                 -- full rounds (4 + 4)
    Rₚ = 64,                -- partial rounds
    r  = 8,                 -- rate (64 bytes)
    c  = 8,                 -- capacity (64 bytes)
    out = 8 elements        -- 64 bytes
)
```

---

## 1. Why a New Name

Hemera adopts the Poseidon2 permutation structure but diverges from the ecosystem in both field selection and parameterization. The overwhelming majority of production Poseidon2 deployments target 31-bit fields — BabyBear (SP1, RISC Zero) or Mersenne-31 (Stwo/Starknet). The few that use 64-bit Goldilocks (Plonky3, Miden) deploy a narrower t=12 width with minimal security margins. Hemera chooses Goldilocks for CPU-native efficiency and curve-independent security, then pushes to t=16 width and R_P=64 partial rounds — a combination no production system has deployed. It is a distinct primitive that inherits Poseidon2's algebraic design but makes fundamentally different engineering commitments.

[[cyber]]'s hash function cannot afford ambiguity. A [[particle]]'s Hemera hash is its permanent, unique address in the [[cybergraph]]. Changing any parameter — a single round constant, the MDS matrix, even the byte order — produces a different hash function and invalidates every address in the graph. The name Hemera absorbs all parameters into a single, unambiguous identifier.

If you say "Hemera," every parameter is determined. If you change any parameter, it is no longer Hemera.

---

## 2. The Permanence Constraint

### 2.1 [[cyber]] vs. Execution-Layer Systems

Every zero-knowledge system deploying Poseidon2 today uses it as an execution-layer primitive: trace commitments that live for seconds, Merkle proofs verified and discarded, parameters updatable in the next release.

[[cyber]] uses Hemera as an identity-layer primitive. A [[particle]]'s Hemera hash is its permanent, unique address in the [[cybergraph]]. Every [[cyberlink]] references [[particles]] by hash. Every [[neuron]]'s state commitment depends on hashes. The global state root depends on every shard.

| Property | zkVM (SP1, RISC Zero) | cyber/core |
|---|---|---|
| Hash lifetime | Seconds to hours | Decades to permanent |
| Parameter update | Software release | Impossible without rehash |
| Rehash cost | Zero (ephemeral) | O(10¹⁵) operations |
| Adversary budget | Current computational | Future computational + quantum |
| Cost of parameter error | Reissue proofs | Lose the graph |

### 2.2 Implication

Parameters chosen at genesis are permanent commitments. The threat model is not "what attacks exist today" but "what attacks will exist over the lifetime of the system." This asymmetry drives every parameter decision in Hemera.

---

## 3. Parameter Decisions

### 3.1 Field: Goldilocks (p = 2⁶⁴ − 2³² + 1)

The field determines the atomic computational element. [[cyber]] requires efficiency across five domains simultaneously: ZK/STARK proving, content addressing, MPC, FHE, and native CPU performance.

Why not 31-bit fields (BabyBear, Mersenne-31): A 31-bit element stores ~4 bytes. Capacity=8 at 31 bits yields only 124 bits of collision resistance — below the 128-bit minimum. These fields optimize for proving speed at the expense of hash throughput and security margin.

Why not 254-bit fields (BN254): Multiprecision arithmetic costs ~10× more than native 64-bit. [[tri-kernel]] ranking requires millions of field operations per second per node. Furthermore, BN254's security is coupled to a specific elliptic curve.

Why Goldilocks:

- Native CPU width: 64-bit multiplication in a single instruction
- Fast reduction: Modular reduction via two shifts and a subtraction
- Large NTT domain: Multiplicative subgroup of order 2³² (4 billion points)
- Curve independence: Security derives from field arithmetic, not elliptic curve assumptions
- 8-byte elements: Practical granularity for content addressing

### 3.2 S-box: d = 7

The S-box must be a bijection over F_p, requiring gcd(d, p−1) = 1.

For Goldilocks: p − 1 = 2³² × (2³² − 1). The factorization of 2³² − 1 includes factors 3 and 5:

- d=3: gcd(3, p−1) = 3 → not invertible ✗
- d=5: gcd(5, p−1) = 5 → not invertible ✗
- d=7: gcd(7, p−1) = 1 → invertible ✓

d=7 is the minimum invertible exponent for Goldilocks. This is not a choice but a mathematical constraint. Multiplicative depth per S-box = 3 (computing x² · x = x³, (x³)² = x⁶, x⁶ · x = x⁷), which is the minimum achievable for this field.

### 3.3 State Width and Capacity: t=16, r=8, c=8

The capacity problem. The ecosystem standard for Goldilocks (Plonky3, Miden) is t=12, rate=8, capacity=4. This yields exactly 128-bit classical collision resistance — the minimum acceptable security level with zero margin.

More critically, the BHT quantum collision bound at capacity=4 is 2⁸⁵, well below 128-bit post-quantum security.

Hemera uses t=16 with rate=8 and capacity=8:

| Security metric | cap=4 (ecosystem) | cap=8 (Hemera) |
|---|---|---|
| Classical collision | 2¹²⁸ (zero margin) | 2²⁵⁶ |
| Quantum collision (BHT) | 2⁸⁵ (insufficient) | 2¹⁷⁰ |
| Quantum preimage (Grover) | 2¹²⁸ | 2²⁵⁶ |

The wider state preserves the same throughput as t=12/cap=4 (both have rate=8 = 64 bytes per permutation) while doubling the security to permanent-grade levels.

### 3.4 Round Counts: R_F=8, R_P=64

Full rounds (R_F=8): The wide trail strategy guarantees ≥8 active S-boxes across any 4 consecutive full rounds (t/4 + 4 = 8 for t=16). Differential probability per trail: (6/2⁶⁴)⁸ ≈ 2⁻⁴⁸⁰, which is 352 bits below the 128-bit target. R_F=8 provides massive margin; additional full rounds do not strengthen the weakest link.

Partial rounds (R_P=64): Partial rounds drive algebraic degree growth. For d=7, R_P=64 yields degree 7⁶⁴ ≈ 2¹⁸⁰. The minimum R_P for 128-bit security on Goldilocks t=16 is approximately 21–24 based on current analysis, though the Ethereum Foundation Poseidon Initiative (2024–2026) has revealed that original security estimates both under- and overestimate required rounds depending on the instantiation. The EF bounty program on the Poseidon-64 instance (Goldilocks, d=7, t=8) demonstrated that R_P=13 at R_F=6 resisted all attacks at 40-bit estimated security through Phase 1 — but attack techniques are advancing rapidly, with Graeffe transform methods (2025) and resultant-based approaches (2026) achieving orders-of-magnitude speedups over prior methods.

For a permanent primitive, the question is not "what margin is sufficient today" but "what margin absorbs everything we cannot foresee." Partial rounds are cheap: each costs ~19 field multiplications (1 S-box + lightweight matrix), compared to ~304 for a full round. The 42 additional partial rounds beyond Plonky3's R_P=22 add only ~19% to total field multiplications while lifting algebraic degree from 2⁶² to 2¹⁸⁰ — a 118-bit increase in the primary algebraic security metric.

### 3.5 Round Structure: 8 + 64 = 72

R_F + R_P = 8 + 64 = 72 total rounds. The total is not a power of 2 — but the total never appears in code. What appears in code are loop bounds and array sizes, and these are:

- Full round loop: `for i in 0..8` → 2³
- Partial round loop: `for i in 0..64` → 2⁶
- Full round constants: `[F; 128]` → 2⁷ (= 8 rounds × 16 elements)
- Partial round constants: `[F; 64]` → 2⁶

R_P=64 was chosen over R_P=56 (which would give 64 total rounds) precisely because the partial round constant array is a data structure you allocate and iterate, while the total round count is an arithmetic sum that never becomes a variable. Optimizing the number that touches memory over the number that exists only on paper.

---

## 4. Complete Specification

### 4.1 Hemera Parameters

```
┌──────────────────────────────────────────────────────────┐
│  HEMERA — Complete Specification                         │
│                                                          │
│  Field:           p = 2⁶⁴ − 2³² + 1 (Goldilocks)       │
│  S-box:           d = 7  (x → x⁷, minimum for field)    │
│  State width:     t = 16                      = 2⁴       │
│  Full rounds:     R_F = 8  (4 + 4)            = 2³       │
│  Partial rounds:  R_P = 64                    = 2⁶       │
│  Rate:            r = 8  elements (64 bytes)  = 2³       │
│  Capacity:        c = 8  elements (64 bytes)  = 2³       │
│  Output:          8  elements (64 bytes)      = 2³       │
│                                                          │
│  Full round constants:    8 × 16 = 128        = 2⁷       │
│  Partial round constants: 64                  = 2⁶       │
│  Total constants:         192                 = 3 × 2⁶   │
│  Total rounds:            72                  = 9 × 2³   │
│                                                          │
│  Classical collision resistance:  256 bits     = 2⁸       │
│  Quantum collision resistance:   170 bits                │
│  Algebraic degree:               2¹⁸⁰                    │
│                                                          │
│  Every parameter that appears in code is a power of 2.   │
└──────────────────────────────────────────────────────────┘
```

Invariant: These parameters are Hemera. If any parameter differs, it is not Hemera.

### 4.2 Computational Elegance

Hemera's parameters are not only secure — they are computationally pretty. Every value that appears as an array size, loop bound, or memory allocation in an implementation is a power of 2:

```
Parameter           Value    Code role                    Power of 2
─────────────────────────────────────────────────────────────────────
t  (state width)      16     [F; 16] array                  2⁴
R_F (full rounds)      8     for i in 0..8 { }              2³
R_P (partial rounds)  64     for i in 0..64 { }             2⁶
r  (rate)              8     absorb chunk [F; 8]            2³
c  (capacity)          8     security region [F; 8]         2³
output                 8     result [F; 8]                  2³
RC_FULL              128     [F; 128] constant table        2⁷
RC_PARTIAL            64     [F; 64] constant table         2⁶
element size        8 B      native u64                     2³
output bytes       64 B      hash output                    2⁶
rate bytes         64 B      per-permutation input          2⁶
capacity bytes     64 B      security capacity              2⁶
state bytes       128 B      full permutation state         2⁷
```

The only non-power-of-2 values are derived sums (72 total rounds = 8 + 64, 192 total constants = 128 + 64) that never appear as code-level quantities, and d = 7 which is not a design choice but a mathematical constraint — the minimum invertible exponent over Goldilocks.

This is not cosmetic. At planetary scale, the permutation executes trillions of times. Power-of-2 array sizes enable SIMD-aligned memory access. Power-of-2 loop bounds enable clean unrolling by any factor. The full-round constant table indexes as `RC_FULL[round * 16 .. round * 16 + 16]` — since both 16 and 128 are powers of 2, every access is naturally aligned to cache-line boundaries on 64-byte architectures.

The permutation loop structure:

```rust
// Initial linear layer
state = m_e.mul(state);                          // 16×16 matrix, once

// First half: 4 full rounds
for i in 0..4 {                                  // 2² iterations
    add_constants(&mut state, &RC_FULL[i*16..]);  // 16-aligned slice
    full_sbox(&mut state);                        // 16 parallel S-boxes
    state = m_e.mul(state);                       // dense 16×16
}

// Middle: 64 partial rounds
for i in 0..64 {                                 // 2⁶ iterations
    state[0] += RC_PARTIAL[i];                   // single constant
    state[0] = state[0].pow7();                  // single S-box
    state = m_i.mul(state);                      // sparse I+diag
}

// Second half: 4 full rounds
for i in 4..8 {                                  // 2² iterations
    add_constants(&mut state, &RC_FULL[i*16..]);  // 16-aligned slice
    full_sbox(&mut state);                        // 16 parallel S-boxes
    state = m_e.mul(state);                       // dense 16×16
}
```

Every loop bound, every array dimension, every slice offset is a power of 2. The implementation writes itself.

### 4.3 One Mode

Hemera has exactly one mode of operation: sponge. There is no compression mode.

```
Initialize:  state ← [0; 16]
Absorb:      for each 8-element chunk of padded input:
               state[0..8] ⊕= chunk
               state ← permute(state)
Squeeze:     output ← state[0..8]
```

Every hash in cyber/core — particle content, Merkle internal nodes, cyberlink edges, neuron identity, state commitments — goes through this single function. A Merkle parent is `Hemera(left ∥ right)`, absorbing 16 elements in two chunks, requiring two permutations instead of one.

Why not a separate compression mode? A compression function (permute the full 16-element input, take 8 elements of output) would halve the cost of Merkle tree construction. Every production Poseidon2 deployment offers this. We deliberately reject it for three reasons:

Practical — ambiguity. Hemera is the identity function for cyber/core. Hash is everything: particle addresses, edge identifiers, commitment roots, neuron keys. All of these share a single 64-byte address space with no type prefix, no mode byte, no version header. If two modes can produce the same 64-byte output from different inputs through different code paths, the address space is no longer a function — it is an ambiguity. Compression mode uses all 16 state elements as input (zero capacity). Sponge mode reserves 8 elements as capacity. They operate on different security assumptions, different domain separation strategies, different input constraints. Two functions sharing one output space means every downstream system must track which function produced each address. That tracking is either a hidden type tag (contradicting our no-header commitment) or an implicit convention (a bug waiting to happen at planetary scale).

Economic — irreversibility. The cost of sponge-only Merkle trees is 2× per internal node. Moore's law eliminates any 2× decision in two years. Design ambiguity is permanent. We accept the 2× and buy back performance through caching, incremental updates, and parallelism — not a second mode.

Mathematical — endofunctions. A sponge hash is an endofunction on the address space. Bytes in, 64 bytes out — and those 64 bytes are valid input to the same function. `Hemera(Hemera(x) ∥ Hemera(y))` type-checks. Composition, chaining, nesting — the algebra closes. A compression function has a different type signature (128 bytes → 64 bytes). The moment two functions with different signatures produce outputs in the same space, you leave the category of endofunctions. Composition breaks. You need to track which function produced which value. The algebra gets dirty. We are not rejecting compression for speed. We are rejecting leaving the category.

One mode. One function. One security argument. `Hemera(x) = Hemera(y)` if and only if `x = y`. No exceptions.

The cost: Merkle trees require 2 permutations per internal node instead of 1. At 10¹⁵ particles with a binary Merkle tree of depth ~50, a full tree rebuild requires ~2 × 10¹⁵ permutations instead of ~10¹⁵. Moore's law eliminates any 2× decision in two years. Design ambiguity is permanent.

### 4.4 Canonical Byte Encoding

1. Bytes → field elements: Pack input bytes into 8-byte little-endian chunks. Each chunk is interpreted as an element of F_p. If the chunk value ≥ p, split into two elements (high byte separate).
2. Padding: Append 0x01 after content, then 0x00 bytes to fill the final 8-byte chunk.
3. Field elements → bytes: 8 bytes, little-endian, canonical range [0, p).

Domain separation between particle hashes, cyberlink hashes, and Merkle nodes is achieved naturally by input structure: different data produces different hashes. No capacity-level domain tags are needed because there is only one mode.

### 4.5 Output Format

A Hemera hash is 64 bytes. Nothing more. No version prefix, no mode byte, no escape hatch. The raw output of 8 Goldilocks field elements in little-endian canonical form IS the particle address.

```
Hemera output = 8 × 8 bytes = 64 bytes (little-endian, canonical range [0, p))
```

If Hemera is ever broken, the entire graph rehashes. Storage proofs make this possible. Versioning headers do not save you — they waste bytes multiplied by 10¹⁵ particles.

### 4.5.1 Content Identifiers: Raw Bytes, No Headers

[[nox]] content identifiers (CIDs) are raw 64-byte Hemera outputs. Period. No multicodec prefix. No multihash header. No version byte. No length indicator. No framing of any kind.

```
IPFS CIDv1:    <version><multicodec><multihash-fn><digest-length><digest>
               1 + 1-2  + 1-2      + 1           + 32-64 bytes
               = 36-69 bytes of which 4-5 are pure overhead

nox CID:      <digest>
               64 bytes. That's it.
```

Why no headers:
1. Overhead at scale. At 10¹⁵ particles, every byte of header overhead costs a petabyte of storage. A 5-byte CID prefix × 10¹⁵ = 5 PB. This is not negligible. It is an architectural tax paid forever, on every lookup, every proof, every edge, every packet. Raw 64 bytes eliminates this tax completely.

2. There is exactly one hash function. Headers exist to disambiguate between multiple possible interpretations of the same bytes. nox has one hash function: Hemera. One field: Goldilocks. One output size: 64 bytes. One encoding: little-endian canonical. There is nothing to disambiguate. A header answering a question nobody asked is not safety — it is noise.

3. Headers create the illusion of upgradability. A version prefix implies "we might change this later." In a content-addressed graph with immutable addresses, there is no "later" for existing addresses. Address A was produced by Hemera from specific bytes. No header can change that. If Hemera is broken, the entire graph rehashes via storage proofs — the header doesn't help. If Hemera is not broken, the header wastes space. In neither scenario does the header provide value.

4. Endofunction closure. A Hemera output is valid Hemera input: `Hemera(Hemera(x) ∥ Hemera(y))` type-checks. Headers break this. A CID with a multicodec prefix is not raw bytes — it is a tagged value that must be stripped before hashing and reattached after. Every Merkle tree node, every proof chain, every composition would require encode/decode at boundaries. The algebra gets dirty. Raw bytes compose cleanly.

5. Flat namespace. Every entity in nox — particle, edge, neuron, commitment, proof — has a 64-byte address in one flat namespace. No type tags. No interpretation hints. The same function produces all identifiers. A `particle_address == edge_id` collision is prevented by domain separation in the hash input (different serialization), not by type prefixes on the output. The output is pure, untagged, universal.

Compatibility with IPFS/libp2p: If interop is needed, a thin translation layer at the network boundary can wrap raw Hemera bytes in CIDv1 format for external systems. Inside nox, the wrapper never exists. Translation is a gateway concern, not a protocol concern.

```
On the wire (nox):      [64 bytes]
On the wire (IPFS):     [0x01 0xNN 0xNN 0x40 ... 64 bytes ...]
                         ↑     ↑     ↑     ↑
                         │     │     │     └─ digest length
                         │     │     └─ multihash function code
                         │     └─ multicodec (content type)
                         └─ CIDv1 version

nox never generates, stores, transmits, or processes the left part.
Gateways add it. Gateways strip it. The graph never sees it.
```

The principle: A content identifier identifies content. It does not identify itself. The 64 bytes ARE the identity — complete, self-sufficient, and universal. Any byte spent saying "this is a Hemera hash" is a byte not spent on security, a byte replicated 10¹⁵ times, and a byte that implies the system might one day be something other than what it is.

### 4.6 Canonical Tree Hashing

Merkle trees in cyber/core use Hemera sponge for both leaves and internal nodes. For subtree hashes to be globally stable and dedupable, the chunking rule and tree shape must be frozen alongside the hash parameters. The chunk size is a permanent parameter — once content has been hashed and addressed, changing it would invalidate every existing address in the graph.

#### 4.6.1 Chunk Size: 4 KB (4096 bytes)

Chunking rule: Content is split into fixed 4 KB chunks (4096 bytes = 512 field elements = 64 absorb blocks). The last chunk is padded normally by the sponge. No content-defined chunking — identical byte ranges always produce identical chunks.

Why 4 KB and not some other size. The chunk size must be a multiple of 64 bytes (Hemera's absorb block). Among powers of two — 256 B, 1 KB, 4 KB, 8 KB, 16 KB, 64 KB — only 4 KB simultaneously satisfies every constraint:

1. Field alignment. 4096 bytes = 64 absorb blocks = 2⁶ permutations per chunk. A clean power of two, consistent with every other nox parameter (t=2⁴, r=2³, c=2³, R_F=2³, R_P=2⁶). The permutation count per chunk is the same as the partial round count — both are 2⁶. This is not coincidence; both reflect the same security depth.

2. OS page alignment. 4 KB is the virtual memory page size on x86 (since 1985), ARM (since 1987), and RISC-V (since 2010). It is the default block size of ext4, XFS, NTFS, and APFS. It is the minimum addressable unit on NVMe drives. `mmap()` reads and writes align to page boundaries without buffering. This means zero-copy I/O between storage and hash function — the OS delivers content in units that map directly to Hemera chunks with no intermediate buffering.

3. L1 cache fit. 4 KB fits in the L1 data cache of every modern CPU (typically 32–64 KB). The entire chunk can be hashed in cache-resident memory. At 8 KB, cache pressure increases; at 16 KB, the chunk exceeds L1 on many architectures and performance degrades from cache misses during hashing.

4. STARK proof granularity. One 4 KB leaf requires 64 permutations × ~1,200 constraints = ~76,800 constraints. This is small enough for efficient recursive proof composition but large enough that proof overhead does not dominate content. At 1 KB (19,200 constraints per leaf), proof metadata costs approach content costs. At 64 KB (1.2M constraints per leaf), individual leaf proofs become expensive.

5. Tree depth and proof size. Practical scaling at 4 KB chunks:

```
Content size    Leaves      Tree depth    Proof size
────────────    ──────      ──────────    ──────────
1 MB            256         8             512 B
1 GB            262,144     18            1,152 B
1 TB            268M        28            1,792 B
1 PB            274B        38            2,432 B
```

All proofs fit in a single network packet. At 256 B chunks, 1 GB content would require depth 22 and 1,408 B proofs — feasible but wasteful. At 64 KB chunks, 1 MB content would have only 16 leaves — too shallow for meaningful structural sharing.

6. Overhead ratio. Merkle tree metadata costs ~1.6% additional storage at 4 KB. At 256 B, overhead is 25% (one quarter of storage is tree, not content). At 64 KB, overhead is 0.1% but granularity is lost. 1.6% is the sweet spot — negligible cost for full verifiability and deduplication.

7. Deduplication quality. 4 KB blocks have meaningful repetition in real-world data: database pages (Postgres, SQLite use 4–8 KB pages), virtual machine disk images, document formats (PDF objects, DOCX zip entries), and versioned content where most chunks stay unchanged between edits. At 64 B, almost no sequences repeat — dedup is noise. At 64 KB, dedup granularity is too coarse. 4 KB is the empirical sweet spot where structural sharing is both frequent and semantically meaningful.

8. Streaming verification. A receiver buffers at most one chunk (4 KB) plus one Merkle proof (~1–2 KB) before verifying. Total memory per verification step: ~6 KB. This allows content to be verified and processed chunk-by-chunk during network reception with minimal memory, enabling verified streaming on constrained devices.

9. Network transport. 4 KB = approximately 3 TCP segments (at 1460-byte MSS) or 1 jumbo Ethernet frame (9000 bytes). A chunk plus its Merkle proof fits comfortably in any transport unit. At 64 KB, a single chunk requires ~46 TCP segments — impractical for chunk-at-a-time delivery.

Note on MTU and the 3-packet question. The ideal would be 1 chunk = 1 network packet — atomic delivery and verification in a single frame. At 4 KB chunks, today's internet MTU (1500 bytes, chosen in 1980 based on Ethernet controller RAM costs) requires 3 TCP segments. This is not a flaw in the chunk size — it is a legacy constraint in the network:

```
1980: RAM cost $6,000/MB → MTU 1500 was a cost compromise
2025: RAM cost $3/GB     → MTU 1500 persists because "it works"
```

Jumbo frames (MTU 9000) have existed since 1998 and are standard in every major cloud datacenter (AWS, Google Cloud, Azure). A nox verification unit — chunk + Merkle proof + frame header ≈ 5.3 KB — fits in a single jumbo frame with room to spare. Between nodes on modern infrastructure, 4 KB chunks already deliver single-frame atomic verification.

For legacy internet paths where MTU 1500 is unavoidable, TCP reassembles the 3 segments transparently — the application sees a single 4 KB read. The fragmentation is invisible to the verification layer.

The design principle is: fit the network to the data, not the data to the network. The chunk size is derived from field alignment, OS page alignment, cache geometry, and proof granularity — mathematical and hardware invariants. MTU is a legacy economic parameter that infrastructure is already evolving past. If nox ever defines a native transport protocol, the minimum transfer unit will be chunk + proof (~5.3 KB), not 1500 bytes from 1980.

10. Bounded locality. Changing one byte in content requires rehashing one 4 KB chunk (64 permutations) plus log₂(n/256) tree nodes up to the root (one permutation each). For 1 GB content: 64 + 18 = 82 permutations. For 1 TB: 64 + 28 = 92 permutations. The local cost (64 permutations) dominates; tree traversal is negligible. At 64 KB chunks, the local cost would be 1,024 permutations — 16× worse locality.

Comparison table:
```
                    256B   1KB     4KB     8KB    16KB    64KB
                    ────   ────   ─────   ────   ─────   ─────
Absorbs/chunk         4     16      64    128     256    1024
Absorbs = 2^k       2²     2⁴     2⁶     2⁷      2⁸    2¹⁰
1GB tree depth       22     20      18     17      16      14
1GB proof (bytes)  1408   1280    1152   1088    1024     896
Overhead ratio       25%     6%    1.6%   0.8%    0.4%    0.1%
OS page aligned       ✗      ✗      ✓      ✗       ✗       ✗
L1 cache fit          ✓      ✓      ✓      ~       ✗       ✗
STARK constraints   4.8K  19.2K   76.8K   154K   307K    1.2M
Streaming buffer   256B     1K      4K     8K     16K     64K
Dedup quality      poor   fair    good   good    fair    poor
Network packets       1      1       3      6      12      46
```

4 KB is the only row with ✓ on both page alignment and L1 cache fit, 2⁶ absorbs per chunk (matching R_P), practical proof size, and meaningful deduplication. The convergence is not forced — it is the unique point where field arithmetic, hardware reality, and graph properties intersect.
Leaf hash: `Hemera(chunk_bytes)` — the chunk goes through the sponge as raw bytes.

Internal node hash: `Hemera(left_id ∥ right_id)` — exactly 128 bytes (two 64-byte hashes concatenated), hashed through the same sponge. The internal node "content" is precisely the concatenation of its children's addresses, with no framing, no length prefix, no metadata.

Tree shape: Binary, left-balanced. For N chunks:

```
If N = 1:     leaf hash is the root
If N > 1:     split = 2^(⌈log₂(N)⌉ - 1)
              left  = chunks[0..split]
              right = chunks[split..N]
              root  = Hemera(hash(left) ∥ hash(right))
```

Left-balanced means the left subtree is always a complete binary tree (power-of-2 leaves). This ensures that the same content prefix always produces the same left subtree hash regardless of what follows — enabling incremental hashing and prefix deduplication.

Consequence: Any node, anywhere, hashing the same content bytes, produces the same root hash, the same intermediate node hashes, and the same leaf hashes. Subtree addresses are globally stable and can be used for deduplication, caching, and verified streaming without coordination.

Performance recovery: The 2× Merkle cost (two permutations per internal node instead of one) is recovered through caching (subtree hashes are stable and reusable), incremental updates (changing one chunk only recomputes its path to root), and parallelism (all leaves hash independently, tree levels are embarrassingly parallel). The performance is bought back through architecture, not by introducing a second mode.

### 4.7 Operational Semantics

Hemera serves every hashing role in [[cyber]] through one function:

[[particle]] addressing. `address = Hemera(content_bytes)`. Small content (< 64 bytes) absorbs in one chunk, one permutation. Large content absorbs in 4 KB chunks with a Merkle tree; the [[particle]] address is the tree root.

[[cyberlink]] identity. `edge_id = Hemera(neuron_id ∥ source ∥ target ∥ weight ∥ time)`. Structured field data serialized to bytes, hashed through sponge.

Merkle proofs. Leaf and internal node hashes use the same function. Proof verification is a uniform chain of Hemera calls — no mode switching, no type disambiguation.

Incremental hashing. The sponge state (16 field elements = 128 bytes) is a complete checkpoint. Save it, resume later, get the same result as a single-pass hash. Nodes can hash content arriving over the network in chunks without buffering.

Streaming verification. Receive content chunk by chunk, verify each chunk against a Merkle proof, process immediately. Never buffer more than one chunk + proof. Reject invalid chunks before storing anything.

Field-native computation. Hemera input and output are [[Goldilocks field]] elements. The hash output is directly usable in [[tri-kernel]] ranking, polynomial commitments, and ZK circuits without conversion. Inside a STARK proof, calling Hemera is just more field arithmetic in the same trace — no bit decomposition, no range checks, no gadgets. Hemera costs ~1,200 constraints in a Goldilocks STARK versus ~25,000 for SHA-256.

[[tri-kernel]] lookup. A 64-byte Hemera output is a headerless, typeless, modeless identifier that requires zero extra context for [[tri-kernel]] lookup. Every address in the graph — [[particle]], edge, commitment, [[neuron]] — lives in one flat namespace produced by one function.

### 4.8 Round Constant Generation

Hemera generates her own round constants. No external primitives — no SHA-256, no ChaCha20, no foreign dependencies.

The permutation structure (S-box x⁷, matrices M_E and M_I, round flow 4+64+4) is fully defined before constants exist. With all constants set to zero, the permutation is still a well-defined nonlinear function — the S-box and MDS matrices provide all the mixing. We call this Hemera₀.

```
1. Define Hemera₀ = Hemera permutation with all 192 round constants = 0
2. Feed the genesis word through Hemera₀ as a sponge:
   
   input = [0x63, 0x79, 0x62, 0x65, 0x72]    — "cyber" as raw bytes
   
   state = [0; 16]
   absorb input into state using Hemera₀
   squeeze 192 field elements from state using Hemera₀
   
3. First 128 elements → RC_FULL[128]
   Next 64 elements   → RC_PARTIAL[64]

4. Hemera = Hemera₀ + these constants. Freeze forever.
```

Seed. The seed is five bytes: `[0x63, 0x79, 0x62, 0x65, 0x72]`. Not "the UTF-8 encoding of the string cyber" — the bytes themselves are the specification. No character set, no encoding, no text convention. Five bytes, specified as hex literals. The fact that these bytes happen to spell "cyber" in ASCII is the human meaning; the cryptographic input is the byte sequence alone.

The parameters do not appear in the seed because they are not data — they are the structure of Hemera₀ itself. The S-box, the matrices, the round count are code, not configuration. The seed is simply a non-zero input that breaks the all-zero fixed point. And `[0x63, 0x79, 0x62, 0x65, 0x72]` is the most inevitable choice for the identity function of the cybernetic graph.

Why self-bootstrapping? Hemera is a system built entirely on Goldilocks field arithmetic. Importing SHA-256 or ChaCha20 to generate round constants would introduce a foreign primitive — a gasoline engine inside an electric car. The zero-constant permutation is already a strong nonlinear function (x⁷ S-box, MDS diffusion, 72 rounds). Using it as its own PRNG is the most honest construction: the security of the constants reduces to the security of the structure itself.

Verifiability: If someone claims the constants are backdoored, they must argue that the zero-constant permutation — using the same S-box, the same matrices, the same round structure as the final Hemera — produces weak output when fed five non-zero bytes. This is a strictly harder claim than attacking any external PRNG.

#### 4.8.1 Security Analysis of Self-Bootstrapping

The non-circularity argument. Self-bootstrapping may appear circular but is strictly one-directional:

```
algebraic structure → Hemera₀ → constants → Hemera (done)
```

Hemera₀ is a fully-specified, independent function. The final Hemera never runs on its own seed and does not need to reproduce its own constants. There is no fixed-point equation to solve, no circularity to resolve.

Coupled security. With an external PRNG (ChaCha20), two independent security assumptions are required: "ChaCha20 output is pseudorandom" AND "the Poseidon2 structure is sound." With self-bootstrapping, there is only one assumption: "the Poseidon2 algebraic structure is sound." If Hemera₀ cannot produce pseudorandom output from a non-trivial input, then the S-box and MDS layers we rely on for the final Hemera are already broken. The self-bootstrap couples constant generation security directly to permutation security. If one fails, both fail — and both would have failed anyway. This is a strictly stronger argument than relying on an unrelated primitive.

The zero-state fixed point. Hemera₀ has a known structural property: the all-zero state is a fixed point. With all round constants equal to zero, `x⁷` maps `0 → 0`, and both M_E and M_I map the zero vector to the zero vector. Therefore `Hemera₀([0; 16]) = [0; 16]`.

This does not affect constant generation because:
- The sponge begins by absorbing the seed into the zero state via XOR
- After absorbing even one non-zero byte, the state is non-zero
- The seed `[0x63, 0x79, 0x62, 0x65, 0x72]` is 5 bytes of non-zero data
- After absorption, the first rate element is non-zero (0x7265627963 packed little-endian)
- The subsequent permutation call operates on a non-zero state, breaking the fixed point immediately

The fixed point is a mathematical property of the zero-constant permutation, not a vulnerability in the constant generation procedure. It should be noted for completeness: do not use Hemera₀ for any purpose other than constant generation from non-trivial seeds.

Reproducibility. The procedure is fully deterministic. Anyone implementing the same S-box (x⁷ over Goldilocks), the same matrix construction (M_E, M_I per Poseidon2 specification for t=16), the same round structure (4+64+4), and the same sponge (rate=8, capacity=8, multi-rate padding), feeding the same seed string, will produce the same 192 field elements. No randomness, no platform dependency, no external library required — only Goldilocks field arithmetic.

### 4.9 Matrix Construction

External matrix M_E (16×16): Circulant of 4×4 MDS sub-blocks, following the Poseidon2 paper (Section 4.2). The 4×4 sub-block uses the Cauchy-matrix construction adapted to Goldilocks.

Internal matrix M_I (16×16): Identity plus diagonal (M_I = I + diag(d₀, ..., d₁₅)), with diagonal elements selected to ensure MDS property over Goldilocks. Construction follows the Plonky3 convention for t=16.

Both matrices are generated by deterministic SageMath scripts and verified for MDS property before freezing.

---

## 5. Ecosystem Context

### 5.1 Poseidon2 Deployment Landscape

| System | Field | t | R_F | R_P | Capacity | Status |
|---|---|---|---|---|---|---|
| Plonky3 | Goldilocks | 12 | 8 | 22 | 4 (128-bit) | Production |
| SP1 | BabyBear | 16 | 8 | 13 | 8 (124-bit) | Production |
| RISC Zero | BabyBear | 16 | 8 | 13 | 8 (124-bit) | Production |
| Stwo/Starknet | M31 | 16 | 8 | 14 | 8 (124-bit) | Production (mainnet) |
| Miden | Goldilocks | 12 | 8 | 22 | 4 (128-bit) | Production |
| Aztec/Noir | BN254 | 4 | 8 | 56 | 1 (127-bit) | Production |
| Hemera | Goldilocks | 16 | 8 | 64 | 8 (256-bit) | Genesis |

### 5.2 What Is Novel, What Is Not

Not novel:- Poseidon2 with t=16 — battle-tested across billions of proofs (SP1, RISC Zero, Starknet)
- Poseidon2 on Goldilocks — battle-tested in Plonky3 and Miden
- The security proof methodology — field-agnostic for identical S-box degree
- The MDS construction — identical across all instantiations

Novel:- The combination of Goldilocks field + t=16 width (no production system uses this pair)
- R_P=64 (no production system uses more than 22 partial rounds on any 64-bit field)

The actual risk is a subtle error in the specific M_E or M_I matrix for Goldilocks t=16. This is mitigated by the verification plan in Section 7.

---

## 6. Performance Characteristics

### 6.1 Native Hash Rate

| Metric | Hemera | Plonky3 Goldilocks t=12 | Ratio |
|---|---|---|---|
| State width | 16 elements | 12 elements | 1.33× |
| Total rounds | 72 | 30 | 2.40× |
| Permutation field muls | ~3,648 | ~2,050 | 1.78× |
| Bytes per permutation | 64 | 64 | 1.00× |
| Estimated hash rate | ~62 MB/s | ~100 MB/s | 0.62× |
| Perms for 1 KB | 16 | 16 | 1.00× |

The 38% native hash rate reduction comes from the wider permutation and additional partial rounds. Throughput per permutation is identical because rate=8 in both cases. Partial rounds are lightweight (~19 field multiplications each vs ~304 for full rounds), so even at R_P=64, they account for only ~1,216 of the total ~3,648 field multiplications per permutation (33%).

### 6.2 Proving Cost

STARK trace width increases from 12 to 16 columns. Trace length increases from 30 to 72 rows per permutation:

- Wider state: ~1.33× proving cost
- More rows (72 vs 30): ~2.40× proving cost
- Combined: ~3.2× proving cost per hash vs Plonky3 baseline
This is the real cost of permanent-grade security. However, hash proving is not the bottleneck in cyber/core — tri-kernel ranking, consensus, and network I/O dominate computational load. If hashing accounts for ~20% of total proving time, the system-level impact is ~44% more total proving work. If hashing is ~40% (Merkle-heavy workloads like storage proofs), the impact is ~88%.

### 6.3 Steady-State Adequacy

At 10¹⁵ particles with 1% annual update rate:
- Required: ~317K particles/sec
- Hemera at ~62 MB/s, 200-byte average particle: ~310K particles/sec per core
- Single core handles steady-state content hashing.
---

## 7. Implementation and Verification Plan

### Phase 1: Parameter Generation (Weeks 1–2)

| Deliverable | Method |
|---|---|
| M_E (16×16 external matrix) | Circulant-of-4×4-MDS construction in SageMath |
| M_I (16×16 internal matrix) | I + diagonal construction in SageMath |
| 192 round constants (128 full + 64 partial) | Self-bootstrapping: Hemera₀ sponge from published seed |
| MDS property proof | Verify all sub-matrix determinants ≠ 0 |

### Phase 2: Security Verification (Weeks 2–4)

| Verification | Method |
|---|---|
| Wide trail bound | Exhaustive truncated differential enumeration over 4 full rounds |
| Invariant subspace analysis | Grassmann variety search for dimensions 1..8 |
| Algebraic degree tracking | Symbolic degree propagation through 72 rounds |
| Branch number verification | Computational proof that branch(M_E) ≥ 5 |

### Phase 3: Reference Implementation (Weeks 3–5)

```rust
/// Hemera — the complete hash primitive for cyber/core.
/// This crate IS the specification. Parameters are constants, not configuration.
/// There is one function. There is no compression mode.
pub struct Hemera;

impl Hemera {
    pub const P: u64 = (1 << 64) - (1 << 32) + 1;  // Goldilocks
    pub const D: u64 = 7;
    pub const T: usize = 16;
    pub const ROUNDS_F: usize = 8;
    pub const ROUNDS_P: usize = 64;
    pub const RATE: usize = 8;
    pub const CAPACITY: usize = 8;

    /// Hash arbitrary bytes to 8 Goldilocks field elements.
    /// This is the only entry point. Merkle nodes, particle content,
    /// cyberlinks, neuron identity — everything goes through here.
    pub fn hash(input: &[u8]) -> [GoldilocksField; 8];
    
    /// Raw permutation over 16 Goldilocks elements.
    /// Exposed for testing and verification, not for direct use.
    pub fn permute(state: &mut [GoldilocksField; 16]);
}
```

Deliverables: `hemera` Rust crate, test vectors JSON, cross-validation with SageMath reference.

### Phase 4: Distributed Verification (Weeks 5–8)

Deploy across network idle compute:

| Campaign | Target |
|---|---|
| Differential search | Random differential pairs through R_P = 1..63 |
| Groebner basis attacks | Reduced-round instances up to 40-bit estimated security |
| Collision fuzzing | 2⁴⁰ random inputs, verify zero collisions |
| Avalanche testing | Bit-flip propagation ≥ 50% |
| Distribution test | Chi-squared on output byte distribution |

### Phase 5: Publication and External Review (Weeks 8–12)

| Action | Purpose |
|---|---|
| Publish all matrices, constants, scripts | Reproducibility |
| Submit to EF Poseidon Initiative | Independent cryptanalysis by world-class team |
| Cyberlink specification into cyber/core graph | Self-referential: the graph contains its own foundation |
| arXiv preprint | Academic record |

---

## 8. Migration and Emergency Protocols

### 8.1 No Algorithm Agility

There is no version byte. There is no escape hatch in the address format. Hemera outputs are raw 64-byte addresses — permanent, unadorned, unversioned.

If Hemera is broken, the response is not graceful coexistence of two address spaces. It is a full graph rehash. Every [[particle]] gets a new address under a new primitive. Every [[cyberlink]] is re-signed. The old graph ceases to exist. The new graph replaces it entirely.

This is not a weakness — it is a design commitment. Versioning headers create the illusion of safety while wasting bytes at planetary scale (5 bytes × 10¹⁵ = 5 petabytes of pure overhead). The actual safety comes from two things: choosing parameters that will not break, and maintaining [[storage proofs]] that enable rehashing if they do.

### 8.2 Storage Proofs as Prerequisite

Migration requires access to original content. Without storage proofs, content may be lost and rehashing is impossible.

```
Hash may need replacement
  → Replacement requires rehashing
    → Rehashing requires content availability
      → Content availability requires storage proofs
        → Storage proofs must be operational before genesis
```

Storage proofs are Phase 1 security infrastructure, not Phase 3 optimization.
### 8.3 Emergency Response

| Timeframe | Action |
|---|---|
| 0–24 hours | Freeze new particle creation |
| 24–48 hours | Activate pre-staged fallback hash |
| Week 1–4 | Begin rehash campaign via storage proof infrastructure |
| Month 1–6 | Complete migration |

At 10¹⁵ particles across 10⁶ nodes: ~17 hours estimated rehash time.

---

## 9. The Name

Hemera (Ἡμέρα) — primordial Greek goddess of Day. Daughter of Erebus (Darkness) and Nyx (Night). One of the Protogenoi, the first-born entities from Chaos.

From arbitrary bytes (darkness), Hemera brings forth a clear, unique, permanent identity (daylight). The hash function does not represent identity — it IS identity. Hemera does not rule the day — she IS the day.

In the genealogy of arithmetization-oriented hash functions:

```
Poseidon  (2019) — the Olympian god of the sea
Poseidon2 (2023) — the optimized successor  
Hemera    (2026) — the Protogenoi: older, deeper, permanent
```

Hemera stands before Poseidon in the mythological hierarchy, as cyber/core's identity layer stands beneath all execution. She is the foundation upon which names exist.

---

## See also

- [[particle]] — content addressing with Hemera
- [[cyberlink]] — edges referencing [[particles]] by Hemera hash
- [[cybergraph]] — the graph Hemera addresses
- [[nox]] — the VM where Hemera executes as a [[nox#Jets|jet]]
- [[tri-kernel]] — probability engine consuming Hemera outputs
- [[Goldilocks field]] — the arithmetic substrate
- [[cyber/proofs]] — STARK proof system built on Hemera
- [[cyber/whitepaper]] — §4 Hemera chapter

## References

1. Grassi, L., Khovratovich, D., Schofnegger, M. "Poseidon2: A Faster Version of the Poseidon Hash Function." IACR ePrint 2023/323.
2. Grassi, L., Khovratovich, D., Rechberger, C., et al. "POSEIDON: A New Hash Function for Zero-Knowledge Proof Systems." IACR ePrint 2019/458.
3. Plonky3. https://github.com/Plonky3/Plonky3
4. Ethereum Foundation Poseidon Initiative. https://www.poseidon-initiative.info/
5. Grassi, L., et al. "Algebraic Cryptanalysis of Poseidon." ToSC 2025.
6. Sanso, A., Vitto, G. "Graeffe Transform Attacks." IACR ePrint 2025/937.
7. Bertoni, G., Daemen, J., Peeters, M., Van Assche, G. "Sponge Functions." Ecrypt Hash Workshop 2007.
8. EIP-7864: Ethereum State Using a Unified Binary Tree.
