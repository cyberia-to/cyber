---
alias: cyberlinks weight, cyberlinks weights, standard inference
tags: cyber
crystal-type: entity
crystal-domain: cyber
stake: 7746278983898673
---
the naive first solution to the [[true-false problem]] — a single-factor contextual weighting that preceded the full [[cyber/truth]] architecture

## the algorithm

for a given query [[particle]]:

- get all [[particles]] cyberlinked with the query, sorted by [[cyberank]]
	- for each [[particle]] get all [[cyberlinks]]
		- for each [[cyberlink]] get [[neuron]]
			- for each [[neuron]]
				- get [[will]] balance
				- get number of [[cyberlinks]]
				- compute average [[will]] per cyberlink
		- sum (average will) votes for every [[particle]]

multiply [[cyberank]] of each particle by sum (average will)

in essence: weight [[will]] of [[neurons]] on global [[attention]] in context

## properties

- requires zero additional information beyond what the [[cybergraph]] already has
- follows Occam's razor — as simple as possible
- single-factor: only [[will]] concentration, no epistemic signal

## what it lacks

standard inference addressed the [[true-false problem]] but left three gaps:

1. no local reconvergence — still uses global [[cyberank]] as base, just reweighted. the full [[tri-kernel]] reconverges locally given context [[particles]], producing [[relevance]] instead of adjusted global rank

2. no honesty mechanism — [[neurons]] can vote strategically. [[Bayesian Truth Serum]] with [[valence]] creates an [[equilibrium]] where honest reporting dominates

3. no market correction — incorrect answers persist until [[neurons]] manually reweight. [[inversely coupled bonding surface|ICBS]] markets suppress false edges economically and continuously

## historical implementations

- [[cy]]
- [[cyb/oracle/ask]] (planned)

## lineage

[[true-false problem]] → standard inference (this page) → [[cyber/truth]] (full solution: [[tri-kernel]] + [[Bayesian Truth Serum|BTS]] + [[inversely coupled bonding surface|ICBS]])
