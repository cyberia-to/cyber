---
tags: trident, cyber
alias: trident language, Trident, Tri, tri
crystal-type: entity
crystal-domain: cyber
---
where the [[field]] is visible and the programmer thinks in constraints. division is exact (multiplicative inverse). every operation becomes a polynomial constraint in the [[zheng]] execution trace

Trident-only primitives: `divine()` (inject prover witness), `hash()` ([[Hemera]], single constraint), `merkle_step()`, `seal` (hashed/private event emission)

| Layer | Scope | Types available | Compilation targets |
|---|---|---|---|
| 0 | Execute Anywhere | U32, Bool, structs, arrays | TASM, EVM, [[CosmWasm]], SVM |
| 1 | Prove Anywhere | + Field, Digest, divine() | TASM (Triton VM) |
| 2 | Platform Powers | + chain-specific stdlib | Single target |

Tri is also the proving tier: [[field]] tower F_{pⁿ} over [[Goldilocks field processor]] (p = 2⁶⁴ − 2³² + 1). each extension is F_p[x]/(f(x)) where f is irreducible of degree n, chosen by the compiler for the algebraic structure required: n=1 for core STARK arithmetic, n=2 (f = x²+1) for complex amplitudes and [[quantum]] gates, n=3 (f = x³−x+1) for recursive [[proof]] soundness in FRI, higher n as needed. the tower is multiplicative — F_{p⁶} contains both F_{p²} and F_{p³} as subfields, so [[quantum]] and recursive [[proof]]s coexist in a common extension. all execution languages compile to Tri for settlement. see [[zheng]] for the STARK implementation architecture

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture
