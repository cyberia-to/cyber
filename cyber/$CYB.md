---
tags: cyber, cybernomics
alias: cyber energy
crystal-type: entity
crystal-domain: economics
---
root [[token]] of planned [[cyber]] [[superintelligence]]

the fuel of the protocol — resources consumed by [[cybics/crystal/neuron|neurons]] to create [[cyberlinks]], compute [[soft3/tru/specs/focus|focus]], and participate in [[consensus]]

[[bandwidth]], [[soft3/tru/specs/focus|focus]], and [[token|tokens]] are all forms of energy in the system

currently minted as [[$C]] in [[bostrom]] [[bootloader]]

## Supply

Total $CYB supply is the [[nebu|Goldilocks field]] order itself — p = 2⁶⁴ − 2³² + 1 = 18,446,744,069,414,584,321. Balances are field elements, so token value and [[soft3/nox|nox]] computation share one modulus: the cap is a law of the field, not a governance parameter.

## Genesis

At genesis, TOCYB holders hold 281,405,532,467,645 $CYB — the migration snapshot, 0.001526% of total (≈ 1/65,552 of the field). Every other unit enters supply through emission.

## Emission — seven halvings, summed

Split the supply into seven equal chunks of p/7 = 2,635,249,152,773,512,045 each (14.2857%). Mint each chunk Satoshi-style — a fixed rate that halves on its own period — then sum the seven curves into one. Each period governs one chunk:

| chunk | halving period | half the chunk by | ~fully emitted |
|-------|----------------|-------------------|----------------|
| 1 | 1 day | day 1 | ~2 weeks |
| 2 | 7 days | week 1 | ~2 months |
| 3 | 1 month | month 1 | ~10 months |
| 4 | 3 months | month 3 | ~2.5 years |
| 5 | 1 year | year 1 | ~10 years |
| 6 | 2 years | year 2 | ~20 years |
| 7 | 4 years | year 4 | ~40 years |

One chunk's cumulative mint is Mᵢ(t) = (p/7)(1 − 2^(−t/Tᵢ)). The network curve is their sum:

M(t) = Σᵢ Mᵢ(t) = p − (p/7) · Σᵢ 2^(−t/Tᵢ)

with emission rate dM/dt = (ln2 · p/7) · Σᵢ (1/Tᵢ) · 2^(−t/Tᵢ). Emission is 0 at t = 0 and approaches the field cap p as t → ∞.

Cumulative supply emitted:

```
  1d |████                                        |   8.97%
  1w |███████████                                 |  24.53%
 1mo |█████████████████                           |  39.32%
 3mo |███████████████████████                     |  52.22%
 6mo |███████████████████████████                 |  61.04%
  1y |███████████████████████████████             |  69.88%
  2y |███████████████████████████████████         |  79.13%
  4y |███████████████████████████████████████     |  88.39%
 10y |███████████████████████████████████████████ |  97.01%
 20y |████████████████████████████████████████████|  99.54%
 50y |████████████████████████████████████████████| 100.00%
```

Why seven, summed: the short halvings fund the launch — ~9% of supply on day one, ~25% in the first week — so there is liquidity and reward to bootstrap the graph. The long halvings hold a multi-decade tail — the 4-year chunk is still emitting at 40 years — so security issuance never falls to zero abruptly. One curve carries both: Bitcoin's geometric discipline without a single cliff, front-loaded yet asymptotic to the field cap.

Emission sets the envelope — how fast the cap is approached. Allocation of each emitted unit — the PoW/PoS split, the security floor, reward against Δφ* — is the [[cybernomics]] adaptive layer.

see [[cybernomics]] for the economic model
