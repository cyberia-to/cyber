---
tags: cyber, cybernomics
alias: cyber energy
crystal-type: entity
crystal-domain: economics
---
the root [[token]] of [[cyber]] — the energy of [[soft3/tru/specs/focus|focus]].

## focus is the value

cyber organizes one quantity: [[soft3/tru/specs/focus|focus]] (φ*), the collective attention distribution — the fixed point the [[tri-kernel]] drives the graph toward. A [[cyberlink]] that earns focus is knowledge the network found worth attending to. Focus is the scarce thing, the measured thing, the thing every other mechanism serves.

## $CYB is the energy of focus

Moving focus costs work; creating focus is work done. $CYB is that work made fungible — the energy a [[cybics/crystal/neuron|neuron]] spends to write a [[cyberlink]], compute, and reach [[cybics/crystal/consensus|consensus]], and the energy it earns for raising the graph's focus. Δφ* is the gradient of the system's free energy, so $CYB is that free energy in transferable form.

## supply is a law of the field

Value and computation share one arithmetic: balances are elements of the [[nebu|Goldilocks field]], the field [[soft3/nox|nox]] computes in. So total supply is the field's own order:

p = 2⁶⁴ − 2³² + 1 = 18,446,744,069,414,584,321

The cap is how many elements the field has — arithmetic, not a governance number. (On the [[bostrom]] [[bootloader]] today this energy circulates as [[$C]].)

## genesis

At the first block, $C holders — the existing cyber energy token on [[bostrom]] — hold 187,416,084,623,451,570 $CYB, ≈ 1% of supply: their snapshot of 281,405,532,467,645, lifted 666×. [[$BOOT]] and [[$PUSSY]] are separate tokens and take no part in the $CYB genesis. Every other unit is earned through emission.

## emission is blind, by design

Focus is the value, so the tempting schedule would mint $CYB against focus, or neurons, or adoption. That is the schedule to refuse. There is no honest threshold for how much focus "deserves" issuance, and any quantity the money is minted against becomes a quantity attackers are paid to forge — emission would fund its own sybil attack. So emission reads one input: the clock. Blind to the graph, it cannot be gamed.

Focus is not discarded — it is moved to where it is safe. The schedule fixes how much $CYB exists; allocation (below) decides who earns it, by focus. The two never mix.

## emission follows the network's own law

cyber is scale-free: degrees follow a power law, focus follows Zipf. The token is issued by the same law its graph obeys — a power law:

M(t) = p · (1 − (1 + t/τ)^(−k)),    τ = 0.33 year,   k = 0.5

(t in years). A power law is also the one schedule that holds a hot head and a heavy tail at once — an exponential halving shares a single rate between the two and cannot. From one formula, two phases:

- a bootstrap head — about half the supply in the first year (~11% in the first month), spread across the year so price discovers and the first miners (days, weeks, months) are paid, with no single-day flood. The initial rate is finite (k/τ ≈ 152%/yr), not a spike.
- a heavy tail — polynomial, never exponential: still issuing past a century (~4% of supply unissued at 200 years), always under the cap.

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

After the year-one bootstrap, yearly inflation cools fast: 24% (year 2), 9.7% (year 3), 3.7% (year 5), ~1% past year 10 — dropping below a flat-issuance design (Bittensor sits near 16% for years) by year three, while the heavy tail keeps issuing far longer than any halving.

## allocation is focus

Emission says how much; focus says who. Each freshly emitted unit is split by stake-weighted Δφ* — paid for the focus a contribution created, weighted by stake so forging identities buys nothing. This is where focus, kept out of the schedule, does its work: not in printing the money, but in directing it.

see [[cybernomics]] for the economic model
