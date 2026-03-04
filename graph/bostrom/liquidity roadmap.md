---
tags: bostrom, cybernomics, operation
crystal-type: process
crystal-domain: cyber
status: active
---
# Liquidity Roadmap

design note from Mar 1 2026 recap. scope: fix [[bostrom]] liquidity via [[cosmwasm]] [[progs]], chain stays unchanged

## 1. Confirmed Problems

### Fix now (frontend/contracts, chain stays unchanged)

| Issue | Problem | Severity | Fix Path |
|-------|---------|----------|----------|
| [#663](https://github.com/cybercongress/go-cyber/issues/663) | [[warp]] swap interface shows unpredictable prices/amounts (large amount overflow) | high | frontend fix + module spec audit |
| [#803](https://github.com/cybercongress/go-cyber/issues/803) | Missing price/volume indexing | medium | [[cyber-maker]] data collection or dedicated indexer |
| [#802](https://github.com/cybercongress/go-cyber/issues/802) | Token withdrawal stuck on some pools | high | investigate root cause first, fix depends on findings |

### Requires chain upgrade (hardening, later)

| Issue | Problem | Severity | Fix Path |
|-------|---------|----------|----------|
| [#801](https://github.com/cybercongress/go-cyber/issues/801) | 18-decimal ERC-style tokens malfunction in [[liquidity]] module pools | high | module-level arithmetic fix |

## 2. Architecture

| Layer | [[teleport]] | [[warp]] | [[nebula]] | [[hub]] |
|-------|----------|------|--------|-----|
| Frontend | send, bridge, swap | pools UI | token discovery, mcap | tokens, states, bridges |
| Contracts | — | wrapper over x/liquidity | market cap + volume | 3 [[cosmwasm]] registries |
| Chain | x/liquidity, [[IBC]], x/bank | x/liquidity | x/bank queries | [[tokenfactory]], [[IBC]] |

[[hub]] contracts: tokens registry (wraps [[tokenfactory]]), states (networks/chains with metadata), bridges ([[IBC]] connections between states)

Strategy: deploy CosmWasm proxy contracts → redirect users from module calls to contract calls → remove modules from core on future upgrade.

## 3. Delivery Plan

### Phase 0: Quick wins (days, frontend bugfixes)

0. Fix swap slippage display in [[teleport]] (cyb [#1196](https://github.com/cyberia-to/cyb/issues/1196), [#1195](https://github.com/cyberia-to/cyb/issues/1195))
1. Fix [[IBC]] transfer status glitch (cyb [#1332](https://github.com/cyberia-to/cyb/issues/1332))
2. Fix [[teleport]] UI rendering (cyb [#1113](https://github.com/cyberia-to/cyb/issues/1113))
3. Fix [[warp]] deposit for edge-case pairs (cyb [#769](https://github.com/cyberia-to/cyb/issues/769))
4. Fix sub-liquidity UX when pool absent (cyb [#849](https://github.com/cyberia-to/cyb/issues/849))
5. Show routed [[$A]]/[[$V]] on /robot (cyb [#690](https://github.com/cyberia-to/cyb/issues/690))
6. Investigate pool withdrawal bug — identify affected pools, root cause, classify fix phase ([#802](https://github.com/cybercongress/go-cyber/issues/802))
7. Stabilize [[Osmosis]] [[IBC]] send/bridge/swap
8. Restore [[IBC]] channel bostrom–[[space-pussy]] ([#804](https://github.com/cybercongress/go-cyber/issues/804))
9. Document all [[liquidity]] bugs with reproduction steps

### Phase 1: Contracts + features (weeks, dependencies between items)

0. Finalize [[hub]] contracts — tokens, states, bridges (cw-cyber [#38](https://github.com/cybercongress/cw-cyber/issues/38))
1. Deploy [[warp]] contract wrapper over x/liquidity
2. Deploy [[nebula]] — token discovery, market cap, volume analytics
3. CW-20/CW-721 integration (cw-cyber [#23](https://github.com/cybercongress/cw-cyber/issues/23)) → unblocks LP with factory tokens ([#30](https://github.com/cybercongress/cw-cyber/issues/30))
4. APR computation contract (cw-cyber [#39](https://github.com/cybercongress/cw-cyber/issues/39))
5. Implement [[value]] tab — sigma/super-sigma portfolio valuation, switchable denominations (cyb [#660](https://github.com/cyberia-to/cyb/issues/660))
6. Build [[cyber-maker]] for automated market making + data collection
7. Restaking automation (cw-cyber [#13](https://github.com/cybercongress/cw-cyber/issues/13))
8. Integrate [[Osmosis]] swap subset into [[teleport]] for seamless exchange

### Phase 2: Chain upgrade (hardening, months)

0. Fork mainnet state → testnet for upgrade testing
1. Fix 18-decimal token handling ([#801](https://github.com/cybercongress/go-cyber/issues/801))
2. Fix pool withdrawal bugs if module-level ([#802](https://github.com/cybercongress/go-cyber/issues/802))
3. Update token denoms ([#762](https://github.com/cybercongress/go-cyber/issues/762))
4. Burn 50% of H gas ([#660](https://github.com/cybercongress/go-cyber/issues/660))
5. Fix x/liquidity `RegisterCustomTypeURL` codec — SDK fork exists for this, eliminate with type assertion on SDK v0.50+ (see [[go-cyber]] upgrade-plan.md)
6. Native indexing plugin (ABCIListener) — replaces [[cyberindex]] for price/volume data, unblocks [#803](https://github.com/cybercongress/go-cyber/issues/803)
7. Configurable denoms from genesis — prerequisite for [#762](https://github.com/cybercongress/go-cyber/issues/762) and [[space-pussy]] unification (upgrade-plan.md item 1.6)

x/liquidity module specification — done ([[go-cyber]] PR #800)

All items above ship as a single hardening upgrade bundled with SDK v0.50 migration. Testnet fork approach: snapshot mainnet state → apply upgraded code → validate → merge back. See [[go-cyber]] upgrade-plan.md for full SDK upgrade roadmap.

## 4. Future Issues (post-hardening)

| Repo | Issue | Title | Depends On |
|------|-------|-------|------------|
| go-cyber | [#761](https://github.com/cybercongress/go-cyber/issues/761) | Migration to Neuron DEX | hardening first, then full module replacement |
| go-cyber | [#631](https://github.com/cybercongress/go-cyber/issues/631) | Investmint BYTE token | new resource token → future pools |
| go-cyber | [#632](https://github.com/cybercongress/go-cyber/issues/632) | GPU token | new resource token → future pools |
| go-cyber | [#671](https://github.com/cybercongress/go-cyber/issues/671) | Custom vesting for resources | locked V/A vs pooled, design after economy stabilizes |
| cyb | [#698](https://github.com/cyberia-to/cyb/issues/698) | move tokens/channels/networks to progs /hub | hub contracts integration |
| cyb | [#753](https://github.com/cyberia-to/cyb/issues/753) | /warp (fully featured dex) | full DEX experience |
| cyb | [#646](https://github.com/cyberia-to/cyb/issues/646) | /teleport | send + swap + memo unified interface |
| cyb | [#774](https://github.com/cyberia-to/cyb/issues/774) | /teleport buy BOOT | onramp via ATOM/OSMO swap |
| cyb | [#1313](https://github.com/cyberia-to/cyb/issues/1313) | Add APRs for DEX and chain | APR display for pools, staking, investminting |
| cyb | [#424](https://github.com/cyberia-to/cyb/issues/424) | liquidity rewards | interchain liquidity incentives |
| cyb | [#1021](https://github.com/cyberia-to/cyb/issues/1021) | /teleport/send network chooser | network selector for IBC sends |
| cyb | [#1020](https://github.com/cyberia-to/cyb/issues/1020) | search token in token chooser | token search in swap/send UI |
| cw-cyber | [#25](https://github.com/cybercongress/cw-cyber/issues/25) | unified farming | CW-20 integration (#23) first |
| cw-cyber | [#19](https://github.com/cybercongress/cw-cyber/issues/19) | prediction markets with AMM | separate AMM design, post-DEX stabilization |
| cw-cyber | [#36](https://github.com/cybercongress/cw-cyber/issues/36) | Add team registry | hub extension, after core registries ship |
| cw-cyber | [#40](https://github.com/cybercongress/cw-cyber/issues/40) | Contract for followers | social layer |
| cw-cyber | [#29](https://github.com/cybercongress/cw-cyber/issues/29) | contracts for events | idea stage |
| cw-cyber | [#20](https://github.com/cybercongress/cw-cyber/issues/20) | booster | neurons/cyberlinks/particles boosting |
| cw-cyber | [#32](https://github.com/cybercongress/cw-cyber/issues/32) | deploy social system | social layer |
| cw-cyber | [#16](https://github.com/cybercongress/cw-cyber/issues/16) | pow gpu token mining | GPU token mining contract |
| cw-cyber | [#24](https://github.com/cybercongress/cw-cyber/issues/24) | zk-nft | research |
| cw-cyber | [#15](https://github.com/cybercongress/cw-cyber/issues/15) | BLS-12-381 cosmwasm vm native | cryptographic primitive, research |
| cw-cyber | [#17](https://github.com/cybercongress/cw-cyber/issues/17) | ethereum light client | bridge infra, long-term |
| cw-cyber | [#18](https://github.com/cybercongress/cw-cyber/issues/18) | bitcoin light client | bridge infra, long-term |
| cw-cyber | [#21](https://github.com/cybercongress/cw-cyber/issues/21) | dao contracts | governance layer |

## Source References

- [[bostrom/liquidity]] — module documentation
- [[aqua style dex]] — draft BIP for future module replacement
- [[liquidity subsidy]] — draft CIP for senate-funded pool subsidies
- [[market making]] — Osmosis pool data
- [[bostrom/infrastructure/ibc]] — IBC channel status
- [[tokenfactory]] — native token registry module
- [[go-cyber]] upgrade-plan.md — SDK v0.50 migration and hardening roadmap
