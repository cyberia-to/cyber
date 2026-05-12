---
tags: cyber, cip
crystal-type: pattern
crystal-domain: cybics
alias: active inference framework
status: draft
---

a framework where perception, action, and [[learning]] are aspects of one optimization: minimizing variational [[free energy]]

originated by [[Karl Friston]] as an extension of the [[free energy principle]]. an agent does not have separate modules for sensing, deciding, and acting — it has one loop that reduces surprise by updating beliefs and selecting actions

## the loop

each [[neuron]] in the [[cybergraph]] runs:

1. observe — local traffic, link arrivals, [[token]] flows
2. predict — generate expected observations from internal model
3. compute prediction error — divergence between expected and actual
4. update beliefs — gradient descent on [[free energy]]: $\theta \leftarrow \theta - \eta \nabla_\theta F$
5. tune [[precision]] — learn confidence weights $\lambda$ for each error channel
6. select action — choose policy $\phi^*$ that minimizes expected [[free energy]]: $G(\phi^*) = \text{risk} + \text{ambiguity}$
7. execute — edit edges, stake, sample [[particles]]

## key mappings to cyber

| active inference | [[cybergraph]] |
|---|---|
| hidden states | latent attributes of [[particles]] and edges |
| observations | measured traffic, link arrivals, weight changes |
| generative model | [[neuron]]'s local model of link dynamics and [[token]] flows |
| prediction error | divergence between expected [[focus]] and realized traffic |
| [[precision]] | adaptive [[token]] staking that amplifies trusted signals |
| [[free energy]] | upper bound on global uncertainty; minimized at [[focus]] [[convergence]] |
| [[Markov blanket]] | boundary between a [[neuron]]'s internal state and the rest of the [[cybergraph]] |

## expected free energy

planning uses expected [[free energy]] $G(\phi^*)$, which decomposes into:

- risk: divergence from preferred observations (the agent wants high-quality links, low spam)
- ambiguity: expected uncertainty about hidden states under the chosen policy

minimizing risk drives exploitation. minimizing ambiguity drives exploration (curiosity). the balance is automatic — no exploration-exploitation tradeoff to tune

## precision as economic signal

[[precision]] (inverse variance of prediction errors) maps naturally to [[token]] staking:

- high [[precision]] on a signal = high stake backing it = strong confidence
- low [[precision]] = low stake = the [[neuron]] is uncertain about this region
- [[precision]] gaming mitigated by slashing on bad forecasts — skin in the game

this makes [[attention]] allocation an economic act: staking [[tokens]] on beliefs about the [[cybergraph]]

## hierarchical [[Markov blankets]]

the [[cybergraph]] naturally decomposes into modules (dense internal edges, sparse external). each module forms a [[Markov blanket]] — internal dynamics can be updated at high frequency, inter-module messages at lower frequency

this gives scalability: local inference within modules, coarse-grained message passing between them

## open questions

- what [[precision]]-staking regime best aligns epistemic efficiency with [[token]] economics under real traffic?
- where are phase transitions in [[emergence]] when adding hierarchical [[Markov blankets]] to the [[collective focus theorem]]?
- how to calibrate preference distributions without central authority while avoiding sybil manipulation?

see [[free energy principle]] for the foundational theory. see [[Karl Friston]] for the person. see [[cybics]] for the integration with the [[tri-kernel]]. see [[contextual free energy model]] for the context-dependent extension