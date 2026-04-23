---
tags: cyber
alias: jali, जाली, polynomial ring, R_q, lattice arithmetic, ring arithmetic
crystal-type: entity
crystal-domain: cyber
---
polynomial ring arithmetic for [[cyber]]. R_q = F_p[x]/(x^n+1) over [[Goldilocks field|Goldilocks]]. jali is the encrypted computation engine — where [[nebu]] proves truth and [[genies]] proves privacy, jali proves that computation happened correctly on data nobody saw.

jali (जाली — lattice, mesh, screen) is the fifth execution algebra. n [[Goldilocks field|Goldilocks]] elements coupled by cyclotomic multiplication. one R_q multiply = 3n F_p multiplies via [[NTT]]. at n=1024: 3072× cost per operation over scalar [[nebu]]. the ring structure is what makes [[Ring-LWE]] hard — and what makes [[TFHE]], lattice [[KEM]], and structured noise possible.

## workloads

| domain | mechanism |
|--------|-----------|
| [[TFHE]] ciphertexts | encrypt/decrypt over R_q, programmable bootstrapping |
| lattice [[KEM]] ([[seal]]) | Module-RLWE key encapsulation over R_q |
| blind rotation | n polynomial multiplies during FHE bootstrapping |
| key switching | Galois automorphisms of R_q (slot permutation) |
| noise tracking | bound estimation through ring operations |
| convolution | native polynomial multiply = convolution |

## nox integration

no new Layer 1 patterns. polynomial multiply decomposes to ntt + pointwise_mul + intt — existing [[nebu]] jets. ring-specific acceleration through Layer 3:

Layer 3 jets: jet_ntt_batch, jet_key_switch, jet_gadget_decomp, jet_noise_track, jet_blind_rotate.

verification: ring operations proved via PCS₃ (ring-aware [[Brakedown]] with [[NTT]] batching). [[zheng]] ring-aware CCS exploits R_q structure. HyperNova folds into [[Goldilocks field|Goldilocks]] accumulator.

## dependency graph

```
nebu (F_p) — scalar arithmetic, NTT roots
  ↓
jali (R_q) ← this repo
  ↓
mudra (veil — FHE scheme over jali)
  ↓
nox (jets)
  ↓
zheng (PCS₃ — ring-aware proving)
  ↓
bbg (encrypted state)
```

discover all [[concepts]]
