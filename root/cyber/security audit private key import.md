---
tags: cyber, cyb, article
crystal-type: entity
crystal-domain: cyber
---
# security audit: private key import

date: 2026-05-12
status: passed — 0 critical, 0 high, 0 medium, 1 low (optional)
scope: addition of raw [[secp256k1]] [[private key]] import to [[cyb]] wallet
part of: [[bostrom/liquidity roadmap]] — wallet and key management improvements

## changes audited

| file | change |
|---|---|
| `defaultAccount.d.ts` | added `private-key` to keys union type |
| `offlineSigner.ts` | `CybPrivateKeySigner` class, `getOfflineSignerFromPrivateKey()` |
| `ConnectWalletModal.tsx` | UI tabs, private key input field |
| `actionBarConnect.tsx` | import routing, encryption, account registration |
| `signerClient.tsx` | unlock and auto-switch for private-key accounts |
| `pocket.ts` | deletion cleanup for private-key accounts |

## threat model

| threat | mitigation | status |
|---|---|---|
| private key in React state | stored in `useRef` | fixed |
| key leak on unmount | ref cleared in cleanup effect | fixed |
| key leak on background | ref + display cleared on `visibilitychange` | fixed |
| clipboard leak on paste | `navigator.clipboard.writeText('')` after paste | fixed |
| pending ref retained after success | `clearState()` zeroes all refs | fixed |
| invalid key accepted | 3-layer validation: regex, `fromHex()`, `fromKey()` | secure |
| error messages expose key material | generic errors only | secure |
| encryption at rest | [[AES-256-GCM]] + [[PBKDF2]] (1M iterations), same as [[mnemonic]] | secure |
| password brute force | 8+ chars, 3/4 character classes if under 12 chars | adequate |
| key type disclosure in Redux | `keys: 'private-key'` visible, contains no key material | accepted |
| Tauri device key in localStorage | pre-existing trade-off | accepted |
| auto-lock timer disabled | pre-existing design decision | accepted |

## encryption format

private key hex encrypted with identical format as mnemonic:

```
version(1 byte) + salt(16 bytes) + iv(12 bytes) + AES-GCM-256(plaintext)
→ base64 → localStorage['cyb:mnemonic:{address}']
```

`decryptMnemonic()` returns any stored plaintext. the account type (`keys` field in Redux) determines whether to call `getOfflineSignerFromMnemonic()` or `getOfflineSignerFromPrivateKey()`.

## [[CosmJS]] validation chain

1. `fromHex(privkeyHex)` — validates hex format, throws on non-hex or odd-length
2. `DirectSecp256k1Wallet.fromKey(privkey)` — validates 32-byte length, validates against [[secp256k1]] curve order
3. `Secp256k1Wallet.fromKey(privkey)` — same validation for Amino signer

all three layers throw before any storage occurs.

## signArbitrary (ADR-036)

`CybPrivateKeySigner.signArbitrary()` composes `Secp256k1Wallet` (Amino) for ADR-036 signing — same MsgSignData format as `CybOfflineSigner`. `hasSignArbitrary()` type guard works via duck-typing.

## findings fixed before commit

1. moved `privateKeyHex` from `useState` to `useRef` — prevents React DevTools exposure
2. added ref cleanup on unmount
3. added ref + display cleanup on `visibilitychange` (background)
4. added clipboard clearing on private key paste
5. added `pendingImportModeRef` reset in `clearState()`

## accepted risks

- device key for Tauri auto-unlock stored in localStorage (pre-existing)
- auto-lock timer disabled (pre-existing design decision)
- account type visible in Redux state (information disclosure only)
