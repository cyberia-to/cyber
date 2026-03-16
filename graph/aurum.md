---
tags: cyber
alias: aurum, Aurum, goldilocks field library
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../aurum
exclude: ".claude/**, target/**"
---
The [[Goldilocks field]] as a standalone Rust crate. Provides field arithmetic (add, sub, mul, inv, eq, lt) and [[NTT]] over roots of unity in $\mathbb{F}_p$ where $p = 2^{64} - 2^{32} + 1$.

aurum is the foundational primitive for the entire [[cyber]] computational stack. [[hemera]] consumes it for hashing, [[nox]] consumes it for VM execution, [[trident]] consumes it for compilation, and [[GFP]] accelerates it in hardware.

## scope

Six field operations matching [[nox]] Layer 1 arithmetic patterns:

| op | definition |
|----|-----------|
| add | $(a + b) \bmod p$ |
| sub | $(a - b) \bmod p$ |
| mul | $(a \times b) \bmod p$ |
| inv | $a^{p-2} \bmod p$ (Fermat) |
| eq | equality test |
| lt | ordering test |

Plus [[NTT]] — the Number Theoretic Transform over $2^{32}$ roots of unity that $p - 1 = 2^{32}(2^{32} - 1)$ provides. Used by [[stark]] proofs, [[TFHE]] polynomial rings, and [[hemera]] permutations.

## dependency graph

```
aurum (field)
  ↓
hemera (hash)
  ↓
nox (VM)
```
