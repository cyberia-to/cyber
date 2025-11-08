- long-term integrity: 64-byte (512-bit) digests provide full 256-bit post-quantum pre-image margin (grover only halves the exponent). this keeps verification sound well past 2040 without needing a risky, late migration.
- migration risk: recomputing hashes later requires the original bytes. if some chunks are only available on third-party nodes, coordination and bandwidth become the bottleneck. starting at 64 bytes now avoids this systemic risk.
- negligible performance cost: blake3’s cost is dominated by message compression, not output size. moving from 32 to 64 bytes adds <5% cpu on large streams and only tens of nanoseconds on small items, while storage proofs roughly double but remain <2% of payload.
- privacy-by-default: publishing only anonymised node ids plus commitments to weights lets anyone audit topology without exposing personal balances or authorship by default. this resists “store now, decrypt later” and casual scraping.
- selective transparency: commitments are homomorphic. any stakeholder can later open chosen edges or identities and prove consistency with the canonical state, enabling targeted audits and disclosures without blanket deanonymisation.
- zk readiness: the ranking and audit logic runs inside zk circuits using poseidon2 (constraint-cheap), while storage and networking continue to use blake3 (hardware-fast). this separation keeps everyday ops fast and proofs practical.
- dedup and upgrade agility: fastcdc chunking plus content addressing preserves dedup across edits. multicodec tagging and auxiliary poseidon2 tags let us evolve circuit-level primitives without changing stored blake3 roots.
- operational simplicity: a single fixed digest width across all merkle nodes, proofs, and indices simplifies implementations, cache behavior, and protocol negotiation.

why these specific primitives
- blake3-xof-512 for storage: saturates modern simd, supports verified streaming, and scales linearly; choosing 64-byte output reclaims full quantum headroom at minimal cost.
- pedersen commitments for weights: perfectly hiding, additively homomorphic, mature libraries, no need to alter storage keys. later migration to kzg/ipa is possible via zk equivalence proofs.
- poseidon2 in circuits: field-friendly hash with low constraint count; we attach poseidon2 tags to blake3 nodes where zk needs to traverse merkle paths.
- anonymised node ids: poseidon2(pubkey ∥ salt) gives unlinkability by default and a clean path for voluntary identity disclosure.
- range proofs on weights: ensure values are non-negative and bounded, preventing pathological or adversarial inputs without revealing exact balances.

foundational choices
- content addressing: blake3-xof, fixed 64-byte digests for all merkle nodes and cids.
- chunking/dedup: variable-size fastcdc with target 32 kiB (min 4 kiB, max 256 kiB). each chunk is stored once and referenced by its 64-byte digest.
- storage encoding: bao-style verified streaming adapted to 64-byte digests.
- multicodec: register blake3-512-xof; cid strings in base32-lowercase.

confidentiality model
- public sees only an anonymised, weighted digraph: edges with commitments to weights and anonymised node identifiers.
- authorship and per-account contributions stay hidden by default.
- anyone can later reveal chosen subgraphs and prove they match the canonical state.

identities and node ids
- real identity key: ed25519/secp256k1 (off-chain).
- anonymised node id: poseidon2(pubkey ∥ salt) → field element.
- disclosure path: reveal salt + pubkey to bind a node id back to an identity when desired.

edge record (canonical on-chain/off-chain object)
- u′, v′: anonymised node ids (field elements).
- cₑ: pedersen commitment to total edge weight wₑ (additively homomorphic).
- πₑ: range proof that 0 ≤ wₑ < 2⁶⁴ (bulletproof or halo2 equivalent).
- leaf hash for storage: blake3-512(u′ ∥ v′ ∥ cₑ ∥ πₑ).

staking/deposit flow (confidential)
- depositor chooses random rᵢ and computes cᵢ = g^{wᵢ} h^{rᵢ}.
- relayer verifies a pedersen-opening proof for cᵢ against policy.
- relayer updates cₑ ← cₑ · cᵢ and republishes the updated leaf + merkle path.
- no public link to the depositor is recorded; only anonymised node ids and commitments are visible.

ranking with zk correctness
- inputs (public): anonymised adjacency list and per-edge commitments cₑ.
- inputs (private witness): openings of commitments (weights), salts for any node-ids needed inside the circuit.
- computation: run k power-iteration steps or chosen ranking algorithm using private weights.
- outputs: rank vector r (quantised) + poseidon2 root of r + zk proof π that r was computed exactly from the hidden weights and published topology.

selective disclosure of subgraphs
- reveal chosen edges by providing their pedersen openings (wᵢ, rᵢ) and verifying ∏ revealed cᵢ equals current cₑ.
- reveal authorship by disclosing the salt for affected node ids to map back to pubkeys.
- partial disclosure is zero-knowledge for everything not revealed; unopened edges remain perfectly hidden.

hashing inside vs outside circuits
- outside (fast path): blake3-xof-512 for all content-addressed storage and merkle nodes.
- inside zk (constraint-minimal): poseidon2 for merkle steps and vector commitments.
- glue: store poseidon2 tags of blake3 nodes where circuits need to traverse the same paths cheaply.

security notes
- integrity: 64-byte digests restore full 256-bit post-quantum pre-image margin (grover-resistant).
- hiding: pedersen commitments are perfectly hiding; range proofs prevent invalid or extreme values.
- anonymity: node ids derive from salted hashes; unlinkable until the salt is disclosed by the owner.
- downgrade resistance: clients that understand commitments/zk must reject content missing required fields once the flag-day is reached.

performance and overhead
- hashing throughput: blake3 remains ~7–10 gb/s per core for large streams; 64-byte outputs add <5% cpu.
- metadata growth: +32 bytes per commitment and ~50 bytes per aggregated range proof per edge (implementation-dependent).
- indexes/outboards: double due to 64-byte digests; still <2% overhead relative to payload for typical chunk sizes.
- zk proving: dominated by poseidon and linear algebra; practical for ~10⁵ edges per proof on today’s hardware; shard larger graphs across multiple proofs.

interoperability and migration
- genesis uses only 64-byte digests; no dual-hash compatibility path required.
- if a new field-friendly hash supersedes poseidon2, add an auxiliary tag without touching blake3 roots.
- commitments can migrate (pedersen ↔ kzg) via a zk equivalence proof without re-indexing storage.

implementation checklist
- update hashing and bao to fixed 64-byte outputs.
- implement fastcdc chunker and content-addressed store keyed by 64-byte blake3 digests.
- define edge record schema and merkle layout.
- integrate pedersen commitments and range proofs for weights.
- build poseidon2 merkle helpers and rank circuit (groth16/plonk/stark as appropriate).
- define selective disclosure api (openings, salt reveal, audit routines).
- set a flag-day after which clients must enforce presence/validity of commitments and range proofs.

open questions / to decide
- exact ranking algorithm parameters (α, iterations k, damping, normalisation).
- choice of proof system (groth16 vs plonk vs stark) based on ecosystem and performance.
- range-proof system (bulletproofs, plonkish custom gates, or halo2 native) and aggregation strategy.
- privacy budget for node-id salts and rotation policy.
- governance policy for subgraph disclosures and audit procedures.

appendix: example edge object (informal)

```
edge {
  uid: blake3-512(u′ ∥ v′ ∥ cₑ ∥ πₑ)
  from: u′            // poseidon2(pubkey ∥ salt)
  to:   v′            // poseidon2(pubkey ∥ salt)
  commit: cₑ          // pedersen commitment to total weight
  range_proof: πₑ     // non-negative, bounded
  tags: [poseidon2(uid)] // optional, for in-circuit traversal
}
```