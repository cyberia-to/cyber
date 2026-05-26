---
tags: cyber, cip, article
crystal-type: process
crystal-domain: cyber
status: draft
---
# hash function selection for [[cybergraph]] [[particles]]

date: 2026-02-10
author: mastercyb
context: [[nox]] — content-addressed [[knowledge graph]] at planetary scale

---

## 1. Decision

Poseidon2 as the primary [[hash]] function for [[particle]] content addressing, with frozen parameters at [[genesis]] and algorithm-agile CID format enabling future migration contingent on storage proof infrastructure.

This is a pragmatic, not permanent choice. Poseidon2 is the best available option across the required capability surface. The architecture must assume this [[hash]] will eventually be replaced — but replacement is only possible if the content behind every CID remains retrievable. Storage and replication proofs are therefore a security-critical prerequisite, not a scaling optimization.

Parameters (round counts, MDS matrix, round constants) are frozen at deployment and never modified. Changing parameters changes the function, which changes every CID in the graph. Parameter updates are identity-breaking events. Conservative round counts with safety margins must be selected before [[genesis]] and treated as protocol constants thereafter.

---

## 2. Problem Statement

[[nox]]'s [[cybergraph]] needs a single canonical [[hash]] function to serve as the identity primitive for [[particles]] (content-addressed nodes). This [[hash]] must simultaneously satisfy requirements from seven distinct domains:

- Content addressing — deterministic, collision-resistant identity for all graph content
- Deduplication — identical content must map to one CID, eliminating storage and bandwidth waste at planetary scale
- [[Zero knowledge proofs]] — efficient arithmetization for stark-based [[verification]]
- Multi-party computation — viable for threshold operations and private collective computation
- Fully homomorphic encryption — compatible with encrypted [[knowledge graph]] queries
- Quantum resistance — survivable under quantum adversaries with Grover and algebraic quantum attacks
- Planetary scale — functional at 10¹⁵ nodes with bounded locality constraints

No [[hash]] function perfectly satisfies all seven. The question is which one covers the most ground with the fewest compromises.

---

## 3. Candidates Evaluated

### 3.1 Classical: Blake3, SHA-256

Strengths: Battle-tested (SHA-256: 23 years), extremely fast native execution (Blake3: ~2 GB/s), hardware acceleration, universal tooling, NIST standardization.

Fatal weakness: Catastrophic in ZK circuits. SHA-256 is 50–100× more expensive than arithmetization-oriented (AO) hashes when proved in starks. Bit-oriented operations (XOR, rotation, shift) that make these fast on CPUs become enormous constraint systems in arithmetic circuits. Every bit operation must be decomposed into [[field]] arithmetic, turning a simple [[hash]] into thousands of constraints.

Verdict: Eliminated. A system that cannot efficiently prove its own state transitions cannot achieve [[verification]] closure.

### 3.2 Algebraic (Lookup-based): Tip5, Monolith, Reinforced Concrete

Strengths: Tip5 achieves ~2.68× faster stark proving than Rescue-Prime in Triton VM. The split-and-lookup S-box design provides structural resistance to Groebner basis attacks (algebraic degree ≈ p ≈ 2⁶⁴).

Fatal weakness: The lookup S-box that gives Tip5 its ZK advantage makes it impossible for MPC and FHE. In MPC, you cannot "look up a table" on secret-shared data — the lookup must be represented as a degree-2⁶⁴ polynomial or an oblivious RAM protocol, both prohibitively expensive. In FHE, the same problem applies to encrypted data. Additionally, Tip5 is locked to the [[Goldilocks field]] while the proving ecosystem has moved to M31 and BabyBear for 2–4× faster proving.

Verdict: Eliminated. Specialist [[hash]] that excels in one domain (stark proving in Triton VM) while being architecturally incompatible with two critical domains (MPC, FHE).

### 3.3 MPC-Optimized: Hydra/Ciminion, RAIN

Strengths: Hydra (Grassi et al., EUROCRYPT 2023) is specifically designed for minimal MPC online communication cost. RAIN is tailored for MPC-in-the-Head proof systems.

Fatal weakness: Negligible ZK ecosystem adoption. No content-addressing usage. Not designed for general-purpose hashing. Cannot serve as a universal identity primitive.

Verdict: Eliminated. Too specialized.

### 3.4 FHE-Optimized: PASTA, Elisabeth, Rubato, LowMC

Strengths: Purpose-built for FHE transciphering with minimal AND-depth / multiplicative depth. PASTA achieves very low noise growth in homomorphic evaluation.

Fatal weakness: Same as MPC-optimized — negligible ZK ecosystem, no content-addressing usage, no general-purpose [[hashing]] capability.

Verdict: Eliminated. Too specialized.

### 3.5 Algebraic (Power-map): Poseidon2

Strengths: See Section 4.

Weaknesses: See Section 5.

Verdict: Selected. See Section 6 for rationale.

---

## 4. Poseidon2 — Capability Assessment

### 4.1 [[Zero Knowledge Proofs]]

| Metric | Value | Source |
|--------|-------|--------|
| Stwo (4-core i7) | 500,000 hashes/sec | StarkWare, Jul 2024 |
| Stwo (M3 Pro) | 620,000 hashes/sec | StarkWare, Jul 2024 |
| Plonky3 (M3 Max) | 1.7M hashes/sec | Lubarov, Oct 2024 |
| Plonky3 (optimized) | 2M+ hashes/sec | Polygon, Oct 2024 |

Poseidon2 is the fastest AO hash without lookups. The x⁷ power map S-box produces low-degree constraints (degree 7 per round), and the HADES partial-round structure minimizes total S-box count. Compression mode (not sponge) provides additional efficiency for Merkle tree computations — up to 5× improvement over Poseidon v1 in plain performance, ~30% in proving systems.

[[Field]] portability: Instantiated and benchmarked over [[Goldilocks field]] (2⁶⁴ − 2³² + 1), M31 (2³¹ − 1), BabyBear (2³¹ − 2²⁷ + 1), BN254, BLS12-381, and binary extensions (Poseidon2b, Jan 2026). This is unique — no other AO [[hash]] has been validated across this many fields.

Ecosystem: Ethereum L1 candidate (EF-backed [[cryptography]] program), Starknet, Polygon (Plonky3), Miden, SP1 (Succinct), Scroll (OpenVM), Zcash. Largest implementation base of any AO [[hash]].

### 4.2 Multi-Party Computation

The x⁷ S-box decomposes as x⁷ = ((x²)·x)²·x, requiring 3 sequential multiplications per S-box (multiplicative depth 3). For Shamir secret-sharing-based MPC, each multiplication requires one round of communication.

Adomnicăi et al. (IACR CiC, Jan 2026) benchmarked Poseidon2 hash chains in MPC with malicious-adversary security. Key findings:
- Instance (31, 16, 3) over 31-bit field achieves best MPC depth/preprocessing tradeoff
- Three-party threshold hash chains complete in <0.5 seconds at 1ms network latency
- Compression mode reduces state size versus sponge, further lowering multiplication count

Assessment: Not optimal (Hydra is purpose-built for MPC), but practical and benchmarked. Sufficient for threshold key generation, distributed state commitments, and private collective operations at the protocol level.

### 4.3 Fully Homomorphic Encryption

Poseidon2 operates natively over 𝔽ₚ, making it compatible with word-level FHE schemes (BGV, BFV, CKKS over matching fields). The multiplicative depth per [[hash]] invocation is bounded by rounds × depth-per-S-box = (R_F + R_P) × 3.

For a typical [[Goldilocks field]] instance (R_F = 8 full rounds, R_P = 56 partial rounds), total multiplicative depth ≈ 8×3 + 56×3 = 192. This is high for FHE — purpose-built ciphers like PASTA achieve depths of 4–8. However:

- Partial rounds apply only 1 S-box (not all 16), so effective noise growth is much lower than 192 sequential full-width multiplications
- Bootstrapping-capable FHE schemes (TFHE, FHEW) can handle this with periodic refresh
- The alternative (Tip5) is impossible for FHE, not merely expensive

Assessment: Viable but expensive. For FHE-heavy workloads, a hybrid approach using PASTA for transciphering and Poseidon2 for identity [[verification]] is the pragmatic path. The critical point: Poseidon2 *can* be evaluated under FHE, unlike Tip5 which categorically cannot.

### 4.4 Content Addressing

Poseidon2 in sponge mode provides a standard hash-to-digest function suitable for content addressing. Properties:

- Deterministic: Same input always produces same output (assuming canonical field element encoding)
- Collision resistant: 128-bit security against collision attacks (with current round counts)
- Preimage resistant: 128-bit security against preimage attacks
- Variable-length input: Sponge construction handles arbitrary-length inputs
- Fixed-length output: 5 [[Goldilocks field]] elements = 40 bytes
- Compression mode: Available for fixed-length inputs ([[merklezation]] internal nodes)

Critical requirement: Canonical byte-to-field-element encoding must be specified. For [[Goldilocks field]]: each [[field]] element holds ~7.5 bytes, padding and endianness must be deterministic and standardized. This encoding spec is as important as the [[hash]] function choice itself.

### 4.5 Deduplication

Content addressing provides deduplication by construction: identical content produces identical CIDs, so duplicate [[particles]] are impossible at the protocol level. This is a structural guarantee of any deterministic [[hash]] function, not a feature that requires additional engineering.

At planetary scale (10¹⁵ [[particles]]), deduplication is a storage and bandwidth survival requirement. Without it, redundant content multiplies storage costs, bloats replication proofs, and inflates [[merklezation]] overhead. With content-addressed identity, every unique piece of content exists exactly once in the graph regardless of how many [[neurons]] reference it.

Poseidon2's deterministic algebraic structure over a canonical [[field]] encoding guarantees that byte-identical content always maps to the same CID. The critical dependency is the canonical encoding specification (§10.2) — any ambiguity in byte-to-field-element mapping (endianness, padding) would produce different CIDs for identical content, silently breaking deduplication. This makes encoding canonicalization a deduplication-critical requirement.

Assessment: Fully satisfied by any deterministic [[hash]] function, including Poseidon2. The real risk is not the [[hash]] but the encoding layer — canonical encoding must be formalized and enforced at the protocol level before [[genesis]].

### 4.6 Planetary Scale

Poseidon2's compression mode enables efficient incremental [[merklezation]] updates. Combined with LtHash (additive homomorphic set commitment) for collection state, the architecture supports:

- O(1) state updates for set mutations (via LtHash)
- O(log n) Merkle proof paths for membership [[verification]]
- O(1) [[verification]] per proof step (single Poseidon2 compression)
- Bounded locality: all operations are local to the mutation point

Native [[hash]] rate on commodity hardware: ~50–100 MB/s over [[Goldilocks field]] (estimated). Slower than Blake3 by 20–40× for raw ingestion. Acceptable for steady-state operation but requires planning for initial bulk migration of existing content.

### 4.7 Quantum Resistance

A [[knowledge graph]] meant to persist for decades must account for quantum adversaries. Two quantum attack classes are relevant to hash functions:

Grover's algorithm: Generic quantum search that reduces n-bit preimage resistance to n/2 bits and collision resistance to n/3 bits. For Poseidon2 at 128-bit classical security, Grover yields ~64-bit preimage and ~43-bit collision security. Mitigation is straightforward: increase digest size. A 256-bit security target (5 [[Goldilocks field]] elements = 320 bits) provides 160-bit post-Grover preimage resistance and ~107-bit collision resistance — both adequate.

Algebraic quantum attacks: Poseidon2's low-degree S-box (x⁷) raises a subtler question. Quantum algorithms for solving low-degree polynomial systems (quantum Groebner basis, quantum linearization) could theoretically exploit the algebraic structure faster than classical attacks. Current research (Jang et al., "Quantum Algebraic Attacks on AO Hash Functions," 2024) suggests that quantum speedups for Groebner basis computation are polynomial, not exponential — the conservative round count margin from §9.2 (+25%) absorbs this.

stark compatibility: starks are inherently post-quantum — they rely on hash function collision resistance only, with no elliptic curve assumptions. This means [[nox]]'s entire proving stack (Poseidon2 inside stark proofs) remains sound under quantum adversaries, provided the [[hash]] itself holds. This is a structural advantage over SNARK-based systems that depend on pairing assumptions broken by Shor's algorithm.

Assessment: Poseidon2 with enlarged digest and conservative round counts provides viable quantum resistance. The stark-native architecture means [[nox]] avoids the pairing-based assumptions that make most ZK systems quantum-vulnerable. The combination of Poseidon2 + starks is among the strongest post-quantum positions available for a [[knowledge graph]] proving system. The remaining risk is algebraic quantum attacks against the S-box — mitigated by round count margins and the algorithm-agile CID format enabling migration if quantum algebraic breakthroughs materialize.

---

## 5. Poseidon2 — Security Analysis (Honest Assessment)

### 5.1 Cryptanalytic History

Poseidon2's security derives from the HADES design strategy (Poseidon v1: USENIX 2021, Poseidon2: AFRICACRYPT 2023). It is the most attacked AO [[hash]] function in existence, which is both concerning and reassuring.

Timeline of significant attacks:

| Date | Attack | Impact |
|------|--------|--------|
| 2020 | Out of Oddity (Beyne et al., CRYPTO) | Zero-sum distinguishers on full-round HadesMiMC |
| 2023 | ACISP (Ashur, Buschman, Mahzoun) | Groebner basis attack cheaper than claimed; security argument fails at ≥384-bit level |
| 2025 May | Graeffe transform (Sanso & Vitto, ePrint 2025/937) | 2¹³× wall-time improvement for interpolation attacks on round-reduced instances |
| 2025 May | Graeffe + FFT bounds (Zhao & Ding, ePrint 2025/950) | Broke EF bounty instances up to 40-bit security |
| 2025 Jun | Subspace trail GB (Grassi et al., ToSC 2025) | Found inaccuracies in original security model; refined round requirements; confirmed overall security |
| 2025 Oct | Combined Graeffe (ePrint 2025/1916) | Merged techniques, constant-factor improvements |
| 2026 Jan | Resultant-based (ePrint 2026/150) | Broke first instances of Poseidon2-31m and Poseidon2-31k challenges |

### 5.2 Current Security Status

Full-round Poseidon2 at 128-bit security: UNBROKEN.

All successful attacks target round-reduced instances in the Ethereum Foundation bounty program. The bounty program exists precisely to calibrate round counts — it's stress-testing, not breaking.

However, the pattern is concerning:
- Original security estimates were overoptimistic in some configurations
- New attack techniques keep providing constant-factor improvements
- The security argument at ≥384-bit levels has known gaps
- Round counts have been adjusted upward in response to findings

The Poseidon(2)b paper (Jan 2026) characterizes Graeffe-transform attacks as providing "only a constant factor" improvement — they don't change asymptotic security. The designers are actively updating parameters.

### 5.3 Comparative Security Assessment

| Hash | Age | Papers attacking it | Full-round broken? | Security confidence |
|------|-----|--------------------|--------------------|---------------------|
| SHA-256 | 23 years | Hundreds | No | Very high |
| Blake3 | 6 years | Dozens | No | High |
| Poseidon2 | 3 years | ~15–20 | No (at 128-bit) | Moderate |
| Tip5 | 3 years | ~5–8 | No | Moderate-low (less scrutiny) |
| Hydra | 3 years | ~3–5 | No | Low (minimal scrutiny) |

Poseidon2 has more cryptanalytic attention than any competitor in its class. This means more known weaknesses, but also more confidence that unknown weaknesses don't exist. The Ethereum Foundation is investing $130K specifically to break it.

### 5.4 Long-Term Bet

Would we bet [[nox]]'s permanent security on Poseidon2? No.

Five years of [[cryptography]] is insufficient for permanent trust. SHA-256 has 23 years. AES has 25 years. Confidence in [[hash]] functions comes from decades of failed attacks, not cleverness of design.

What we bet on instead: Algorithm agility. The CID format must support migration. Poseidon2 is the best available choice *today*, and the architecture must be designed so that "today" is the only timescale that matters.

---

## 6. Decision Rationale

### 6.1 The Generalist vs. Specialist Tradeoff

The alternative to Poseidon2 is a multi-hash architecture:
- Blake3 for fast content ingestion
- Tip5 for stark proving
- Hydra for MPC
- PASTA for FHE
- A lattice-based or SHA-3 construction for quantum resistance

This requires five [[hash]] functions, five identity systems, five trust assumptions, five security analyses, and a coherence nightmare. Two identities for the same content means no identity — and deduplication, which depends on a single canonical CID per content, becomes impossible across domains.

Poseidon2 is the only [[hash]] function that is viable (not optimal, but viable) across all seven required domains. For a system whose design principle is "purpose. link. [[energy]]." — one [[universal hash]] that works everywhere is worth more than five specialists.

### 6.2 Ecosystem Gravity

Poseidon2 has the largest ecosystem of any AO [[hash]]:
- Ethereum Foundation is evaluating it for L1 integration
- StarkWare (Stwo), Polygon (Plonky3), Succinct (SP1), Scroll (OpenVM) all use it
- Most implementations, most audits, most benchmarks, most papers
- [[Field]]-portable: works over [[Goldilocks field]], M31, BabyBear, BN254, BLS12-381, binary extensions

If Poseidon2 breaks, it breaks Ethereum's ZK roadmap. This means the strongest incentive structure in crypto is aligned with keeping it secure, and the fastest response capability will be deployed if issues arise.

### 6.3 The Immutability Constraint

Unlike execution-layer ZK systems where parameter updates are routine, content addressing demands permanent parameter commitment. This means:

- [[nox]] cannot benefit from post-deployment security improvements to Poseidon2
- Round counts must be chosen conservatively before [[genesis]], with margins for unknown future attacks
- The EF's ongoing [[cryptography]] program (through Dec 2026) should complete before [[nox]] freezes parameters
- Once frozen, [[nox]]'s Poseidon2 instantiation diverges from the broader ecosystem's evolving parameters — it becomes its own primitive

This is the fundamental cost of using an AO [[hash]] for content addressing. Classical hashes (SHA-256) have stable parameters because they're 23 years old. AO hashes are still in their parameter-discovery phase. [[nox]] must wait for parameter stabilization or accept the risk of choosing prematurely.

### 6.4 Storage Proofs as the Escape Hatch

The only migration path from Poseidon2 to any successor requires rehashing original content. This is impossible without guaranteed content availability. Therefore:

Storage proofs are the single highest-priority infrastructure component in [[nox]]. Without them, the [[hash]] function choice is irreversible and [[nox]] is permanently coupled to a 3-year-old primitive. With them, Poseidon2 becomes a replaceable component — the correct architectural relationship.

This dependency inverts the typical development sequence. Most blockchain projects build storage proofs after achieving [[consensus]] and execution. [[nox]] must build storage proofs *before* or *simultaneously with* the [[hash]] function deployment, because the [[hash]] function's survivability depends on them.

---

## 7. CID Format Specification

### 7.1 Structure

```
CID = [version | hash_algo | param_set_id | field_id | digest_length | digest]
```

| Field | Size | Description |
|-------|------|-------------|
| version | 1 byte | CID format version (0x01 initially) |
| hash_algo | 1 byte | Hash algorithm identifier |
| param_set_id | 1 byte | Exact frozen parameter instantiation (round counts, MDS, constants) |
| field_id | 1 byte | Finite field identifier |
| digest_length | 1 byte | Number of field elements in digest |
| digest | variable | Field elements in canonical encoding |

Critical invariant: A (hash_algo, param_set_id, field_id) triple uniquely and permanently defines a specific hash function. The function NEVER changes. New parameters create new triples.

### 7.2 Algorithm Registry

| ID | Algorithm | Status |
|----|-----------|--------|
| 0x01 | Poseidon2 (sponge) | Active |
| 0x02 | Poseidon2 (compression) | Active |
| 0x03 | Reserved (future AO hash) | — |
| 0xFE | Blake3 | Legacy/bridge only |
| 0xFF | SHA-256 | Legacy/bridge only |

### 7.3 Field Registry

| ID | Field | Size | Notes |
|----|-------|------|-------|
| 0x01 | Goldilocks (2⁶⁴ − 2³² + 1) | 8 bytes/element | Miden, Triton VM |
| 0x02 | M31 (2³¹ − 1) | 4 bytes/element | Stwo, Circle starks |
| 0x03 | BabyBear (2³¹ − 2²⁷ + 1) | 4 bytes/element | Plonky3, SP1, RISC Zero |
| 0x04 | BN254 scalar | 32 bytes/element | Ethereum L1 settlement |
| 0x05 | BLS12-381 scalar | 32 bytes/element | Zcash, Filecoin |

### 7.4 Canonical Encoding

For each field, the encoding must be deterministic:
- Byte order: Little-endian (matches all major implementations)
- Padding: Append 0x01 byte after content, then 0x00 bytes to fill final field element
- Alignment: Each field element uses exactly its field's byte width
- Normalization: All field elements must be in canonical range [0, p)

### 7.5 Example: [[Particle]] CID over [[Goldilocks field]]

Content: "hello" (5 bytes)

```
Encoding: [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x01, 0x00, 0x00] → 1 field element
Poseidon2 sponge hash (param_set_id=0x01) → 5 field elements × 8 bytes = 40 bytes digest

CID: [0x01, 0x01, 0x01, 0x01, 0x05, <40 bytes>] = 45 bytes total
       ver   algo  params field  len   digest
```

---

## 8. Commitment Layer Architecture

The [[hash]] function serves only as the identity layer. Higher-order properties (set membership, similarity, [[tri-kernel]]) are derived through the graph structure, not encoded in the CID.

```
Layer 0 — Identity (immutable, stored)
  Particle:   Poseidon2(content) → CID
  Cyberlink:  Poseidon2(from ∥ to ∥ weight ∥ neuron ∥ timestamp) → CID

Layer 1 — Collection State (derived, O(1) update)
  Neuron state:  LtHash(all CIDs of neuron's cyberlinks) → commitment
  Shard state:   LtHash(all neuron state commitments) → commitment

Layer 2 — Global State (derived, O(log S) update)
  Global root:   Poseidon2 Merkle tree over shard commitments

Layer 3 — Indices (derived, ephemeral, rebuildable)
  Similarity:    Embedding vectors stored as particles, linked via cyberlinks
  Ranking:       φ* (focus vector) computed by tri-kernel dynamics
  Search:        HNSW/IVF indices over embedding cyberlinks
```

Key principle: Similarity is a [[cyberlink]], not a CID property. Different [[neurons]] may map the same content to different similarity coordinates because similarity is subjective and context-dependent. The base layer stays clean — pure [[cryptographic proof]] of identity, no bloat.

Homomorphic property of Layer 1: LtHash over 𝔽ₚ provides:
- Add [[cyberlink]]: new_state = old_state + H(new_link). O(1).
- Remove [[cyberlink]]: new_state = old_state − H(old_link). O(1).
- Merge shards: merged = state_A + state_B. O(1).
- stark-provable: Addition is linear → free in arithmetization.

---

## 9. Parameter Immutability and the Content Addressing Constraint

### 9.1 The Fundamental Problem

AO hash functions are not static. The Poseidon2 designers have updated round counts in response to cryptanalytic findings in 2023, 2024, and 2025. The Ethereum Foundation's ongoing cryptanalysis initiative (Phase 2 through Dec 2026) may produce further updates. This is normal and healthy for execution-layer ZK systems.

For content addressing, it is catastrophic.

Content addressing requires a single, eternal function: H("hello") must produce the same CID today, in 5 years, and in 50 years. If round counts change, the function changes. If the function changes, the identity of every [[particle]] in the graph is broken. A parameter update in an identity-layer [[hash]] function is equivalent to a hard fork of all [[knowledge]].

Therefore: [[nox]] must freeze Poseidon2 parameters at [[genesis]] and never modify them. Whatever round counts, MDS matrix, and round constants are deployed — those become part of the protocol specification, as immutable as SHA-256's initial [[hash]] values.

### 9.2 Parameter Freezing Strategy

1. Wait for EF Phase 2 completion (Dec 2026). Do not freeze parameters based on current (potentially insufficient) round counts.
2. Choose conservative round counts. Add a safety margin of +25% rounds beyond the EF's final recommendation. The cost is slower hashing; the benefit is decades of margin against future cryptanalytic improvements.
3. Freeze permanently. Publish the exact parameter set (field, round counts, MDS matrix entries, round constants, S-box exponent) as an immutable protocol constant. This IS Poseidon2-nox. It does not change.
4. Encode in CID format. The CID includes a `param_set_id` that identifies the exact frozen instantiation:

```
CID = [version | hash_algo | param_set_id | field_id | digest]
```

If a second parameter set is ever needed (see §9.4), it creates a new identity space, not a mutation of the existing one.

### 9.3 Migration Requires Content Availability — Storage Proofs as Security Infrastructure

The only possible migration path from one [[hash]] function to another is to rehash the original content. You cannot compute Poseidon3(content) from Poseidon2(content) — you need the content itself.

This has a profound architectural consequence: storage and replication proofs are not a scaling feature — they are a security-critical prerequisite for hash function survivability.

Without storage proofs guaranteeing that every [[particle]]'s content remains retrievable, choosing Poseidon2 becomes a permanent, irreversible, single-point-of-failure commitment to a 3-year-old [[cryptography]] primitive at planetary scale. This is unacceptable under zero-tolerance-for-error principles.

The dependency chain:

```
Hash function may need replacement (honest assessment)
  → Replacement requires rehashing all content
    → Rehashing requires content availability
      → Content availability requires storage/replication proofs
        → Storage proofs are Phase 1 security, not Phase 3 optimization
```

Requirements for storage proof system:

- Coverage: Every [[particle]] in the graph must have at least k verified replicas (k ≥ 3 recommended)
- Continuous [[verification]]: Storage proofs must be checked periodically, not just at creation time
- Content-complete: Proofs must verify the actual content bytes, not just the CID (otherwise rehashing is impossible)
- Retrievability: The proof system must guarantee that content can be retrieved within bounded time, not just that it "exists somewhere"
- Incentive-aligned: [[Neurons]] storing content must be economically rewarded for maintaining availability, and penalized for loss

Without this system operational, [[nox]] has no escape path from Poseidon2. This makes storage proofs the single highest-priority infrastructure component in the entire architecture.

### 9.4 Hash Function Migration Protocol (Requires §9.3)

If and only if the storage proof system guarantees content availability, migration to a new hash function proceeds as follows:

1. New identity space. The new [[hash]] function gets a new `param_set_id`. It does not replace the old one — it creates a parallel identity layer.
2. Rehash campaign. Every [[particle]]'s content is retrieved from the storage network and rehashed under the new function. The new CID is linked to the old CID via a canonical bridge [[cyberlink]].
3. Dual-CID period. Both old and new CIDs are valid references. [[Cyberlinks]] can reference either. Proofs accept both during transition.
4. Cutoff. After full rehash coverage is verified, new content creation requires the new [[hash]]. Old CIDs remain valid as read-only historical references.
5. Estimated duration at scale: For 10¹⁵ [[particles]] at ~50 MB/s algebraic [[hash]] throughput, full rehash takes approximately 10¹⁵ × 100 bytes / 50 MB/s ≈ 2 × 10⁹ seconds ≈ 63 years on one core. Parallelized across 10⁶ nodes: ~17 hours. Storage proof coverage and network bandwidth become the bottleneck, not [[hash]] speed.

### 9.5 Emergency Response (Poseidon2 Broken)

If a practical attack breaks full-round Poseidon2 at 128-bit security:

1. Immediate: Freeze new [[particle]] creation. Existing CIDs remain valid — collision resistance may be weakened but existing content identity is not retroactively compromised.
2. 48 hours: Activate pre-staged fallback [[hash]]. The CID format's algorithm agility allows this without protocol redesign.
3. Weeks 1–4: Begin rehash campaign under new [[hash]] (requires storage proof system from §9.3).
4. Months 1–6: Complete migration. Old CIDs archived as historical references.

If storage proofs are not operational when this happens, [[nox]] cannot migrate. This is the single most important reason to prioritize storage proof implementation.

---

## 10. Open Questions

### 10.1 [[Field]] Choice — [[Goldilocks field]]

[[nox]] operates over the [[Goldilocks field]] (p = 2⁶⁴ − 2³² + 1). This determines the Poseidon2 instantiation and the proving ecosystem.

Rationale: [[Goldilocks field]] provides 64-bit native arithmetic on commodity hardware, is the native [[field]] of Triton VM (the [[trident]] compilation target), and has the deepest integration with [[nox]]'s proving stack. The 2-adicity (2³² | p−1) enables efficient NTT-based stark proving.

Standard Poseidon2 parameters over [[Goldilocks field]] (the most widely deployed configuration):

| Parameter | Sponge mode | Compression mode |
|-----------|-------------|------------------|
| State width (t) | 12 | 8 |
| Full rounds (R_F) | 8 | 8 |
| Partial rounds (R_P) | 22 | 22 |
| S-box | x⁷ | x⁷ |
| Security | 128-bit | 128-bit |
| Capacity | 4 elements | — |
| Rate | 8 elements | 8 elements |

These parameters are used by Plonky2, Miden VM (Poseidon2 variant), and the HorizenLabs reference implementation. [[nox]] should adopt these as the baseline, with the +25% round count margin from §9.2 applied before freezing at [[genesis]]. The exact frozen parameter set (including MDS matrix entries and round constants) must be published as an immutable protocol specification.

### 10.2 Canonical Encoding Specification

The byte-to-field-element encoding is as critical as the hash function itself. Needs formal specification covering:
- Padding scheme (multi-rate padding vs. 10*1 padding vs. domain separation)
- Endianness (little-endian consensus, but must be formalized)
- Maximum message length handling
- Domain separation tags for different content types

### 10.3 LtHash Security Parameters

LtHash over 𝔽ₚ needs:
- Output vector dimension (determines security level and commitment size)
- Inner hash function (Poseidon2 for individual element hashing?)
- Formal security reduction to lattice/field assumptions
- Concrete parameter selection for 128-bit security

### 10.4 Poseidon2 Round Count Finalization

The baseline parameters (R_F = 8, R_P = 22 over [[Goldilocks field]]) are the ecosystem default used by Plonky2 and Miden. The Ethereum Foundation's [[cryptography]] initiative (Phase 2 through Dec 2026) may result in updated round count recommendations. [[nox]] should track these findings and apply the +25% safety margin from §9.2 to the final EF-recommended counts before freezing at [[genesis]].

---

## 11. References

1. Grassi, Khovratovich, Schofnegger. "Poseidon2: A Faster Version of the Poseidon Hash Function." AFRICACRYPT 2023. ePrint 2023/323.
2. Grassi, Koschatko, Rechberger. "Poseidon and Neptune: Groebner Basis Cryptanalysis Exploiting Subspace Trails." ToSC 2025(2):34-86.
3. Ashur, Buschman, Mahzoun. "Algebraic Cryptanalysis of the HADES Design Strategy." ACISP 2024.
4. Sanso, Vitto. "Attacking Poseidon via Graeffe-Based Root-Finding over NTT-Friendly Fields." ePrint 2025/937.
5. Zhao, Ding. "Breaking Poseidon Challenges with Graeffe Transforms." ePrint 2025/950.
6. Zhao, Sanso, Vitto, Ding. "Graeffe-Based Attacks on Poseidon and NTT Lower Bounds." ePrint 2025/1916.
7. Grassi et al. "Poseidon(2)b: Binary Field Versions of Poseidon/Poseidon2." IACR CiC 2(4), Jan 2026.
8. Adomnicăi et al. "Towards Practical Multi-Party Hash Chains using AO Primitives." IACR CiC 2(4), Jan 2026.
9. Szepieniec et al. "The Tip5 Hash Function for Recursive starks." ePrint 2023/107.
10. Grassi et al. "From Farfalle to Megafono via Ciminion: The PRF Hydra for MPC Applications." EUROCRYPT 2023.
11. Ethereum Foundation Poseidon Cryptanalysis Initiative. poseidon-initiative.info. 2024–2026.
12. ePrint 2026/150. "Claiming bounties on small scale Poseidon and Poseidon2 instances using resultant-based algebraic attacks." Jan 2026.