---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-26
---
# 16 languages for superintelligence

[[Trident]] is one compiler. 16 languages are std libraries — type definitions + functions that compile to [[nox]] patterns. types determine [[five algebras|algebra]]. algebra determines [[lens]]. the chain is fully type-driven. zero annotations, zero configuration.

## the type-driven pipeline

```
.tri source → Trident frontend → typed AST → NounBuilder → nox noun
                                    ↓
                          type of each expression
                          determines everything:
                                    ↓
              Field → nebu → Brakedown → 1 constraint per mul
              BitVec → kuro → Binius → 1 constraint per op
              RingElement → jali → Ikat → batched
              Tropical → trop → Assayer → witness-proportional
              Curve → genies → Porphyry → 1 F_q per op
```

the programmer writes typed code. the types ARE the dispatch. no `#[algebra(...)]`. no backend selection. no prover hints. one nox noun comes out. one accumulator at the end.

## 16 languages × 5 arithmetics

| # | language | types | arithmetic | regime | [[lens]] | domain |
|---|----------|-------|-----------|--------|----------|--------|
| 1 | [[Tri]] | Fp2, Fp3, Fp4 | [[nebu]] | nebu²/³/⁴ | nebu | field tower, proofs |
| 2 | [[Tok]] | UTXO, Balance, Conservation | [[nebu]] | nebu | nebu | economic transactions |
| 3 | [[Arc]] | Object, Morphism, Functor | [[nebu]] | nebu | nebu | schema, graph structure |
| 4 | [[Seq]] | Order, Timestamp, Causality | [[nebu]] | nebu | nebu | sequence, ordering |
| 5 | [[Inf]] | Term, Clause, Substitution | [[nebu]] | nebu | nebu | logic, unification |
| 6 | [[Bel]] | Distribution, Probability | [[nebu]] | nebu | nebu | belief, self-model |
| 7 | [[Ren]] | Multivector, Rotor, Blade | [[nebu]] | nebu | nebu | geometry, rendering |
| 8 | [[Dif]] | DualNumber, Manifold | [[nebu]] | nebu | nebu | continuous dynamics |
| 9 | [[Sym]] | PhaseSpace, Hamiltonian | [[nebu]] | nebu | nebu | physics simulation |
| 10 | [[Ten]] | Matrix, Tensor | [[nebu]] | nebu | nebu | neural networks |
| 11 | [[Rs]] | u32, u64, bool, BoundedVec | [[nebu]] | nebu | nebu | systems programming |
| 12 | [[Wav]] | RingElement, NTTForm | [[jali]] | jali | jali | FHE, signal processing |
| 13 | [[Bt]] | BitVec, BitMatrix, Packed128 | [[kuro]] | kuro | kuro | binary, quantized inference |
| 14 | [[Qu]] | Qubit, Gate, Amplitude | [[nebu]] | nebu² | nebu | quantum simulation |
| 15 | [[Opt]] | Tropical, CostMatrix, Graph | [[trop]] | trop | trop | optimization, decisions |
| 16 | [[Sec]] | Curve, Secret, StealthAddress | [[genies]] | genies | genies | privacy, anonymity |

every algebra has at least one language. every language has types that map to one algebra. the mapping is exhaustive — no orphan types, no orphan algebras.

## what a language IS

a language is NOT a separate compiler. it is:

1. **types** — domain-specific structs in trident/std/
2. **functions** — operations on those types that compile to nox patterns
3. **jets** — recognized formula compositions that accelerate execution

```
trident/std/
├── tri/      Fp2, Fp3, Fp4 types + tower arithmetic
├── tok/      UTXO types + conservation constraints
├── arc/      Object, Morphism + category operations
├── seq/      Order types + causality
├── inf/      Term, Clause + unification
├── bel/      Distribution types + Bayesian update
├── ren/      Multivector types + geometric product
├── dif/      DualNumber types + automatic differentiation
├── sym/      PhaseSpace types + Hamiltonian evolution
├── ten/      Matrix, Tensor types + contraction
├── rs/       u32, u64, BoundedVec + systems ops
├── wav/      RingElement types + NTT multiply
├── bt/       BitVec, BitMatrix + binary ops
├── qu/       Qubit, Gate + quantum circuit
├── opt/      Tropical types + optimization algorithms
└── sec/      Curve types + privacy protocols
```

each directory: ~500-2000 LOC Trident. types + functions. no separate parser, no separate compiler, no separate runtime. one Trident, 16 libraries.

## cross-language composition

a single Trident program freely mixes types from different languages:

```trident
use std::ten::Matrix;       // nebu regime
use std::bt::BitMatrix;     // kuro regime
use std::opt::Tropical;     // trop regime

fn inference_with_optimization(
    weights: &BitMatrix,     // kuro: binary quantized weights
    input: &Matrix,          // nebu: field-valued input
    costs: &[Tropical],      // trop: routing costs
) -> Matrix {
    let quantized = bt::quantize(input);          // nebu → kuro boundary
    let hidden = bt::binary_matvec(weights, &quantized); // kuro regime
    let output = bt::dequantize(&hidden);         // kuro → nebu boundary
    let route = opt::shortest_path(costs);        // nebu → trop boundary
    ten::gather(&output, &route)                  // back to nebu
}
```

the compiler sees type transitions and inserts hemera commitments at boundaries automatically. the programmer never thinks about regimes or lenses.

## how mixing works end to end

```
source (typed)
  ↓ Trident frontend (typecheck)
typed AST (each expression tagged with type → algebra)
  ↓ NounBuilder (type-aware lowering)
nox noun (sub-trees correspond to different algebras)
  ↓ nox VM (executes uniformly, 16 patterns)
trace (each row carries operand types)
  ↓ zheng (partitions trace by type)
partition per algebra → prove via native lens
  ↓ HyperNova (folds all partitions)
one accumulator → one decider → one proof

programmer sees: types
compiler sees:   types → algebra
nox VM sees:     patterns (uniform)
zheng sees:      trace rows → lens per partition
verifier sees:   one proof (89 constraints)
```

## the 11 nebu languages

11 of 16 languages compile to the same nebu (F_p) regime. they differ in TYPES and SEMANTICS, not in algebra:

- Arc uses cons/compose patterns for category composition
- Ten uses mul/add patterns for tensor contraction
- Bel uses mul/add/inv patterns for Bayesian update
- Ren uses mul/add/sub patterns for geometric product
- Dif uses mul/add patterns on DualNumber (automatic differentiation)

the patterns are identical. the types give them meaning. a tensor contraction and a Bayesian update are the SAME nox patterns — different types, different interpretation.

this is not redundancy. each language adds a TYPE SYSTEM that prevents errors: you cannot multiply a Distribution by a Tensor. the types enforce domain boundaries that raw field arithmetic does not.

## why 16

every candidate was tested against the irreducibility criterion: removing the language creates a class of programs that cannot be conveniently expressed with the remaining 15.

- remove Opt → no native optimization types (Tropical, CostMatrix), no provable optimization
- remove Sec → no native privacy types (Curve, StealthAddress), no anonymous computation
- remove Wav → no native ring types (RingElement), no FHE
- remove Bt → no native binary types (BitVec), quantized inference forced through F_p (32× overhead)

the 11 nebu languages are individually removable in the algebraic sense — they all compile to the same regime. but they are semantically irreducible — each domain has types that prevent cross-domain errors.

discover all [[concepts]]
