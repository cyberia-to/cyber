---
tags: trident, cyber, article
date: 2026-02-26
---

# Trident v0.1

The first version of [[trident]] is released.

[[trident]] is a smart contract language for provable computation whose native data type is a [[Goldilocks field]] element. Every variable, every operation, every function call compiles to arithmetic over the field. The compilation target is [[Triton VM]], which generates [[STARK]] proofs — hash-based, post-quantum secure, no trusted setup.

## what ships in v0.1

The core language: `Field` as the native type, bounded loops, `divine()` for non-deterministic witness injection, and lookup tables for nonlinear operations.

The [[trident standard library]] foundation layer: `std.field` for prime field arithmetic, `std.math` for mathematical utilities, `std.data` for provable data structures, `std.crypto` for hash functions and commitments, and `std.io` for blockchain interaction.

Every program written in trident compiles to an arithmetic circuit and produces a [[STARK]] proof of correct execution.

## the thesis

Three computational revolutions — [[zero knowledge proofs]], artificial intelligence, and [[quantum computing]] — share a common algebraic foundation in prime field arithmetic. Trident sits at their intersection because its native `Field` type simultaneously satisfies the requirements of all three worlds. See [[trident thesis]] for the full argument, [[trinity]] for the conceptual overview.

## what comes next

The [[trident standard library]] specifies 15 modules across foundation, three pillars ([[trident verifiable AI]], privacy, quantum), three intersections, and three application layers. v0.1 ships the foundation. The pillars follow:

- `std.nn` — neural network primitives over the field. Train and prove inference natively, with zero quantization overhead. See [[trident verifiable AI]] for the zkML deep dive.
- `std.private` — zero-knowledge privacy toolkit. Anonymous credentials, private transactions, provable compliance.
- `std.quantum` — quantum computing primitives with dual compilation to classical simulation and quantum hardware. See [[trident quantum computing]] for the structural argument.

## why it matters

Existing zkML frameworks start from floating-point models and painfully convert to field arithmetic. Trident starts from the field. Existing quantum languages produce results with zero verifiability. Trident proves every computation. Existing smart contract languages operate over 256-bit words with no path to proof systems. Trident compiles to STARKs by construction.

One language. One field. One proof.
