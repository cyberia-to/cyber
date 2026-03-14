---
alias: tx, transaction, txs, transactions, signals, signaling
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
crystal-size: enzyme
stake: 23154625001185704
---
a bundle of [[cyberlinks]] a [[neuron]] commits in a single [[step]] — the atomic broadcast unit in [[cyber]]. each link in the signal consumes [[focus]], making every statement a [[costly signal]]

## structure

$$s \;=\; (\nu,\; \vec\ell,\; \pi_\Delta,\; \sigma,\; t)$$

| field | name | type | semantics |
|-------|------|------|-----------|
| $\nu$ | [[subject]] | $N$ | signing [[neuron]] |
| $\vec\ell$ | links | $L^+$ | one or more [[cyberlinks]] — each a 7-tuple $(\nu, p, q, \tau, a, v, t)$ |
| $\pi_\Delta$ | focus delta | $(P \times \mathbb{F}_p)^*$ | sparse [[focus]] update: how the batch of links shifts $\pi^*$ |
| $\sigma$ | proof | $\Pi$ | [[STARK]] proof that $\pi_\Delta$ is correct against the current [[BBG]] root |
| $t$ | at | $\mathbb{Z}_{\geq 0}$ | block height |

the signal separates what a [[neuron]] asserts (the [[cyberlinks]]) from what the assertion computes (the focus shift). a single [[STARK]] proof covers the entire batch — proving $n$ links together costs less than $n$ separate proofs because shared neighborhood state is proved once

## focus delta

$\pi_\Delta$ is a sparse vector of (particle_id, $\Delta\pi$) pairs — the neuron's locally computed shift to the [[focus]] distribution $\pi^*$ caused by adding all links in $\vec\ell$ to the [[cybergraph]]. the [[locality theorem]] bounds the support to $O(\log(1/\varepsilon))$ hops from affected [[particles]] — most entries are zero, so the sparse representation is compact

the neuron computes $\pi_\Delta$ by running the [[tri-kernel]] locally on their neighborhood, adding their links, and measuring how $\pi$ shifts. the result is whatever the math says — there is no target or threshold

## verification

$\sigma$ proves $\pi_\Delta$ was correctly computed against a specific $\text{bbg\_root}$ from the current header. any verifier checks $\sigma$ in $O(\log n)$ without recomputing the [[tri-kernel]]

## two effects

validation of a signal produces two outcomes:

1. each link in $\vec\ell$ enters $L$ — conviction UTXOs are created for each [[cyberlink]]
2. if $\|\pi_\Delta\| > 0$ and $\sigma$ is valid, the [[neuron]] self-mints [[$CYB]] proportional to the proven shift — a reward UTXO is created for $\nu$

the conviction UTXOs (tokens spent into links) and the reward UTXO (tokens minted for contribution) are separate token movements within one atomic signal. see [[cyber/rewards]] for the full reward specification

## conservation

total minting per epoch is bounded by the actual global $\Delta\pi$, verifiable from consecutive headers. if the sum of individual claims exceeds the actual shift (overlapping neighborhoods), all claims are scaled proportionally. see §6.9 and §14.2 of the [[cyber/whitepaper]]

see [[signal types]], [[cyber/link]], [[cyber/network]]

discover all [[concepts]]
