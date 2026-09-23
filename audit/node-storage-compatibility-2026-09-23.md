---
title: Native storage compatibility qualification
tags: cyber, audit, storage, release
crystal-type: entity
crystal-domain: cyber
date: 2026-09-23
status: withdrawn experiment
---
# Native storage compatibility qualification

Withdrawn following the owner's correction on 2026-09-23. The pre-production
node retains its existing native layout. The format-2 bump, automatic promotion
and older-writer exclusion added needless complexity with unchanged root
semantics; they have been removed. The executable compatibility check now
requires successful writes and restart through both binaries. The text below
preserves the withdrawn experiment's evidence. Current qualification is recorded
in [native storage simplification](native-layout-2026-09-23.md).

The previous source-pinned Cyber binary and the format-2 candidate were tested
against empty and populated temporary homes. Existing compatible state and
receipts survive the upgrade. Opening and exact retry preserve version 1; the
first new operation atomically promotes metadata to version 2. Earlier writers
then refuse the store, and the new writer reopens and returns stable receipts.

## Change and ownership

BBG's native metadata used version 1 across commitment changes. Its new version
2 identifies the current record/commitment contract and provides framing/version
validation. Cybergraph checks that version before replay. Legacy acceptance
requires all historical roots, receipts and exact records to match, with solely
the metadata version word allowed to differ. Genesis/profile/instance derivation
and root algorithms remain unchanged.

Promotion is part of the accepted operation's transaction, including operations
that leave the root unchanged. Failure injection on Fjall and redb verifies
rollback of the version, history, receipt and working state. Cached metadata is
checked in the write transaction so a stale coordinator cannot overwrite an
external format or pruning-policy change.

Unknown formats and incompatible legacy state are refused while preserving
logical records. This qualifies the previous pinned binary below; it does not
automatically convert every historical version-1 root. Such a conversion would
change committed history and needs its own explicit protocol.

## Executed checks

| scope | result |
|---|---|
| BBG all-feature suite | 166 passed, zero ignored |
| Cybergraph library, native format/storage/import, application and text archive tests | 58 passed; both backends exercised by the staged-promotion failure test |
| additional archive-boundary suite, release mode | 4 passed in 154.78 seconds, including the 63 MiB multi-transaction/concurrent-capacity case |
| product source checkout from GitHub | 15 dependency repositories and 38 local packages verified |
| product debug acceptance | 1 unit + 6 process tests passed |
| release binary acceptance | all 6 process tests passed |
| previous/current process qualification | empty and populated homes passed create/read/retry/write/old-reader refusal/restart scenarios |
| final source gate | all inputs clean and exactly pinned |

The unrestricted Cybergraph suite currently fails to compile its existing
proof experiments: `tests/common/mod.rs` refers to unpublished Zheng state
execution, and `stack_call` additionally requires private execution APIs. This
is a separate test/profile integration gap, not passing coverage. Those tests
were left visible. The duplicate long-running debug archive group was stopped
after the complete optimized group passed; that interrupted command is not
reported as a completed debug suite. Three existing vendored Fjall warnings remain.

## Exact binaries and repetition

| input | identity |
|---|---|
| predecessor product source | `87f8359e1ab64e14e23d5ba434089c2313e93ad1` |
| predecessor artifact SHA-256 | `5cb3b61388d2232738a56187f01e7846af59a582a1939971cd2958af936e3181` |
| candidate product source | `43cb5947` |
| candidate BBG / Cybergraph | `d4b812f` / `cf0a5106` |
| candidate artifact SHA-256 | `67db0ad193f7e12df13731c8d3ec87e099ccfb2249c701048f703db19c33567b` |
| platform | macOS arm64, Rust 1.95.0 |

Retained candidate checkout:
`/Users/master/cyber/.node-builds/20260923-format2`. Its `cyber/dist/build.json`
records `sources_locked: true` and the predecessor's hash in
`compatibility_previous_sha256`. The binary and SHA256SUMS are beside it.

From an independently assembled source checkout:

```nu
nu scripts/release.nu --locked-sources --previous-binary /path/to/previous/cyber
```

The optional predecessor check uses disposable homes and leaves operator data
alone. The [CLI contract](../specs/cli.md) describes the first-write rollback
boundary. [Cybergraph evidence](../../cybergraph/audit/native-format-2026-09-23.md)
and [BBG evidence](../../bbg/audit/native-format-2026-09-23.md) record component
contracts and tests. Source changes are tracked in
[BBG PR 22](https://github.com/cyberia-to/bbg/pull/22) and
[Cybergraph PR 8](https://github.com/cyberia-to/cybergraph/pull/8).
