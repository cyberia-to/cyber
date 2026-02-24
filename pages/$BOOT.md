---
tags: cybernomics
alias: BOOT
crystal-type: entity
crystal-domain: economics
---
[[consensus]] [[token]] of [[bostrom]]

denom: `boot`

[[$BOOT]] is the base layer. It does not grant direct access to network services — its role is to secure [[bostrom/consensus]], enable [[governance]], and anchor the value of everything built on top.

### Supply

Total [[$BOOT]] supply: ~480 trillion (480T). [[inflation]] mints new [[$BOOT]] each block and distributes it to [[heroes]] and delegators proportionally to their stake.

| Parameter | Value |
|---|---|
| Current [[inflation]] rate | ~1.09% annually |
| Minimum [[inflation]] | 1.09% |
| Maximum [[inflation]] | 5.46% |
| Target bonded ratio | 25.49% |
| Max [[heroes]] (validators) | 92 |
| Unbonding period | 8 days (691,200 s) |

When the bonded ratio is below target, [[inflation]] increases toward the maximum to incentivize [[staking]]. When above target, it decreases toward the minimum. All parameters are adjustable by [[governance]].


## Staking

[[$BOOT]] holders delegate to [[heroes]] via standard [[cosmos-sdk]] DPoS. [[heroes]] and delegators earn [[inflation]]-based [[$BOOT]] rewards through [[delegation rewards]]. The [[bostrom]] staking module is a custom fork of the [[cosmos-sdk]] staking module.

[[delegation]] of [[$BOOT]] simultaneously creates [[$H]] in the delegator's account at 1:1. Undelegation destroys the corresponding [[$H]]. This is handled natively in the cyberbank module via [[delegation]] hooks — no separate liquid [[staking]] protocol is needed.

## Governance

On-chain [[basic governance]] uses [[$BOOT]]-weighted voting across four proposal types:

- ideas — non-binding signals and directional proposals
- upgrades — binary software upgrade proposals
- parameters — all protocol parameters are adjustable by [[governance]] within validated bounds
- fund — disbursements from the community pool

## Smart Contract Execution Fees

[[cosmwasm]] smart contracts on [[bostrom]] pay execution fees in [[$BOOT]]. The fee distribution is split 80/20: 80% returns directly to the program creator, 20% goes to [[heroes]] and the community pool through standard distribution. This is hardcoded in the dmn module and creates a native revenue model for [[autonomous progs]].

[[bostrom/tokenomics]]
