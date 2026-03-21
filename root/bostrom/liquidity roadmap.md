---
tags: bostrom, cybernomics, operation
crystal-type: process
crystal-domain: cyber
status: active
focus: 0.000139030878015579
gravity: 1
---
# Liquidity Roadmap

design note from Mar 1 2026 recap. scope: fix [[bostrom]] liquidity via [[cosmwasm]] [[progs]], chain stays unchanged

## 1. Confirmed Problems

### Fix now (frontend/contracts, chain stays unchanged)

| Issue | Problem | Severity | Status |
|-------|---------|----------|--------|
| [#663](https://github.com/cybercongress/go-cyber/issues/663) | [[warp]] swap interface shows unpredictable prices/amounts (large amount overflow) | high | done — cyb [#1379](https://github.com/cyberia-to/cyb/pull/1379), [#1382](https://github.com/cyberia-to/cyb/pull/1382) |
| [#1380](https://github.com/cyberia-to/cyb/issues/1380) | swap rejects small amounts without user-friendly error | medium | done — cyb [#1382](https://github.com/cyberia-to/cyb/pull/1382) |
| [#1381](https://github.com/cyberia-to/cyb/issues/1381) | tx confirmation status bar hangs indefinitely | medium | done — cyb [#1382](https://github.com/cyberia-to/cyb/pull/1382) |
| [#803](https://github.com/cybercongress/go-cyber/issues/803) | Missing price/volume indexing | medium | open |

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

0. ~~Fix swap slippage display in [[teleport]]~~ done — cyb [#1379](https://github.com/cyberia-to/cyb/pull/1379) (cyb [#1196](https://github.com/cyberia-to/cyb/issues/1196), [#1195](https://github.com/cyberia-to/cyb/issues/1195))
1. ~~Fix [[IBC]] transfer status glitch~~ done — cyb `5a55413a` ([#1332](https://github.com/cyberia-to/cyb/issues/1332))
2. Fix [[teleport]] UI rendering (cyb [#1113](https://github.com/cyberia-to/cyb/issues/1113))
3. Fix [[warp]] deposit for edge-case pairs (cyb [#769](https://github.com/cyberia-to/cyb/issues/769))
4. Fix sub-liquidity UX when pool absent (cyb [#849](https://github.com/cyberia-to/cyb/issues/849))
5. Show routed [[$A]]/[[$V]] on /robot (cyb [#690](https://github.com/cyberia-to/cyb/issues/690))
6. Block dust withdrawal on frontend — validate minimum pool coin amount before tx (cyb [#1377](https://github.com/cyberia-to/cyb/issues/1377))
7. Restore [[IBC]] channel bostrom–[[space-pussy]] ([#804](https://github.com/cybercongress/go-cyber/issues/804))
8. Document all [[liquidity]] bugs with reproduction steps
9. ~~Block swap below chain minimum (100 base units)~~ done — cyb [#1382](https://github.com/cyberia-to/cyb/pull/1382) ([#1380](https://github.com/cyberia-to/cyb/issues/1380))
10. ~~Fix tx confirmation status bar timeout~~ done — cyb [#1382](https://github.com/cyberia-to/cyb/pull/1382) ([#1381](https://github.com/cyberia-to/cyb/issues/1381))
12. ~~Fix swap status bar not updating after tx confirmation~~ done — cyb [#1384](https://github.com/cyberia-to/cyb/pull/1384) ([#1383](https://github.com/cyberia-to/cyb/issues/1383))
11. ~~Display swap fee (0.3%) next to slippage~~ done — cyb [#1382](https://github.com/cyberia-to/cyb/pull/1382)
13. Amount > 10% pool reserves shows warning (cyb [#1382](https://github.com/cyberia-to/cyb/pull/1382))
14. ~~Auto-refresh destination balance after [[IBC]] transfer completes on [[teleport]]~~ done — cyb `343352fb` ([#1385](https://github.com/cyberia-to/cyb/issues/1385))
15. ~~Fix [[IBC]] denom hash — sha256 received string instead of Uint8Array → wrong denom → balance always 0~~ done — cyb `343352fb` ([#1386](https://github.com/cyberia-to/cyb/issues/1386))

### Phase 1: Contracts + features (weeks, dependencies between items)

0. Finalize [[hub]] contracts — tokens, states, bridges (cw-cyber [#38](https://github.com/cybercongress/cw-cyber/issues/38))
1. Deploy [[warp]] contract wrapper over x/liquidity
2. Deploy [[nebula]] — token discovery, market cap, volume analytics
3. CW-20/CW-721 integration (cw-cyber [#23](https://github.com/cybercongress/cw-cyber/issues/23)) → unblocks LP with factory tokens ([#30](https://github.com/cybercongress/cw-cyber/issues/30))
4. APR computation contract (cw-cyber [#39](https://github.com/cybercongress/cw-cyber/issues/39))
5. Implement [[value]] tab — sigma/super-sigma portfolio valuation, switchable denominations (cyb [#660](https://github.com/cyberia-to/cyb/issues/660))
6. Restaking automation (cw-cyber [#13](https://github.com/cybercongress/cw-cyber/issues/13))
7. Integrate [[Osmosis]] swap subset into [[teleport]] for seamless exchange
8. Stabilize [[Osmosis]] [[IBC]] send/bridge/swap

### Phase 2: Chain upgrade (hardening, months)

0. Fork mainnet state → testnet for upgrade testing
1. Fix 18-decimal token handling ([#801](https://github.com/cybercongress/go-cyber/issues/801))
2. Fix pool withdrawal overflow + dust rounding ([#802](https://github.com/cybercongress/go-cyber/issues/802))
3. Update token denoms ([#762](https://github.com/cybercongress/go-cyber/issues/762))
4. Fix x/liquidity `RegisterCustomTypeURL` codec — SDK fork exists for this, eliminate with type assertion on SDK v0.50+ (see [[go-cyber]] upgrade-plan.md)
5. Native indexing plugin (ABCIListener) — replaces [[cyberindex]] for price/volume data, unblocks [#803](https://github.com/cybercongress/go-cyber/issues/803)
6. Configurable denoms from genesis — prerequisite for [#762](https://github.com/cybercongress/go-cyber/issues/762) and [[space-pussy]] unification (upgrade-plan.md item 1.6)
7. Build [[cyber-maker]] for automated market making + data collection

x/liquidity module specification — done ([[go-cyber]] PR #800)

All items above ship as a single hardening upgrade bundled with SDK v0.50 migration. Testnet fork approach: snapshot mainnet state → apply upgraded code → validate → merge back. See [[go-cyber]] upgrade-plan.md for full SDK upgrade roadmap.

## Source References

- [[bostrom/liquidity]] — module documentation
- [[aqua style dex]] — draft BIP for future module replacement
- [[liquidity subsidy]] — draft CIP for senate-funded pool subsidies
- [[market making]] — Osmosis pool data
- [[bostrom/infrastructure/ibc]] — IBC channel status
- [[tokenfactory]] — native token registry module
- [[go-cyber]] upgrade-plan.md — SDK v0.50 migration and hardening roadmap