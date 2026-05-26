---
tags: cyber, core
crystal-type: property
crystal-domain: cyber
crystal-size: enzyme
---
the constraint that every operator must compute from neighbors only

at planetary scale (10¹⁵ nodes), any algorithm requiring global state is physically impossible. [[locality]] is the filter that selects which operators can exist in the [[tri-kernel]]

## why only three operators survive

the [[tri-kernel architecture]] begins with all known graph operators and applies one test: can this operator produce a correct answer by reading only the h-hop neighborhood, where h = O(log(1/ε))?

three families pass:

[[diffusion]] — geometric decay via teleport parameter α. a random walker forgets its origin exponentially fast. influence beyond O(log(1/ε)) hops falls below ε

[[springs]] — exponential decay via screening parameter μ. the Green's function of the screened [[Laplacian]] $(L + μI)^{-1}$ decays as $e^{-\sqrt{μ} \cdot d}$ with graph distance d

[[heat]] — Gaussian tail decay via temperature τ. the heat kernel $H_τ = \exp(-τL)$ concentrates mass within O(√τ) hops

every other operator family (global spectral methods, all-pairs shortest paths, full matrix inversions) fails the locality test. they require reading the entire graph and cannot scale

## consequence

locality means edits are cheap: when a [[neuron]] creates a [[cyberlink]], only the local neighborhood needs recomputation. the rest of the [[cybergraph]] is unaffected up to error ε. this is what makes [[collective focus]] computable in real time on a planetary network

see [[tri-kernel architecture]] for the full derivation. see [[collective focus theorem]] for the locality radius proof

discover all [[concepts]]
