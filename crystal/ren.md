---
tags: cyber, language
alias: Ren, render language, clifford
crystal-type: entity
crystal-domain: cyber
---
Clifford geometric [[algebra]] G(p,q,r). unifies [[vector]]s, bivectors, rotors. rotations, reflections, translations in one [[algebra]] over F_p

| Op | Action |
|---|---|
| `geometric_product(a, b)` | Full Clifford product of multivectors |
| `inner(a, b)` | Inner (dot) product — grade lowering |
| `outer(a, b)` | Outer (wedge) product — grade raising |
| `reverse(a)` | Reverse multivector (conjugation for rotors) |
| `dual(a)` | Poincaré dual — complement in the algebra |
| `sandwich(r, x)` | Rotor application: r x r̃ |
| `grade(a, k)` | Extract grade-k component |

covers Euclidean G(n,0,0), Projective G(n,0,1), Conformal G(n+1,1,0). fixes the [[Arc]] → SVG compilation gap: [[Arc]] provides [[topology]], Ren provides spatial embedding, compiler produces [[vector]] output. STARK-provable now — geometric product is F_p [[algebra]] with extra structure. compiles to [[Trident]]

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture