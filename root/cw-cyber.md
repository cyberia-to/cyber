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
diffusion: 0.00023180658033276647
springs: 0.00020458021895444965
heat: 0.0002349584776041494
focus: 0.00022426905137354914
gravity: 5
density: 4.53
---
[[cosmwasm]] smart contracts for [[bostrom]] — the bootloader chain

mono-repo workspace: shared [[rs]] packages + individual contracts compiled to .wasm

contracts: [[cw-cyber/contracts/cybernet]], [[cw-cyber/contracts/hub-channels]], [[cw-cyber/contracts/hub-tokens]], [[cw-cyber/contracts/litium-core]], [[cw-cyber/contracts/cw-cyber-passport]], [[cw-cyber/contracts/cw-cyber-gift]]

packages: [[cw-cyber/packages/cyber-std]] (Cyber-specific bindings), [[cw-cyber/packages/cyber-std-test]]

dependencies: [[daodao]], [[neutron-sdk]], [[neutron-dex]]

runtime: Cosmos SDK v0.47, wasmd v0.46, wasmvm v1.5.9

github.com/cyberia-to/cw-cyber