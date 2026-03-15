---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: nox spec, nox patterns, reduction patterns, sixteen patterns, cyber/patterns
stake: 43936669831471920
---
# nox/spec

formal specification of the [[nox]] virtual machine. sixteen deterministic reduction patterns (Layer 1), one non-deterministic witness injection (Layer 2), five jets for efficient recursive [[stark]] verification (Layer 3).

## field

```
FIELD: Goldilocks
  p = 2^64 - 2^32 + 1 = 18446744069414584321
  Primitive root: 7
  2^32-th root of unity: 1753635133440165772

  Efficient reduction:
    a mod p = a_lo - a_hi × (2^32 - 1) + correction

HASH: Poseidon-Goldilocks
  State: 12 field elements, Rate: 8 elements
  Rounds: 8 full + 22 partial + 8 full
  Cost: ~300 stark constraints per permutation
  Status: CONFIGURABLE (Poseidon is reference, not mandated)

DOMAIN SEPARATION
  COMMITMENT_DOMAIN  = 0x4E4F582020524543  // "NOX  REC"
  NULLIFIER_DOMAIN   = 0x4E4F5820204E554C  // "NOX  NUL"
  MERKLE_DOMAIN      = 0x4E4F5820204D524B  // "NOX  MRK"
  OWNER_DOMAIN       = 0x4E4F5820204F574E  // "NOX  OWN"

STRUCTURAL HASH
  H(Atom a)       = HASH(0x00 ‖ type_tag(a) ‖ encode(a))
  H(Cell(l, r))   = HASH(0x01 ‖ H(l) ‖ H(r))
```

## value tower

```
┌───────────────────────────────────────────────────────────────────────┐
│  TYPE TAG    │  REPRESENTATION     │  VALID RANGE    │  USE          │
├──────────────┼─────────────────────┼─────────────────┼───────────────┤
│  0x00: field │  Single F_p element │  [0, p)         │  Arithmetic   │
│  0x01: word  │  Single F_p element │  [0, 2^64)      │  Bitwise      │
│  0x02: hash  │  4 × F_p elements   │  256-bit digest │  Identity     │
└───────────────────────────────────────────────────────────────────────┘

COERCION RULES
  field → word:  Valid iff value < 2^64 (always true for Goldilocks)
  word → field:  Always valid (injection)
  hash → field:  Extract first element (lossy, for compatibility only)
  field → hash:  Forbidden (use HASH pattern)

TYPE ERRORS
  Bitwise op on hash → ⊥_error
  Arithmetic on hash (except equality) → ⊥_error
```

## reduction signature

```
reduce : (Subject, Formula, Focus) → Result

Result = (Noun, Focus')     — success with remaining focus
       | Halt               — focus exhausted
       | ⊥_error            — type/semantic error
       | ⊥_unavailable      — referenced content not retrievable
```

---

## Layer 1: sixteen deterministic patterns

the deterministic core. both prover and verifier execute these identically. confluent by Huet-Levy (1980): orthogonal rewrite system, any evaluation order yields the same result

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                       LAYER 1: REDUCTION PATTERNS                         ║
╠═══════════════════════════════════════════════════════════════════════════╣
║  STRUCTURAL (5)              FIELD ARITHMETIC (6)                         ║
║  0: axis — navigate          5: add — (a + b) mod p                       ║
║  1: quote — literal          6: sub — (a - b) mod p                       ║
║  2: compose — recursion      7: mul — (a × b) mod p                       ║
║  3: cons — build cell        8: inv — a^(p-2) mod p                       ║
║  4: branch — conditional     9: eq  — equality test                       ║
║                              10: lt — less-than                           ║
║                                                                           ║
║  BITWISE (4)                 HASH (1)                                     ║
║  11: xor    12: and          15: hash — structural H(x)                   ║
║  13: not    14: shl                                                       ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

### structural patterns

```
PATTERN 0: AXIS
reduce(s, [0 a], f) → (axis(s, eval(a)), f - 1 - depth)

  axis(s, 0) = H(s)           ; hash introspection
  axis(s, 1) = s              ; identity
  axis(s, 2) = head(s)        ; left child (⊥_error if atom)
  axis(s, 3) = tail(s)        ; right child (⊥_error if atom)
  axis(s, 2n) = axis(axis(s,n), 2)
  axis(s, 2n+1) = axis(axis(s,n), 3)


PATTERN 1: QUOTE
reduce(s, [1 c], f) → (c, f - 1)

Returns c literally, unevaluated.


PATTERN 2: COMPOSE
reduce(s, [2 [x y]], f) =
  let (rx, f1) = reduce(s, x, f - 2)
  let (ry, f2) = reduce(s, y, f1)
  reduce(rx, ry, f2)

PARALLELISM: reduce(s,x) and reduce(s,y) are INDEPENDENT.


PATTERN 3: CONS
reduce(s, [3 [a b]], f) =
  let (ra, f1) = reduce(s, a, f - 2)
  let (rb, f2) = reduce(s, b, f1)
  ([ra, rb], f2)

PARALLELISM: reduce(s,a) and reduce(s,b) are INDEPENDENT.


PATTERN 4: BRANCH (lazy!)
reduce(s, [4 [test [yes no]]], f) =
  let (t, f1) = reduce(s, test, f - 2)
  if t = 0 then reduce(s, yes, f1)
           else reduce(s, no, f1)

CRITICAL: Only ONE branch evaluated. Prevents infinite recursion DoS.
```

### arithmetic patterns

```
PATTERN 5: ADD
reduce(s, [5 [a b]], f) →
  let (v_a, f1) = reduce(s, a, f - 1)
  let (v_b, f2) = reduce(s, b, f1)
  ((v_a + v_b) mod p, f2)


PATTERN 6: SUB
reduce(s, [6 [a b]], f) → ((v_a - v_b) mod p, f2)


PATTERN 7: MUL
reduce(s, [7 [a b]], f) → ((v_a × v_b) mod p, f2)


PATTERN 8: INV
reduce(s, [8 a], f) →
  let (v_a, f1) = reduce(s, a, f - 64)
  if v_a = 0 then ⊥_error
  (v_a^(p-2) mod p, f1)

RATIONALE: Execution cost reflects real work (~64 multiplications
in square-and-multiply for Fermat's little theorem).
stark verification cost = 1 constraint (verifier just checks a × a⁻¹ = 1).


PATTERN 9: EQ
reduce(s, [9 [a b]], f) → (0 if v_a = v_b else 1, f2)


PATTERN 10: LT
reduce(s, [10 [a b]], f) → (0 if v_a < v_b else 1, f2)
```

### bitwise patterns

```
Valid on word type [0, 2^64). Bitwise on hash → ⊥_error.

PATTERN 11: XOR
reduce(s, [11 [a b]], f) → (v_a ⊕ v_b, f2)

PATTERN 12: AND
reduce(s, [12 [a b]], f) → (v_a ∧ v_b, f2)

PATTERN 13: NOT
reduce(s, [13 a], f) → (v_a ⊕ (2^64 - 1), f1)

PATTERN 14: SHL
reduce(s, [14 [a n]], f) → ((v_a << v_n) mod 2^64, f2)
```

### hash pattern

```
PATTERN 15: HASH
reduce(s, [15 a], f) →
  let (v_a, f1) = reduce(s, a, f - 300)
  (H(v_a), f1)

Result is 4-element hash (256 bits).

Hash CAN be expressed as pure Layer 1 patterns (~2800 field ops for Poseidon).
Pattern 15 is also the first Layer 3 jet. Jets accelerate; semantics unchanged.
```

---

## Layer 2: non-deterministic input

one instruction: `hint`. the prover injects a witness value from outside the VM; Layer 1 constraints verify it.

```
PATTERN 16: HINT
reduce(s, [16 constraint], f) =
  let (check, f1) = reduce(s, constraint, f - 1)
  let w = PROVER_INJECT()
  assert check(w) = 0            — Layer 1 verifies the constraint
  (w, f1)

PROVER_INJECT: → Noun
  Source:   external to the VM. prover-only.
  Verifier: NEVER executes hint directly.
             checks constraint satisfaction via stark (multilinear trace + sumcheck).
  Cost:     1 + cost(constraint). witness search is external.
  Memo:     NOT memoizable (different provers, different valid witnesses).
```

---

## Layer 3: jets

five jets selected by analyzing the stark verifier bottleneck. every jet has an equivalent Layer 1 program producing identical output on all inputs.

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                          LAYER 3: JETS                                    ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  JET 0: HASH                                                              ║
║  hash(x) → 4 × F_p digest                                                ║
║  Pure equivalent: ~2,800 field ops (Poseidon2 permutation)                ║
║  Jet cost: 300                                                            ║
║  Accelerates: Fiat-Shamir challenges, Merkle tree construction            ║
║                                                                           ║
║  JET 1: POLY_EVAL                                                         ║
║  poly_eval(coeffs, point) → F_p                                           ║
║  Horner evaluation of degree-N polynomial at a single point               ║
║  Pure equivalent: ~2N patterns (N muls + N adds)                          ║
║  Jet cost: N                                                              ║
║  Accelerates: WHIR query verification, constraint evaluation              ║
║                                                                           ║
║  JET 2: MERKLE_VERIFY                                                     ║
║  merkle_verify(root, leaf, path, index) → {0, 1}                         ║
║  Verify authentication path of depth d                                    ║
║  Pure equivalent: d × ~310 patterns (hash + conditional per level)        ║
║  Jet cost: d × 300                                                        ║
║  Accelerates: stark proof checking (500K → 50K of verifier cost)          ║
║                                                                           ║
║  JET 3: FRI_FOLD                                                          ║
║  fri_fold(poly_layer, challenge) → poly_layer_next                        ║
║  One round of FRI folding: split by parity, combine with challenge        ║
║  Pure equivalent: ~N patterns (N/2 muls + N/2 adds + restructuring)       ║
║  Jet cost: N/2                                                            ║
║  Accelerates: WHIR verification (log(N) folding rounds)                   ║
║                                                                           ║
║  JET 4: NTT                                                               ║
║  ntt(values, direction) → transformed values                              ║
║  Number Theoretic Transform (forward or inverse) over F_p                 ║
║  Pure equivalent: ~2N·log(N) patterns (butterfly operations)              ║
║  Jet cost: N·log(N)                                                       ║
║  Accelerates: polynomial multiplication, WHIR commitment, proof aggregation║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

jet semantic contract: every jet MUST have an equivalent pure Layer 1 expression producing identical output on all inputs. jets are OPTIMIZATION. semantics unchanged.

the five jets map to the four [[Goldilocks field processor]] hardware primitives:

| GFP primitive | jets it accelerates |
|---------------|---------------------|
| fma (field multiply-accumulate) | poly_eval (Horner's method = iterated FMA) |
| ntt (NTT butterfly) | ntt (direct correspondence) |
| p2r (Poseidon2 round) | hash, merkle_verify (hash-dominated) |
| lut (lookup table) | activation functions via Layer 1 patterns |

---

## cost table

```
Layer │ Pattern      │ Exec Cost      │ stark Constraints
──────┼──────────────┼────────────────┼───────────────────
  1   │ 0 axis       │ 1 + depth      │ ~depth
  1   │ 1 quote      │ 1              │ 1
  1   │ 2 compose    │ 2              │ 2
  1   │ 3 cons       │ 2              │ 2
  1   │ 4 branch     │ 2              │ 2
  1   │ 5 add        │ 1              │ 1
  1   │ 6 sub        │ 1              │ 1
  1   │ 7 mul        │ 1              │ 1
  1   │ 8 inv        │ 64             │ 1
  1   │ 9 eq         │ 1              │ 1
  1   │ 10 lt        │ 1              │ ~64
  1   │ 11-14 bit    │ 1              │ ~64 each
  2   │ 16 hint      │ 1 + constraint │ constraint rows
  3   │ hash         │ 300            │ ~300
  3   │ poly_eval(N) │ N              │ ~N
  3   │ merkle_v(d)  │ d × 300        │ ~d × 300
  3   │ fri_fold(N)  │ N/2            │ ~N/2
  3   │ ntt(N)       │ N·log(N)       │ ~N·log(N)
```

## stark verifier cost with jets

```
Component               │ Layer 1 only │ With jets  │ Reduction
────────────────────────┼──────────────┼────────────┼──────────
Parse proof             │     ~1,000   │    ~1,000  │  1×
Fiat-Shamir challenges  │    ~30,000   │    ~5,000  │  6×
Merkle verification     │   ~500,000   │   ~50,000  │ 10×
Constraint evaluation   │    ~10,000   │    ~3,000  │  3×
WHIR verification       │    ~50,000   │   ~10,000  │  5×
────────────────────────┼──────────────┼────────────┼──────────
TOTAL                   │   ~600,000   │   ~70,000  │ ~8.5×
```

## parallel reduction

Layer 1 patterns form an orthogonal rewrite system: each has a unique tag, no two overlap, left-hand sides are linear. by Huet-Levy (1980), orthogonal systems are confluent without requiring termination

corollary: parallel and sequential reduction yield identical results for Layer 1

Layer 2 (`hint`) breaks confluence intentionally — multiple valid witnesses may satisfy the same constraints. soundness is preserved: no invalid witness passes the constraint check

Layer 3 (jets) preserves confluence — jets are observationally equivalent to their Layer 1 expansions

```
PATTERN PARALLELISM
───────────────────

Pattern 2 (compose):  [2 [x y]]
  reduce(s,x) ∥ reduce(s,y)  — INDEPENDENT
  Then: reduce(result_x, result_y)

Pattern 3 (cons):     [3 [a b]]
  reduce(s,a) ∥ reduce(s,b)  — INDEPENDENT
  Then: Cell(result_a, result_b)

Patterns 5-7, 9-12:   [op [a b]]
  reduce(s,a) ∥ reduce(s,b)  — INDEPENDENT
  Then: apply op

Pattern 4 (branch):   [4 [t [c d]]]
  reduce(s,t) first
  Then: ONE of reduce(s,c) or reduce(s,d)  — NOT parallel (lazy)
```

## global memoization

```
GLOBAL CACHE
────────────
Key:   (H(subject), H(formula))
Value: H(result)

Properties:
- Universal: Any node can contribute/consume
- Permanent: Results never change (determinism)
- Verifiable: Result hash checkable against proof

LAYER SCOPE:
- Layer 1: fully memoizable (deterministic)
- Layer 2: NOT memoizable (hint results are prover-specific)
- Layer 3: fully memoizable (jets are deterministic)

Computations containing hint are excluded from the global cache.
Pure subexpressions within a hint-containing computation remain memoizable.
```

## test vectors

```
add(1, 2) = 3
mul(p-1, p-1) = 1
inv(2) = 9223372034707292161

reduce([1,2], [5 [[0 2] [0 3]]], 100) = (3, 96)
  // add(axis 2, axis 3) = add(1, 2) = 3
```

## cost examples

```
Simple addition: 4 patterns
  [5 [[0 2] [0 3]]]
  Cost: 1 (add) + 2 (axis) + overhead = ~4

Poseidon hash: 300 (jet) or ~2800 (pure Layer 1)
  [15 [0 1]]
  Jet cost: 300

Merkle verification (32 levels): ~9,600 (jet) or ~10,000 (pure Layer 1)
  merkle_verify(root, leaf, path, 32)
  Jet cost: 32 × 300 = 9,600

stark verifier (one recursion level): ~70,000 (with jets)
  Without jets: ~600,000 Layer 1 patterns
```

see [[nox]] for the design philosophy, [[cyber/stark]] for the proof pipeline, [[Goldilocks field]] for the arithmetic, [[Goldilocks field processor]] for hardware acceleration
