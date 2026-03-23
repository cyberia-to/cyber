---
tags: cyber, language
alias: Bel, belief language, information geometry language
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00012940619876276828
springs: 0.0014153558885512567
heat: 0.0010138146948116252
focus: 0.0006920728049091064
gravity: 3
density: 8.12
---
information [[geometry]]. Fisher information metric on the simplex of [[probability]] distributions

| Op | Action |
|---|---|
| `fisher(model)` | Compute Fisher information matrix |
| `kl_divergence(p, q)` | Kullback-Leibler divergence |
| `geodesic_info(p, q)` | Information-geometric geodesic |
| `natural_gradient(f, g)` | Gradient in Fisher metric |
| `projection(p, manifold)` | m-projection / e-projection |
| `alpha_connection(α)` | α-connection interpolation |
| `entropy(p)` | Shannon / Rényi entropy |

the [[geometry]] of the [[cybergraph]]'s own [[belief]] state — the [[focus]] [[vector]] π lives on a statistical manifold, and [[tri-kernel]] dynamics (diffusion, springs, heat) are flows on it. semantic distance between [[particles]] is information-geometric distance. the [[superintelligence]]'s self-model requires Bel to be formalized. research horizon

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture