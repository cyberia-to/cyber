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

## Emission — a power law

Emission reads only the clock, never a network metric. Coupling supply to neurons, cyberlinks, or adoption would have no defensible "enough" threshold and would pay attackers to forge whatever was measured; the schedule stays exogenous and Sybil-immune by construction.

The curve is a power law — the same law the [[soft3/cybergraph|cybergraph]] itself obeys (scale-free degrees, Zipf [[soft3/tru/specs/focus|focus]]):

M(t) = p · (1 − (1 + t/τ)^(−k)),    τ = 0.5 year,   k = 0.5

(t in years). A power law is the one schedule that holds a hot head and a heavy tail at once — an exponential halving cannot, since its head and its tail share a single rate. Two phases fall out of one formula:

- bootstrap head — ~7% of supply in the first month, ~42% in the first year: sharp price discovery and real reward for the first miners (days, weeks, months), spread smoothly so no single day dumps. The initial rate is finite (k/τ = 100%/yr), not a spike.
- heavy tail — polynomial, not exponential: still emitting past a century, always under the cap, never greedy. The tail is the security budget that lasts as many years as possible.

Cumulative supply emitted:

```
 1mo |███                                           |    7.4%
 3mo |████████                                      |   18.4%
 6mo |█████████████                                 |   29.3%
  1y |███████████████████                           |   42.3%
  2y |█████████████████████████                     |   55.3%
  4y |███████████████████████████████               |   66.7%
  8y |███████████████████████████████████           |   75.7%
 16y |██████████████████████████████████████        |   82.6%
 32y |████████████████████████████████████████      |   87.6%
 64y |██████████████████████████████████████████    |   91.2%
128y |███████████████████████████████████████████   |   93.8%
```

Yearly inflation cools fast once the bootstrap passes: 31% (year 2), 12.5% (year 3), 4.8% (year 5), 1.5% (year 10), 0.5% (year 20). That undercuts a fixed 4-year halving (≈16% at year 5) and a flat-issuance design (Bittensor sits near 16% for years) while keeping a longer tail than either.

Emission fixes only the envelope — how much $CYB exists at time t. Allocation of each emitted unit — who earns it — is the stake-weighted Δφ* reward, where merit and Sybil-resistance belong. The layers stay separate: the schedule is dumb and credible; the distribution is earned.

see [[cybernomics]] for the economic model
