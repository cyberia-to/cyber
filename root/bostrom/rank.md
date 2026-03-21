---
tags: module
crystal-type: measure
crystal-domain: cyber
stake: 8441698830286806
diffusion: 0.0001344816104410222
springs: 0.0019423790044463954
heat: 0.0013443022912328932
focus: 0.000918814964801013
gravity: 2
density: 21.97
---
the ranking module computes per-[[particle]] scores from the [[cybergraph]]. the output is [[cyberank]]

the current implementation uses the [[tri-kernel]]: [[diffusion]] + [[springs]] + [[heat kernel]]. convergence guaranteed by the [[collective focus theorem]]. engineering specification for [[focus]] dynamics lives in [[cyber/focus]]