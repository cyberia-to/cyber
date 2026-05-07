---
tags: mathematics, cyber
alias: sheaves, sheafs, presheaf, presheaves
crystal-type: pattern
crystal-domain: mathematics
---
a mathematical structure that assigns data consistently to every open region of a [[topology|topological space]], satisfying two axioms: restriction and gluing

for a topological space $X$, a sheaf $\mathcal{F}$ assigns to each open set $U \subseteq X$ a set (or group, ring, module...) $\mathcal{F}(U)$ of sections, with restriction maps $\rho_{UV}: \mathcal{F}(U) \to \mathcal{F}(V)$ whenever $V \subseteq U$

the two axioms that distinguish a sheaf from a presheaf:

- locality — if two sections agree on every element of a cover, they are equal
- gluing — if sections on the pieces of a cover agree on all overlaps, they can be assembled into a unique global section

the sheaf condition is the formal statement that local consistency implies global coherence

---

on a [[knowledge graph]], a sheaf assigns data to neighborhoods of [[particles]] — local semantic frames — such that wherever two neighborhoods overlap, their frames agree. the [[tri-kernel]] fixed point is a sheaf-theoretic object: the [[focus|focus distribution]] is the unique global section consistent with every local diffusion, spring, and heat constraint simultaneously

[[knowledge topology]] acquires sheaf structure when every local assignment (what a [[neuron]] knows about its neighborhood) can be glued into a consistent global picture without contradiction — the definition of aligned [[collective focus]]

sheaf cohomology measures the obstruction to gluing: $H^1(\mathcal{F}) \neq 0$ means local sections cannot always be assembled globally. in a [[cybergraph]], nonzero cohomology corresponds to topological inconsistencies in the [[knowledge]] structure — contradictions that no amount of additional [[cyberlink|linking]] within the current topology can resolve

---

a presheaf satisfies only the restriction maps, not the gluing axiom. every sheaf is a presheaf; not every presheaf is a sheaf

sheafification is the canonical procedure that forces any presheaf into the nearest sheaf — analogous to taking the completion of a metric space

in [[category theory]], sheaves on a site (a [[category]] with a Grothendieck topology) are the objects of a [[topos]] — the categorical generalization of a topological space

---

see also: [[topology]], [[knowledge topology]], [[category theory]], [[collective focus theorem]], [[tri-kernel]]