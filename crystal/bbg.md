---
tags: cyber
alias: bbg, Big Badass Graph, authenticated state, cyber/bbg
crystal-type: entity
crystal-domain: cyber
---
the authenticated state layer for [[cyber]]. the entire [[cybergraph]] commits to a single polynomial:

$$\text{BBG\_root} = \text{Lens.commit}(\text{BBG\_poly}) \quad \text{(32 bytes)}$$

one polynomial. all state. every query is a polynomial opening (~200 bytes, 10-50 μs). cross-index consistency is structural — different evaluation dimensions of the same polynomial cannot disagree.

bbg is to [[cybergraph]] what a database engine is to a schema. cybergraph defines WHAT. bbg implements HOW.

## three laws

1. bounded locality — operation cost $\propto$ what it touches, not total state size
2. constant-cost verification — one [[lens]] opening: ~200 bytes proof, 10-50 μs
3. structural security — polynomial binding prevents lying. post-quantum ([[Brakedown]], no pairings)

## structure

BBG_poly is a single multivariate polynomial with three dimensions:

$$\text{BBG\_poly}(\text{index}, \text{key}, t) = \text{value}$$

| index | domain | key | value |
|-------|--------|-----|-------|
| 0: particles | content-addressed nodes | CID | energy, φ*, axon fields |
| 1: axons_out | outgoing edges by source | source CID | axon pointer, weight |
| 2: axons_in | incoming edges by target | target CID | axon pointer, weight |
| 3: neurons | agent state | neuron ID | focus, karma, stake |
| 4: locations | spatial association | neuron ID | geohash, attestation |
| 5: coins | fungible tokens | denomination | supply, parameters |
| 6: cards | non-fungible assets | card ID | owner, content CID |
| 7: files | content availability | CID | DAS commitment, chunk count |
| 8: time | historical snapshots | time namespace | BBG_root at that time |
| 9: signals | finalized signal batches | step | signal hash |

no NMT. no [[LogUp]]. cross-index consistency is FREE — axons_out and axons_in are different evaluation dimensions of the same committed polynomial.

## private state

individual [[cyberlinks]] are private. polynomial commitments handle this:
- commitment polynomial A(x): all committed private records. membership = one [[lens]] opening
- nullifier polynomial N(x): all spent nullifiers. non-membership = one [[lens]] opening showing N(c) ≠ 0

~5,000 constraints per private operation (was ~40,000 with SWBF + MMR). 32-byte witness (was 128 KB).

## dependency graph

```
hemera (hash) → lens (commit) → nox (run) → zheng (prove) → bbg (store) ← this repo
```

see [[cyber/research/bbg]] for the full polynomial state specification, [[lens]] for commitment backends, [[Brakedown]] for the polynomial commitment scheme

discover all [[concepts]]
