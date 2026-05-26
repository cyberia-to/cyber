---
tags: cyber, cyb, runtime
alias: wysm runtime, wasm runtime, wasm sandbox, webassembly runtime, soul runtime
crystal-type: entity
crystal-domain: cyber
---
# wysm

the WASM runtime tier of [[soft3]] — cyber's name for the wasm sandbox layer. a hard fork of [wasmi](https://github.com/wasmi-labs/wasmi) v2 living at `~/cyber/wysm`, extended with four cyber-native enhancements: jet substitution, finite-wasm metering, lunatic-pattern actor harness, and a [[trident]]-lowering on-ramp. the runtime [[cyb]] embeds to execute conventional programs ([[souls]]) loaded from [[radio]].

canonical spec: `~/cyber/wysm/specs/wysm.md`.

## naming

`wysm` is cyber's name for the runtime. `wasm`/`WASM` is the WebAssembly format. `wasmi` is the upstream project this fork tracks. cyber-side crates use the `wysm-` prefix; upstream crates keep their `wasmi_*` names.

## position in soft3

| runtime | what runs | proof contract |
|---------|-----------|----------------|
| [[nox]] | proven .nox programs, jets | unconditional |
| [[glia]] | .model inference | conditional on model |
| **wysm** | WASM modules, rune host jets | conditional on host |
| wgpu | GPU compute shaders | conditional on host |

wysm is the seat for conventional programs — anything that compiles to WebAssembly, deployed as a [[particle]], executed as a soul. its proof contract is the weakest of the four: a host call returns a noun, the noun is a witness in the calling [[rune]] program, the surrounding pure computation is provable conditional on that witness.

the long-term arc is migration: WASM → [[trident]] LIR → [[nox]] (with [[zheng]] proof). wysm is the soft path; nox is the hard one. souls start in wysm and graduate to nox as their inner loops get lowered through trident.

## why a fork of wasmi

cyber does substantive work on the runtime itself — jet substitution dispatches below the executor, metering instruments at module-rewrite time, the harness needs internal hooks, and the trident lowering reads `wasmi_ir` 3.0's public types tightly. clean as an internal fork, ugly as a wrapping dependency.

upstream wasmi is small (~30k LOC), audited (SRLabs 2023, Runtime Verification 2024), and on a roughly-quarterly release cadence. the fork commits to merging upstream every release. divergence lives in **new** crates: `wysm-jet`, `wysm-meter`, `wysm-harness`, `wysm-trident`. upstream crates are touched only at hook points.

## related

- [[soft3]] — full stack table
- [[wasm]] — the WebAssembly format itself
- [[rune]] — calls into wysm via `~host:%wasm.module.fn.args`
- [[trident]] — consumer of `wysm-trident` for WASM → LIR lowering
- [[cyb]] — the embedding host; owns soul lifecycle
- [[nox]] — sibling runtime; jet design ancestry
- [[radio]] — source of soul particles loaded into the engine

---

discover all [[concepts]]
