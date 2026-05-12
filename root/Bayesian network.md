---
tags: cybics, mathematics, article, draft, research
alias: Bayesian network, Bayesian networks, belief network, belief networks, directed graphical model, probabilistic graphical model
crystal-type: pattern
crystal-domain: cybics
crystal-size: bridge
---

a directed acyclic graph where nodes are random variables and edges encode conditional dependence — the structure of beliefs about a domain, made explicit as topology

---

## the core idea

a Bayesian network specifies a joint probability distribution over $n$ variables $X_1, \ldots, X_n$ by decomposing it into conditional probabilities along a DAG:

$$P(X_1, \ldots, X_n) = \prod_{i=1}^n P(X_i \mid \text{parents}(X_i))$$

each node stores a conditional probability table (CPT): for each combination of parent values, the probability distribution over the node's values. the graph encodes which variables directly influence which others; the CPTs encode the strength of those influences.

---

## what the graph structure means

an edge $A \to B$ in a Bayesian network means: A is a direct cause of B (in the modeling assumption). it is a structural claim: knowing A provides direct probabilistic evidence about B, above and beyond any other variables.

the absence of an edge is also a claim: A and B are conditionally independent given some set of other variables. Bayesian networks make independence assumptions explicit in the graph topology — they are compressed representations of a distribution that would otherwise require exponentially many parameters.

---

## d-separation

d-separation (directional separation) is the graphical test for conditional independence. two nodes X and Y are d-separated given observed set Z if all paths between them are blocked given Z.

three path patterns:

chain: $X \to Z \to Y$. Z blocks the path when observed — conditioning on the middle node cuts the dependence.

fork: $X \leftarrow Z \to Y$. Z blocks when observed — conditioning on the common cause removes the correlation.

collider: $X \to Z \leftarrow Y$. Z is open by default but blocks when observed — conditioning on a common effect creates dependence between its causes. counter-intuitive: observing the effect makes the causes dependent even if they were independent a priori.

---

## belief propagation

inference in a Bayesian network means computing posterior marginals $P(X_i \mid \text{evidence})$ for nodes of interest given observed values at other nodes.

belief propagation (Pearl, 1988) is the message-passing algorithm for exact inference in trees. each node sends two messages to each neighbor: the product of messages from all other neighbors (belief from the rest of the graph) and the likelihood given observed evidence. iteration propagates beliefs until convergence.

exact inference is NP-hard in general graphs (loopy graphs). loopy belief propagation applies the same algorithm to graphs with cycles and often converges approximately — it is the foundation of modern deep learning (the forward pass of a neural network is one-shot loopy belief propagation with learned message functions).

---

## connection to [[cybergraph]]

the [[cybergraph]] is a generalization of a Bayesian network:

| Bayesian network | [[cybergraph]] |
|---|---|
| random variables | [[particles]] |
| directed edges (DAG) | [[cyberlinks]] (directed, allow cycles) |
| CPT at each node | [[focus]] distribution from [[tri-kernel]] |
| exact inference | [[tri-kernel]] diffusion to φ* |
| belief propagation | [[tri-kernel]] iterations |
| prior on variables | [[prior]] weighted by [[karma]] |
| posterior after evidence | φ* — the [[focus]] distribution |

the key differences: the cybergraph is not restricted to DAGs (cycles are permitted — the tri-kernel handles them via the heat kernel damping), edges are staked assertions from [[neurons]] rather than fixed model parameters, and the CPTs are not stored explicitly but emerge from the aggregate of all [[cyberlinks]] weighted by stake and market price.

the [[tri-kernel]] $\mathcal{R} = \lambda_d D + \lambda_s S + \lambda_h H_\tau$ is a generalized belief propagation over the [[cybergraph]]. each iteration of $\mathcal{R}$ is one step of message passing. φ* is the fixed point — the posterior distribution of [[focus]] given all evidence.

---

## the [[cybergraph]] as a living Bayesian network

a classical Bayesian network has fixed structure and fixed parameters. the [[cybergraph]] is dynamic on both dimensions:

structure changes. new [[cyberlinks]] add edges. each new edge is a new conditional dependence assertion. the joint distribution shifts with every link creation.

weights change. [[karma]] re-weights [[neuron]] contributions. ICBS market prices re-weight edge strengths. the effective CPTs are continuously updated from collective beliefs.

no oracle. classical Bayesian networks require exact prior specification. the [[cybergraph]] is self-specifying: the prior on each edge emerges from the economic market (ICBS), and the prior on each [[neuron]] emerges from [[karma]] history. the [[cybergraph]] learns its own Bayesian network structure from collective assertion and collective market behavior.

---

## from Bayesian networks to [[Bayesian Truth Serum]]

a Bayesian network models dependencies between random variables. [[Bayesian Truth Serum]] extends this to the social level: it models the dependencies between agents' beliefs. the meta-prediction $m_i$ in BTS is an agent's model of the collective belief distribution — a Bayesian network with agents as nodes and belief correlations as edges.

BTS succeeds because it exploits the structure of belief correlations (just as belief propagation exploits graph structure) to extract the signal component — what an agent knows that the collective doesn't already account for.

see [[Bayes theorem]] for the update rule. see [[belief]] for the probability-as-belief interpretation. see [[prior]] and [[posterior]] for the Bayesian distributions. see [[tri-kernel]] for the [[cybergraph]]'s belief propagation. see [[focus flow computation]] for the convergence proof.