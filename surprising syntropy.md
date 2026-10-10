---
tags: cyber, core, article
alias: surprising syntropy, surprise-weighted syntropy, what the mint pays for
crystal-type: measure
crystal-domain: cyber
---
# surprising syntropy

the one thing new money pays for in [[cyber]]: order the network gained that the crowd did not already expect.

two words, two filters. [[syntropy]] asks *did the shared picture get clearer?* surprise asks *did anyone need you to make it clearer?* the [[cyber/$CYB|$CYB]] mint pays only where both answers are yes.

## syntropy — the picture got clearer

[[focus]] $\phi^*$ is where the whole [[cybergraph]] puts its attention. syntropy is how sharp that attention is, in bits: $J = D_{KL}(\phi^* \,\|\, u)$ — zero when attention is spread evenly over everything (noise), large when it settles on a connected core that matters.

a [[cyberlink]] reshapes the graph, the [[tri-kernel]] settles again, and $J$ moves. the part of that move a link caused, counted only when it is downhill, is its [[impulse]] $\Delta\phi^+$. a link that adds noise moves $J$ the wrong way and earns nothing.

## surprise — nobody saw it coming

syntropy alone would pay a copy: a perfect copy of a good link produces the same focus shift as the original. so every contribution also carries a [[Bayesian Truth Serum|serum]] report — the neuron's belief and its prediction of what everyone else believes. the serum scores how far the report beat the crowd's prediction of itself, and squashes it to a weight $\rho \in [0,1]$.

- a copy, a herd vote, an obvious link the crowd already predicted: $\rho \to 0$.
- a true link the crowd under-predicted: $\rho \to 1$.

the honest report is the equilibrium — lying about your belief lowers your own expected score (Prelec 2004; with no judge to resolve the market, the surprisingly-popular rule selects the truth — [[strong truthfulness]]).

## the product of the two

the mint weighs every contributed link by its surprise before measuring the shift, then splits the result fairly among the contributors by their [[Shapley value|Shapley]] shares:

$$R(\nu) = \text{Shapley}_\nu(v^\star), \qquad v^\star(S) = \Delta\phi^+\!\big(A^{\text{eff}} \cup \rho\cdot S\big)$$

in words: you are paid your fair share of the clarity the network gained from what you said and others did not expect. that quantity is surprising syntropy.

three consequences fall out:

- **copying earns nothing.** a copy joins the mint with zero weight; its capital still ranks, so stake keeps its influence, it just mints no money.
- **idle capital earns nothing.** a passive stake has no report to surprise anyone with. [[staking]] earns from the mint only as an active bet.
- **noise earns nothing.** a surprising link that makes the picture blurrier has $\Delta\phi^+ = 0$.

## what is not settled

- the discovery leak — a link that is right too early looks wrong at settlement: low market price, low surprise because the crowd predicted failure. it can earn later through the annuity, not through the pulse.
- coordinated rings can still manufacture surprise; the serum is proven against a lone liar, not a cartel.

both are tracked in the [[reward specification]] (§5, §12, §15).

see [[syntropy]] · [[impulse]] · [[Bayesian Truth Serum]] · [[Shapley value]] · [[karma]] · [[staking]] · [[mining]] · [[rewards]] · [[cyber/$CYB|$CYB]]

discover all [[concepts]]
