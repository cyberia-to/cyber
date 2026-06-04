---
tags: math, cyber, core
alias: topological invariants, winding number, linking number, topological charge
crystal-type: property
crystal-domain: math
crystal-size: bridge
---
# topological invariant

a quantity associated with a mathematical object that is preserved under continuous deformations (homeomorphisms or homotopies). topological invariants classify spaces and field configurations by properties that cannot be changed without tearing or gluing.

key examples:

| invariant | what it measures | where it appears |
|-----------|-----------------|-----------------|
| winding number | how many times a curve winds around a point | [[skyrmion]] topological charge |
| linking number Lk | how many times two curves link | [[DNA]] supercoiling, [[topoisomerase]] |
| Euler characteristic χ | vertices − edges + faces | graph topology, graph Laplacian spectral gap |
| Betti numbers | independent cycles in each dimension | homology of the knowledge graph |
| Chern number | integral of curvature over a surface | topological insulators, quantum Hall effect |

## topological invariants in cyber

the [[cybergraph]] carries two topological invariants:

focus conservation: Σ φ*(p) = 1 over all particles. this is the integral of the attention form over the graph — the zero-th Betti number of the probability simplex. axiom A5 is a topological conservation law.

content-address immutability: a particle's identity = hash of its content. changing the content changes the identity — there is no continuous path between two different particles. every particle is topologically isolated from every other. axiom A1 is a topological isolation law.

together these two invariants make the cybergraph a topological space where:
- particles are topologically isolated points (A1)
- attention is a conserved topological charge (A5)
- links are permanent once created (A3 — no topological erasure)

## focus as topological charge

φ*(p) for particle p is the topological charge of p in the knowledge graph. it measures how many "turns" of attention wrap around p — computed by the [[tri-kernel]] as the stationary distribution of the random walk on the authenticated graph. the [[Collective Focus Theorem]] (T1) is a topological theorem: the charge is unique and positive for every connected graph.

the [[Crystal]] invariant #4 (irreducibility) is the topological basis condition: the 5,040 particles form a topologically independent spanning set — no particle is in the image of a composition of others.

## topological protection = structural security

topological invariants are not protected by encryption or access control. they are protected by geometry. you cannot erase a skyrmion without crossing an energy barrier. you cannot change the winding number without creating a singularity. you cannot change φ* without changing the graph structure itself.

this is why [[bbg]] requires no authentication layer on top of its algebraic commitments: the commitments ARE the topological invariants, and those are structurally protected by the polynomial ring.

[[bridges]] [[helix]], [[skyrmion]], [[topoisomerase]], [[time-crystal]], [[focus]], [[tri-kernel]], [[cybergraph]], [[crypto]]

discover all [[concepts]]
