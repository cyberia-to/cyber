---
tags: cyber, rs, rust, language
alias: Rs, rs language
icon: "\u2699\uFE0F"
crystal-type: entity
crystal-domain: cyber
stake: 2440926101748440
subgraph: true
repo: ../rs
exclude: ".claude/**, target/**"
diffusion: 0.00022511157146064474
springs: 0.00011689529764623842
heat: 0.00017090740252070542
focus: 0.0001818058555283403
gravity: 7
density: 4.22
---
[[Rust]] with everything dynamically-sized removed. no heap. no `Vec`. no `String`. no unbounded recursion. every value has a known size at compile time. every loop has a known bound

the hidden truth: every `u64` in Rs is secretly a `word` — type tag 0x01 — which is secretly a [[field]] element with a range constraint. the programmer writes conventional-looking systems code, but every operation is field-compatible. the `Addressed` type derive emits [[Hemera]] CIDs natively — commitment interface is baked into the type system

```
Rust        → full language, heap, strings, anything
  ↓ restrict
Rs          → strict subset, bounded, looks like systems code
  ↓ reveal
Trident     → same restrictions, but the field is visible
```

Rs and [[Trident]] share the [[Goldilocks field processor]] substrate but present opposite mental models. a byte programmer thinks in registers and bit patterns. a field programmer thinks in algebraic constraints. same representation, opposite intent

```
.rs file  → parser (Rust subset) → TIR → TASM / backend
.tri file → parser (Trident)     → TIR → TASM / backend
                                    ↑
                              same IR, same value tower
```

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture