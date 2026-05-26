---
tags: cyber, cyb, article
crystal-type: entity
crystal-domain: cyber
---
# security audit: private key import

date: 2026-05-12. status: passed. part of [[bostrom/liquidity roadmap]].

scope: raw [[secp256k1]] [[private key]] import into [[cyb]] wallet. 6 files, 331 lines changed.

## result

0 critical. 0 high. 0 medium. 1 low (optional defensive null-check).

## threat model

| threat | mitigation | status |
|---|---|---|
| key in React state | `useRef` | fixed |
| leak on unmount | ref cleared in cleanup | fixed |
| leak on background | cleared on `visibilitychange` | fixed |
| clipboard retention | cleared after paste | fixed |
| refs retained after success | zeroed in `clearState()` | fixed |
| invalid key stored | 3-layer: regex → `fromHex` → `fromKey` | secure |
| key in error messages | generic text only | secure |
| encryption at rest | [[AES-256-GCM]] + [[PBKDF2]] 1M iterations | secure |
| password strength | 8+ chars, 3/4 classes under 12 | adequate |
| key type in Redux | no key material | accepted |
| Tauri device key in localStorage | pre-existing | accepted |
| auto-lock disabled | pre-existing | accepted |

## encryption

```
version(1) + salt(16) + iv(12) + AES-GCM-256(plaintext) → base64
```

same format as [[mnemonic]]. `keys` field in Redux routes to correct signer on decrypt.

## validation

`fromHex` → `DirectSecp256k1Wallet.fromKey` → `Secp256k1Wallet.fromKey`. all throw before storage.

## fixes applied

1. `useState` → `useRef` for key material
2. cleanup on unmount, background, success
3. clipboard cleared on paste
4. refs reset in `clearState()`
