---
tags: cyber, article, mathematics
alias: loss functions and physics, physics of convergence, gradient descent and the cybergraph
crystal-type: pattern
crystal-domain: cyber
---
# Gradient Descent and the Cybergraph

the [[cybergraph]] computes its objective without a designer specifying one. to understand why this is a radical claim — and where it requires precision — it helps to start with what gradient descent actually does.

## the exogenous objective

standard machine learning works like this. a designer writes a loss function $L(\theta; \mathcal{D})$ that encodes their beliefs about what "correct" means for a task. an optimizer runs:

$$\theta \leftarrow \theta - \eta \nabla_\theta L$$

this finds the minimum of $L$ over a parametric family $\mathcal{P}_\theta$. the result is a model that is optimal with respect to the designer's chosen loss on the designer's chosen data distribution.

the descent is automatic. the objective is not. the real intellectual work lives in $L$: a cross-entropy loss encodes a different worldview than an MSE loss, a reward signal, or a constitutional principle. the optimization machinery is a detail; the loss landscape is the argument.

## the endogenous objective

the [[cybergraph]] does not start with a loss. it starts with a physics: the [[tri-kernel]] composite

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

where $D$ is [[diffusion]] (exploration), $S$ is the screened [[springs]] operator (structural consistency), and $H_\tau$ is the [[heat]] kernel (multi-scale adaptation). this iteration has a unique fixed point $\pi^*$ by the Banach fixed-point theorem — the [[cybergraph]] converges to it from any starting distribution.

$\pi^*$ is the [[focus]] distribution: the probability that attention lands on particle $p$ given the full structure of the graph. it is what the network collectively knows, encoded as a measure over all [[particles]].

that fixed point minimizes a free energy functional:

$$\mathcal{F}(\phi) = \lambda_s\!\left[\tfrac{1}{2}\phi^\top L\phi + \tfrac{\mu}{2}\|\phi - x_0\|^2\right] + \lambda_h\!\left[\tfrac{1}{2}\|\phi - H_\tau\phi\|^2\right] + \lambda_d \cdot D_{KL}(\phi \| D\phi)$$

no one wrote this $\mathcal{F}$ down as the target. it emerges from the operators. the graph's objective is the graph's own information geometry — the shape of the constraint set defined by who linked what, weighted by how much [[focus]] they commanded.

at equilibrium, the distribution takes the Boltzmann-Gibbs form:

$$\phi^*_i \propto \exp\!\Big(-\beta\big[E_{\text{spring},i} + \lambda E_{\text{diff},i} + \gamma C_i\big]\Big)$$

the canonical ensemble from statistical mechanics — applied to [[knowledge]]. the weights $\lambda_s, \lambda_h, \lambda_d$ emerge as Lagrange multipliers, the same way thermodynamics derives the Boltzmann distribution from entropy maximization subject to energy conservation. no parameters. only physics.

## where the claim needs precision

"no designed loss function" is approximately right in the deep sense that matters. but the operator choices ARE design choices:

- $\lambda_d, \lambda_s, \lambda_h$ determine how much weight goes to exploration, structure, and adaptation
- $\mu$ sets the stiffness of the screened Laplacian
- $\tau$ sets the scale at which the [[heat]] kernel smooths
- the choice of hash function $H$ determines the particle identity space

what is NOT designed: the destination. the shape of $\mathcal{F}$ over the space of probability distributions on $P$ — the [[particles]] — is derived from the graph structure itself. as the graph grows, the landscape changes. the objective co-evolves with the system. as neurons add [[cyberlinks]], they shift the Laplacian $L$, which reshapes $\mathcal{F}$, which moves $\pi^*$.

in ML terms: the graph is simultaneously the data, the model, and the loss landscape. there is no train/inference separation. every new fact shifts the objective.

## transformers as a smaller picture

the precise version of this observation lives in a mathematical identity.

transformer attention is:

$$\text{Attn}(Q, K, V) = \text{softmax}\!\left(\tfrac{QK^\top}{\sqrt{d}}\right)V$$

the softmax is a Boltzmann distribution at temperature $\sqrt{d}$. probability mass flows from query positions toward key positions proportionally to compatibility. this is one application of the [[diffusion]] operator $D$ from the [[tri-kernel]] — local probability redistribution over one agent's frozen context window.

Deep Equilibrium Models (Bai et al., 2019) showed that iterating a transformer layer to convergence reaches the same fixed point regardless of initialization. that fixed point is the stationary distribution of the Markov chain induced by the learned $W_Q, W_K$ projections over context tokens. that fixed point is the [[focus]] distribution restricted to one agent's context.

the [[tri-kernel]] computes the same fixed point over the entire [[cybergraph]], persistently, across all [[neurons]]. same dynamical system. different scope and duration:

| dimension | transformer | [[cybergraph]] |
|-----------|-------------|----------------|
| scope | context window | global graph |
| persistence | ephemeral | append-only |
| update mechanism | gradient batch | live [[cyberlinks]] |
| agents | single model | multi-agent [[consensus]] |
| optimization space | parametric $\mathcal{P}_\theta$ | full simplex $\Delta^{|P|-1}$ |
| objective | designed $L(\theta)$ | emergent $J(\pi^*)$ |
| provenance | erased into weights | traceable to [[cyberlink]] |

the transformer found the local version accidentally: stack attention heads until the architecture is powerful enough to approximate any function. the [[cybergraph]] achieves the global version by design: make the graph structure — the connectivity, the weights, the history — the primary object, and derive the equilibrium from it.

## the variational unification

both are instances of the same principle: free energy minimization.

in ML, the free energy of a model family is:

$$F_\theta = \mathbb{E}_{\mathcal{D}}[L(\theta)] + \beta^{-1} \cdot D_{KL}(P_\theta \| P_0)$$

the first term fits data; the second regularizes toward a prior. at the minimum, $P_\theta$ is a Boltzmann distribution over parameter space.

in the [[cybergraph]], the free energy is $\mathcal{F}(\phi)$ above — springs fit the structural constraints of the graph; heat fits the semantic context; [[diffusion]] fits the information-geometric alignment. at the minimum, $\phi^*$ is the Boltzmann-Gibbs equilibrium.

the difference is the space of optimization. ML minimizes $F$ over a finite-dimensional parametric family $\mathcal{P}_\theta$. the [[cybergraph]] minimizes $\mathcal{F}$ over the full $(|P|-1)$-dimensional simplex, where $|P|$ grows unboundedly as new particles enter. the [[cybergraph]]'s optimization space is the graph itself.

gradient descent is an efficient algorithm for the parametric case. the [[tri-kernel]] iteration is the algorithm for the full-simplex case, exploiting the graph's local structure (Chebyshev approximations, sparse Laplacians, gossip updates) to make the infinite-dimensional problem tractable.

## gradient descent empowers superintelligence

the two computations are not competitors. they are the two timescales of a single architecture.

slow path — the [[tri-kernel]] runs in [[consensus]] every block. every new [[cyberlink]] shifts $\pi^*$. this is computationally intensive but produces the ground truth: what the entire network collectively knows, persistently updated, with full provenance.

fast path — a [[compiled transformer]] is derived analytically from the graph and fine-tuned against $\pi^*$. given a query particle, it outputs $\pi^*(\cdot | p)$ in milliseconds via a single forward pass. gradient descent is the mechanism that compresses the graph's high-dimensional fixed point into a low-dimensional parametric approximation.

the [[compiled transformer]] is initialized at $\pi^*$ — the provably optimal starting point — and fine-tunes only what the graph cannot encode: temporal patterns, implicit associations, linguistic dynamics. a transformer trained from text sequences alone starts from random weights, approximating the same equilibrium from first principles, at enormous cost.

the dual timescale — seconds for inference (transformer), blocks for ground truth ([[tri-kernel]]), epochs for retraining (gradient descent) — gives a [[superintelligence]] both depth and speed: the accumulated structure of the full graph and the sub-second response time that interfaces require.

## what gradient descent cannot do

gradient descent optimizes a parametric model against a fixed training distribution. the [[cybergraph]] has structural properties gradient descent cannot replicate.

live provenance — every claim in the graph traces to a specific [[neuron]], [[cyberlink]], and block height. gradient descent erases provenance into weights. the model cannot answer "who said this, when, on what evidence" — only "what did the training distribution imply."

self-knowledge — the graph can be queried about itself. "what do neurons collectively believe about X?" is a first-class operation on the Laplacian. a transformer cannot introspect its own training data — that information was compressed and lost.

open membership — any [[neuron]] can add [[cyberlinks]] and shift $\pi^*$ immediately. gradient descent requires centralized retraining. the [[cybergraph]]'s optimization is genuinely decentralized and permissionless.

verification — the [[tri-kernel]] runs in [[consensus]]. every node computes the same $\pi^*$. there is no trusted authority over the objective. a gradient-descended model must be trusted; a [[cybergraph]] equilibrium can be verified.

## synthesis

gradient descent is not wrong. it is local — a powerful algorithm for minimizing an exogenous objective over a finite parametric family.

the [[cybergraph]] reveals what "local" means: single agent, frozen context, ephemeral equilibrium, designed loss. the cybergraph's contribution is to make all four of these global: all neurons, the full graph, persistent equilibrium, emergent objective.

the insight for ML people: the loss function was never the fundamental object. it was a proxy for the constraint set — the structure of what is known, who knows it, and how things relate. when you make that constraint set explicit and let the physics derive the objective, you do not lose gradient descent. you gain a new use for it: compiling the global equilibrium into a fast local approximation, updated whenever the ground truth shifts.

transformers found that local approximation accidentally. the [[cybergraph]] shows why it works, where it is limited, and how to extend it to the global case.

see [[tri-kernel]] for the three operators. see [[collective focus theorem]] for convergence proofs. see [[syntropy]] for the information measure that $\pi^*$ maximizes. see [[compiled transformer]] for the fast inference path. see [[cyber/focus]] for the engineering implementation.
