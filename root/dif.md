---
tags: cyber, language
alias: Dif, differential language, differential geometry language
crystal-type: entity
crystal-domain: cyber
---
differential [[geometry]]. Riemannian manifolds, tangent spaces, geodesics, Laplace-Beltrami operator. the [[geometry]] of continuous curved space

| Op | Action |
|---|---|
| `chart(M, coords)` | Define coordinate patch on manifold |
| `metric(g_ij)` | Specify Riemannian metric tensor |
| `christoffel(g)` | Compute connection coefficients |
| `geodesic(p, v, t)` | Trace geodesic from point p with velocity v |
| `covariant_deriv(T, v)` | Parallel transport / covariant derivative |
| `curvature(g)` | Riemann curvature tensor |
| `laplacian(f, g)` | Laplace-Beltrami operator on manifold |

required for: latent space embeddings, [[tri-kernel]] diffusion formalized as heat flow on manifolds, physics simulation. programming model: coordinate charts, metric tensors, covariant derivatives — none of which exist in [[Ren]]. [[proof]]-hard over finite [[field]]s. research horizon

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture