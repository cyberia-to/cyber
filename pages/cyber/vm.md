---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
status: draft
alias: CORE vm, CORE patterns, reduction patterns, sixteen patterns, cyber/patterns
---
# CORE virtual machine

the execution model of [[CORE]]: sixteen orthogonal reduction patterns over Goldilocks field elements, a three-type value tower, deterministic cost model, confluent parallel reduction, and global memoization

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
  Cost: ~300 STARK constraints per permutation
  Status: CONFIGURABLE (Poseidon is reference, not mandated)

DOMAIN SEPARATION
  COMMITMENT_DOMAIN  = 0x434F524520524543  // "CORE REC"
  NULLIFIER_DOMAIN   = 0x434F5245204E554C  // "CORE NUL"
  MERKLE_DOMAIN      = 0x434F5245204D524B  // "CORE MRK"
  OWNER_DOMAIN       = 0x434F5245204F574E  // "CORE OWN"

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

## sixteen patterns

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                       CORE REDUCTION PATTERNS                              ║
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

## structural patterns

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

## arithmetic patterns

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
STARK verification cost = 1 constraint (verifier just checks a × a⁻¹ = 1).


PATTERN 9: EQ
reduce(s, [9 [a b]], f) → (0 if v_a = v_b else 1, f2)


PATTERN 10: LT
reduce(s, [10 [a b]], f) → (0 if v_a < v_b else 1, f2)
```

## bitwise patterns

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

## hash pattern

```
PATTERN 15: HASH
reduce(s, [15 a], f) →
  let (v_a, f1) = reduce(s, a, f - 300)
  (H(v_a), f1)

Result is 4-element hash (256 bits).

NOTE: Hash CAN be expressed as pure patterns (~2800 field ops for Poseidon).
Pattern 15 is OPTIMIZATION. Jets accelerate; semantics unchanged.
```

## cost table

```
Pattern │ Exec Cost │ STARK Constraints
────────┼───────────┼───────────────────
0 axis  │ 1+depth   │ ~depth
1 quote │ 1         │ 1
2 comp  │ 2         │ 2
3 cons  │ 2         │ 2
4 branch│ 2         │ 2
5 add   │ 1         │ 1
6 sub   │ 1         │ 1
7 mul   │ 1         │ 1
8 inv   │ 64        │ 1
9 eq    │ 1         │ 1
10 lt   │ 1         │ ~64
11-14   │ 1         │ ~64 each
15 hash │ 300       │ ~300
```

cost is deterministic: depends only on syntactic structure, never on runtime values. cache hits reduce work but cost is charged in full — cost is the right to a result, not payment for computation

## parallel reduction

the 16 patterns form an orthogonal rewrite system: each has a unique tag, no two overlap, left-hand sides are linear. by Huet-Levy (1980), orthogonal systems are confluent without requiring termination

corollary: parallel and sequential reduction yield identical results

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

Poseidon hash: ~300 patterns
  [15 [0 1]]
  Cost: 300

Merkle verification (32 levels): ~10,000 patterns
  32 × (hash + conditional) ≈ 32 × 310
```
