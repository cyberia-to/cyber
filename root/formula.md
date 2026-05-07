---
tags: cyb, cyber, core
alias: formula particle, latex, mathml, equation, mathematical notation
crystal-type: entity
crystal-domain: cyb
---
mathematical meaning as [[particle]]. the native format for equations, proofs, chemical notation, physical laws, and any knowledge that requires precise symbolic structure

source format: LaTeX, MathML — the universal notation systems of mathematics, physics, and chemistry

---

## rendering

```
latex/mathml source → parse → glyph layout (symbols, fractions, integrals) + path rasterization (Vello, for curves) → GPU composite
```

formula rendering combines two pipelines: text glyphs for symbols and alphanumerics, Vello vector paths for integral signs, root radicals, brackets, and other structural curves. the result is publication-quality mathematical notation rendered directly on the GPU without any external math rendering library

## in the cybergraph

formula is how precision enters the [[cybergraph]]. a claim in [[text]] says "energy equals mass times the speed of light squared." the formula particle IS $E = mc^2$ — unambiguous, unparaphraseable, linked directly to the particles that define each symbol

types of formula particles: physical laws, mathematical theorems, chemical reactions, statistical models, differential equations, proofs (each step a formula particle in a chain), dosing equations, orbital mechanics, economic models, protein binding affinities, quantum operators, cryptographic primitives, field equations, financial derivatives pricing models, biological growth equations

formula particles are the most precise objects in the [[cybergraph]]. a text particle about a drug says "effective in reducing tumor size." the formula particle states the exact dose-response curve. when the [[cyb/oracle]] returns an answer about a medical question, formula particles carry the quantitative truth

## properties

- symbol-linked — each symbol in a formula particle can link to the particle that defines it. $E$ links to the energy particle. $m$ links to the mass particle. the formula is not just notation — it is a navigable structure in the graph
- proof chains — a mathematical proof is a sequence of formula particles where each step follows from the previous. [[linkchains]] in the [[cybergraph]] naturally encode proof structure: the [[tri-kernel]] finds the shortest valid path from axioms to conclusion
- chemically complete — LaTeX includes chemistry notation (via mhchem) and structural formula conventions. a full chemical reaction with reagents, conditions, and products is a single formula particle
- executable — a formula particle can link to a [[component]] particle that evaluates it. the equation and its calculator are the same object in the graph

## relation to other languages

formula states what must be true with precision. [[text]] argues for it in prose. [[table]] contains the data that supports it. [[vector]] visualizes the geometry it describes. [[component]] makes it interactive. together they are the complete scientific artifact

see [[latex]] for the primary source format. see [[text]] for prose argumentation. see [[table]] for quantitative data that formulas model

discover all [[concepts]]
