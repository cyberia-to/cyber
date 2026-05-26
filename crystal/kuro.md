---
tags: cyber
alias: kuro, 黒, F2 tower, binary field arithmetic, binary field
crystal-type: entity
crystal-domain: cyber
---
F₂ tower field arithmetic for binary proving. kuro is to F₂ what [[nebu]] is to [[Goldilocks field|Goldilocks]].

## the tower

```
F₂ → F₂² → F₂⁴ → F₂⁸ → F₂¹⁶ → F₂³² → F₂⁶⁴ → F₂¹²⁸
 1b    2b    4b    8b    16b     32b     64b     128b
```

each extension: $F_{2^{2k}} = F_{2^k}[x] / (x^2 + x + \alpha_k)$

at the top: F₂¹²⁸ = 128 F₂ elements packed in one u128 machine word. one XOR = 128 additions. one AND = 128 multiplications. SIMD-native.

## why kuro exists

bitwise operations in [[Goldilocks field|Goldilocks]] (F_p) cost ~32 constraints each — bit decomposition is expensive in a prime field. in F₂, they cost 1 constraint. the 32× gap is the algebraic distance between F_p and F₂.

two workloads dominate the binary regime:
- quantized AI inference: [[BitNet]] 1-bit models. matrix-vector multiply = XOR + popcount
- [[tri-kernel]] SpMV: quantized axon weights for φ* iteration

kuro provides the field arithmetic. the binary PCS ([[zheng]]) provides the commitment. [[nox]] Bt instantiation (nox<F₂>) provides the execution. together: binary workloads at native cost.

## dependency graph

```
nebu (Goldilocks)    kuro (F₂) ← this repo
       ↓                ↓
     hemera (hash, trust anchor)
       ↓
     nox (VM)
       ↓
     zheng (proofs, binary PCS uses kuro)
       ↓
     bbg (state, binary traces fold into Goldilocks accumulator)
```

discover all [[concepts]]
