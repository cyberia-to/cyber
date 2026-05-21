---
tags: cyber
alias: WebAssembly, web assembly
crystal-type: entity
crystal-domain: cyber
---
# wasm

WebAssembly — a portable bytecode format and stack-based virtual machine, designed for safe, fast execution across browsers, edge runtimes, blockchains, and embedded hosts. one binary instruction set; many host implementations.

[webassembly.org](https://webassembly.org) · [feature status](https://webassembly.org/features/)

## cyber's runtime

cyber embeds [[wysm]] as its wasm runtime — a hard fork of [wasmi](https://github.com/wasmi-labs/wasmi) v2, extended with jet substitution, finite-wasm metering, an actor harness, and a [[trident]]-lowering on-ramp. lives at `~/cyber/wysm`.

## ecosystem

- runtimes: wasmi, wasmtime, wasmer, WAMR, TinyWasm, Stitch
- chains: [[cosmwasm]], [[substrate]], Stellar Soroban, NEAR, DFINITY Internet Computer
- languages with stable wasm targets: [[rust]], [[ts]], [[go]], C/C++, Zig, AssemblyScript
- browser support: universal since 2019 (Chromium, Firefox, Safari, Edge)

---

discover all [[concepts]]
