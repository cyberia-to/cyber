---
tags: cyber, bostrom
icon: "\U0001F4DC"
crystal-type: entity
crystal-domain: cyber
alias: cw-cyber
---
[[cosmwasm]] smart contracts for [[bostrom]] — the bootloader chain

mono-repo workspace: shared [[rs]] packages + individual contracts compiled to .wasm

contracts: [[cw-cyber/contracts/cybernet]], [[cw-cyber/contracts/hub-channels]], [[cw-cyber/contracts/hub-tokens]], [[cw-cyber/contracts/litium-core]], [[cw-cyber/contracts/cw-cyber-passport]], [[cw-cyber/contracts/cw-cyber-gift]]

packages: [[cw-cyber/packages/cyber-std]] (Cyber-specific bindings), [[cw-cyber/packages/cyber-std-test]]

dependencies: [[daodao]], [[neutron-sdk]], [[neutron-dex]]

runtime: Cosmos SDK v0.47, wasmd v0.46, wasmvm v1.5.9

github.com/cyberia-to/cw-cyber
