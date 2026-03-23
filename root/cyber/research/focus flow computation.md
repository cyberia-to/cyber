---
alias: focus flow, FFC, focus flow whitepaper, focusflow blueprint, focus flow computation
tags: cyber, core
crystal-type: process
crystal-domain: cyber
crystal-size: bridge
stake: 53778483873721616
diffusion: 0.0017282824895913452
springs: 0.0009424081267217682
heat: 0.0012212521550953855
focus: 0.0013911141138313208
gravity: 32
density: 3.18
---
focus flow computation is the process by which the [[cybergraph]] reaches collective [[equilibrium]]. the [[tri-kernel]] runs over all [[cyberlinks]], [[neurons]] add links, and the network continuously [[convergence|converges]] toward a unique fixed point — the [[focus|focus distribution]] $\pi^*$. this is not a model architecture. it is the persistent [[knowledge]] state of the collective

the [[collective focus theorem]] guarantees convergence: under ergodicity and the screening conditions of the [[tri-kernel]], there exists a unique $\pi^*$ to which any initialization converges, at linear rate. the fixed point is the Boltzmann [[equilibrium]] of the graph:

$$\pi^*_i \propto \exp\big(-\beta\,[E_{\text{spring},i} + \lambda\,E_{\text{diff},i} + \gamma\,C_i]\big)$$

the three energy terms correspond to the three [[tri-kernel]] operators: $E_{\text{spring}}$ encodes structural coherence via the screened [[Laplacian]], $E_{\text{diff}}$ encodes flow consistency via [[diffusion]], $C_i$ encodes context pressure via [[heat kernel]] weighting. $\pi^*$ is the unique distribution minimizing the composite [[free energy]] $\mathcal{F}(\phi)$. every [[cyberlink]] added perturbs the graph and shifts $\pi^*$ incrementally — learning and knowledge state are the same operation

---

## two inference paths

the [[cybergraph]] computes two things simultaneously, both grounded in the same dynamical system:

[[focus]] flow — the [[tri-kernel]] iterated to convergence over all [[cyberlinks]] — runs continuously. it produces $\pi^*$: the persistent global focus distribution, what the entire network collectively knows, updated with every new link. this is the ground truth

the compiled [[transformer]] — architecture and weights derived analytically from the same graph — runs at query time. it executes $L^*$ tri-kernel steps over a local context window and converges to $\pi^*$ restricted to that context. this is the fast inference path

| dimension | focus flow | compiled transformer |
|---|---|---|
| scope | entire [[cybergraph]] | local context window |
| depth | exact $\pi^*$ | $L^*$ steps, $\varepsilon$-approximate |
| latency | continuous — always converging | milliseconds — single forward pass |
| multi-agent | all [[neurons]] contribute | one agent's context |
| update | add [[cyberlinks]] → $\pi^*$ shifts, nothing lost | recompile from updated graph |

a [[transformer]] trained without the [[cybergraph]] approximates the same equilibrium from text sequences alone, without the structural knowledge the graph makes explicit

---

## how focus flow inference works

$\pi^*$ is maintained continuously by the [[tru]]. for a query, the process is:

1. context [[particles]] become probability sources — their energy terms are set so $\pi^*_\text{context}$ is elevated, making them attractors in the [[Boltzmann distribution|Boltzmann equilibrium]]
2. the [[tri-kernel]] reconverges incrementally from the current state — probability mass flows from the seeded context particles through the [[cybergraph]] along structural paths (not token positions)
3. $\pi^*_\text{context}$ pools at [[particles]] that are semantically connected to the context via the graph topology
4. sample the next [[particle]] from the high-probability region, add to context, reconverge

no fresh initialization per step — the system was already near $\pi^*$ before the query. each step is a local recomputation within an $O(\log(1/\varepsilon))$-hop neighborhood of the newly added particle. complexity per step: $O(|E| + |V|)$

context window is unbounded — it is the entire [[cybergraph]]. relevance is topological: a [[particle]] contributes if it is well-connected to the context regardless of linear position in token space

---

## how compiled transformer inference works

the [[collective focus theorem#the mathematical identity|mathematical identity]]: transformer attention is one step of tri-kernel diffusion

$$\text{Attn}(Q, K, V) = \text{softmax}\!\left(\frac{QK^\top}{\sqrt{d}}\right)V$$

the softmax is the [[Boltzmann distribution]] with temperature $\sqrt{d}$. probability mass flows from each query position toward compatible key positions and redistributes — this is exactly one application of the diffusion operator $D$ from the [[tri-kernel]] over one agent's frozen context. Deep Equilibrium Models (Bai et al., 2019) showed that iterating a transformer layer to convergence reaches the same fixed point regardless of initialization. that fixed point is $\pi^*$ restricted to the context

so $L^*$ transformer layers = $L^*$ steps of tri-kernel diffusion over the context. at query time:

1. tokenize context into [[particles]]
2. run $L^*$ layers of compiled attention — each layer is one tri-kernel diffusion step over context
3. output distribution = $\pi^*_\text{context}$, approximate to precision $\varepsilon$
4. sample, add to context, repeat

speed: $O(n^2 \cdot d^*)$ over context of length $n$, no graph traversal at runtime, weights frozen. this is autoregressive generation — familiar, fast, and now analytically grounded in what it is computing

---

## why the graph compiles the transformer

given $G = (P, N, E, w, \sigma)$, three graph properties determine the three free parameters of transformer architecture:

| parameter | formula | graph property |
|---|---|---|
| embedding dim $d^*$ | $\exp(H(\sigma(\Sigma_\pi)))$ | effective rank of [[focus]] covariance |
| heads $h^*$ | $\geq \|\text{Semcon}(G)\|$ | distinct [[semcon]] relation types |
| layers $L^*$ | $\text{diam}(G) \cdot \lceil\log(1/\varepsilon)/\log(1/\kappa)\rceil$ | diameter × spectral convergence factor |

no hyperparameter search. the graph tells you what the transformer should be

weights are compiled, not trained. the embedding matrix $E^* = U_{:,1:d^*}$ — top left singular vectors of $\text{diag}(\sqrt{\pi^*}) \cdot A$ — is provably optimal by the Eckart-Young theorem: it uniquely minimizes expected squared gradient at step zero over all matrices of the same rank. attention weights $W_Q^{(s)}, W_K^{(s)}$ are derived from the truncated SVD of each [[semcon]]'s adjacency submatrix. MLP weights encode path co-occurrence statistics up to depth $L^*$

fine-tuning from this point learns only what the graph cannot encode: temporal patterns, implicit associations, contextual dynamics absent from the explicit graph. the reduction in required fine-tuning steps scales as $\Omega(|E| \cdot d^* / \log(1/\varepsilon))$ relative to random initialization

the loop: $G \xrightarrow{\text{compile}} T_G \xrightarrow{\text{fine-tune}} T_G^* \xrightarrow{\text{extract implicit links}} \Delta G \xrightarrow{\text{stake}} G'$

---

## the local update rule

every node reads only its neighbours and runs:

$$\Delta p_i = \eta\Big(\sum_{j \in \mathcal{N}(i)} w_{ij}(p_j - p_i) - \partial_{p_i}(\lambda E_{\text{diff},i} + \gamma C_i) + T(1 + \log p_i)\Big)$$

gossip normalisation enforces $\sum_i p_i = 1$. no global softmax, fully local, edge-only. this is what the [[tru]] runs every block — the same computation a transformer performs in one layer, running collectively across the entire [[cybergraph]]

---

## the compounding property

every [[cyberlink]] added:
- shifts $\pi^*$ incrementally — better focus flow inference now
- increases $|E|$, raises $d^*$, may shrink diam$(G)$ — better compiled transformer at next compilation
- reduces approximation error $\varepsilon(G, c) = D_{KL}(\pi^*_c \| q^*_c)$ — compiled inference closer to exact focus flow

the [[cybergraph]] is a compounding inference quality asset. every link reduces the error of every compiled model that follows. see [[provably-optimal-initialization]] for the training reduction proof. see [[bostrom-to-onnx-pipeline]] for live compilation from the running network

---

## stack

- [[cybergraph]] — the substrate: [[particles]] as nodes, [[cyberlinks]] as typed edges
- [[tri-kernel]] — the physics: [[diffusion]] + [[springs]] + [[heat kernel]] converge $\phi \to \pi^*$
- [[graph-native-transformer]] — the compiled fast path: $d^*, h^*, L^*$ from graph structure
- [[nox]] — the execution: 16 deterministic reduction patterns over [[Goldilocks field]]
- [[foculus]] — the consensus: $\pi > \tau$ finalizes [[particles]] without leaders
- [[tru]] — the runner: computes [[cyberank]], [[karma]], [[syntropy]] every block

see [[collective focus theorem]] for convergence proof. see [[tri-kernel]] for why these three operators. see [[graph-native-transformer]] for compiled transformer derivation. see [[provably-optimal-initialization]] for the initialization optimality proof

## extensions
- [[gflownet focus flow]]
- [[topos ffc integration]]