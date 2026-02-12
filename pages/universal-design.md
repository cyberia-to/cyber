# Trident: Universal Language for Provable Computation

**Design Document — v0.2 Draft**
**February 2026**

*From single-target ZK language to universal compilation source for all provable virtual machines.*

---

## 1. Executive Summary

Trident is a minimal, security-first programming language originally targeting Triton VM for zero-knowledge provable computation. This document outlines the design for evolving Trident into a **universal source language** capable of compiling to any zkVM — including Triton VM, Miden VM, Cairo VM (StarkWare), SP1/RISC Zero (RISC-V zkVMs), and NockVM — while preserving the core properties that make it valuable: bounded execution, cost transparency, and auditability.

### 1.1 Thesis

Approximately **76% of Trident's language surface** is already portable or trivially abstractable across zkVMs. The remaining ~24% consists of **backend extensions** — target-specific capabilities that each VM exposes through a uniform extension mechanism. This makes Trident an unusually strong candidate for a universal provable computation language, requiring architectural refactoring rather than language redesign.

### 1.2 Core Insight

Every zkVM computes over a finite field. `Field` is the universal primitive of provable computation — the specific prime is an implementation detail of the proof system, not a semantic property of the program. A Trident program that multiplies two field elements and asserts the result means the same thing on every zkVM. The developer reasons about field arithmetic abstractly; the backend implements it concretely.

This is analogous to how `int` in C means "integer of platform-native width." You write arithmetic, the compiler picks the encoding. `Field` in Trident means "element of the target VM's native field." Programs should never depend on the specific modulus — and the language design enforces this.

### 1.3 Architecture

```
┌───────────────────────────────────────────┐
│         Trident Universal Core            │
│   (types, control flow, modules, field    │
│    arithmetic, I/O, cost transparency)    │
├───────────────────────────────────────────┤
│         Abstraction Layer                 │
│   (hash, memory, stack/register mgmt,    │
│    Merkle ops, cost model, events)        │
├─────────┬─────────┬─────────┬────────────┤
│ Triton  │  Miden  │  Cairo  │  SP1/RZ    │
│ Backend │ Backend │ Backend │  Backend   │
│         │         │         │            │
│  + ext  │  + ext  │  + ext  │  + ext     │
└─────────┴─────────┴─────────┴────────────┘
```

Each backend implements the abstraction layer for its target VM and may publish **backend extensions** — additional types, intrinsics, and standard library modules that expose target-specific capabilities. Programs that use extensions are explicitly bound to that backend.

### 1.4 Design Goals

1. **Write once, prove anywhere.** A single Trident program compiles to multiple zkVM targets with target-appropriate optimizations.
2. **Preserve auditability.** Direct emission (no IR) for stack-machine targets; minimal IR for register-machine targets.
3. **Cost transparency per target.** Every function annotated with proving cost in the target VM's native cost model.
4. **Backend extensions, not limitations.** Target-specific features are capabilities a backend *adds* to the universal core, not restrictions on portability.
5. **Incremental adoption.** Existing Triton-targeting Trident programs continue to compile unchanged.

### 1.5 Non-Goals

- General-purpose programming (no strings, no heap, no unbounded execution)
- Competing with Rust on RISC-V zkVMs for general workloads
- Supporting non-ZK virtual machines (EVM, WASM, SVM) as primary targets
- Package registry or ecosystem tooling (premature at this stage)

---

## 2. Target zkVM Landscape

### 2.1 Supported Targets

| Target | Architecture | Field | Hash | Proof System | Priority |
|--------|-------------|-------|------|-------------|:--------:|
| **Triton VM** | Stack (16-element) | Goldilocks (2⁶⁴−2³²+1) | Tip5 | STARK | Native |
| **Miden VM** | Stack (16-element) | Goldilocks (2⁶⁴−2³²+1) | RPO | STARK | 1 |
| **Cairo VM** | Register (AP, FP, PC) | 252-bit prime | Poseidon/Pedersen | STARK | 2 |
| **SP1 / RISC Zero** | Register (RISC-V rv32im) | Various | Various | STARK | 3 |
| **NockVM (Zorp)** | Combinator reduction | Arbitrary-precision | TBD | STARK | 4 (exploratory) |

### 2.2 Architectural Families

**Family A: Stack Machines** — Triton VM, Miden VM

- 16-element operational stack
- Stack manipulation instructions (swap, dup, pop)
- Direct emission from AST traversal (no IR needed)
- Trident's current emission model works with minor adaptation

**Family B: Register Machines** — Cairo VM, RISC-V zkVMs (SP1, RISC Zero)

- Named registers or register file
- Requires register allocation
- Needs a lightweight IR between type checking and emission

**Family C: Reduction Machines** — NockVM

- Binary tree (noun) data model
- Combinator-based computation
- Fundamentally different paradigm; long-term exploratory target

---

## 3. The Universal Core

These features compile identically to any target with no adaptation. They constitute ~55% of the language surface.

### 3.1 Type System

```
Field           → Native field element of the target VM
                  (Goldilocks on Triton/Miden, 252-bit on Cairo, etc.)
                  The universal primitive of provable computation.

Bool            → 0 or 1; native on all targets
U32             → 32-bit unsigned integer; native or emulated everywhere
[T; N]          → Fixed array; flattened to sequential memory on all targets
(T1, T2)        → Tuple; flattened to sequential elements
struct { ... }  → Named product type; compiler-only abstraction
```

`Field` is Tier 1 — universally portable. The prime differs per target but the *semantics* are identical: elements of a finite field with addition, multiplication, and inversion. Programs should never depend on the specific modulus. The only place where field size leaks into program semantics is `split()`, where the number of U32 limbs depends on field width — handled via the `FIELD_LIMBS` target constant.

All composite types have compile-time-known widths. The width *unit* (field elements vs bytes) varies per target, but the width *computation algorithm* is universal.

### 3.2 Field Arithmetic

```
a + b           → field addition (mod p)
a * b           → field multiplication (mod p)
inv(a)          → multiplicative inverse
neg(a)          → additive inverse (p − a)
sub(a, b)       → field subtraction (a + neg(b))
a == b          → field equality
```

All field operations are universally portable. The instruction encoding differs per backend but the mathematical semantics are identical across all targets.

### 3.3 Integer Arithmetic

```
a < b           → u32 comparison
a & b           → bitwise and
a ^ b           → bitwise xor
a /% b          → divmod → (quotient, remainder)
log2(a)         → floor log base 2
pow(base, exp)  → exponentiation
popcount(a)     → Hamming weight (population count)
```

All U32 operations are universally portable. Every zkVM supports 32-bit integer operations, whether natively (Triton, Miden) or through standard instructions (RISC-V) or range-checked felt decomposition (Cairo).

### 3.4 Control Flow

```
if / else                     → Universal branching
for i in 0..N                 → Bounded loop (constant); unrollable everywhere
for i in 0..n bounded MAX     → Bounded loop (runtime variable); universal pattern
match expr { ... }            → Desugared to nested if/else at AST level
assert(condition)             → "Halt if false"; fundamental ZK verification primitive
assert_eq(a, b)               → Sugar for assert(a == b)
```

Bounded execution is the single most important portability property. Every zkVM requires deterministic trace length. Trident's mandatory loop bounds, prohibition of recursion, and static memory satisfy this universally.

### 3.5 Functions and Modules

```
fn definitions (no recursion)       → Call/return on stack VMs; jump-and-link on register VMs
pub / private visibility            → Compiler-only; never reaches the VM
module / use imports                → Resolved at compile time; emitted as flat code
DAG dependency enforcement          → Compiler-only
const declarations                  → Inlined at compile time; zero runtime cost
Size-generic fn foo<N>()            → Monomorphization; backend never sees generics
#[test] functions                   → Compiler-only
#[cfg(flag)] conditional compilation → Compiler-only
```

### 3.6 Expressions

All universally portable:

- Integer and boolean literals
- Array, struct, and tuple initialization
- Field and index access (`p.x`, `arr[i]`)
- Variable binding (`let` / `let mut`) and assignment
- Tuple destructuring (`let (a, b) = ...`)
- Block expressions

### 3.7 Project Infrastructure

- `trident.toml` project configuration
- File layout conventions
- CLI commands: `build`, `check`, `fmt`, `test`, `doc`, `init`, `lsp`
- Editor support (Zed, Helix, LSP)

---

## 4. The Abstraction Layer

Features that exist on all zkVMs but with different concrete representations. Each needs a thin interface that backends implement. ~21% of the language surface.

### 4.1 I/O Primitives

Every zkVM distinguishes **public input**, **public output**, and **private witness**. The mechanism differs but the semantic model is identical.

| Operation | Triton | Miden | Cairo | SP1 |
|-----------|--------|-------|-------|-----|
| Read public input | `read_io N` | `adv.push` | Program input segment | `sp1_io::read()` |
| Write public output | `write_io N` | Output stack | Program output segment | `sp1_io::commit()` |
| Read private witness | `divine N` | Advice provider | Hint block | `sp1_io::read()` (witness) |

**User-facing syntax unchanged:**

```
let a: Field = pub_read()       // Read from public input
pub_write(result)               // Write to public output
let s: Field = divine()         // Read from private witness
```

**Backend interface:**

```rust
trait IOBackend {
    fn emit_pub_read(&mut self, count: usize);
    fn emit_pub_write(&mut self, count: usize);
    fn emit_divine(&mut self, count: usize);
}
```

### 4.2 Memory Access

| Aspect | Triton/Miden | Cairo | RISC-V |
|--------|-------------|-------|--------|
| Addressing | Word-addressed (1 field element per cell) | Felt-addressed (write-once) | Byte-addressed (read/write) |
| First read (uninitialized) | Prover-supplied value (non-deterministic) | Undefined | Returns zero |
| Block read/write | Native instructions | Loop of single reads | Load/store instructions |

**User-facing syntax unchanged:**

```
ram_write(address, value)
let v: Field = ram_read(address)
```

**Backend interface:**

```rust
trait MemoryBackend {
    fn emit_read_word(&mut self, addr_on_stack: bool);
    fn emit_write_word(&mut self, addr_on_stack: bool);
    fn emit_read_block(&mut self, count: usize);
    fn emit_write_block(&mut self, count: usize);
    fn is_non_deterministic_on_first_read(&self) -> bool;
    fn is_write_once(&self) -> bool;
}
```

The compiler emits warnings when targeting write-once memory (Cairo) if a program writes to the same address twice.

### 4.3 Stack / Register Management

| Aspect | Stack VMs (Triton, Miden) | Register VMs (Cairo, SP1/RZ) |
|--------|--------------------------|--------------------------------------|
| Variable storage | Stack positions (16-element limit) | Registers (3 on Cairo, 32 on RISC-V) |
| Spill strategy | RAM spill when >16 live variables | Register spill to stack frame |
| Operand access | `swap N` / `dup N` to bring to top | Direct register addressing |

**Backend interface:**

```rust
trait AllocationStrategy {
    fn allocate_variable(&mut self, name: &str, width: usize) -> Location;
    fn emit_load(&mut self, loc: &Location);
    fn emit_store(&mut self, loc: &Location);
    fn emit_spill(&mut self, loc: &Location);
    fn max_fast_slots(&self) -> usize;  // 16 for stack VMs, 32 for RISC-V
}
```

The user never sees the difference. Current Trident's `stack.rs` becomes the stack-VM implementation; a new `regalloc.rs` serves register-machine targets.

### 4.4 Hash Primitives

Hash functions are the cryptographic backbone of every zkVM but each uses a different one.

| VM | Hash Function | Digest Width | Native Cost |
|----|--------------|:------------:|-------------|
| Triton | Tip5 | 5 field elements | 1cc + 6 hash rows |
| Miden | RPO (Rescue Prime Optimized) | 4 field elements | ~1cc equivalent |
| Cairo | Poseidon / Pedersen | Varies | Builtin coprocessor |
| SP1 | SHA-256, Keccak, Poseidon (precompiles) | 32 bytes | Precompile cost |

**User-facing syntax unchanged:**

```
let d: Digest = hash(input)
sponge_init()
sponge_absorb(elements)
let squeezed = sponge_squeeze()
```

**Target constants exposed to programs:**

```
DIGEST_WIDTH    → 5 (Triton/Tip5), 4 (Miden/RPO), varies (others)
HASH_RATE       → 10 (Tip5), 8 (RPO), 3 (Poseidon), varies
FIELD_LIMBS     → 2 (Goldilocks), 8 (252-bit Cairo)
```

**`Digest` type:** Defined as `[Field; DIGEST_WIDTH]` where `DIGEST_WIDTH` is a compile-time constant set by the target. This preserves Trident's "what you see is what you prove" philosophy — the width is visible, not hidden behind an opaque type.

**Backend interface:**

```rust
struct HashConfig {
    name: &'static str,      // "tip5", "rpo", "poseidon"
    rate: usize,             // Elements absorbed per round
    digest_width: usize,     // Elements per digest
    is_native: bool,         // Native instruction vs library implementation
}
```

### 4.5 Merkle Tree Operations

Merkle verification is algorithmically identical across all zkVMs: iterate from leaf to root, hashing at each level with the sibling. The difference is whether the VM has a native instruction for it.

| VM | Merkle Support | Implementation |
|----|---------------|----------------|
| Triton | Native `merkle_step` instruction | Single instruction |
| Miden | Native `mtree_get`, `mtree_set` | Single instruction |
| Cairo | Library code using Poseidon | Hash loop |
| SP1/RZ | Library code using precompile | Hash loop |

**Abstraction strategy:** `std.merkle` becomes target-polymorphic. On VMs with native Merkle instructions, the body compiles to a single instruction. On others, it compiles to a loop of hash operations with `divine()` for sibling digests:

```
pub fn verify(root: Digest, leaf: Digest, index: U32, depth: U32) {
    let mut current = leaf
    let mut idx = index
    for _ in 0..depth bounded 64 {
        current = merkle_step(idx, current)   // native or hash-loop per target
        idx = idx >> 1
    }
    assert_digest(current, root)
}
```

### 4.6 Cost Model Framework

Every zkVM has a proving cost; the specific dimensions change but the computation framework is universal.

| Aspect | Universal | Target-specific |
|--------|-----------|----------------|
| AST traversal for cost computation | ✅ Same algorithm | — |
| Per-instruction cost lookup | — | Different cost tables |
| Table structure | — | Triton: 6 tables. Miden: chiplets. Cairo: steps |
| Padded height (power-of-2) | STARK-based VMs | — |
| `--costs` / `--hotspots` CLI | ✅ Same framework | Numbers differ |
| Boundary proximity warnings | STARK-based VMs only | — |

**Backend interface:**

```rust
trait CostModel {
    type Profile;
    fn zero() -> Self::Profile;
    fn instruction_cost(&self, op: &Op) -> Self::Profile;
    fn add(a: &Self::Profile, b: &Self::Profile) -> Self::Profile;
    fn max(a: &Self::Profile, b: &Self::Profile) -> Self::Profile;
    fn scale(p: &Self::Profile, n: usize) -> Self::Profile;
    fn padded_height(&self, p: &Self::Profile) -> u64;
    fn dominant_table(&self, p: &Self::Profile) -> String;
    fn format_report(&self, p: &Self::Profile) -> String;
}
```

### 4.7 Events System

```
emit Event { ... }    → Structured public output (fields visible to verifier)
seal Event { ... }    → Hashed output (only digest visible)
```

Events compose from I/O and hash abstractions — `emit` serializes to public output, `seal` hashes first then outputs the digest. No additional backend work needed.

---

## 5. Backend Extensions

Backend extensions are capabilities that a target VM **adds** to the universal core. They are not limitations or second-class features — they are the mechanism by which each backend exposes its unique power.

### 5.1 Extension Model

```
Trident Universal Core
  + Backend Extensions
  = Complete program for a specific zkVM
```

Programs that use backend extensions are explicitly bound to that target via `#[cfg(target)]` guards or target-specific module imports. The compiler enforces this:

```
error[E0100]: module `ext.triton.xfield` requires target `triton`
  --> main.tri:3:5
   |
3  |     use ext.triton.xfield
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: compile with `--target triton` or remove this import
```

### 5.2 Extension Categories

Each backend may provide extensions in four categories:

| Category | What it provides | Example |
|----------|-----------------|---------|
| **Types** | Additional primitive or composite types | `XField` (Triton), `Felt252` (Cairo) |
| **Intrinsics** | Native VM instructions exposed as functions | `xx_dot_step` (Triton), `mtree_set` (Miden) |
| **Inline Assembly** | Direct access to target instruction set | `asm(triton) { dup 0 add }` |
| **Standard Library Modules** | Higher-level APIs built on target capabilities | `ext.triton.kernel`, `ext.miden.account` |

### 5.3 Triton Backend Extensions

The Triton backend extends the universal core with capabilities specific to Triton VM's ISA and the Neptune Cash ecosystem.

**Extension Types:**

```
XField          → Cubic extension field F_p[X]/(X³−X+1)
                  3 field elements wide
                  Native arithmetic: xx_add, xx_mul, x_invert, xb_mul
```

**Extension Intrinsics:**

| Function | TASM Instruction | Purpose |
|----------|-----------------|---------|
| `xx_dot_step(acc, ptr_a, ptr_b)` | `xx_dot_step` | Extension field dot product (STARK verifier inner loop) |
| `xb_dot_step(acc, ptr_a, ptr_b)` | `xb_dot_step` | Mixed-field dot product (STARK verifier inner loop) |
| `sponge_absorb_mem(ptr)` | `sponge_absorb_mem` | Absorb from RAM (combines sponge + memory read) |
| `merkle_step_mem(ptr, idx, d)` | `merkle_step_mem` | Merkle step from RAM (reusable auth paths) |

**Extension Standard Library:**

```
ext.triton.xfield       → XField type, arithmetic, dot products
ext.triton.kernel        → Neptune kernel interface (authenticate_field, tree_height)
ext.triton.utxo          → UTXO verification
ext.triton.stark         → Recursive STARK verifier components
```

**Inline Assembly:**

```
asm(triton) {
    dup 0
    add
    swap 5 pop 1
}
```

### 5.4 Miden Backend Extensions

The Miden backend extends the universal core with Miden VM's account model and advanced advice provider capabilities.

**Extension Intrinsics:**

| Function | Miden Instruction | Purpose |
|----------|------------------|---------|
| `mtree_set(root, val, idx, depth)` | `mtree_set` | Merkle tree update (not just verification) |
| `adv_pipe(ptr, count)` | `adv_pipe` | Batch read from advice provider to memory |
| `exec_kernel(proc)` | `exec.kernel::proc` | Execute kernel procedure |

**Extension Standard Library:**

```
ext.miden.account        → Miden account model (account ID, nonce, storage)
ext.miden.note           → Miden note system (create, consume, verify)
ext.miden.advice         → Extended advice provider API
ext.miden.wallet         → Wallet operations (send, receive)
```

**Inline Assembly:**

```
asm(miden) {
    dup.0
    add
    movdn.5 drop
}
```

### 5.5 Cairo Backend Extensions

The Cairo backend extends the universal core with StarkNet-specific capabilities and Cairo's 252-bit field properties.

**Extension Types:**

```
Felt252         → Explicit 252-bit field element
                  (alias for Field on Cairo target, distinct type for clarity)
```

**Extension Intrinsics:**

| Function | Cairo Builtin | Purpose |
|----------|--------------|---------|
| `pedersen_hash(a, b)` | Pedersen builtin | Pedersen hash (legacy, widely used on StarkNet) |
| `ec_point_add(p, q)` | EC_OP builtin | Elliptic curve point addition |
| `bitwise_and(a, b)` | Bitwise builtin | Native bitwise operations |

**Extension Standard Library:**

```
ext.cairo.starknet       → StarkNet contract interface
ext.cairo.felt252        → 252-bit field specific utilities
ext.cairo.builtin        → Builtin runner access (range_check, ECDSA, etc.)
```

**Inline Assembly:**

```
asm(cairo) {
    [ap] = [ap-1] + [ap-2]; ap++
}
```

### 5.6 SP1/RISC Zero Backend Extensions

The RISC-V zkVM backends extend the universal core with precompile access and standard RISC-V capabilities.

**Extension Intrinsics:**

| Function | Precompile | Purpose |
|----------|-----------|---------|
| `sha256(data)` | SHA-256 precompile | SHA-256 hash (standard, not ZK-optimized) |
| `keccak256(data)` | Keccak precompile | Keccak-256 hash (Ethereum compatible) |
| `secp256k1_verify(sig, msg, pk)` | secp256k1 precompile | ECDSA signature verification |

**Extension Standard Library:**

```
ext.sp1.io               → SP1-specific I/O patterns
ext.sp1.precompile       → Precompile access (SHA, Keccak, secp256k1, ed25519)
ext.risczero.journal     → RISC Zero journal (public output) API
```

### 5.7 Third-Party Backend Extensions

The extension model is open. A third-party backend can define its own extensions by:

1. Providing a target configuration TOML file
2. Implementing the backend trait (StackBackend or RegisterBackend + IR)
3. Publishing extension modules under `ext.<target_name>/`
4. Registering intrinsics in the target's intrinsic table

This means future zkVMs can be supported without modifying the Trident core compiler — only a new backend + extension set is needed.

### 5.8 Extension Usage Patterns

**Portable program (no extensions):**

```
program portable_verifier

use std.crypto.merkle

fn main() {
    let root: Digest = pub_read_digest()
    let leaf: Digest = divine_digest()
    let index: U32 = as_u32(pub_read())
    std.crypto.merkle.verify(root, leaf, index, 20)
}
// Compiles to: triton, miden, cairo, sp1
```

**Program with backend extension (target-bound):**

```
program triton_stark_verifier

use std.crypto.merkle
use ext.triton.xfield        // ← Binds to Triton backend
use ext.triton.stark

fn main() {
    let claim = read_claim()
    ext.triton.stark.verify(claim)
}
// Compiles to: triton only
```

**Program with conditional extensions (multi-target with specialization):**

```
program optimized_verifier

use std.crypto.merkle

#[cfg(triton)]
use ext.triton.xfield

fn verify_inner(commitment: Digest) {
    #[cfg(triton)]
    {
        // Use native extension field dot products for maximum performance
        let acc: XField = xfield(0, 0, 0)
        // ... optimized Triton path using xx_dot_step
    }

    #[cfg(not(triton))]
    {
        // Portable fallback using standard field arithmetic
        let acc: Field = 0
        // ... portable verification path
    }
}
// Compiles to: all targets, with Triton-optimized fast path
```

---

## 6. Compiler Architecture

### 6.1 Current Architecture (Single-Target)

```
Source (.tri)
  → Lexer (lexer.rs, lexeme.rs)
  → Parser (parser.rs) → AST (ast.rs)
  → Module Resolver (resolve.rs)
  → Type Checker (typeck.rs, types.rs)
  → TASM Emitter (emit.rs, stack.rs)
  → Linker (linker.rs)
  → Output: single .tasm file
```

### 6.2 Universal Architecture (Multi-Target)

```
Source (.tri)
  → Lexer (lexer.rs, lexeme.rs)              ← UNCHANGED
  → Parser (parser.rs) → AST (ast.rs)        ← UNCHANGED
  → Module Resolver (resolve.rs)             ← MINOR: target-aware std/ext resolution
  → Type Checker (typeck.rs, types.rs)       ← MINOR: target-parameterized constants
  → Target Configuration (target.rs)          ← NEW
  →  ┌─────────────────────────────────────┐
     │ Branch by target family              │
     │                                      │
     │  Stack VMs (Triton, Miden):          │
     │    → Direct Emitter (emit_stack.rs)  │
     │    → Linker (linker.rs)              │
     │    → Output: .tasm / .masm           │
     │                                      │
     │  Register VMs (Cairo, SP1/RZ):       │
     │    → IR Lowering (ir.rs)              ← NEW
     │    → Register Allocator (regalloc.rs) ← NEW
     │    → Target Emitter (emit_reg.rs)     ← NEW
     │    → Output: .sierra / .elf           │
     └─────────────────────────────────────┘
  → Cost Analyzer (cost.rs)                  ← MINOR: pluggable CostTable
  → Diagnostics (diagnostic.rs)              ← UNCHANGED
```

### 6.3 Shared Frontend (~80% of compiler)

| Module | LOC (est.) | Changes |
|--------|:----------:|---------|
| `lexer.rs` + `lexeme.rs` | ~800 | None |
| `parser.rs` | ~1200 | None (AST is target-agnostic) |
| `ast.rs` | ~400 | Add target-tagged `asm` variant |
| `resolve.rs` | ~300 | Target-aware std/ext resolution |
| `typeck.rs` | ~1500 | Parameterize `DIGEST_WIDTH`, `FIELD_LIMBS`, `HASH_RATE` |
| `types.rs` | ~200 | Add target constants |
| `format.rs` | ~600 | None |
| `diagnostic.rs` | ~200 | None |
| `lsp.rs` | ~800 | Target-aware completions and hover |
| `span.rs` | ~100 | None |
| **Total shared** | **~6100** | **~90% unchanged** |

### 6.4 Target Configuration

Each target is defined as a TOML file shipped with the compiler:

```toml
# targets/triton.toml
[target]
name = "triton"
family = "stack"
output_extension = ".tasm"

[field]
name = "goldilocks"
prime = "18446744069414584321"    # 2^64 - 2^32 + 1
width = 1
limbs = 2

[hash]
name = "tip5"
rate = 10
digest_width = 5

[io]
max_batch_read = 5
max_batch_write = 5
max_batch_divine = 5

[memory]
word_size = 1
non_deterministic = true
write_once = false

[stack]
depth = 16

[cost]
model = "triton_6table"

[extensions]
types = ["XField"]
modules = ["ext.triton.xfield", "ext.triton.kernel", "ext.triton.utxo", "ext.triton.stark"]
```

```toml
# targets/miden.toml
[target]
name = "miden"
family = "stack"
output_extension = ".masm"

[field]
name = "goldilocks"
prime = "18446744069414584321"
width = 1
limbs = 2

[hash]
name = "rpo"
rate = 8
digest_width = 4

[io]
max_batch_read = 4
max_batch_write = 4
max_batch_divine = 4

[memory]
word_size = 1
non_deterministic = true
write_once = false

[stack]
depth = 16

[cost]
model = "miden_chiplets"

[extensions]
types = []
modules = ["ext.miden.account", "ext.miden.note", "ext.miden.advice", "ext.miden.wallet"]
```

```toml
# targets/cairo.toml
[target]
name = "cairo"
family = "register"
output_extension = ".sierra"

[field]
name = "stark252"
prime = "3618502788666131213697322783095070105623107215331596699973092056135872020481"
width = 1
limbs = 8

[hash]
name = "poseidon"
rate = 3
digest_width = 1

[io]
max_batch_read = 1
max_batch_write = 1
max_batch_divine = 1

[memory]
word_size = 1
non_deterministic = false
write_once = true

[registers]
count = 3

[cost]
model = "cairo_steps"

[extensions]
types = ["Felt252"]
modules = ["ext.cairo.starknet", "ext.cairo.felt252", "ext.cairo.builtin"]
```

### 6.5 Backend Trait System

**Stack VM Backend:**

```rust
trait StackBackend {
    fn emit_push(&mut self, value: u64);
    fn emit_pop(&mut self, count: usize);
    fn emit_dup(&mut self, depth: usize);
    fn emit_swap(&mut self, depth: usize);
    fn emit_add(&mut self);
    fn emit_mul(&mut self);
    fn emit_eq(&mut self);
    fn emit_assert(&mut self);
    fn emit_call(&mut self, label: &str);
    fn emit_return(&mut self);
    fn emit_label(&mut self, label: &str);
    fn emit_skiz(&mut self);
    fn emit_hash(&mut self);
    fn emit_divine(&mut self, count: usize);
    fn emit_read_io(&mut self, count: usize);
    fn emit_write_io(&mut self, count: usize);
    fn emit_read_mem(&mut self, count: usize);
    fn emit_write_mem(&mut self, count: usize);
    fn emit_raw(&mut self, asm: &str);       // Inline assembly passthrough
    fn emit_extension_intrinsic(&mut self, name: &str, args: &[Operand]);
    fn output_extension(&self) -> &str;
}
```

`TritonBackend` and `MidenBackend` share ~70% of emission logic (function prologue/epilogue, loop structure, if/else branching, stack layout). They differ in instruction mnemonics, hash operations, and extension intrinsics.

**Register VM Backend (IR-based):**

```rust
// Minimal SSA-like IR for register machines
enum IRInst {
    // Arithmetic
    Add { dst: Reg, lhs: Operand, rhs: Operand },
    Mul { dst: Reg, lhs: Operand, rhs: Operand },
    Inv { dst: Reg, src: Operand },

    // Memory
    Load { dst: Reg, addr: Operand },
    Store { addr: Operand, val: Operand },

    // Control flow
    Branch { cond: Operand, then_label: Label, else_label: Label },
    Jump { target: Label },
    Label(Label),
    Call { target: Label, args: Vec<Operand>, dst: Option<Reg> },
    Return { value: Option<Operand> },

    // ZK-specific
    PublicRead { dst: Reg },
    PublicWrite { src: Operand },
    Divine { dst: Reg },
    Assert { cond: Operand },
    Hash { dst: Reg, inputs: Vec<Operand> },
    ExtensionCall { name: String, args: Vec<Operand>, dst: Option<Reg> },

    // Constants
    Const { dst: Reg, value: u64 },
}
```

~15 instruction types. Deliberately minimal. The IR serves as the common lowering target for all register-machine backends. Each backend implements a `RegisterEmitter` trait that converts IR to target-specific assembly.

### 6.6 CLI Integration

```bash
# Build for specific targets
trident build main.tri --target triton     # Default (backward compatible)
trident build main.tri --target miden
trident build main.tri --target cairo

# Cost analysis per target
trident build main.tri --target triton --costs
trident build main.tri --target miden --costs

# Check compatibility
trident check main.tri --target miden

# Build for all supported targets (as listed in trident.toml)
trident build main.tri --target all
```

---

## 7. Standard Library Architecture

### 7.1 Layered Module Structure

```
std/
├── core/                        # Universal Core — zero VM dependencies
│   ├── array.tri                #   sum, fill, reverse, contains, index_of
│   ├── bool.tri                 #   and, or, not, xor (as field arithmetic)
│   ├── convert.tri              #   as_u32, as_field (with range checks)
│   └── math.tri                 #   min, max, abs, clamp
│
├── io/                          # Abstraction Layer — per-target intrinsic dispatch
│   ├── io.tri                   #   pub_read, pub_write, divine
│   └── mem.tri                  #   ram_read, ram_write, ram_read_block, ram_write_block
│
├── crypto/                      # Abstraction Layer — hash-parameterized
│   ├── hash.tri                 #   hash(), sponge_init/absorb/squeeze
│   ├── merkle.tri               #   verify, verify_mem, authenticate_leaf
│   └── auth.tri                 #   verify_preimage, verify_digest_preimage
│
├── assert/                      # Abstraction Layer
│   └── assert.tri               #   is_true, eq, digest
│
└── target.tri                   # Target detection constants
    pub const TARGET_NAME         #   "triton", "miden", "cairo", etc.
    pub const DIGEST_WIDTH        #   5 for Tip5, 4 for RPO, etc.
    pub const FIELD_LIMBS         #   2 for Goldilocks, 8 for 252-bit
    pub const HASH_RATE           #   10 for Tip5, 8 for RPO, 3 for Poseidon

ext/
├── triton/                      # Triton Backend Extensions
│   ├── xfield.tri               #   XField type, xx_add, xx_mul, x_invert
│   ├── stark.tri                #   xx_dot_step, xb_dot_step, recursive verifier
│   ├── kernel.tri               #   Neptune kernel interface
│   └── utxo.tri                 #   UTXO verification
│
├── miden/                       # Miden Backend Extensions
│   ├── account.tri              #   Miden account model
│   ├── note.tri                 #   Miden note system
│   ├── advice.tri               #   Extended advice provider API
│   └── wallet.tri               #   Wallet operations
│
├── cairo/                       # Cairo Backend Extensions
│   ├── starknet.tri             #   StarkNet contract interface
│   ├── felt252.tri              #   252-bit field utilities
│   └── builtin.tri              #   Builtin runner access
│
└── sp1/                         # SP1 Backend Extensions
    ├── precompile.tri           #   SHA-256, Keccak, secp256k1, ed25519
    ├── io.tri                   #   SP1-specific I/O patterns
    └── journal.tri              #   RISC Zero journal API
```

### 7.2 Cross-Target Standard Library Implementation

Standard library modules use multi-target intrinsic annotations with portable fallbacks:

```
// std/crypto/hash.tri
module std.crypto.hash

/// Hash RATE field elements into a Digest.
/// Dispatches to the target VM's native hash instruction.
#[intrinsic(triton::hash)]
#[intrinsic(miden::hperm)]
pub fn hash_native(input: [Field; HASH_RATE]) -> Digest

/// Initialize sponge state.
#[intrinsic(triton::sponge_init)]
#[intrinsic(miden::hperm_init)]
pub fn sponge_init()

/// Absorb RATE elements into sponge.
#[intrinsic(triton::sponge_absorb)]
#[intrinsic(miden::hperm_absorb)]
pub fn sponge_absorb(input: [Field; HASH_RATE])

/// Squeeze RATE elements from sponge.
#[intrinsic(triton::sponge_squeeze)]
#[intrinsic(miden::hperm_squeeze)]
pub fn sponge_squeeze() -> [Field; HASH_RATE]
```

The compiler selects the intrinsic matching the current target. If no intrinsic matches, the function body provides a portable fallback.

### 7.3 Fallback Pattern

```
// std/crypto/merkle.tri — native on Triton/Miden, software on others
module std.crypto.merkle

use std.crypto.hash

pub fn verify(root: Digest, leaf: Digest, index: U32, depth: U32) {
    let mut current = leaf
    let mut idx = index
    for _ in 0..depth bounded 64 {
        current = merkle_step_impl(idx, current)
        idx = idx >> 1
    }
    assert_digest(current, root)
}

/// Native on Triton/Miden; hash-loop fallback on others.
#[intrinsic(triton::merkle_step)]
#[intrinsic(miden::mtree_get)]
fn merkle_step_impl(idx: U32, current: Digest) -> Digest {
    // Portable fallback: divine sibling, hash in correct order
    let sibling: Digest = divine_digest()
    if idx & 1 == 0 {
        hash_pair(current, sibling)
    } else {
        hash_pair(sibling, current)
    }
}
```

When a native intrinsic exists, the function body is ignored and the native instruction is emitted. When no intrinsic matches, the body compiles normally. Native performance where available; correct behavior everywhere.

---

## 8. Language Extensions Required

### 8.1 Backward-Compatible Changes

| Extension | Description | Effort |
|-----------|-------------|--------|
| Target-tagged `asm` blocks | `asm(triton) { ... }` alongside existing `asm { ... }` | 2 days |
| Multi-target intrinsics | Multiple `#[intrinsic]` annotations per function | 2 days |
| Target constants | `DIGEST_WIDTH`, `FIELD_LIMBS`, `HASH_RATE` from config | 1 day |
| `--target` CLI flag | Select compilation target | 1 day |
| Target-specific `#[cfg]` | `#[cfg(triton)]`, `#[cfg(miden)]`, etc. | Already implemented |
| `ext.*` module namespace | Backend extension modules | 1 day |

### 8.2 Breaking Changes (Managed via Edition)

| Change | Impact | Migration |
|--------|--------|-----------|
| `Digest` width target-dependent | Hardcoded index `d[4]` may break on Miden (4 elements) | Use `DIGEST_WIDTH` constant |
| `split()` returns `[U32; FIELD_LIMBS]` | Tuple destructuring `let (hi, lo) = split(x)` breaks on Cairo | Use array indexing; `split_lo()`/`split_hi()` helpers for Goldilocks compat |
| `sponge_absorb()` arity changes | 10 args on Triton, 8 on Miden, 3 on Cairo | Array argument: `sponge_absorb(elements: [Field; HASH_RATE])` |
| Bare `asm { }` deprecated | Needs target tag for multi-target builds | Bare `asm { }` treated as `asm(triton) { }` with deprecation warning |

**Edition strategy:**

```toml
[project]
name = "my_project"
edition = "2026"            # Opt into multi-target semantics
entry = "main.tri"
```

Programs without an edition default to Triton-compatible behavior. Edition `"2026"` activates multi-target semantics.

---

## 9. Implementation Plan

### 9.1 Phase 0 — Internal Refactoring (No New Targets)

**Duration:** 2-3 weeks | **Risk:** Zero — no external-facing changes

Restructure compiler internals to support pluggable backends without changing any output.

| Task | Files affected | Effort |
|------|---------------|--------|
| Extract `TargetConfig` struct from hardcoded constants | New `target.rs` | 2 days |
| Refactor `emit.rs` → `emit_stack.rs` + `StackBackend` trait | `emit.rs` | 3 days |
| Refactor `stack.rs` to accept `stack_depth` parameter | `stack.rs` | 1 day |
| Refactor `cost.rs` to accept pluggable `CostTable` | `cost.rs` | 2 days |
| Parameterize `typeck.rs` for target constants | `typeck.rs`, `types.rs` | 2 days |
| Add `--target` CLI flag (only `triton` accepted) | `main.rs` | 1 day |
| Add target TOML loading | `target.rs` | 1 day |
| Restructure `std/` into layered directories, create `ext/` | `std/`, `resolve.rs` | 1 day |
| **Validation:** all 350+ existing tests pass unchanged | | 1 day |

**Deliverable:** Same compiler, same output, cleaner architecture. `TritonBackend` is the only backend but accessed through the trait interface.

### 9.2 Phase 1 — Miden VM Backend

**Duration:** 6-8 weeks | **Prerequisite:** Phase 0

Why Miden first: same field (Goldilocks), same architecture (stack, 16-element), same proof system family (STARK). Maximum code reuse, minimum risk.

| Task | Effort |
|------|--------|
| Create `targets/miden.toml` | 1 day |
| Implement `MidenBackend` (`StackBackend` trait) | 2 weeks |
| Adapt hash intrinsics (RPO instead of Tip5) | 1 week |
| Implement Miden cost model (chiplet-based) | 1 week |
| Merkle operations → `mtree_get` / `mtree_set` | 3 days |
| I/O mapping (advice provider) | 3 days |
| Miden linker (MAST program packaging) | 1 week |
| Miden backend extension modules (`ext.miden.*`) | 3 days |
| Port examples to dual-target | 1 week |
| Test suite (~100 new tests) | 1 week |

**Deliverable:** `trident build --target miden` produces valid Miden Assembly. Programs using only universal core + abstraction layer compile to both targets unchanged.

### 9.3 Phase 2 — Cairo/Sierra Backend

**Duration:** 3-4 months | **Prerequisite:** Phase 1

Introduces the register-machine IR and first non-stack backend.

| Task | Effort |
|------|--------|
| Design and implement IR (`ir.rs`) | 2 weeks |
| AST → IR lowering | 3 weeks |
| Register allocator for Cairo's AP/FP model | 2 weeks |
| Sierra IR emitter | 3 weeks |
| 252-bit field support in type checker | 1 week |
| Poseidon hash intrinsics | 1 week |
| Write-once memory model validation | 3 days |
| Cairo cost model (steps-based) | 1 week |
| Cairo backend extension modules (`ext.cairo.*`) | 1 week |
| Test suite (~150 new tests) | 2 weeks |

**Deliverable:** `trident build --target cairo` produces valid Sierra IR. The IR infrastructure enables SP1/RISC Zero with significantly less effort.

### 9.4 Phase 3 — SP1/RISC Zero Backend

**Duration:** 3-4 months | **Prerequisite:** Phase 2 (reuses IR)

| Task | Effort |
|------|--------|
| RISC-V rv32im emitter (from IR) | 4 weeks |
| RISC-V register allocator (32 registers) | 2 weeks |
| SP1/RISC Zero precompile mapping (SHA, Keccak, secp256k1) | 2 weeks |
| ELF output packaging | 1 week |
| SP1 I/O syscall mapping | 1 week |
| SP1 backend extension modules (`ext.sp1.*`) | 1 week |
| Test suite (~150 new tests) | 2 weeks |

### 9.5 Phase 4 — NockVM (Exploratory)

**Duration:** TBD (research phase) | **Prerequisite:** Phase 2+

| Task | Effort |
|------|--------|
| Research Nock compilation targets and Zorp proving model | 2 weeks |
| Prototype: Trident subset → Nock formulas (arithmetic + conditionals) | 4 weeks |
| Evaluate feasibility of full backend | 2 weeks |
| **Decision point:** proceed to full backend or park | — |

### 9.6 Timeline Summary

```
Month 1-2:    Phase 0 (refactor) + Phase 1 start (Miden)
Month 2-3:    Phase 1 complete (Miden backend shipping)
Month 4-6:    Phase 2 (Cairo/Sierra backend)
Month 7-9:    Phase 3 (SP1/RISC Zero backend)
Month 10+:    Phase 4 (NockVM research) + ecosystem maturation
```

---

## 10. Testing Strategy

### 10.1 Equivalence Testing

For programs using only universal core + abstraction layer, the same source must produce identical results across all targets:

1. Compile for all supported targets
2. Execute on each VM (or emulator) with identical inputs
3. Compare public outputs for exact equality
4. Report divergence as compiler bug

```bash
trident test main.tri --target all --equivalence
```

### 10.2 Test Suite Structure

| Category | Count (est.) | Scope |
|----------|:------------:|-------|
| Universal core | ~200 | Run on ALL targets |
| Abstraction layer | ~100 | Run on ALL targets; verify semantic equivalence |
| Hash/Merkle | ~50 per target | Run per-target; verify against reference implementations |
| Backend extensions | ~30 per target | Run only on matching target |
| Existing regression | ~350 | Triton regression suite (must never break) |
| **Total** | **~800+** | |

### 10.3 Cross-Target Fuzzing

Property-based fuzzing for multi-target correctness:

- Generate random valid Trident programs (universal core + abstraction layer only)
- Compile to all supported targets
- Execute on each VM
- Assert output equivalence

Catches semantic divergences between backends that unit tests might miss.

---

## 11. Success Criteria

### Phase 1 (Miden)

- [ ] `trident build --target miden` produces valid Miden Assembly for all universal programs
- [ ] Merkle verification runs correctly on both Triton and Miden from same source
- [ ] Cost reports show Miden-specific chiplet structure
- [ ] Existing Triton programs compile unchanged with `--target triton`
- [ ] At least 5 non-trivial programs compile and verify on both targets
- [ ] Compiler size under 15,000 lines (from ~12,000)

### Phase 2 (Cairo)

- [ ] `trident build --target cairo` produces valid Sierra IR
- [ ] IR is clean enough that a new register-machine backend can be added in <6 weeks
- [ ] Universal programs produce equivalent results across Triton, Miden, and Cairo
- [ ] Cairo cost model correctly reflects step-based proving cost

### Overall

- [ ] Developer can write a Merkle verifier once and deploy to any supported zkVM
- [ ] Adding a new stack-machine target takes <4 weeks
- [ ] Adding a new register-machine target takes <8 weeks (with IR reuse)
- [ ] Third-party backends can be added without modifying compiler core
- [ ] Trident is the most auditable multi-target ZK compiler available

---

## 12. Risk Analysis

| Risk | Likelihood | Impact | Mitigation |
|------|:----------:|:------:|------------|
| Field mismatch causes semantic divergence | Medium | High | Extensive equivalence testing; programs should never depend on specific modulus |
| Miden Assembly format changes (pre-1.0) | Medium | Medium | Pin to specific Miden version; abstract behind trait |
| IR complexity exceeds "minimal" | Low | High | Strict budget (~20 instruction types max); resist feature creep |
| Cost model abstraction loses precision | Medium | Medium | Each target keeps its own cost table; framework shared, numbers are not |
| Register allocator bugs (Cairo/RISC-V) | Medium | High | Proven algorithms (linear scan); extensive fuzzing |
| Target VM breaking changes (pre-1.0 VMs) | High | Medium | Version-pin targets; compatibility layers |
| Extension proliferation fragments ecosystem | Medium | Medium | Core standard library must remain universal; extensions are opt-in |

---

## 13. Complete Feature Portability Matrix

| # | Feature | Layer | Triton | Miden | Cairo | SP1/RZ | NockVM |
|:-:|---------|:-----:|:------:|:-----:|:-----:|:------:|:------:|
| | **Types** | | | | | | |
| 1 | `Field` (native field element) | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 2 | `Bool` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 3 | `U32` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 4 | `[T; N]` fixed arrays | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 5 | `(T1, T2)` tuples | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 6 | `struct` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 7 | `Digest` (`[Field; DIGEST_WIDTH]`) | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| 8 | `XField` (cubic extension) | Extension | ✅ | — | — | — | — |
| 9 | `Felt252` (explicit 252-bit) | Extension | — | — | ✅ | — | — |
| | **Field Arithmetic** | | | | | | |
| 10 | `a + b` (field add) | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 11 | `a * b` (field mul) | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 12 | `inv(a)` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 13 | `neg(a)` / `sub(a, b)` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 14 | `a == b` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 15 | `split(a)` → `[U32; FIELD_LIMBS]` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **Integer Arithmetic** | | | | | | |
| 16 | `a < b` (u32 compare) | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 17 | `a & b`, `a ^ b` (bitwise) | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 18 | `a /% b` (divmod) | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 19 | `log2`, `pow`, `popcount` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **Control Flow** | | | | | | |
| 20 | `if / else` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 21 | `for` bounded loops | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 22 | `match` expressions | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 23 | `assert()` / `assert_eq()` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 24 | `assert_digest()` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **Functions & Modules** | | | | | | |
| 25 | `fn` definitions | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 26 | `pub` / private visibility | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 27 | `module` / `use` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 28 | `const` declarations | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 29 | Size-generic `fn<N>()` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 30 | `#[cfg()]` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| 31 | `#[test]` | Core | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **I/O** | | | | | | |
| 32 | `pub_read()` / `pub_write()` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| 33 | `divine()` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| 34 | `pub input` / `pub output` / `sec input` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **Memory** | | | | | | |
| 35 | `ram_read()` / `ram_write()` | Abstraction | ✅ | ✅ | ⚠️¹ | ✅ | ✅ |
| 36 | `ram_read_block` / `ram_write_block` | Abstraction | ✅ | ✅ | ⚠️¹ | ✅ | ✅ |
| | **Cryptographic Primitives** | | | | | | |
| 37 | `hash()` → `Digest` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| 38 | `sponge_init/absorb/squeeze` | Abstraction | ✅ | ✅ | ✅ | ⚠️² | ⚠️ |
| 39 | `merkle.verify()` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **Events** | | | | | | |
| 40 | `emit Event { ... }` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| 41 | `seal Event { ... }` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **Cost Model** | | | | | | |
| 42 | `--costs` / `--hotspots` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| 43 | `--annotate` / `--compare` | Abstraction | ✅ | ✅ | ✅ | ✅ | ✅ |
| | **Backend Extensions** | | | | | | |
| 44 | `XField` arithmetic | Extension | ✅ | — | — | — | — |
| 45 | `xx_dot_step` / `xb_dot_step` | Extension | ✅ | — | — | — | — |
| 46 | `sponge_absorb_mem` | Extension | ✅ | — | — | — | — |
| 47 | `merkle_step_mem` | Extension | ✅ | — | — | — | — |
| 48 | Neptune kernel / UTXO | Extension | ✅ | — | — | — | — |
| 49 | Miden account model | Extension | — | ✅ | — | — | — |
| 50 | Miden note system | Extension | — | ✅ | — | — | — |
| 51 | Pedersen hash | Extension | — | — | ✅ | — | — |
| 52 | EC point operations | Extension | — | — | ✅ | — | — |
| 53 | SHA-256 / Keccak precompile | Extension | — | — | — | ✅ | — |
| 54 | secp256k1 verification | Extension | — | — | — | ✅ | — |
| 55 | Inline assembly | Extension | ✅³ | ✅³ | ✅³ | ✅³ | ✅³ |

**Notes:**
¹ Cairo memory is write-once; compiler warns on multiple writes to same address
² Software sponge implementation via precompile; higher cost than native
³ Target-tagged: `asm(triton) { ... }`, `asm(miden) { ... }`, etc. Each backend accepts only its own assembly syntax

### Layer Summary

| Layer | Feature count | % of language | Description |
|:-----:|:------------:|:-------------:|-------------|
| **Universal Core** | 31 | **56%** | Compiles identically to all targets |
| **Abstraction Layer** | 12 | **22%** | Same syntax, per-target dispatch |
| **Backend Extensions** | 12+ | **22%** | Target-specific capabilities (open-ended) |

---

*Trident Universal — Write once, prove anywhere.*
*This document is a living design. Extensions are driven by ecosystem needs and validated by working implementations.*
