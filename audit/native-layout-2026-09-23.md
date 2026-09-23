---
title: Native storage simplification
tags: cyber, audit, storage, release
crystal-type: entity
crystal-domain: cyber
date: 2026-09-23
---
# Native storage simplification

The pre-production node retains its existing native layout. The format-2
promotion, legacy normalization and older-reader exclusion have been removed.
Exact replay continues to validate committed records, roots and receipts.
Production migration policy can be specified when the protocol is frozen.
The earlier [format experiment](node-storage-compatibility-2026-09-23.md)
is withdrawn; its historical evidence remains available.

## Executed checks

| scope | result |
|---|---|
| BBG native transitions | 13 passed; production source and native-state spec match the previously qualified `1467b68` candidate |
| Cybergraph library and native storage | 24 + 10 passed; production source and tests match `0eae7ae` |
| clean checkout from GitHub | 15 dependency repositories and 38 local packages verified |
| product debug / release acceptance | 1 unit + 6 process tests / 6 process tests passed |
| two-binary compatibility | empty and populated temporary homes passed read, exact retry, writes by both executables, rollback and restart; state, receipts and configuration preserved |
| final source check | all inputs clean and pinned |

Compatibility was tested against the last qualified binary before the withdrawn
format experiment. It covers this concrete binary pair. The format-2 experiment
has no conversion path in this build. Three existing vendored Fjall warnings
remain. Broader protocol and production-readiness gates remain in the roadmap.

## Exact artifact

| input | identity |
|---|---|
| product source | `847bb7fe1d416eb35d76c04cadd64fade48b610f` |
| BBG source | `74065b8be92e35575131b4492adec0a3516cb00d` |
| Cybergraph source | `b5b13fc5bc42d8ec54d6c13e5397dda9b12be1a4` |
| predecessor SHA-256 | `5cb3b61388d2232738a56187f01e7846af59a582a1939971cd2958af936e3181` |
| candidate SHA-256 | `f916c45c28e13503b29a2ee001a5662a41e52724ad185d0bb884cf64dfce91ab` |
| platform | macOS arm64, Rust 1.95.0 |

Reproduce through the [source checkout and release workflow](../specs/node-sources.md),
passing the predecessor to `scripts/release.nu --locked-sources --previous-binary`.
The local artifact and its complete source provenance are in `dist/cyber` and
`dist/build.json`.
