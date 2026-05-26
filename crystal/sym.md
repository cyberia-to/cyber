---
tags: cyber, language
alias: Sym, symplectic language, symplectic geometry language
crystal-type: entity
crystal-domain: cyber
---
symplectic [[geometry]]. phase space with 2-form ω, Hamiltonian flows, canonical transformations, [[conservation]] laws

| Op | Action |
|---|---|
| `symplectic_form(M)` | Define closed non-degenerate 2-form ω |
| `hamiltonian(H, q, p)` | Specify Hamiltonian function |
| `flow(H, state, dt)` | Symplectic integration step |
| `poisson(f, g)` | Poisson bracket {f, g} |
| `canonical(T)` | Verify canonical transformation |
| `conserved(H, f)` | Test if f is conserved under H |
| `action(L, path)` | Compute action integral |

natural language of classical and semi-classical mechanics. required for: physical simulation with energy [[conservation]], [[quantum]]-classical interface, molecular dynamics. the [[conservation]] law structure (ω is closed: dω = 0) has no analog in Clifford or Riemannian [[geometry]]. research horizon

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture