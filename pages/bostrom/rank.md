---
tags: module
crystal-type: measure
crystal-domain: cyber
---
The ranking module computes per-[[particle]] scores from the [[cybergraph]].

The current implementation ([[cyberank]]) uses a tri-kernel approach: diffusion + springs + heat kernel. The mathematical foundation is the [[cft]]. The engineering specification for [[focus]] dynamics, conservation laws, and convergence lives in [[cyber/focus]].
