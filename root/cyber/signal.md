---
alias: cyber signal, cyber signals
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
crystal-size: enzyme
stake: 23154625001185704
focus: 0.001661539940663904
gravity: 7
---
a bundle of [[cyberlinks]] a [[neuron]] commits in a single [[step]] — the atomic broadcast unit in [[cyber]]. each link in the signal consumes [[focus]], making every statement a [[costly signal]]

## structure

$$s \;=\; (\nu,\; \vec\ell,\; \pi_\Delta,\; \sigma,\; t)$$

| field | name | type | semantics |
|-------|------|------|-----------|
| $\nu$ | [[subject]] | $N$ | signing [[neuron]] |
| $\vec\ell$ | links | $L^+$ | one or more [[cyberlinks]] — each a 7-tuple $(\nu, p, q, \tau, a, v, t)$ |
| $\pi_\Delta$ | [[cyber/impulse]] | $(P \times \mathbb{F}_p)^*$ | sparse [[focus]] update: how the batch of links shifts $\pi^*$ |
| $\sigma$ | proof | $\Pi$ | recursive [[stark]] proof covering the [[cyber/impulse]], all conviction UTXO movements, and [[cyberlink]] validity against the current [[BBG]] root |
| $t$ | at | $\mathbb{Z}_{\geq 0}$ | block height |

the signal separates what a [[neuron]] asserts (the [[cyberlinks]]) from what the assertion computes (the [[cyber/impulse]]). see [[cyber/impulse]] for how $\pi_\Delta$ is computed and why the name

## proof

$\sigma$ is a single recursive [[stark]] proof that covers the entire signal atomically:

- correctness of each [[cyberlink]] in $\vec\ell$ (valid signatures, valid particle references)
- validity of all conviction UTXO movements (each link's $(\tau, a)$ spend is backed by an unspent output)
- correctness of the [[cyber/impulse]] $\pi_\Delta$ (the [[tri-kernel]] computation against $\text{bbg\_root}$ from the current header)

one proof for everything. proving $n$ links together costs less than $n$ separate proofs because shared neighborhood state and UTXO set are proved once. any verifier checks $\sigma$ in $O(\log n)$ without recomputing anything

## two effects

validation of a signal produces two outcomes:

1. each link in $\vec\ell$ enters $L$ — conviction UTXOs are created for each [[cyberlink]]
2. if $\|\pi_\Delta\| > 0$ and $\sigma$ is valid, the [[neuron]] self-mints [[$CYB]] proportional to the proven shift — a reward UTXO is created for $\nu$

the conviction UTXOs (tokens spent into links) and the reward UTXO (tokens minted for contribution) are separate token movements within one atomic signal. see [[cyber/rewards]] for the full reward specification

## conservation

total minting per epoch is bounded by the actual global $\Delta\pi$, verifiable from consecutive headers. if the sum of individual claims exceeds the actual shift (overlapping neighborhoods), all claims are scaled proportionally. see §6.9 and §14.2 of the [[cyber/whitepaper]]

see [[signal types]], [[cyber/link]], [[cyber/impulse]], [[cyber/network]]

discover all [[concepts]]