---
tags: mathematics
crystal-type: pattern
crystal-domain: mathematics
---
the study of properties of [[space]] preserved under continuous deformation — stretching, bending, twisting — but not tearing or gluing. two shapes are topologically identical if one can be continuously deformed into the other; a coffee cup and a torus are the same, a sphere and a torus are not

the foundational shift: topology replaces distance with *openness*. a topology on a set $X$ is a collection $\tau$ of subsets — the open sets — satisfying: $\emptyset$ and $X$ are open; arbitrary unions of open sets are open; finite intersections of open sets are open. from these three axioms, without any notion of measurement, the entire structure of [[continuity]], [[convergence]], and connectedness follows

---

## continuous maps and equivalence

a map $f: X \to Y$ is continuous when preimages of open sets are open — this is the intrinsic definition, free of $\varepsilon$-$\delta$ language. a [[homeomorphism]] is a bicontinuous bijection: the strongest notion of topological equivalence. spaces that are homeomorphic are indistinguishable by any topological property

weaker equivalences matter too. a [[homotopy equivalence]] allows deformation retracts: the circle and the punctured plane are homotopy equivalent but not homeomorphic. [[homotopy]] is the study of continuous deformations between maps, organized into [[homotopy groups]] $\pi_n(X)$. the [[fundamental group]] $\pi_1(X)$ — loops up to continuous deformation — classifies which holes a space has at dimension one

---

## invariants: counting what topology cannot change

topological invariants are quantities preserved by homeomorphism. they are the tools for distinguishing spaces without finding an explicit homeomorphism or proving none exists

the [[Euler characteristic]] $\chi = V - E + F$ for a polyhedron, generalized by the alternating sum of Betti numbers: $\chi = \sum_n (-1)^n b_n$. for the sphere $\chi = 2$, the torus $\chi = 0$, the double torus $\chi = -2$

[[homology]] groups $H_n(X)$ count $n$-dimensional holes algebraically: $H_0$ measures connected components, $H_1$ measures loops that bound no disc, $H_2$ measures enclosed voids. homology is computable and turns topological questions into [[linear algebra]]

[[cohomology]] $H^n(X)$ is the dual theory, assigning functions to holes rather than counting chains. cohomology carries a ring structure — the cup product — and connects to differential forms via de Rham's theorem. it is the natural home of [[sheaf|sheaf cohomology]]

---

## sheaves: local-to-global consistency

a [[sheaf]] $\mathcal{F}$ on a topological space $X$ assigns data $\mathcal{F}(U)$ to each open set $U$, with consistent restriction maps whenever $V \subseteq U$, satisfying two axioms: locality (sections that agree locally are equal) and gluing (locally consistent sections assemble into a unique global section)

sheaves are the precise language for asking: when does locally consistent data extend to a globally consistent picture? sheaf cohomology $H^n(X, \mathcal{F})$ measures the obstruction. $H^0$ is global sections. $H^1 \neq 0$ means locally consistent data that cannot be globally assembled — a topological contradiction encoded algebraically

the passage from a topological space to a [[Grothendieck topology|site]] (a [[category]] equipped with a coverage axiom) and then to a [[topos]] (the category of sheaves on a site) generalizes topology beyond point-set foundations. every topos has an internal logic; every topological space is a special case

---

## branches

point-set topology (general topology) — foundations: separation axioms (Hausdorff, normal, regular), compactness, connectedness, metrization theorems. the infrastructure on which all other branches rest

[[algebraic topology]] — homotopy groups, homology, cohomology, spectral sequences, K-theory. turns topological questions into algebraic computations

differential topology — smooth manifolds, tangent bundles, Morse theory, cobordism. topology with a smooth structure, the arena of [[relativity]] and [[quantum mechanics]]

geometric topology — knots, 3-manifolds, geometric structures (hyperbolic, spherical, Euclidean). the Poincaré conjecture (proved by Perelman, 2003) and geometrization belong here

---

## topology in the cybergraph

[[knowledge topology]] is the shape of [[knowledge]] as revealed by graph structure. the [[Laplacian]] $L = D - A$ encodes topology algebraically; its spectral gap $\lambda_2$ ([[Miroslav Fiedler|Fiedler value]]) measures how well-connected — how topologically robust — the knowledge is

the [[tri-kernel]] fixed point, the [[focus|focus distribution]] $\phi^*$, is a sheaf-theoretic object: the unique global section consistent with every local diffusion, spring, and heat constraint. [[sheaf]] cohomology of the [[cybergraph]] measures contradictions in the knowledge structure that linking cannot resolve without topological change

the [[Seven Bridges of Koenigsberg]] — Euler's 1736 problem that founded graph theory — was the first topological argument: the question had no answer that depended on distances or shapes, only on connectivity. the [[cybergraph]] inherits this tradition

see also: [[category theory]], [[algebra]], [[sheaf]], [[knowledge topology]], [[Laplacian]], [[spectral gap]], [[homology]], [[differential equations]]