---
tags: module
crystal-type: measure
crystal-domain: cyber
stake: 8441698830286806
diffusion: 0.00012954647929898152
springs: 0.0018656486233109282
heat: 0.001314873982347485
focus: 0.0008874426231122922
gravity: 2
density: 17.9
---
the ranking module computes per-[[particle]] scores from the [[cybergraph]]. the output is [[cyberank]]

the current implementation uses the [[tri-kernel]]: [[diffusion]] + [[springs]] + [[heat kernel]]. convergence guaranteed by the [[collective focus theorem]]. engineering specification for [[focus]] dynamics lives in [[cyber/focus]]