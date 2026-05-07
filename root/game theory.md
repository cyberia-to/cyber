---
tags: discipline, game, math, socio
crystal-type: entity
crystal-domain: game
---
the study of strategic interaction — what happens when the outcome of your choice depends on the choices of others

## from first principles

a [[game]] has three primitives:

- players — agents who choose. in [[cyber]], these are [[neurons]]
- strategies — the available actions. in [[cyber]], which [[cyberlinks]] to create, where to stake [[focus]]
- payoffs — the consequences. in [[cyber]], [[karma]], [[focus]] shifts, [[delegation rewards]]

the central question: given that every player reasons about what others will do, what happens? the answer is [[equilibrium]] — the state where no player gains by unilaterally changing strategy. Nash (1950) proved every finite game has at least one. in [[cyber]], [[equilibrium]] is the fixed point where [[focus]] distribution across the [[cybergraph]] ceases to shift

## the four archetypes

every strategic situation reduces to one of four archetypes:

| archetype | structure | key tension | in [[cyber]] |
|---|---|---|---|
| [[prisoner's dilemma]] | mutual [[cooperation]] pays more, but defection dominates | trust vs self-interest | [[free rider]] on [[public goods]] |
| stag hunt | [[cooperation]] is optimal if others cooperate, safe defection otherwise | [[coordination]] risk | multi-[[neuron]] [[cyberlink]] campaigns |
| chicken | mutual aggression destroys both, yielding pays if the other holds | commitment credibility | [[staking]] on disputed [[cyberlinks]] |
| matching pennies | pure conflict with no stable pure strategy | information hiding | adversarial ranking manipulation |

## branches

non-cooperative game theory — each agent optimizes alone. Nash [[equilibrium]], dominant strategies, mixed strategies. the workhorse for modeling [[consensus]], [[auction]], and adversarial behavior in open protocols

[[cooperative games]] — agents form coalitions and share joint gains. the [[Shapley value]] gives the unique fair attribution satisfying efficiency, symmetry, null player, and additivity. in [[cyber]], distributes [[focus]] rewards proportionally to each [[neuron]]'s causal impact via [[probabilistic shapley attribution]]. see also core stability and Nash bargaining

[[mechanism design]] — the inverse of game theory: given a desired outcome, design the rules so self-interested agents produce it. Myerson (1981) showed how to build incentive-compatible mechanisms. the [[cyberlink market protocol]], [[auction]] formats, [[token engineering]], and [[governance]] quadrants are all mechanism design

evolutionary game theory — strategies reproduce proportionally to their fitness. replicator dynamics, evolutionarily stable strategies. explains the emergence of [[cooperation]] without rationality: kin selection (Hamilton's rule $r \cdot B > C$), reciprocal altruism (Trivers), indirect reciprocity through reputation (Nowak). in [[cyber]], [[karma]] serves as reputation enabling indirect reciprocity at planetary scale

algorithmic game theory — computational complexity of finding [[equilibrium]]. some equilibria are PPAD-complete to compute. [[probabilistic shapley attribution]] addresses this by reducing $O(n!)$ Shapley computation to $O(k \cdot n)$ via Monte Carlo sampling

## information and signaling

games differ fundamentally in who knows what:

- complete information — all players know all payoffs (chess)
- incomplete information — private types, Bayesian reasoning (Harsanyi, 1967)
- imperfect information — simultaneous moves, hidden actions (poker)

information asymmetry creates two pathologies:

adverse selection — the informed party exploits ignorance before contracting. solved by screening and signaling (Spence, 1973)

moral hazard — hidden action after contracting. solved by monitoring, bonding, incentive alignment

in [[cyber]], the [[costly signal]] resolves both: a [[cyberlink]] costs [[focus]] to create, making it an honest indicator of what the [[neuron]] values. [[focus]] is the cost, [[cyberlink]] is the signal, [[cyberank]] is the outcome. cheap talk is impossible when the signal burns a scarce resource

## information aggregation

aggregating dispersed knowledge across agents:

[[wisdom of the crowds]] — [[Condorcet]] jury theorem (1785): independent voters with $p > 0.5$ converge to truth as group size grows. fails under correlated errors, herding, conformity bias

[[prediction markets]] — prices aggregate private information weighted by stake. [[LMSR]] for thin markets, [[inversely coupled bonding surface]] for self-scaling liquidity. the [[cyberlink market protocol]] makes every [[cyberlink]] simultaneously a structural assertion and a market on its own truth

[[Bayesian Truth Serum]] — extracts honest beliefs without ground truth. rewards beliefs more popular than predicted. a [[proper scoring rules|proper scoring rule]] applied peer-to-peer. in [[cyber]], implemented via the valence field in [[cyberlinks]]

[[proper scoring rules]] unify all three: log score, Brier score, and ICBS settlement factors are all instantiations of the same Bregman divergence structure. honesty is enforced because distortion always costs

## public goods and externalities

[[public goods]] — non-excludable, non-rival. the [[cybergraph]] is a public good: anyone can query or extend it. the [[free rider]] problem leads to underprovision. solutions: quadratic funding, [[staking]] incentives, [[token engineering]]

[[externality]] — costs or benefits to non-participants. every [[cyberlink]] generates positive [[externalities]] by enriching the shared [[knowledge graph]]. Pigouvian taxes internalize negatives; Coase theorem handles bilateral cases with clear property rights

## coordination and stigmergy

[[coordination]] asks how agents synchronize. Schelling focal points: convergence without communication through shared salience. [[coordination graphs]] model dependencies among agent actions for optimal joint decisions

[[stigmergy]] — indirect coordination through a shared environment. ants leave pheromones; [[neurons]] leave [[cyberlinks]]. the [[cybergraph]] is a stigmergic medium at planetary scale

## in cyber

the [[cyber]] protocol is a game-theoretic construction from the ground up:

| layer | game-theoretic mechanism |
|---|---|
| [[consensus]] | Byzantine agreement — [[proof of stake]], Tendermint BFT |
| ranking | [[cooperative games]] — [[Shapley value]] via [[cybernet]] |
| markets | [[proper scoring rules]] — [[inversely coupled bonding surface]], [[Bayesian Truth Serum]] |
| signaling | [[costly signal]] — [[focus]]-weighted [[cyberlinks]] |
| [[governance]] | [[mechanism design]] — conviction voting, quadratic mechanisms, futarchy |
| bandwidth | [[auction]] — [[staking]] weight determines resource priority |
| slashing | commitment devices — [[uptime slashing]], sensor-driven penalties |
| attribution | [[probabilistic shapley attribution]] — fair reward distribution |

the [[governance]] quadrant maps the design space:

| | no personal incentive | personal incentive |
|---|---|---|
| discrete | democracy | [[prediction markets]] |
| continuous | gauge voting | [[Shapley value]] |

## key figures

[[John von Neumann]] — founded the field (1928), minimax theorem, zero-sum games
[[John Nash]] — Nash [[equilibrium]] (1950), existence proof via fixed-point theorem
[[Lloyd Shapley]] — [[Shapley value]] (1953), stable matching (Nobel 2012)
[[William Vickrey]] — Vickrey [[auction]], revelation principle (Nobel 1996)
[[Condorcet]] — jury theorem (1785), foundation of [[wisdom of the crowds]]
[[Thomas Schelling]] — focal points, commitment strategies (Nobel 2005)
Leonid Hurwicz — [[mechanism design]] (Nobel 2007)

see [[cooperative games]] for coalition theory. see [[coordination]] for synchronization. see [[cooperation]] for evolutionary foundations. see [[cybernomics]] for token economy design. see [[token engineering]] for applied mechanism blueprints