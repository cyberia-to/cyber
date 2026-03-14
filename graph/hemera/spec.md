---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: Hemera specification, Hemera spec, Hemera_Hash_Primitive_Specification
stake: 43936669831471920
---
# Hemera: A Permanent Hash Primitive for Planetary-Scale Collective Intelligence

Version: 1.0
Status: Decision Record
Authors: mastercyb, Claude (Anthropic)
Date: February 2026

---

## Abstract

Hemera is the complete cryptographic hash primitive for [[cyber]], a knowledge graph designed to operate at planetary scale (10¹⁵ nodes) with immutable content identifiers. Hemera serves two roles: permanent content addressing (particle identity) and universal tree commitment (the foundation for every Merkle tree, MMR, namespace tree, and FRI polynomial commitment in the system). Hemera adopts the Poseidon2 permutation structure but diverges from the ecosystem in field selection ([[Goldilocks field]] rather than the dominant BabyBear/M31), state width (t=16), and round count (R_P=64), yielding parameters chosen for permanent-grade security: 256-bit classical collision resistance, 170-bit quantum collision resistance, and algebraic degree 7⁶⁴ ≈ 2¹⁸⁰ — far beyond any foreseeable attack capability. A Hemera hash is 64 raw bytes — no version prefix, no header, no escape hatch.

The name Hemera (Ἡμέρα, "Day") denotes the primordial Greek goddess who brings light from darkness — as the hash function brings clear, deterministic identity from arbitrary content. Hemera is a complete primitive over the [[Goldilocks field]]: the name specifies the prime, S-box, state width, round counts, rate, capacity, padding scheme, encoding rules, and output format. There is exactly one Hemera, and it has exactly one mode: sponge. No compression function, no qualifiers, no variants.

```
Hemera = Poseidon2(
    p  = 2⁶⁴ − 2³² + 1,   -- Goldilocks
    d  = 7,                 -- S-box: x → x⁷
    t  = 16,                -- state width
    Rꜰ = 8,                 -- full rounds (4 + 4)
    Rₚ = 64,                -- partial rounds
    r  = 8,                 -- rate (56 input bytes, 7 B/element)
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

The wider state preserves the same throughput as t=12/cap=4 (both have rate=8 = 56 input bytes per permutation with 7-byte encoding) while doubling the security to permanent-grade levels.

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
│  Rate:            r = 8  elements              = 2³       │
│  Input rate:      56 bytes/block (7 B/element) = 7 × 2³   │
│  Capacity:        c = 8  elements (64 bytes)   = 2³       │
│  Output:          8  elements (64 bytes)       = 2³       │
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
input rate bytes   56 B      per-permutation input      7 × 2³
output bytes       64 B      hash output                    2⁶
capacity bytes     64 B      security capacity              2⁶
state bytes       128 B      full permutation state         2⁷
```

The only non-power-of-2 values are derived sums (72 total rounds = 8 + 64, 192 total constants = 128 + 64) that never appear as code-level quantities, input rate (56 = 7 × 8), and d = 7. Both 7s are the same mathematical constraint: d = 7 is the minimum invertible S-box exponent for Goldilocks, and 7 bytes is the maximum whole-byte count fitting [0, p) without reduction. The Goldilocks prime forces 7 twice — once in the S-box, once in the encoding.

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

### 4.3 One Sponge, Structured Capacity

Hemera has exactly one primitive: the sponge. There is no compression mode. Every hash — particle content, Merkle leaves, Merkle internal nodes, cyberlink edges, key derivation — passes through the same permutation, the same absorption, the same squeezing. The sponge is universal.

But universality demands disambiguation. A Merkle leaf, a Merkle parent, and a plain content hash all produce 64-byte outputs in one flat namespace. Without structural binding, an attacker who controls content can craft a leaf chunk that parses as a valid internal node (two 64-byte hashes concatenated), substituting an entire subtree. Without position binding, identical chunks at different offsets produce identical hashes, enabling reordering attacks. Without root finalization, a single-chunk file hashes identically to a non-root leaf in a multi-chunk tree.

Hemera solves this not by adding modes (which would break the endofunction property), but by encoding context into the capacity region of the same sponge. The capacity — 8 field elements that never receive input data — carries structural metadata that binds every hash to its role, position, and finalization status.

#### 4.3.1 Capacity Layout

```
State:     state[0..8]  = rate region (input absorption / output squeezing)
           state[8..16] = capacity region (structural context, never XORed with input)

Capacity:  state[8]  = counter       chunk position in file (0-based, u64)
           state[9]  = flags         structural role (bitfield, see below)
           state[10] = msg_length    total input byte count (sponge only)
           state[11] = domain_tag    API mode selector (see below)
           state[12] = ns_min        namespace lower bound (NMT only, zero otherwise)
           state[13] = ns_max        namespace upper bound (NMT only, zero otherwise)
           state[14..16] = 0         reserved, must be zero
```

#### 4.3.2 Flags (state[9])

Three single-bit flags, combined via bitwise OR:

```
FLAG_ROOT   = 0x01    This hash finalizes a tree root
FLAG_PARENT = 0x02    This hash combines two child hashes (internal node)
FLAG_CHUNK  = 0x04    This hash derives a leaf chaining value
```

Valid combinations:

| Context | Flags | Value |
|---|---|---|
| Plain sponge hash | (none) | 0x00 |
| Non-root leaf | CHUNK | 0x04 |
| Root leaf (single-chunk file) | CHUNK \| ROOT | 0x05 |
| Non-root internal node | PARENT | 0x02 |
| Root internal node (tree root) | PARENT \| ROOT | 0x03 |

Flags encode what the hash IS, not what it contains. A flag combination that does not appear in the table above is invalid.

#### 4.3.3 Domain Tags (state[11])

The sponge API exposes four entry points. All use the same permutation. Domain tags prevent cross-mode collisions:

```
DOMAIN_HASH             = 0x00    Plain hash (default)
DOMAIN_KEYED            = 0x01    Keyed hash (MAC)
DOMAIN_DERIVE_KEY_CTX   = 0x02    Key derivation — context phase
DOMAIN_DERIVE_KEY_MAT   = 0x03    Key derivation — material phase
```

Domain tags are set before the first absorption and never modified. They are orthogonal to flags — a keyed hash of a Merkle leaf would have `state[9] = FLAG_CHUNK` and `state[11] = DOMAIN_KEYED`.

#### 4.3.4 Sponge Operation

```
Initialize:  state ← [0; 16]
             state[11] ← domain_tag

Absorb:      for each 8-element block of padded input:
               state[0..8] += block        (Goldilocks field addition, element-wise)
               state ← permute(state)

Finalize:    state[10] ← total_input_bytes
             state ← permute(state)

Squeeze:     output ← state[0..8]           (8 elements = 64 bytes)
```

Absorption uses Goldilocks field addition (mod p), not XOR and not wrapping addition. This preserves the algebraic structure that Poseidon2's security proof relies on.

#### 4.3.5 Why Not Compression Mode

A compression function (permute the full 16-element input, take 8 elements of output) would halve the cost of Merkle tree construction. Every production Poseidon2 deployment offers this. We deliberately reject it for three reasons:

Practical — ambiguity. Compression mode uses all 16 state elements as input (zero capacity). Sponge mode reserves 8 elements as capacity. They operate on different security assumptions. Two functions sharing one output space means every downstream system must track which function produced each address. That tracking is either a hidden type tag (contradicting our no-header commitment) or an implicit convention (a bug waiting to happen at planetary scale).

Economic — irreversibility. The cost of sponge-only Merkle trees is 2× per internal node. Moore's law eliminates any 2× decision in two years. Design ambiguity is permanent. We accept the 2× and buy back performance through caching, incremental updates, and parallelism — not a second mode.

Mathematical — endofunctions. A sponge hash is an endofunction on the address space. Bytes in, 64 bytes out — and those 64 bytes are valid input to the same function. `Hemera(Hemera(x) ∥ Hemera(y))` type-checks. Composition, chaining, nesting — the algebra closes. A compression function has a different type signature (128 bytes → 64 bytes). We are not rejecting compression for speed. We are rejecting leaving the category.

### 4.4 Canonical Byte Encoding

#### 4.4.1 Input Encoding (Bytes → Field Elements)

Pack input bytes into **7-byte** little-endian chunks. Each 7-byte chunk is zero-extended to 8 bytes and interpreted as a u64 in little-endian order, producing one Goldilocks field element.

```
bytes[0..7]   → element 0    (zero-extend to u64 LE)
bytes[7..14]  → element 1
bytes[14..21] → element 2
...
bytes[49..56] → element 7    (= one full rate block)
```

Why 7 bytes, not 8: The maximum 7-byte value is 2⁵⁶ − 1 = 0x00FF_FFFF_FFFF_FFFF. The Goldilocks prime is p = 0xFFFF_FFFF_0000_0001. Since 2⁵⁶ − 1 < p, every 7-byte value is a valid field element without reduction. No conditional splitting, no branching, no overflow handling. The encoding is a single `u64::from_le_bytes` with a zero high byte — branchless, constant-time, and injective.

At 8 bytes per element, approximately 1 in 2³² inputs would require splitting (when the value ≥ p), making encoding data-dependent and variable-length. The 7-byte encoding trades 12.5% rate reduction (56 vs 64 bytes per block) for unconditional simplicity. At planetary scale, branch-free encoding is worth one extra permutation per 8 rate blocks.

Rate block: 8 elements × 7 bytes = **56 input bytes** per absorption. One permutation processes 56 bytes of content.

#### 4.4.2 Output Encoding (Field Elements → Bytes)

Output uses the full canonical u64 representation: **8 bytes per element**, little-endian. Output elements are guaranteed to be in [0, p) by the permutation — no reduction needed.

```
element 0 → bytes[0..8]     (u64 to LE bytes)
element 1 → bytes[8..16]
...
element 7 → bytes[56..64]   (= 64-byte hash output)
```

The asymmetry — 7 bytes in, 8 bytes out — is deliberate. Input encoding must be injective for collision resistance. Output encoding must preserve full field element fidelity for algebraic composability. These are different constraints with different optima.

#### 4.4.3 Padding (Hemera: 0x01 ∥ 0x00*)

After all input bytes are buffered:

1. Append a single `0x01` byte (padding marker)
2. Pad with `0x00` bytes to fill the rate block (56 bytes total)
3. Encode the padded block as 8 field elements and absorb
4. Store total input byte count in `state[10]` (capacity length field)

The padding is rate-aligned: every message, regardless of length, ends with exactly one padded absorption. The 0x01 marker distinguishes `message ∥ 0x00` from `message` — standard multi-rate padding adapted to the 7-byte element encoding.

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

Chunking rule: Content is split into fixed 4 KB chunks (4096 bytes). Each chunk is hashed via `hash_leaf(data, counter, is_root)` — the two-pass construction defined in §4.6.2. At 56 input bytes per rate block, one 4 KB chunk requires ⌈4096/56⌉ = 74 absorptions. The last chunk may be shorter than 4096 bytes; its sponge pads normally. No content-defined chunking — identical byte ranges always produce identical chunks.

Why 4 KB and not some other size. The chunk size must be a multiple of 64 bytes (Hemera's absorb block). Among powers of two — 256 B, 1 KB, 4 KB, 8 KB, 16 KB, 64 KB — only 4 KB simultaneously satisfies every constraint:

1. Field alignment. 4096 bytes = 2¹² bytes, consistent with the power-of-2 design philosophy. At 56 bytes per rate block (7-byte encoding), one chunk requires 74 absorptions + 1 structural binding = 75 permutations per leaf. While 75 is not a power of 2, the chunk size itself is — and it is the chunk size (a data structure boundary) that must align cleanly with memory, I/O, and the OS, not the permutation count (a derived runtime cost).

2. OS page alignment. 4 KB is the virtual memory page size on x86 (since 1985), ARM (since 1987), and RISC-V (since 2010). It is the default block size of ext4, XFS, NTFS, and APFS. It is the minimum addressable unit on NVMe drives. `mmap()` reads and writes align to page boundaries without buffering. This means zero-copy I/O between storage and hash function — the OS delivers content in units that map directly to Hemera chunks with no intermediate buffering.

3. L1 cache fit. 4 KB fits in the L1 data cache of every modern CPU (typically 32–64 KB). The entire chunk can be hashed in cache-resident memory. At 8 KB, cache pressure increases; at 16 KB, the chunk exceeds L1 on many architectures and performance degrades from cache misses during hashing.

4. STARK proof granularity. One 4 KB leaf requires 75 permutations × ~1,200 constraints = ~90,000 constraints. This is small enough for efficient recursive proof composition but large enough that proof overhead does not dominate content. At 1 KB (~24,000 constraints per leaf), proof metadata costs approach content costs. At 64 KB (~1.4M constraints per leaf), individual leaf proofs become expensive.

5. Tree depth and proof size. Practical scaling at 4 KB chunks:

```
Content size    Leaves      Tree depth    Proof size
────────────    ──────      ──────────    ──────────
1 MB            256         8             512 B
1 GB            262,144     18            1,152 B
1 TB            268M        28            1,792 B
1 PB            274B        38            2,432 B
1 EB  (10¹⁸)   2.4×10¹⁴    48            3,072 B
1 ZB  (10²¹)   2.4×10¹⁷    58            3,712 B
1 YB   (10²⁴)  2.4×10²⁰    68            4,352 B
```

the content tree scales to 1 YB (10²⁴ bytes) at depth 68, proof 4.25 KB — one chunk plus one proof still fits in a single jumbo frame.

the system-level target is 10²⁴ [[cyberlinks]]. cyberlinks do not live in a single content tree — they are indexed by the [[BBG]]'s polynomial commitments (WHIR), batched into NMT blocks for DAS, and tracked in the AOCL MMR for UTXO lifecycle. the deepest tree at 10²⁴ cyberlinks is the MMR: at ~10²³ signals (each bundling ~10 cyberlinks), the tallest MMR peak has depth ⌈log₂(10²³)⌉ ≈ 77, proof size 77 × 64 = 4,928 B. still fits a jumbo frame. NMT blocks are per-epoch (depth ~14 for ~10K cyberlinks per block). FRI trees are per-proof (depth ~10-20). no tree in the system exceeds depth ~80 at planetary scale.

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

10. Bounded locality. Changing one byte in content requires rehashing one 4 KB chunk (75 permutations: 74 absorptions + 1 binding) plus log₂(N) parent nodes up to the root (2 permutations each). For 1 GB content: 75 + 2×18 = 111 permutations. For 1 TB: 75 + 2×28 = 131 permutations. The local cost (75 permutations) dominates; tree traversal is negligible. At 64 KB chunks, the local cost would be ~1,171 permutations — 16× worse locality.

Comparison table:
```
                    256B   1KB     4KB     8KB    16KB    64KB
                    ────   ────   ─────   ────   ─────   ─────
Absorbs/chunk         5     19      74    147     293    1171
Perms/leaf (†)        6     20      75    148     294    1172
1GB tree depth       22     20      18     17      16      14
1GB proof (bytes)  1408   1280    1152   1088    1024     896
Overhead ratio       25%     6%    1.6%   0.8%    0.4%    0.1%
OS page aligned       ✗      ✗      ✓      ✗       ✗       ✗
L1 cache fit          ✓      ✓      ✓      ~       ✗       ✗
STARK constraints   7.2K  24.0K   90.0K   178K   353K    1.4M
Streaming buffer   256B     1K      4K     8K     16K     64K
Dedup quality      poor   fair    good   good    fair    poor
Network packets       1      1       3      6      12      46

(†) Includes absorptions + padding + structural binding permutation
```

4 KB is the only row with ✓ on both page alignment and L1 cache fit, practical proof size, and meaningful deduplication. The convergence is not forced — it is the unique point where field arithmetic, hardware reality, and graph properties intersect.
#### 4.6.2 Leaf Hashing: `hash_leaf(data, counter, is_root) → Hash`

A leaf chaining value is computed in two passes. The first pass hashes the chunk data through the plain sponge. The second pass binds the hash to its structural position via a single flag-injection permutation.

```
Pass 1 — content hashing:
    hasher ← Hasher::new()              (plain sponge, domain_tag = 0x00)
    hasher.absorb(data)
    base_hash ← hasher.finalize()       (64-byte sponge output)

Pass 2 — structural binding:
    state ← [0; 16]
    state[0..8] ← bytes_to_elements(base_hash)    (8 Goldilocks elements)
    state[8]    ← counter                          (chunk position, 0-based)
    state[9]    ← FLAG_CHUNK | (FLAG_ROOT if is_root)
    state ← permute(state)
    output ← elements_to_bytes(state[0..8])        (64-byte chaining value)
```

Why two passes? The sponge is a general-purpose primitive that knows nothing about trees. The flag-injection is a tree-level concern. Separating them means: (1) the sponge implementation is pure and reusable, (2) tree logic is layered on top without modifying the sponge, (3) `base_hash` is cacheable — if the same chunk appears at different positions, only the cheap second pass (one permutation) differs.

The counter in `state[8]` prevents chunk reordering: the same data at position 0 and position 5 produces different chaining values. The CHUNK flag prevents leaf/node confusion: a 128-byte chunk can never be mistaken for an internal node because CHUNK (0x04) and PARENT (0x02) are distinct flags in a region the sponge never touches with input data. The ROOT flag distinguishes root finalization: a single-chunk file (is_root=true) hashes differently from the same chunk when it is leaf 0 of a multi-chunk file (is_root=false).

Cost: N absorptions (for data) + 1 permutation (for binding). At 4096-byte chunks with 7-byte encoding: ⌈4096/56⌉ = 74 absorptions + 1 binding = 75 permutations per leaf.

#### 4.6.3 Internal Node Hashing: `hash_node(left, right, is_root) → Hash`

A parent chaining value combines two child hashes (each 64 bytes = 8 field elements) through two sponge absorptions with flags pre-loaded in capacity. No padding — the input is always exactly 128 bytes.

```
state ← [0; 16]
state[9] ← FLAG_PARENT | (FLAG_ROOT if is_root)

// Absorb left child (8 elements = one full rate block)
state[0..8] += bytes_to_elements(left)     (field addition, element-wise)
state ← permute(state)

// Absorb right child (8 elements = one full rate block)
state[0..8] += bytes_to_elements(right)    (field addition, element-wise)
state ← permute(state)

output ← elements_to_bytes(state[0..8])    (64-byte chaining value)
```

No padding step. The input to `hash_node` is always exactly two 64-byte hashes — always exactly two rate blocks. Padding exists to disambiguate variable-length inputs; here the length is fixed by construction. Omitting padding saves one absorption and eliminates a codepath that can never vary.

The PARENT flag (0x02) in `state[9]` is set before the first absorption. It domain-separates internal nodes from all other hash contexts. Since flags live in the capacity region (state[8..16]) and absorption only touches the rate region (state[0..8]), the flag is preserved through both permutations — it is mixed into the output but never overwritten by input.

Order matters: `hash_node(A, B)` ≠ `hash_node(B, A)`. Left is absorbed first, right second. The sponge state after absorbing left carries forward into the right absorption. The tree structure is committed, not just the child hashes.

Cost: 2 permutations per internal node.

#### 4.6.4 Namespace-Aware Parent: `hash_node_nmt(left, right, ns_min, ns_max, is_root) → Hash`

[[NMT]] (Namespace Merkle Tree) nodes carry namespace bounds — the minimum and maximum namespace values in their subtree. these bounds enable completeness proofs: "these are ALL entries in namespace N, nothing was withheld." the namespace bounds must be committed into the hash, not carried as external metadata.

```
state ← [0; 16]
state[9]  ← FLAG_PARENT | (FLAG_ROOT if is_root)
state[12] ← ns_min        (minimum namespace in subtree)
state[13] ← ns_max        (maximum namespace in subtree)

// Absorb left child (8 elements = one full rate block)
state[0..8] += bytes_to_elements(left)     (field addition, element-wise)
state ← permute(state)

// Absorb right child (8 elements = one full rate block)
state[0..8] += bytes_to_elements(right)    (field addition, element-wise)
state ← permute(state)

output ← elements_to_bytes(state[0..8])    (64-byte chaining value)
```

when `ns_min = ns_max = 0`, `hash_node_nmt` reduces to `hash_node`. content trees, MMR, and FRI commitment trees all use `hash_node` (zero namespace bounds). only NMT uses non-zero namespace bounds. the capacity layout carries the distinction — no separate function, no mode switch, just different values in the same fields.

the verifier checking a namespace completeness proof reconstructs `hash_node_nmt` at each tree level, verifying that namespace bounds narrow correctly from root to leaf: `parent.ns_min ≤ left.ns_max < right.ns_min ≤ parent.ns_max` (for sorted NMT). a single `hash_node_nmt` call costs 2 permutations — same as `hash_node`.

#### 4.6.5 Tree Shape

Binary, left-balanced, in-order indexed. For N chunks:

```
If N = 1:     hash_leaf(data, 0, is_root=true)  is the root
If N > 1:     split = 2^(⌈log₂(N)⌉ - 1)
              left  = tree_hash(chunks[0..split],       is_root=false)
              right = tree_hash(chunks[split..N],        is_root=false)
              root  = hash_node(left, right, is_root=true)
```

Left-balanced means the left subtree is always a complete binary tree (power-of-2 leaves). This ensures that the same content prefix always produces the same left subtree hash regardless of what follows — enabling incremental hashing and prefix deduplication.

In-order indexing: Leaves are at even positions (0, 2, 4, ...). Parents are at odd positions with level = trailing_ones(index). This compact representation enables O(1) parent/child navigation without storing an explicit tree.

#### 4.6.6 Root Finalization

Every tree has exactly one root, marked by `FLAG_ROOT = 0x01`:

- Single-chunk file: `hash_leaf(data, 0, is_root=true)` — leaf IS the root, flags = `CHUNK | ROOT = 0x05`
- Multi-chunk file: `hash_node(left, right, is_root=true)` — top parent IS the root, flags = `PARENT | ROOT = 0x03`

Non-root leaves have flags = `CHUNK = 0x04`. Non-root parents have flags = `PARENT = 0x02`. The root flag ensures that a subtree hash (used internally during tree construction) never collides with a file hash (the externally-visible content identifier). This prevents a valid subtree from being presented as a valid standalone file.

#### 4.6.7 Security Properties

The tree construction provides the following guarantees:

| Attack | Defense | Mechanism |
|---|---|---|
| Leaf/node confusion | Prevented | CHUNK (0x04) vs PARENT (0x02) in capacity |
| Chunk reordering | Prevented | Counter in state[8] binds position |
| Chunk duplication | Prevented | Counter distinguishes identical chunks at different offsets |
| Subtree substitution | Prevented | ROOT flag separates file identity from subtree identity |
| Length extension | Prevented | Length in state[10] (sponge) + counter (binding) |
| Second preimage via tree | Prevented | All of the above, combined |

These properties hold unconditionally — they follow from the capacity layout, not from any assumption about hash output distribution.

#### 4.6.8 Complete Example

A 12 KB file (3 chunks at 4096 bytes each):

```
Content: [chunk_0: 4096B] [chunk_1: 4096B] [chunk_2: 4096B]

Step 1 — Leaf chaining values:
    cv_0 = hash_leaf(chunk_0, counter=0, is_root=false)   flags=0x04
    cv_1 = hash_leaf(chunk_1, counter=1, is_root=false)   flags=0x04
    cv_2 = hash_leaf(chunk_2, counter=2, is_root=false)   flags=0x04

Step 2 — Left subtree (complete, power-of-2):
    left = hash_node(cv_0, cv_1, is_root=false)           flags=0x02

Step 3 — Root (left-balanced: left subtree has 2 leaves, right has 1):
    root = hash_node(left, cv_2, is_root=true)             flags=0x03

The file's Hemera address is `root` — 64 bytes.
```

Consequence: Any node, anywhere, hashing the same content bytes, produces the same root hash, the same intermediate node hashes, and the same leaf hashes. Subtree addresses are globally stable and can be used for deduplication, caching, and verified streaming without coordination.

#### 4.6.9 Performance

Cost breakdown for a file of size S bytes:

```
Leaves:   ⌈S / 4096⌉ chunks × (⌈4096/56⌉ + 1) = N × 75 permutations
Parents:  (N − 1) internal nodes × 2 = 2(N − 1) permutations
Total:    75N + 2(N − 1) ≈ 77N permutations

For 1 GB:  N = 262,144 → ~20.2M permutations
For 1 TB:  N = 268M    → ~20.6B permutations
```

The 2× Merkle cost (two permutations per internal node instead of one with compression mode) is recovered through caching (subtree hashes are stable and reusable), incremental updates (changing one chunk only recomputes its path to root), and parallelism (all leaves hash independently, tree levels are embarrassingly parallel). The performance is bought back through architecture, not by introducing a second mode.

Incremental update cost: Changing one byte requires rehashing one 4 KB chunk (75 permutations) plus log₂(N) parent nodes to the root (2 permutations each). For 1 GB: 75 + 2×18 = 111 permutations. The local cost dominates; tree traversal is negligible.

#### 4.6.10 The Universal Tree Primitive

`hash_node` is the universal internal node combiner for every tree structure in [[cyber]]. the tree specification is not just "how to hash big files" — it is the foundation the entire proof system and graph database stand on.

four tree types, one `hash_node`:

| tree type | shape | used by | leaves contain | `hash_node` variant |
|---|---|---|---|---|
| content tree | left-balanced binary, 4 KB chunks | [[particle]] addressing, verified streaming | content chunks via `hash_leaf` | `hash_node` (standard) |
| MMR | append-only mountain range | AOCL in [[BBG]] Layer 4, SWBF inactive chunks | addition records, bloom filter chunks | `hash_node` (standard) |
| NMT | balanced binary, sorted by namespace | data availability sampling, namespace sync | namespace-tagged [[cyberlinks]] | `hash_node_nmt` (namespace bounds) |
| FRI commitment | balanced binary | [[WHIR]] polynomial commitments inside every [[STARK]] proof | polynomial evaluations at each folding round | `hash_node` (standard) |

the first three serve the [[BBG]] — the graph database. the fourth serves [[WHIR]] — the polynomial commitment scheme inside every [[STARK]] proof. together they cover every tree-structured commitment in the system.

how `hash_node` enters each structure:

content tree. the tree defined in §4.6.1–§4.6.9. `hash_leaf` for leaves, `hash_node` for internal nodes. the root is the [[particle]] address. streaming verification via pre-order traversal.

MMR (Merkle Mountain Range). the append-only commitment list ([[BBG]] Layer 4) stores UTXO addition records. appending a new record creates a new leaf and merges peaks via `hash_node`. the MMR accumulator is O(log N) peak hashes. membership proofs are standard Merkle paths using `hash_node` at each level. cost: ~8,000 constraints for AOCL membership (from [[cyber/proofs]]) = ~13 `hash_node` calls in-circuit for a tree of depth 26 (~67M entries).

NMT (Namespace Merkle Tree). the [[BBG]]'s data availability layer organizes [[cyberlinks]] by namespace. each internal node commits to the namespace range of its subtree via `hash_node_nmt` (§4.6.3.1). this enables namespace-aware DAS: a light client requesting "give me everything for namespace N" receives data plus a completeness proof — the namespace bounds at each tree level cryptographically prove nothing was withheld. the same 2-permutation cost as standard `hash_node`, with namespace bounds riding in capacity fields that are otherwise zero.

FRI commitment trees. [[WHIR]] is the multilinear polynomial commitment scheme inside every [[STARK]] proof. FRI (Fast Reed-Solomon IOP) works by:
1. committing polynomial evaluations into a Merkle tree using `hash_node`
2. folding the polynomial (halving degree via random challenge)
3. committing folded evaluations into another Merkle tree using `hash_node`
4. repeating until the polynomial is small enough to send directly

the verifier checks random positions via Merkle inclusion proofs at each FRI round — each inclusion proof is a chain of `hash_node` calls. this is why the [[cyber/proofs]] table shows polynomial inclusion at ~1,000 constraints vs Merkle inclusion at ~9,600: FRI's algebraic folding replaces most hash-path verification with field arithmetic (cheap), but the binding commitment at each round is still a `hash_node` Merkle tree.

WHIR low-degree testing (~10,000 constraints) involves multiple FRI rounds, each with its own `hash_node` commitment tree. proof aggregation (~70,000 constraints) includes WHIR verification, which includes FRI Merkle trees. at every level of the [[cyber/proofs]] stack — from a single [[cyberlink]] to recursive proof composition — `hash_node` is the tree primitive.

the relationship between tree types:

```
hash_node (standard)
  ├── content tree     →  particle identity (§4.6)
  ├── MMR              →  AOCL peaks, SWBF chunks (BBG Layer 4)
  └── FRI commitment   →  WHIR polynomial commitments (every STARK)

hash_node_nmt (namespace-aware, extends hash_node)
  └── NMT              →  namespace DAS, completeness proofs (BBG DA layer)
```

all four tree types share one security analysis: the capacity-based domain separation (§4.6.7) prevents cross-tree confusion. a content tree internal node (flags = PARENT, ns_min = ns_max = 0) cannot collide with an NMT internal node (flags = PARENT, ns_min/ns_max non-zero) because different capacity values produce different permutation outputs even for identical rate inputs. FRI and MMR trees use the same `hash_node` as content trees — they are distinguished by what their leaves contain (polynomial evaluations vs addition records vs content chunks), not by the combining operation.

the constraint cost table for in-circuit tree verification:

```
operation                          hash_node calls    constraints
─────────────────────────────────  ────────────────   ───────────
content Merkle inclusion (d=32)    32                 ~9,600
MMR membership (AOCL, d=26)        26                 ~8,000
NMT completeness (d=20)            ~40 (path + bounds) ~12,000
FRI opening (per round, d=10)       10                 ~3,000
WHIR polynomial inclusion           ~3 FRI rounds      ~1,000
```

one primitive. one security proof. every tree in [[cyber]].

### 4.7 Operational Semantics

Hemera serves every hashing role in [[cyber]] through one function:

[[particle]] addressing. Small content (≤ 4096 bytes): `address = hash_leaf(content, 0, is_root=true)`. Large content: split into 4 KB chunks, build left-balanced Merkle tree via `hash_leaf` + `hash_node`; the [[particle]] address is the tree root.

[[cyberlink]] identity. `edge_id = Hemera(neuron_id ∥ source ∥ target ∥ weight ∥ time)`. Structured field data serialized to bytes, hashed through sponge.

Merkle proofs. Leaf and internal node hashes use the same sponge and permutation. Proof verification is a uniform chain of `hash_leaf` and `hash_node` calls — no mode switching, no type disambiguation. The flags in capacity bind each hash to its structural role automatically.

Incremental hashing. The sponge state (16 field elements = 128 bytes) is a complete checkpoint. Save it, resume later, get the same result as a single-pass hash. Nodes can hash content arriving over the network in chunks without buffering.

Streaming verification. Receive content chunk by chunk, verify each chunk against a Merkle proof, process immediately. Never buffer more than one chunk + proof. Reject invalid chunks before storing anything.

MMR peaks. the AOCL (append-only commitment list) in [[BBG]] Layer 4 is a Merkle mountain range. each append creates a leaf and merges peaks via `hash_node`. the MMR accumulator — O(log N) peak hashes — is part of the BBG root. UTXO membership proofs are `hash_node` paths from leaf to peak.

NMT commitments. the data availability layer organizes block data by namespace. `hash_node_nmt` commits namespace bounds at each tree level, enabling completeness proofs for namespace-aware DAS. a light client syncing one namespace receives cryptographic proof that nothing was withheld.

FRI/WHIR polynomial commitments. every [[STARK]] proof in [[cyber]] uses [[WHIR]], which internally commits polynomial evaluations via `hash_node` Merkle trees at each FRI folding round. `hash_node` is the binding primitive inside every proof — from a single [[cyberlink]] to recursive proof composition.

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
- The sponge begins by absorbing the seed into the zero state via field addition
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
| Input bytes per permutation | 56 | 56 | 1.00× |
| Estimated hash rate | ~53 MB/s | ~86 MB/s | 0.62× |
| Perms for 1 KB | 19 | 19 | 1.00× |

The 38% native hash rate reduction comes from the wider permutation and additional partial rounds. Throughput per permutation is identical because both use rate=8 with 7-byte encoding (56 input bytes per block). Partial rounds are lightweight (~19 field multiplications each vs ~304 for full rounds), so even at R_P=64, they account for only ~1,216 of the total ~3,648 field multiplications per permutation (33%).

### 6.2 Proving Cost

STARK trace width increases from 12 to 16 columns. Trace length increases from 30 to 72 rows per permutation:

- Wider state: ~1.33× proving cost
- More rows (72 vs 30): ~2.40× proving cost
- Combined: ~3.2× proving cost per hash vs Plonky3 baseline
This is the real cost of permanent-grade security. However, hash proving is not the bottleneck in cyber/core — tri-kernel ranking, consensus, and network I/O dominate computational load. If hashing accounts for ~20% of total proving time, the system-level impact is ~44% more total proving work. If hashing is ~40% (Merkle-heavy workloads like storage proofs), the impact is ~88%.

### 6.3 Steady-State Adequacy

At 10¹⁵ particles with 1% annual update rate:
- Required: ~317K particles/sec
- Hemera at ~53 MB/s, 200-byte average particle: ~265K particles/sec per core
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
/// One sponge. No compression mode. Structured capacity for tree binding.

// ── Sponge API ────────────────────────────────────────────────
pub struct Hasher { /* sponge state + buffer */ }

impl Hasher {
    pub fn new() -> Self;                           // domain_tag = 0x00
    pub fn new_keyed(key: &[u8; 64]) -> Self;       // domain_tag = 0x01
    pub fn update(&mut self, data: &[u8]) -> &mut Self;
    pub fn finalize(&self) -> Hash;                 // squeeze 8 elements = 64 bytes
    pub fn finalize_xof(&self) -> OutputReader;     // extendable output
}

// ── Tree API ─────────────────────────────────────────────────
pub fn hash_leaf(data: &[u8], counter: u64, is_root: bool) -> Hash;
pub fn hash_node(left: &Hash, right: &Hash, is_root: bool) -> Hash;
pub fn hash_node_nmt(left: &Hash, right: &Hash, ns_min: u64, ns_max: u64, is_root: bool) -> Hash;
pub fn root_hash(data: &[u8]) -> Hash;
pub fn build_tree(data: &[u8]) -> TreeNode;
pub fn prove(data: &[u8], chunk_index: u64) -> (Hash, InclusionProof);
pub fn prove_range(data: &[u8], start: u64, end: u64) -> (Hash, InclusionProof);
pub fn verify_proof(chunk_data: &[u8], proof: &InclusionProof, root: &Hash) -> bool;
pub fn verify_node_proof(node_hash: &Hash, proof: &InclusionProof, root: &Hash) -> bool;

// ── Key derivation ────────────────────────────────────────────
pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; 64];

// ── Output type ───────────────────────────────────────────────
pub struct Hash([u8; 64]);  // 8 Goldilocks elements, LE canonical
```

Deliverables: `cyber-hemera` Rust crate, test vectors JSON, cross-validation with SageMath reference.

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
- [[cyber/bbg]] — graph database whose every tree structure uses `hash_node`
- [[WHIR]] — polynomial commitment scheme whose FRI trees use `hash_node`
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
