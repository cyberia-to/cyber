---
tags: cyber, bostrom
icon: "\U0001F4DC"
crystal-type: entity
crystal-domain: cyber
alias: cw-cyber
subgraph: true
repo: ../cw-cyber
exclude: ".claude/**, target/**, artifacts/**, vendor/**"
stake: 14176898798954938
diffusion: 0.00023337565778954285
springs: 0.0001756069176030584
heat: 0.00021393010501395527
focus: 0.00021215592517847726
gravity: 6
density: 4.04
---
[[cosmwasm]] smart contracts for [[bostrom]] — the bootloader chain

mono-repo workspace: shared [[rs]] packages + individual contracts compiled to .wasm

contracts: [[cw-cyber/contracts/cybernet]], [[cw-cyber/contracts/hub-channels]], [[cw-cyber/contracts/hub-tokens]], [[cw-cyber/contracts/litium-core]], [[cw-cyber/contracts/cw-cyber-passport]], [[cw-cyber/contracts/cw-cyber-gift]]

packages: [[cw-cyber/packages/cyber-std]] (Cyber-specific bindings), [[cw-cyber/packages/cyber-std-test]]

dependencies: [[daodao]], [[neutron-sdk]], [[neutron-dex]]

runtime: Cosmos SDK v0.47, wasmd v0.46, wasmvm v1.5.9

github.com/cyberia-to/cw-cyber