---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: Hemera hash
---
Hemera is the [[hash function]] of the [[cyber]] protocol. It is a [[STARK]]-friendly hash built on [[Poseidon2]] over the [[Goldilocks field]], designed for efficient arithmetic circuit evaluation.

parameters: p = [[Goldilocks field|Goldilocks]], t = 16 (state width), Rf = 8 (full rounds, d=7 S-box x⁷), Rp = 16 (partial rounds, S-box x⁻¹), r = 8 (rate), c = 8 (capacity). 24 total rounds. single function, single mode (sponge), 32-byte output (4 field elements). ~736 constraints per permutation in a [[zheng]] proof. round constants via zero-constant permutation (Hemera₀).

every [[particle]] in the [[cybergraph]] receives a content identifier computed by Hemera. this content-addressed scheme ensures that identical data always maps to the same hash, enabling deduplication and integrity verification across the network.

Hemera powers the [[Merkle tree]] structures that commit to protocol state. each state root is a Hemera digest, anchoring the entire graph in a single compact value that any [[neuron]] can verify.

inside [[zheng]], the proof system of cyber, Hemera serves as the algebraic hash for [[commitment scheme]] operations. its arithmetization-friendly design keeps proof generation fast and verification cheap.

three implementations: rs (Rust), wgsl (GPU), cli (command-line). all cross-verify against shared test vectors.

Hemera transforms raw knowledge into verifiable [[particles]], giving the [[cybergraph]] its cryptographic backbone.

discover all [[concepts]]
