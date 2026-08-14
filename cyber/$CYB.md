---
tags: cyber, cybernomics
alias: $CYB, CYB, cyber energy
crystal-type: entity
crystal-domain: economics
icon: "⚡"
---
the root [[token]] of [[cyber]] — the energy of [[focus]]. stake, fees, and rewards for teaching the [[cybergraph]]. emission follows proven Δφ*; supply is a law of the field. full model in the [[whitepaper]] and below

## focus is the value

cyber organizes one quantity: [[soft3/tru/specs/focus|focus]] (φ*), the collective attention distribution — the fixed point the [[tri-kernel]] drives the graph toward. A [[cyberlink]] that earns focus is knowledge the network found worth attending to. Focus is the scarce thing, the measured thing, the thing every other mechanism serves.

## $CYB is the energy of focus

Moving focus costs work; creating focus is work done. $CYB is that work made fungible — the energy a [[cybics/crystal/neuron|neuron]] spends to write a [[cyberlink]], compute, and reach [[cybics/crystal/consensus|consensus]], and the energy it earns for raising the graph's focus. Δφ* is the gradient of the system's free energy, so $CYB is that free energy in transferable form.

## supply is a law of the field

Value and computation share one arithmetic: balances are elements of the [[nebu|Goldilocks field]], the field [[soft3/nox|nox]] computes in. So total supply is the field's own order:

p = 2⁶⁴ − 2³² + 1 = 18,446,744,069,414,584,321

The cap is how many elements the field has — arithmetic, not a governance number. (On the [[bootloader/bostrom|bostrom]] [[bootloader]] today this energy circulates as [[$C]].)

## genesis

At the first block, [[$C]] holders hold 187,416,084,623,451,570 $CYB, ≈ 1% of supply: their snapshot of 281,405,532,467,645, lifted 666×.

## emission answers to time alone

Supply at age t is M(t): a function of the clock and nothing else — identical on every node, known in full from genesis. With time as the only input, the schedule is a fixed commitment, predictable in advance and immune to forgery.

Focus enters on the other side. The clock sets how much $CYB exists; focus sets who earns it (see allocation). Supply is a law of time, reward a law of φ* — kept apart.

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

Yearly inflation — new supply ÷ circulating supply at year start. Year one is the bootstrap: the 1% genesis fills to half the supply, a one-time ~50× expansion. From year two it cools fast:

```
  2y |████████████████████████████████████████████|  23.8%
  3y |██████████████████                          |   9.7%
  4y |██████████                                  |   5.6%
  5y |███████                                     |   3.7%
  6y |█████                                       |   2.7%
  8y |███                                         |   1.6%
 10y |██                                          |   1.1%
 20y |█                                           |   0.4%
 50y |                                            |   0.1%
```

Inflation drops below a flat-issuance design (Bittensor sits near 16% for years) by year three, while the heavy tail keeps issuing far longer than any halving.

## allocation is focus

Emission says how much; focus says who. Each freshly emitted unit is split by stake-weighted Δφ* — paid for the focus a contribution created, weighted by stake so forging identities buys nothing. This is where focus, kept out of the schedule, does its work: not in printing the money, but in directing it.

see [[cybernomics]] for the economic model
