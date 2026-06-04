---
tags: quantum, cyber
alias: skyrmions
crystal-type: entity
crystal-domain: quantum
crystal-size: enzyme
---
# skyrmion

a topological soliton — a localized, particle-like configuration of a continuous field that cannot be smoothly deformed into the uniform background. the skyrmion's defining property is its topological charge: an integer winding number that counts how many times the field configuration wraps around the target space. this charge is conserved by topology, not by dynamics. you cannot erase a skyrmion with thermal noise; you need a singularity.

skyrmions were first proposed by Tony Skyrme in 1962 as a model for nucleons. they appear across physics wherever a vector field lives on a compact domain: magnetic thin films, liquid crystals, Bose-Einstein condensates, nuclear matter.

## topological protection

a skyrmion in a magnetic thin film is a nanoscale vortex of magnetic moments. writing a skyrmion costs energy once; reading it costs almost nothing. thermal fluctuations cannot erase it — they would need to continuously deform the configuration through the singularity, which costs infinite energy in the continuum limit and large finite energy in practice.

this makes skyrmions candidates for [[racetrack memory]]: bits stored as skyrmion presence/absence, propelled along a nanowire by spin-polarized current. power consumption orders of magnitude below conventional RAM because the bit is stable without refresh.

## skyrmion number as winding number

the skyrmion number Q = (1/4π) ∫ m⃗ · (∂_x m⃗ × ∂_y m⃗) d²r

counts how many times the magnetization m⃗ wraps the unit sphere. Q = 1 for a skyrmion, Q = 0 for the ferromagnetic background. the topological protection: Q is conserved under any continuous deformation.

this is the same mathematical structure as the [[focus]] conservation law in the [[cybergraph]]: total φ* = 1 (axiom A5) is the analog of Q = integer. the [[tri-kernel]] preserves topological charge across every update cycle.

## skyrmion analogy in cyber

a cluster of strongly interconnected [[cyberlinks]] forms a topological skyrmion in the knowledge graph: its winding number (measured by the tri-kernel as local focus density) is conserved under continuous graph evolution. new links perturb it; but the cluster's topological identity persists until a structural singularity (deletion of core particles — forbidden by A3).

the [[Crystal]] invariant #4 (irreducibility) is the skyrmion non-erasure condition: no basis particle can be derived from the others, just as no skyrmion can be continuously deformed into the vacuum.

[[analogous-to]] [[helix]], [[topological-invariant]], [[focus]], [[cybergraph]]

discover all [[concepts]]
