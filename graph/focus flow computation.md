---
alias: focus flow, FFC, focus flow whitepaper, focusflow blueprint
tags: cyber, core
crystal-type: process
crystal-domain: cyber
crystal-size: bridge
stake: 53778483873721616
---
[[transformers]] compute [[attention]] as a one-shot matrix multiply: $\text{softmax}(QK^T/\sqrt{d})V$. focus flow computes [[attention]] as an iterative physical process: [[cyberlinks]] define a graph, the [[tri-kernel]] evolves probability mass toward [[equilibrium]], and the fixed point $p^*$ is the network's collective [[focus]]

## how it replaces transformers

| phase | transformer | focus flow |
|---|---|---|
| training | backprop on fixed corpus | add [[cyberlinks]] — graph IS the model |
| inference | forward pass, softmax, sample | converge $p^*$ from context, sample |
| adaptation | retrain. catastrophic forgetting | add links. $p^*$ shifts, nothing lost |

in transformers, training and inference are separate algorithms. in focus flow, they are the same operation: a [[neuron]] adds a [[cyberlink]], the [[tri-kernel]] reconverges, and the new $p^*$ simultaneously encodes the learned relation and is available for inference

## the local update rule

every node reads only its neighbours and runs:

$$\Delta p_i = \eta\Big(\sum_{j \in \mathcal{N}(i)} w_{ij}(p_j - p_i) - \partial_{p_i}(\lambda E_{\text{diff},i} + \gamma C_i) + T(1 + \log p_i)\Big)$$

gossip normalisation enforces $\sum_i p_i = 1$. no global softmax, fully local, edge-only. the system converges to [[Boltzmann distribution]] [[equilibrium]]:

$$p_i^* \propto \exp\big(-\beta[E_{\text{spring},i} + \lambda E_{\text{diff},i} + \gamma C_i]\big)$$

this is the equation everything else serves. the [[collective focus theorem]] proves it converges. the [[tri-kernel architecture]] explains why these three energy terms. [[foculus]] turns $p^*$ into finality

## inference

1. encode context [[particles]] as active nodes with elevated $C_i$
2. run local updates — focus mass flows from context through the [[cybergraph]]
3. $p^*$ converges — high-probability [[particles]] are the network's response
4. sample next [[particle]] from $p^*$, add to context, repeat

complexity per step: $O(|E| + |V|)$. context window is unbounded — it is the entire graph. relevance is not positional but topological: distant but well-connected [[particles]] contribute naturally

## properties

| property | transformer | focus flow |
|---|---|---|
| complexity | $O(n^2)$ memory and compute | $O(n)$ — sparse, local |
| stable state | no — recomputed each forward pass | yes — converges to $p^*$ |
| multi-agent | single model | native — every [[neuron]] contributes [[cyberlinks]] |
| consensus | external | built-in via [[foculus]] |
| explainability | low | high — trace any $p_i$ to its contributing links |
| context window | fixed (4k-128k tokens) | unbounded — the entire [[cybergraph]] |

## the stack

  - [[cybergraph]] — the substrate: [[particles]] as nodes, [[cyberlinks]] as typed edges (h/d/c)
  - [[tri-kernel]] — the physics: [[diffusion]] + [[springs]] + [[heat kernel]] evolve $p$ toward $p^*$
  - [[nox]] — the execution: 16 deterministic reduction patterns over Goldilocks field
  - [[foculus]] — the consensus: $\pi > \tau$ finalizes [[particles]] without leaders or blocks
  - [[cyber/tokenomics]] — the incentives: rewards $\propto \Delta\pi$, fees burned

each layer is specified independently. together they form a self-organizing system where [[computation]], [[inference]], and [[consensus]] are the same process

see [[collective focus theorem]] for convergence proofs. see [[tri-kernel architecture]] for why these three operators. see [[cybergraph llm architecture]] for the generative model.

## extensions
- [[gflownet focus flow]]
- [[topos ffc integration]]
