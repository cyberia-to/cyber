---
alias: heat kernel, multi-scale smoothing, adaptation, thermostat, heat
tags: cyber
crystal-type: entity
crystal-domain: biology
stake: 7625453141862128
diffusion: 0.005060699451848776
springs: 0.0005257949912566704
heat: 0.0020218444794611627
focus: 0.0030924571191937125
gravity: 51
density: 3.28
---
third operator of the [[tri-kernel]]

`∂H/∂τ = -LH`, `H₀ = I`, so `H_τ = exp(-τL)`

temperature τ controls scale

answers: "what does the graph look like at scale τ?"

high τ explores (annealing), low τ commits (crystallization)

provides adaptive context — the thermostat of collective attention

positivity-preserving, semigroup: `H_{τ₁} H_{τ₂} = H_{τ₁+τ₂}`

Chebyshev polynomial approximation gives h-local computation with bounded error

locality: Gaussian tail decay, `h = O(log(1/ε))` hops

the adaptation force — metabolism, phase changes, the ability to shift

universal pattern

- physics: thermostat, phase changes
- biology: metabolism, immune plasticity
- ecology: seasons, succession, disturbance
- economics: booms, busts, revolutions

together with [[diffusion]] and [[springs]] forms the [[tri-kernel]] that computes [[cyberank]]

see [[tri-kernel]] for completeness proof

discover all [[concepts]]