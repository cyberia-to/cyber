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

At genesis, TOCYB holders hold 187,416,084,623,451,570 $CYB — ≈ 1% of total supply (the migration snapshot of 281,405,532,467,645, lifted 666×). Every other unit enters supply through emission.

## Emission — a power law

Emission reads only the clock, never a network metric. Coupling supply to neurons, cyberlinks, or adoption would have no defensible "enough" threshold and would pay attackers to forge whatever was measured; the schedule stays exogenous and Sybil-immune by construction.

The curve is a power law — the same law the [[soft3/cybergraph|cybergraph]] itself obeys (scale-free degrees, Zipf [[soft3/tru/specs/focus|focus]]):

M(t) = p · (1 − (1 + t/τ)^(−k)),    τ = 0.33 year,   k = 0.5

(t in years). A power law is the one schedule that holds a hot head and a heavy tail at once — an exponential halving cannot, since its head and its tail share a single rate. Two phases fall out of one formula:

- bootstrap head — ~11% of supply in the first month, ~50% in the first year (half the supply): sharp price discovery and real reward for the first miners (days, weeks, months), spread across the first year so no single day dumps. The initial rate is finite (k/τ ≈ 152%/yr), not a spike.
- heavy tail — polynomial, not exponential: still emitting past a century (≈4% of supply unissued at 200 years), always under the cap, never greedy.

Cumulative supply emitted:

```
 1mo |█████                                         |   10.6%
 3mo |███████████                                   |   24.6%
 6mo |█████████████████                             |   36.9%
  1y |███████████████████████                       |   50.2%
  2y |█████████████████████████████                 |   62.4%
  4y |█████████████████████████████████             |   72.4%
  8y |█████████████████████████████████████         |   80.1%
 16y |███████████████████████████████████████       |   85.8%
 32y |█████████████████████████████████████████     |   89.9%
 64y |███████████████████████████████████████████   |   92.8%
128y |████████████████████████████████████████████  |   94.9%
```

Share of supply minted in each year — the bootstrap lands in year one, then the tail:

```
  1y |██████████████████████████████████████████████|  50.2%
  2y |███████████                                   |  12.2%
  3y |██████                                        |   6.2%
  4y |████                                          |   3.9%
  5y |██                                            |   2.7%
  6y |██                                            |   2.0%
  8y |█                                             |   1.3%
 10y |█                                             |   0.9%
 20y |                                              |   0.3%
 50y |                                              |   0.1%
```

Year one is the bootstrap — ~50% of supply, the once-only launch flood (the 1% genesis base aside). After it, yearly inflation cools fast: 24% (year 2), 9.7% (year 3), 3.7% (year 5), ~1% past year 10 — dropping below a flat-issuance design (Bittensor sits near 16% for years) by year three, while the heavy tail keeps issuing far longer than any halving.

Emission fixes only the envelope — how much $CYB exists at time t. Allocation of each emitted unit — who earns it — is the stake-weighted Δφ* reward, where merit and Sybil-resistance belong. The layers stay separate: the schedule is dumb and credible; the distribution is earned.

see [[cybernomics]] for the economic model
