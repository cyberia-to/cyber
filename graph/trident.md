---
tags: trident, cyber
alias: trident language, Trident
crystal-type: entity
crystal-domain: cyber
stake: 37851441059813056
---

Provable programming language. Every variable, every operation, every function compiles to arithmetic over the [[Goldilocks field]] ($p = 2^{64} - 2^{32} + 1$). Programs produce [[STARK]] proofs — hash-based, post-quantum secure, no trusted setup.

Runs on [[neptune]] via [[Triton VM]].

```trident
program hello_proof

fn main() {
    let a: Field = secret_read()
    let b: Field = secret_read()
    pub_write(a + b)
}
```

A [[cryptographic proof]] that `a + b = 20` without revealing `a` or `b`.

## why a new language

Provable VMs are arithmetic machines, not byte-addressable CPUs. The machine word is a field element, not a byte. Trident's primitives — `Field`, `Digest`, `XField` — map directly to what the VM computes.

| Operation | Trident on Triton VM | Rust on SP1 | Rust on RISC Zero |
|-----------|:---:|:---:|:---:|
| One hash (Tip5 / SHA-256) | 1 cycle | ~3,000 cycles | ~1,000 cycles |
| Merkle proof (depth 32) | ~100 cycles | ~96,000 cycles | ~32,000 cycles |

Bounded execution: all loops have explicit bounds. No recursion. No heap. No halting problem. This is what makes programs provable, costs predictable, and compilation to quantum hardware possible.

## the [[rosetta stone]]

A single lookup table over [[Goldilocks field]] simultaneously functions as:

| Reading | Role |
|---------|------|
| Cryptographic S-box | Hash nonlinearity (security) |
| Neural activation | Network expressiveness (intelligence) |
| FHE bootstrap | Encrypted evaluation (privacy) |
| STARK lookup | Proof authentication (verifiability) |

One table. One field. Four purposes. See [[trinity]] for the conceptual overview, [[trident thesis]] for the full argument.

## formal verification

Trident's restrictions — bounded loops, no recursion, finite field arithmetic — make verification decidable. Annotate contracts, the compiler proves correctness automatically:

```trident
#[requires(amount > 0)]
#[requires(sender_balance >= amount)]
#[ensures(result == sender_balance - amount)]
fn transfer(sender_balance: Field, amount: Field) -> Field {
    assert(amount > 0)
    assert(sender_balance >= amount)
    sender_balance - amount
}
```

## content-addressed code

Every function has a unique [[cryptographic proof]] identity derived from its normalized AST. Names are metadata. The [[hash]] is the truth. Rename a function — the hash stays the same. Publish independently from the other side of the planet — same code, same hash.

## self-hosting

The compiler self-hosts: trident source compiles trident source, and the execution produces a [[STARK]] proof that compilation was faithful. Three producers compete on the same scoreboard: compiler output, expert hand-written assembly, and a neural model learning to emit better assembly than both.

## neptune programs

[[neptune]] is the only blockchain with recursive [[STARK]] proofs in production — a proof verifies another proof inside it, so any chain of transactions collapses into a single cryptographic check.

Proposed standards written in trident:

| Program | What it proposes |
|---------|-----------------|
| Coin (TSP-1) | Fungible token — pay, lock, mint, burn, composable hooks |
| Card (TSP-2) | Non-fungible token — royalties, creator immutability |
| Lock scripts | Multisig, timelock, symmetric spending authorization |
| Type scripts | Token conservation laws verified in every transaction |

## standard library

Implemented: `std.field` · `std.crypto` · `std.math` · `std.data` · `std.io` · `std.compiler`

In development: `std.nn` (field-native neural networks) · `std.private` (ZK + FHE + MPC) · `std.quantum` (gates, error correction)

See [[trident standard library]] for the full `std.*` architecture.

## source tree

```
src/    Compiler in Rust            ~36K lines, 5 runtime dependencies
vm/     VM intrinsics in Trident    hash, I/O, field ops
std/    Standard library in Trident crypto, math, neural networks, compiler
os/     OS bindings in Trident      per-OS config, programs, extensions
```

## design principles

1. Field elements all the way down. The machine word is `Field`.
2. Bounded execution. Explicit loop bounds. No recursion. No halting problem.
3. Compile-time everything. Types, array sizes, and costs known statically.
4. Constraints are features. No heap, no dynamic dispatch — safety guarantees.
5. Provable first. Designed for ZK. These constraints make great conventional programs too.
6. Minimal dependencies. 5 runtime crates.

## deep dives

- [[trident thesis]] — one language for quantum, AI, and [[zero knowledge proofs]]
- [[trident verifiable AI]] — why the next generation of zkML starts from prime fields
- [[trident quantum computing]] — structural necessity of prime field arithmetic for quantum advantage
- [[trident standard library]] — complete `std.*` architecture
- [[trident review passes]] — review pass system
