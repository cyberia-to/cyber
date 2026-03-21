---
tags: bostrom, cybernomics, article
alias: bostrom tokenomics, bostrom token model
crystal-type: article
crystal-domain: economics
stake: 4994622989397658
diffusion: 0.0002487806240017912
springs: 0.0012315558319380884
heat: 0.0009171820299469175
focus: 0.0006772934675717089
gravity: 7
density: 5.21
---
# Bostrom Tokenomics

## The Four Tokens

[[bostrom]] separates four economic functions that most [[blockchains]] compress into a single [[token]]:

| Token | Role | Issuance |
|---|---|---|
| [[$BOOT]] | [[bostrom/security]] and [[governance]] | [[inflation]] (~1.09% annually) |
| [[$H]] | liquid representation of [[bostrom/staking]] | [[mint]] 1:1 on [[$BOOT]] [[bostrom/staking]]<br>[[burn]] 1:1 on unstaking |
| [[$V]] | write access to the [[knowledge graph]] | [[burn]] of [[$H]] via [[bostrom/mint]] |
| [[$A]] | [[relevance machine]] focus influence | [[burn]] of [[$H]] via [[bostrom/mint]] |

Every token derives from the one above it. [[$H]] requires staked [[$BOOT]]. [[$V]] and [[$A]] require burned [[$H]]. Every unit of network resource has a provable, on-chain opportunity cost denominated in committed stake.

## Why Tokens Grow

- Supply decay is irreversible — every [[bostrom/mint]] makes the next one more expensive, embedded in protocol math
- The graph gets more valuable — more [[cyberlinks]] attract more [[neurons]], demand grows while supply gets scarcer
- Writing is scarcer than reading — [[$V]] gets expensive 8x faster than [[$A]]
- Speculation feeds the machine — 2% [[burn fee on moving A and V]] permanently destroys supply on every transfer
- Everything costs stake — [[$V]] and [[$A]] require [[burn]] of [[$H]] which requires [[bostrom/staking]] [[$BOOT]], spam is economically impossible

## The Learning Loop

The [[knowledge graph]] learns through economic commitments. Every token operation is part of a cycle that makes the graph more valuable over time.

```
BOOT --stake--> H --mint(burn H)--> V + A
  ^                                  |   |
  |                          cyberlinks  focus weight
  |                          (burn V)    |
  |                              v       v
  |                        knowledge graph <-- diffusion (GPU)
  |                              |
  |                          cyberank
  |                              |
  +--- 80% exec fees <-- autonomous programs <-- energy routes (V/A)
```

1. [[neuron]] stakes [[$BOOT]] → receives [[$H]] → burns [[$H]] → receives [[$V]] or [[$A]]
2. [[neuron]] spends [[$V]] to create [[cyberlinks]] — each link is a [[costly signal]], an economic commitment that two [[particles]] are related
3. [[diffusion]] computes [[focus]] distribution across all [[particles]] on GPU, weighted by [[$A]] balances
4. [[cyberank]] measures particle quality — emerges from the graph structure without external votes

The more [[neurons]] link, the better [[cyberank]] gets, the more valuable [[$V]] and [[$A]] become.

## Energy Mint

A [[neuron]] burns [[$H]] through [[bostrom/mint]] to create [[$V]] or [[$A]]. The cost follows a supply decay curve: every [[bostrom/mint]] makes the next one more expensive. Details and formulas: [[bostrom/mint]].

## Fees

- [[burn fee on moving A and V]] — 2% burn on every [[$V]] and [[$A]] transfer. Speculators pay a tax that permanently reduces supply.
- [[collect fee on moving A and V]] — 1% fee on [[$V]] and [[$A]] transfers directed into reward pools for [[staking on particles]] and [[staking on cyberlinks]].
- [x/liquidity](https://github.com/cyberia-to/go-cyber/tree/main/x/liquidity) — 0.3% swap fee (retained in pool reserves), 40M [[$BOOT]] pool creation fee (community pool).

## Energy Grid

The grid module routes [[$V]] and [[$A]] to [[cosmwasm]] programs via energy routes. Programs that receive routed [[$V]] can create [[cyberlinks]] — enabling autonomous [[knowledge graph]] expansion. [[cosmwasm]] execution fees return 80% to the program creator, creating a reinvestment loop back into [[bostrom/staking]] → [[$H]] → [[$V]]/[[$A]].

## Source References

- [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) — [[bostrom/mint]] logic, supply decay
- [x/cyberbank](https://github.com/cyberia-to/go-cyber/tree/main/x/cyberbank) — [[$H]] [[mint]]/[[burn]], 2% transfer burn
- [x/rank](https://github.com/cyberia-to/go-cyber/tree/main/x/rank) — [[diffusion]] (GPU/CUDA)
- [x/grid](https://github.com/cyberia-to/go-cyber/tree/main/x/grid) — energy routing
- [x/graph](https://github.com/cyberia-to/go-cyber/tree/main/x/graph) — [[cyberlinks]], [[$V]] and [[$A]] tracking