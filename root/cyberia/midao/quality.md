---
tags: cyber, article
alias: quality control, review passes, code quality
crystal-type: pattern
crystal-domain: cyber
diffusion: 0.00011233815923477823
springs: 0.00008246788381555257
heat: 0.000037649645370807764
focus: 0.00008843937383621902
gravity: 0
density: 0
---
# quality control

a methodology for reviewing code in projects where correctness matters.
every line of code may end up in a proof circuit, a financial system,
or an autonomous agent. quality means soundness.

## file size limit

no single source file should exceed 500 lines. if it does, split it
into submodules. entry point files (re-exports only) are the only
exception.

## review passes

invoke passes by number. on full audit — run all passes in parallel
using agents, persist results, prepare a fix plan before applying.

### pass 1: determinism

- no floating point in deterministic paths
- no hash map iteration (non-deterministic order)
- no system clock, no randomness without explicit seed
- serialization is canonical — single valid encoding per value
- cross-platform: same input → same output, always

"find any source of non-determinism in this code."

### pass 2: bounded locality

- every function's read-set is O(k)-bounded
- no hidden global state
- graph walks have explicit depth/hop limits
- state updates touch only declared write-set
- local change cannot trigger unbounded cascade

"what is the maximum read-set and write-set?"

### pass 3: domain arithmetic correctness

- all reductions correct — no overflow before reduce
- multiplication uses widening where needed
- inverse/division handles zero explicitly
- batch operations: individual vs batch results match
- edge values correct: 0, 1, max-1, max

"check edge cases. does reduction overflow?"

### pass 4: crypto hygiene

- no secret-dependent branching (constant-time)
- no secret data in error messages, logs, or debug output
- zeroize sensitive memory on drop
- hash domain separation — unique prefix per use
- constraints: neither under-constrained nor over-constrained

"is there any path where secret material leaks?"

### pass 5: type safety and invariants

- newtypes for distinct domains (UserId != PostId)
- states encoded in types (Unverified vs Verified)
- unsafe blocks have safety comments
- no unwrap on fallible paths
- invalid state construction prevented by type system

"can a caller construct an invalid state?"

### pass 6: error handling and degradation

- every error type is meaningful
- errors propagate with context
- no panic in library code
- resource cleanup on all error paths
- partial failure does not corrupt shared state

"what happens when this fails halfway through?"

### pass 7: adversarial input

- all external inputs validated before processing
- sizes, lengths, indices bounds-checked
- no allocation proportional to untrusted input without cap
- malformed input rejected before expensive computation

"what is the cheapest input an attacker can craft for maximum damage?"

### pass 8: architecture and composability

- single responsibility per module
- dependencies point inward (domain ← application ← infra)
- traits/interfaces define boundaries
- no circular dependencies
- public API is minimal

"can I replace this implementation without touching callers?"

### pass 9: readability and naming

- names match domain terminology
- functions do what their name says — no hidden side effects
- comments explain why, not what
- magic numbers are named constants with units
- code reads top-down

"can someone reading only this file understand what it does and why?"

### pass 10: compactness and elimination

- no dead code, no commented-out blocks
- no premature abstraction — one impl does not need a trait
- no duplicate logic
- no unnecessary allocations
- "what can I delete?" before "what should I add?"

"what can be removed without changing behavior?"

### pass 11: performance and scalability

- hot path is allocation-free
- no O(n^2) without justification and n-bound
- batch operations for anything called in loops
- cache-friendly access patterns
- profiled, not guessed

"what is the complexity at 10^9 items? where does it break first?"

### pass 12: testability

- pure functions where possible
- side effects injected (trait objects, closures)
- property-based tests for invariants
- edge case tests: empty, one, max, overflow, malicious
- test names describe the property, not the method

"what property should always hold? write a property test for it."

## severity tiers

| tier | passes | when |
|------|--------|------|
| every commit | 1, 5, 6, 9 | determinism, types, errors, readability |
| every PR | + 2, 7, 8, 10 | locality, adversarial, architecture, compactness |
| every release | + 3, 4, 11, 12 | crypto, arithmetic, performance, full test coverage |

## audit protocol

1. launch parallel agents partitioned by module scope (no overlapping files)
2. each agent runs assigned passes, writes findings to a results directory
3. main session reads findings, summarizes, prepares a fix plan
4. user confirms the fix plan before any changes are applied
5. fixes applied as atomic commits. stale findings cleaned up