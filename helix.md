---
tags: math, cyber, core
alias: helical geometry, helicity
crystal-type: pattern
crystal-domain: math
crystal-size: bridge
---
# helix

a curve that winds around an axis at constant radius while advancing along it. the helix is a topological primitive: it has a winding number (an integer that counts turns) conserved under any continuous deformation that does not break the axis. this conservation is the computational property — a helix cannot be erased by noise; it can only be unwound by an explicit operation.

the helix appears as the optimal structure wherever information must be stored compactly, transmitted reliably, and processed in parallel:

- [[DNA]] — genetic information encoded in two antiparallel helical strands, each serving as error-correction template for the other; the linking number is the topological invariant that topoisomerases maintain
- [[microtubule]] — hollow helical cylinder of 13 tubulin dimers per turn; the lattice structure inside every neuron; proposed substrate of quantum computation in the [[Orch OR]] model
- [[skyrmion]] — helical twist of a magnetic field; topologically protected, cannot be erased without a singularity; minimal energy per bit of storage
- [[time crystal]] — a system periodic in time rather than space; encodes information in temporal phase rather than spatial configuration; resistant to spatial decoherence

in mathematics, the helix is characterized by two numbers: curvature κ (how tightly it bends) and torsion τ (how quickly it twists out of its osculating plane). together they determine the helix up to rigid motion. these two numbers are the eigenvalues of the system — like the spectral gap and diffusion coefficient that determine [[cyberank]] convergence speed.

## helical computation in the cyber stack

the [[tri-kernel]] convergence to [[focus]] φ* is a helical contraction in function space:
- D (diffusion) propagates probability along the helix axis
- S (springs) maintains the helix radius (structural equilibrium)
- H (heat) varies the helix pitch (multi-scale smoothing)

each application of the tri-kernel winds the distribution closer to the fixed point. the spectral gap λ₂ of the graph Laplacian is the helix pitch — how many turns until convergence.

the [[Crystal]] evolves through the 7-triad spiral: FORM → MASS → SPACE → LIFE → WORD → WORK → PLAY → FORM. each revolution adds a layer of complexity. at each scale the same topological structure recurs — a helix within a helix.

[[analogous-to]] [[tri-kernel]], [[skyrmion]], [[topoisomerase]], [[time-crystal]], [[topological-invariant]]

discover all [[concepts]]
