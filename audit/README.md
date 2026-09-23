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
