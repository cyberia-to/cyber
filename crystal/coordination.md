---
tags: cyber
crystal-type: process
crystal-domain: biology
---
aligning agents toward shared goals when actions are interdependent

[[cooperation]] asks why agents help each other. coordination asks how they synchronize

## the theory

- focal points (Schelling, 1960) — agents converge on shared expectations without communication. culture, convention, and salience guide choice
- [[coordination graphs]] — model dependencies among agent actions in a network, allowing optimal joint decisions (max-plus, variable elimination)
- mechanism design — design rules of the game so that self-interested agents produce socially optimal outcomes (Myerson, 1981)
- common knowledge — coordination requires agents to know that others know the same thing (Lewis, 1969). shared state enables synchronized action

## coordination failures

- tragedy of the commons — shared resources deplete when agents optimize individually (Hardin, 1968)
- [[prisoner's dilemma]] — mutual cooperation is optimal but individual defection is dominant
- stag hunt — cooperation yields the best outcome but requires trust that others will cooperate too
- information asymmetry — agents with private information make suboptimal collective decisions

## in nature

- flocking (Reynolds, 1987) — three local rules (separation, alignment, cohesion) produce global coordination without leaders
- quorum sensing in bacteria — cells coordinate behavior through chemical signal thresholds
- [[stigmergy]] in ant colonies — pheromone trails coordinate foraging without direct communication

## in cyber

protocol mechanisms solve coordination at scale:

- [[consensus]]: coordination on a single history of events
- [[automated market maker]]: coordination on [[value]] measurements through [[liquidity]]
- [[auction]]: coordination on [[value]] establishment through competitive bidding
- [[cybernet]]: [[cooperative games]] optimizing for positive-sum outcomes ([[yuma]])
- [[prediction markets]]: coordination on future states through skin-in-the-game forecasting
- [[governance]]: coordination on rule changes through collective decision-making

the [[cybergraph]] itself is a coordination tool: each [[cyberlink]] is a public signal that guides others. [[stigmergy]] at planetary scale

see [[collective]] for the four collective processes. see [[egregore]] for the broader framework