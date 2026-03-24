---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: Hemera hash
stake: 31195035580345060
subgraph: true
repo: ../hemera
exclude: ".claude/**, target/**, bench/target/**"
diffusion: 0.0014078565592466588
springs: 0.00012848967675181902
heat: 0.0005629636161339202
focus: 0.0008550679058756842
gravity: 51
density: 0
---
Hemera is the [[hash function]] of the [[cyber]] protocol. It is a [[STARK]]-friendly hash built on [[Poseidon2]] over the [[Goldilocks field]], designed for efficient arithmetic circuit evaluation.

Every [[particle]] in the [[cybergraph]] receives a content identifier computed by Hemera. This content-addressed scheme ensures that identical data always maps to the same hash, enabling deduplication and integrity verification across the network.

Hemera powers the [[Merkle tree]] structures that commit to protocol state. Each state root is a Hemera digest, anchoring the entire graph in a single compact value that any [[neuron]] can verify.

Inside [[zheng]], the proof system of cyber, Hemera serves as the algebraic hash for [[commitment scheme]] operations. Its arithmetization-friendly design keeps proof generation fast and verification cheap.

The hash operates natively over the [[Goldilocks field]] (p = 2^64 - 2^32 + 1), making it a natural fit for the 64-bit arithmetic used throughout the protocol stack.

Hemera transforms raw knowledge into verifiable [[particles]], giving the [[cybergraph]] its cryptographic backbone.

discover all [[concepts]]