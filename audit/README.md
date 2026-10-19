---
title: cyber audits
tags: cyber, audit
crystal-type: entity
crystal-domain: cyber
alias: cyber audit reports
---

# cyber audits

Observed behavior, implementation reviews, measurements and validation evidence
live here. Reports retain the scope and date of their checks. Product contracts
live in [specs](../specs/README.md).

| report | evidence |
|---|---|
| [TOCYB allocations and the CYB genesis, 2026-10-19](genesis/tocyb-allocations-2026-10-19.md) | the 2021 allocation (gift 70 %, congress 11.6 %, epizode zero 8.3 %, senate 5.1 %, great web foundation 5 %) traced to the halt and to CYB: 97 T TOCYB left on the gift multisig after the v6 burn, the GWF moved to the congress key, 52.5 % dormant at the cyber genesis; IBC escrow and Osmosis holders |
| [release automation, 2026-09-23](release-2026-09-23/README.md) | three native CI platforms, mandatory soft3 inventories, publication guard failures and downloaded archive qualification |
| [cryptographic assumptions, 2026-10-08](cryptography-2026-10-08.md) | mudra and the stack against the hash-vs-structure debate; Hemera is the permanent-path risk, foculus VDF broken, ranked pre-launch fixes |
| [game-theoretic foundations, 2026-10-02](game-theory.md) | state→credit→honesty triad (Banach/PF · Shapley · BTS+SP) + the work/stake hybrid as fourth pillar; the seam finding; six missing results to apply; cross-doc drift |
| [native storage simplification, 2026-09-23](native-layout-2026-09-23.md) | existing layout restored; clean build and successful writes, retries and restart through both qualified binaries |
| [withdrawn format-2 experiment, 2026-09-23](node-storage-compatibility-2026-09-23.md) | historical evidence; artificial version promotion and reader exclusion removed |
| [node dependencies, 2026-09-23](node-dependencies-2026-09-23.md) | committed sources fetched from GitHub, source-lock rejection tests and clean release-binary acceptance |
| [launch review, 2026-09-23](launch-review-2026-09-23.md) | nine merged PRs, rejected regressions, current local release acceptance and remaining node gates |
| [node readiness](node-readiness.md) | release binary, normal replay and failure probes; [raw results](node-readiness.json), [build provenance](node-readiness-build.json) |
| [component implementation](component-implementation.md) | prior work-package status, code map and test commands |
| [rewards completeness](rewards-completeness.md) | 2026-08-11 reward pipeline report |
| [full-flow claims](full-flow-claims.md) | 2026-08-11 library claims and validation results |

Moving the historical reports here preserves their original claims; their
figures and conclusions have not been revalidated by the directory cleanup.

discover all [[concepts]]
