---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: state management, world state, state transitions
---
# State Management

The [[nox]] world state and transition rules. All state lives in the [[cyber/bbg]] — a unified polynomial structure. Transitions are proven valid by [[zheng]] proofs.

## World State

```
WORLD STATE W
─────────────

W = (BBG, edge_store, privacy_state)

where BBG = Big Badass Graph commitment:
  BBG_root = H(
    by_neuron.commit ‖
    by_particle.commit ‖
    focus.commit ‖
    balance.commit ‖
    commitment_poly.commit ‖
    nullifier_set.commit
  )

and edge_store = content-addressed storage:
  edge_store : H(edge) → edge

and privacy_state = UTXO state:
  particle_energy : PolynomialCommitment
  commitment_poly : PolynomialCommitment
  nullifier_set   : PolynomialCommitment
  total_energy    : u64
```

## State Transitions

```
TRANSITION: W × Transaction → W' | ⊥

TRANSACTION TYPES
─────────────────
1. Cyberlink: Add edge to graph
   tx = (neuron, from_particle, to_particle, weight, signature)

2. Transfer: Move balance between neurons (public)
   tx = (from_neuron, to_neuron, amount, signature)

3. PrivateTransfer: Move energy between records (ZK)
   tx = (nullifiers, commitments, deltas, fee, proof)

4. Computation: Execute nox reduction
   tx = (neuron, subject, formula, budget, signature)


VALIDITY CONDITIONS
───────────────────
1. Authorization: signature valid OR ZK proof valid
2. Balance: sufficient balance for any transfers
3. Focus: sufficient focus for any computation
4. Conservation: Σ focus' = 1, Σ balance' = B_total, energy conserved
5. Consistency: all indexes updated correctly
6. Availability: all referenced content retrievable
7. Double-spend: nullifiers not in nullifier_set


PROOF OF VALID TRANSITION
─────────────────────────
stark proves:
  1. All validity conditions hold
  2. New state correctly derived from old state + tx
  3. All indexes (by_neuron, by_particle) updated consistently
  4. ZK proofs verified (for private transactions)

Proof size: O(log |W|)
Verification: O(log |W|)
```