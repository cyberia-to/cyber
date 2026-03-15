---
tags: trident, cyber
alias: Goldilocks prime, Goldilocks, F_p, goldilocks
crystal-type: entity
crystal-domain: cyber
stake: 27174830290765380
---
The prime field $\mathbb{F}_p$ where $p = 2^{64} - 2^{32} + 1$. Native arithmetic substrate for [[trident]], [[stark]] proofs, [[TFHE]] ciphertexts, neural network inference, and quantum simulation.

## why this prime

- 64-bit — fits in one CPU register, one [[GFP]] field element
- NTT-friendly — $p - 1 = 2^{32}(2^{32} - 1)$ gives $2^{32}$ roots of unity for fast [[NTT]]
- prime — proper field structure (unlike $2^{64}$), enables multiplicative inverses
- fast reduction — $p = 2^{64} - 2^{32} + 1$ means modular reduction is two 64-bit ops instead of division

## four domains, one field

| domain | algebraic home | how $\mathbb{F}_p$ helps |
|--------|---------------|--------------------------|
| ZK proofs | arithmetic circuits over $\mathbb{F}_p$ | [[trident]] programs are circuits by construction |
| AI | matrix operations over $\mathbb{F}_p$ | weights and activations are field elements, no quantization |
| FHE | polynomial ring $R_p = \mathbb{F}_p[X]/(X^N+1)$ | when ciphertext modulus $q = p$, proof impedance vanishes |
| quantum | unitary matrices over $\mathbb{F}_{p^2}$ | prime dimension eliminates gate decomposition overhead |

See [[rosetta stone]] for why one lookup table over this field serves all four domains simultaneously.

## hardware

The [[GFP]] (Goldilocks Field Processor) has four primitives optimized for this field: `fma` (field multiply-accumulate), `ntt` ([[NTT]] butterfly), `p2r` ([[Poseidon2]] round), `lut` (lookup table). See [[Goldilocks field processor]].
