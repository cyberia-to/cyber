---
tags: cyber, core
crystal-type: pattern
crystal-domain: cyber
crystal-size: enzyme
stake: 9710004032755294
diffusion: 0.013574505239494859
springs: 0.0006055814391129039
heat: 0.004807784932265426
focus: 0.007930484037934619
gravity: 192
density: 12.06
---
three local operators whose fixed point is [[cyberank]]

  - [[diffusion]] — explore via random walks
  - [[springs]] — structural consistency via screened Laplacian
  - [[heat]] — adaptation via graph heat kernel

the only operator families that survive the locality constraint required for planetary-scale computation. the [[tru]] runs the tri-kernel on the [[cybergraph]] in [[consensus]], producing [[focus]] per [[particle]]

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

  - [[cyber/tri-kernel]] — formal specification
  - [[tri-kernel architecture]] — why these three operators
  - [[collective focus theorem]] — convergence proofs

discover all [[concepts]]