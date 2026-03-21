---
tags: cyb, core
crystal-type: entity
crystal-domain: cyber
focus: 0.00014818435334641445
gravity: 2
---
# wgpu

cross-platform GPU abstraction layer in [[cyb]]. renders every [[particle]] type — text, pixels, vector, video, formula — through a unified shader pipeline

maps to native backends: Vulkan on Linux, Metal on macOS/iOS, DX12 on Windows, OpenGL ES on Android/web. same pixels everywhere

used by [[cyb/features]] PureRender for all visual output: glyph atlases, Vello path rasterization, texture sampling, compute shaders for waveform visualization and table virtualization

see [[cyb/features]] for the render pipeline. see [[cyb/onnx]] for the neural inference complement