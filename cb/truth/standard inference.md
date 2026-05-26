---
alias: cyberlinks weight, cyberlinks weights, standard inference
tags: cyber
crystal-type: entity
crystal-domain: cyber
---
the naive first solution to the [[true-false problem]] — a single-factor contextual weighting that preceded the full [[cyber/truth]] architecture

## the algorithm

given a query [[particle]] Q, compute a contextual score for each candidate answer:

```
candidates = particles cyberlinked with Q
for each candidate P in candidates:
    links = cyberlinks between Q and P
    weight = 0
    for each link in links:
        neuron = link.neuron
        avg_will = neuron.will_balance / neuron.total_cyberlinks
        weight += avg_will
    score(P) = cyberank(P) × weight
return candidates sorted by score
```

the intuition: a [[neuron]] who concentrates [[will]] across few [[cyberlinks]] signals stronger conviction per link. a [[neuron]] who spreads [[will]] across thousands of links contributes less per link. the score multiplies global [[cyberank]] (what the graph thinks matters) by concentrated [[will]] in context (what committed [[neurons]] think matters here)

## why it works against the [[true-false problem]]

if `true` has [[cyberank]] 10 and `false` has [[cyberank]] 9, global rank always picks `true`. but if the [[neurons]] who linked a specific question to `false` have higher concentrated [[will]] than those who linked it to `true`, the contextual score can flip the answer. the concentration signal breaks the global rank tie

## what it lacks

standard inference addressed the [[true-false problem]] but left three gaps:

1. no local reconvergence — still uses global [[cyberank]] as base, just reweighted. the full [[tri-kernel]] reconverges locally given context [[particles]], producing [[relevance]] instead of adjusted global rank

2. no honesty mechanism — [[neurons]] can vote strategically. [[serum]] with [[valence]] creates an [[equilibrium]] where honest reporting dominates

3. no market correction — incorrect answers persist until [[neurons]] manually reweight. [[coupling|ICBS]] markets suppress false edges economically and continuously

## lineage

[[true-false problem]] → standard inference → [[cyber/truth]] ([[tri-kernel]] + [[serum|BTS]] + [[coupling|ICBS]])