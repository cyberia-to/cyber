---
tags: trident, cyber
alias: review passes, development review, trident_review_passes
crystal-type: reference
crystal-domain: cyber
---
# [[trident]] Development Review Passes

> Instead of "make it perfect", invoke passes by number.
> Example: "Run PASS 3 and PASS 7 on this module."

---

## PASS 1: DETERMINISM
- [ ] No floating point anywhere — all arithmetic over [[Goldilocks field]] p = 2⁶⁴ − 2³² + 1
- [ ] No HashMap iteration (non-deterministic order) — use BTreeMap or indexed vec
- [ ] No system clock, no randomness without explicit seed
- [ ] Serialization is canonical — single valid encoding per value
- [ ] Cross-platform: same input → same state root, always

Ask: *"Find any source of non-determinism in this code."*

---

## PASS 2: BOUNDED LOCALITY
- [ ] Every function's read-set is O(k)-bounded — trace it
- [ ] No hidden global state access (singletons, lazy_static with mutation)
- [ ] Graph walks have explicit depth/hop limits
- [ ] State updates touch only declared write-set — no side effects beyond it
- [ ] Verify: local change cannot trigger unbounded cascade

Ask: *"What is the maximum read-set and write-set of this function? Can a local change cascade globally?"*

---

## PASS 3: FIELD ARITHMETIC CORRECTNESS
- [ ] All reductions are correct mod p over [[Goldilocks field]] — no overflow before reduce
- [ ] Multiplication uses proper widening (u64 → u128 → reduce)
- [ ] Inverse/division handles zero case explicitly (panic or Option)
- [ ] Batch operations maintain invariant: individual vs batch results match
- [ ] Montgomery/Barrett boundaries are correct at edge values (0, 1, p−1, p)

Ask: *"Check edge cases: 0, 1, p−1, and values near 2⁶⁴. Does reduction ever overflow?"*

---

## PASS 4: CRYPTO HYGIENE
- [ ] No secret-dependent branching (constant-time operations)
- [ ] No secret data in error messages, logs, or Debug impls
- [ ] Zeroize sensitive memory on drop
- [ ] Hash domain separation — every use has unique prefix/tag
- [ ] Commitment scheme: binding + hiding properties preserved
- [ ] Proof constraints are neither under-constrained (soundness hole) nor over-constrained (completeness break)

Ask: *"Is there any path where secret material leaks through timing, errors, or logs?"*

---

## PASS 5: TYPE SAFETY & INVARIANTS
- [ ] Newtypes for distinct domains: ParticleId ≠ NeuronId ≠ CyberlinkId
- [ ] States encoded in types (e.g., `Unverified<Proof>` vs `Verified<Proof>`)
- [ ] `unsafe` blocks have safety comments explaining why invariant holds
- [ ] No `.unwrap()` on fallible paths — use `?` or explicit error
- [ ] Phantom types or sealed traits prevent invalid state construction

Ask: *"Can a caller construct an invalid state? Can types from different domains be accidentally mixed?"*

---

## PASS 6: ERROR HANDLING & DEGRADATION
- [ ] Every error type is meaningful — no `anyhow` in library code
- [ ] Errors propagate without losing context (error chains)
- [ ] No panic in library code — only in binary entry points or proved-unreachable
- [ ] Resource cleanup on all error paths (RAII, Drop impls)
- [ ] Partial failure doesn't corrupt shared state

Ask: *"What happens when this fails halfway through? Is state still consistent?"*

---

## PASS 7: ADVERSARIAL INPUT
- [ ] All external inputs are validated before processing
- [ ] Sizes, lengths, indices are bounds-checked
- [ ] No allocation proportional to untrusted input without cap
- [ ] Graph operations handle: empty graph, single node, disconnected components, self-loops, duplicate edges
- [ ] Malformed proofs/signatures rejected before expensive computation

Ask: *"What's the cheapest input an attacker can craft to cause maximum damage (CPU, memory, state corruption)?"*

---

## PASS 8: ARCHITECTURE & COMPOSABILITY
- [ ] Module has single responsibility — one reason to change
- [ ] Dependencies point inward (domain ← application ← infra)
- [ ] Traits define boundaries — implementations are swappable
- [ ] No circular dependencies between modules
- [ ] Public API is minimal — nothing exposed without reason

Ask: *"Can I replace the implementation behind this trait without touching callers? What would break?"*

---

## PASS 9: READABILITY & NAMING
- [ ] Names match the whitepaper terminology exactly (π, τ, Δπ, neuron, particle, cyberlink)
- [ ] Functions do what their name says — no hidden side effects
- [ ] Complex logic has comments explaining *why*, not *what*
- [ ] Magic numbers are named constants with units in the name (e.g., `MAX_HOP_DEPTH`, `CONVERGENCE_EPSILON`)
- [ ] Code reads top-down — high-level flow visible without diving into helpers

Ask: *"Can someone reading only this file understand what it does and why, without reading other files?"*

---

## PASS 10: COMPACTNESS & ELIMINATION
- [ ] No dead code, no commented-out blocks
- [ ] No premature abstraction — if only one impl exists, don't trait it yet
- [ ] No duplicate logic — if two functions share structure, extract or explain why not
- [ ] No unnecessary allocations (clone, to_vec, collect where iter suffices)
- [ ] Ask: "what can I delete?" before "what should I add?"

Ask: *"What can be removed from this code without changing behavior?"*

---

## PASS 11: PERFORMANCE & SCALABILITY
- [ ] Hot path is allocation-free (pre-allocated buffers, arena allocation)
- [ ] No O(n²) or worse without explicit justification and n-bound
- [ ] Batch operations exist for anything called in loops
- [ ] Cache-friendly access patterns (sequential over random)
- [ ] Profiled, not guessed — benchmark before and after optimization

Ask: *"What is the complexity of this at 10⁹ nodes? Where does it break first?"*

---

## PASS 12: TESTABILITY
- [ ] Pure functions where possible — output depends only on input
- [ ] Side effects are injected (trait objects, closures), not hardcoded
- [ ] Property-based tests for invariants (proptest/quickcheck)
- [ ] Edge case tests: empty, one, max, overflow, malicious
- [ ] Test names describe the property being verified, not the method being called

Ask: *"What property should always hold? Write a proptest for it."*

---

## Quick Reference — Copy-Paste Prompts

```
"Run PASS 1 (determinism) on this module."
"Run PASS 4 (crypto) and PASS 7 (adversarial) on this function."
"Run PASS 10 (compactness) — what can I delete?"
"Run PASS 3 (field arithmetic) — check edge values 0, 1, p−1."
"Run PASS 8 (architecture) — draw the dependency graph."
"Run all security passes (1, 3, 4, 7) on this PR."
```

### Severity Tiers

| Tier | Passes | When |
|------|--------|------|
| Every commit | 1, 5, 6, 9 | Determinism, types, errors, readability |
| Every PR | + 2, 7, 8, 10 | Locality, adversarial, architecture, compactness |
| Every release | + 3, 4, 11, 12 | Crypto, field math, performance, full test coverage |