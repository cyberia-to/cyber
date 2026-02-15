---
tags: trident, cyber
alias: trident language, Trident
crystal-type: entity
crystal-domain: cyber
---
Smart contract language for [[neptune]] whose native data type is a [[Goldilocks field]] element. Every variable, every operation, every function call compiles to arithmetic over $\mathbb{F}_p$ where $p = 2^{64} - 2^{32} + 1$.

Compilation target is [[Triton VM]], which generates [[STARK]] proofs — hash-based, post-quantum secure, no trusted setup. Programs are arithmetic circuits by construction.

## the thesis

Three computational revolutions — zero-knowledge cryptography, artificial intelligence, and quantum computing — share a common algebraic foundation in prime field arithmetic. [[trident]] sits at the unique intersection because its native `Field` type simultaneously satisfies the requirements of all three worlds. See [[trinity]] for the conceptual overview, [[trident-trinity-zk-ai-quantum]] for the full thesis with competitive analysis.

## key primitives

- `divine()` — non-deterministic witness injection. For privacy: injects secret data. For AI: injects model weights. For quantum: injects measurement outcomes. Same mechanism, different semantics, one proof.
- bounded loops — every loop has a compile-time bound. This is simultaneously a ZK constraint (finite circuit), a neural network layer iterator, and a quantum circuit depth bound.
- lookup tables — the [[rosetta-stone]] mechanism. One table serves as cryptographic S-box, neural activation, FHE bootstrap function, and [[STARK]] authentication.

## standard library

[[trident-complete-stdlib]] specifies the full `std.*` architecture: 15 modules spanning foundation (`std.field`, `std.crypto`), three pillars (`std.nn`, `std.private`, `std.quantum`), three intersections, and three application layers.

## compilation targets

- [[Triton VM]] / TASM — [[STARK]] proof of correct execution (primary)
- Cirq qudit circuits — quantum hardware execution
- ONNX — ML ecosystem interoperability

## whitepapers

- [[trinity]] — three-pillar overview (quantum, privacy, AI)
- [[trident-trinity-zk-ai-quantum]] — full thesis with competitive analysis
- [[trident-ai-zkml-deep-dive]] — AI and zkML deep dive
- [[std-quantum-deep-dive]] — quantum standard library
- [[goldilocks-fhe-construction]] — FHE as fourth pillar
- [[privacy-trilateral]] — ZK + FHE + MPC privacy stack
- [[rosetta-stone]] — lookup table unification
- [[gfp-spec]] — [[GFP]] hardware specification
- [[CORE Master Plan]] — system roadmap
- [[trident-complete-stdlib]] — complete stdlib architecture
- [[trident_review_passes]] — development review checklist