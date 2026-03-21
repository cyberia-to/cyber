---
tags: cyb, core
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00014635047450391414
springs: 0.00014684291465052643
heat: 0.000171012144487055
focus: 0.00015143054054452405
gravity: 3
density: 4.89
---
# onnx

local neural inference runtime in [[cyb]]. runs ONNX models on device via burn-webnn — dispatches to NPU (CoreML, NNAPI, WebNN), GPU ([[cyb/wgpu]] fallback), or CPU depending on hardware

provides local AI without cloud dependency:

- SLM (~100-300M params) for legacy CSS bridge in [[cyb/features]]
- embedding models for local semantic search in [[cyb/oracle]]
- classification models for content type detection
- custom models loaded by [[cyb/robot]] progs

three execution targets:

| platform | backend |
|----------|---------|
| browser | WebNN API → NPU/GPU |
| desktop | burn → CoreML (macOS) / DirectML (Windows) / CPU |
| mobile | burn → CoreML (iOS) / NNAPI (Android) |

see [[cyb/features]] for how inference integrates with the render and compatibility layers. see [[cyb/wgpu]] for GPU compute fallback