---
tags: cyber, language
alias: Ten, tensor language, linear language
crystal-type: entity
crystal-domain: cyber
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

dense and sparse. SpMV over sparse adjacency matrices = [[graph]] computation ([[focus]] [[vector]] φ*, [[tri-kernel]] diffusion). quantized [[inference]] (int4, int8 matmul) = contraction over Z/2ⁿ. full-precision neural layers = contraction over F_p. Ten is the compute engine for both the [[cybergraph]] and AI [[inference]]. CYBERRANK is literally repeated `matmul`. compiles to Tri

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture