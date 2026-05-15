---
tags: trisha, cyber
crystal-type: entity
crystal-domain: cyber
alias: trisha warrior, Triton VM warrior
subgraph: true
repo: ../trisha
---

[[trident]] runtime warrior. execute, prove, verify on [[Triton VM]].

trisha implements four traits — Runner, Prover, Verifier, Deployer — dispatched through a GPU backend (Metal / Vulkan / DX12 via wgpu). WGSL shaders handle the seven hot paths: Tip5 batch hashing, iNTT, NTT, Merkle tree, FRI fold, GEMV, and mining. the full prover pipeline runs on GPU with kernel fusion eliminating per-layer CPU↔GPU sync. 4.3× proving speedup on M1 Max. mining at 24M H/s.

source → [[trident]] compile → [[trisha]] run → prove → verify → deploy on [[neptune-core]].

deploy is a stub: program digest is computed and printed, UTXO construction and neptune-core RPC await integration.
