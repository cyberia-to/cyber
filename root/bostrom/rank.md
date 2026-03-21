---
tags: module
crystal-type: measure
crystal-domain: cyber
stake: 8441698830286806
focus: 0.0001344816104410222
gravity: 2
---
the ranking module computes per-[[particle]] scores from the [[cybergraph]]. the output is [[cyberank]]

the current implementation uses the [[tri-kernel]]: [[diffusion]] + [[springs]] + [[heat kernel]]. convergence guaranteed by the [[collective focus theorem]]. engineering specification for [[focus]] dynamics lives in [[cyber/focus]]