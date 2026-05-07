---
tags: research, math
crystal-type: pattern
crystal-domain: cyber
alias: categorical patches, commutative patches
---
mathematical framework for version control where changes are morphisms in a [[category]], independent changes commute, and [[conflicts]] are first-class algebraic objects

originated in work by Pierre-Etienne Meunier on categorical semantics of version control. the key departure from snapshot-based systems ([[git]]): a patch is defined by what it changes, independently of the history that produced the source state

## core structure

let Repo be a category where:
- objects are repository states S (sets of tracked content)
- morphisms are patches P: S₁ → S₂
- composition is sequential application: P₂ ∘ P₁
- identity morphism is the null patch ε

## three relations between patches

for two patches P and Q acting on state S:

independent (P ⊥ Q) — P and Q operate on disjoint regions. they commute:

```
apply(Q, apply(P, S)) = apply(P, apply(Q, S))
```

dependent (P → Q) — Q operates on content created or modified by P. Q cannot be applied without first applying P

conflicting (P ⊗ Q) — P and Q make incompatible changes to the same region. the [[conflict]] is a typed algebraic object, resolvable by a further patch R that has both P and Q in its dependency closure

## commutativity theorem

if all patches in a set are pairwise independent, applying them in any order produces the same result. merge is set union. this eliminates the order-dependence that causes phantom conflicts in snapshot systems

## dependency DAG

the set of all patches with dependency edges forms a DAG. applying a patch Q requires first applying its transitive closure in any topological ordering — the result is the same for all valid orderings ([[confluence]])

## why it matters

- parallel agents can work simultaneously on disjoint regions without coordination
- conflict resolution is permanent — once resolved, the resolution propagates to all views
- content addressing makes patches globally unique without a central registry
- the formalism maps directly to [[cybergraph]] primitives: patches are [[cyberlinks]], tracked content is [[particles]]

see [[cyber/patch]] for the cyber implementation, [[cyber/patch/spec]] for the full specification

## references

- Meunier, P.-E. "A Categorical Theory of Patches." 2017
- Mimram, S. and Di Giusto, C. "A Categorical Theory of Patches." ENTCS 2013
- Jacobson, S. "A Formalization of Darcs Patch Theory Using Inverse Semigroups." 2009