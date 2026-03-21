---
tags: cyber, language
alias: Bel, belief language, information geometry language
crystal-type: entity
crystal-domain: cyber
diffusion: 0.0001350842356534836
springs: 0.001519030939132888
heat: 0.0010648542037193388
focus: 0.0007362222403104797
gravity: 3
density: 7.98
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