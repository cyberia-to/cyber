---
tags: cyber, language
alias: Ten, tensor language, linear language
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00013556010574394792
springs: 0.0009578773815330045
heat: 0.0007055830722013264
focus: 0.0004962598817721431
gravity: 4
density: 7
---
the tensor language. `Tensor<[D1, D2, ..., Dk]>` where dimensions are compile-time constants. shape mismatches are compile errors

| Op | Action |
|---|---|
| `matmul(A, B)` | Matrix multiplication |
| `einsum(spec, ...)` | Einstein summation |
| `reshape(T, shape)` | Reshape tensor |
| `broadcast(T, dims)` | Broadcast to higher dimensions |
| `transpose(T, perm)` | Permute dimensions |
| `reduce(T, axis, op)` | Reduce along axis |
| `conv2d(X, K)` | 2D convolution |
| `softmax(T, axis)` | Softmax activation |

dense and sparse. SpMV over sparse adjacency matrices = [[graph]] computation ([[focus]] [[vector]] π, [[tri-kernel]] diffusion). quantized [[inference]] (int4, int8 matmul) = contraction over Z/2ⁿ. full-precision neural layers = contraction over F_p. Ten is the compute engine for both the [[cybergraph]] and AI [[inference]]. CYBERRANK is literally repeated `matmul`. compiles to Tri

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture