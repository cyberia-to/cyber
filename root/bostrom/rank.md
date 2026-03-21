---
tags: module
crystal-type: measure
crystal-domain: cyber
stake: 8441698830286806
diffusion: 0.00012364884272239444
springs: 0.0017219867722324462
heat: 0.001232697339904155
focus: 0.0008249599210117513
gravity: 2
density: 17.9
---
the ranking module computes per-[[particle]] scores from the [[cybergraph]]. the output is [[cyberank]]

the current implementation uses the [[tri-kernel]]: [[diffusion]] + [[springs]] + [[heat kernel]]. convergence guaranteed by the [[collective focus theorem]]. engineering specification for [[focus]] dynamics lives in [[cyber/focus]]