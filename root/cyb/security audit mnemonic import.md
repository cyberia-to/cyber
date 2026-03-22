---
tags: cyb, operation, article
crystal-type: process
crystal-domain: cyber
date: 2026-03-20
scope: mnemonic import, wallet encryption, Keplr removal, Ledger integration, error messages
auditor: Claude Opus 4.6
audits: 30
cross-verifications: 5
---
# security audit: mnemonic import

30 audits + 5 cross-verifications of [[cyb]] wallet security. scope: [[mnemonic]] import/unlock, [[Keplr]] removal, [[Ledger]] hardware integration, error message sanitization.

surface: `D` = desktop, `M` = mobile, `DM` = both

## encryption

| parameter | value |
|-----------|-------|
| algorithm | AES-256-GCM (authenticated) |
| KDF | PBKDF2-SHA256, 1,000,000 iterations |
| salt | 16 random bytes per encryption |
| IV | 12 random bytes per encryption |
| format | `v2: 0x02 ∥ salt(16) ∥ iv(12) ∥ ciphertext ∥ tag(16)` |
| backward compat | legacy v1 (600k iterations, no version byte) auto-detected |
| storage | per-address key only: `cyb:mnemonic:{bech32}` |
| password | 12+ chars accepted; 8–11 chars require 3/4 character classes |

## mnemonic lifecycle

```
import:     words → password (2x) → PBKDF2 → AES-GCM encrypt → localStorage
unlock:     password → decrypt → address verify → mnemonicRef (useRef) → activateWalletSigner
signing:    getSignerForChain(chainId) → mnemonicRef → BIP44 derivation → signer
IBC:        getSignerForChain(ibcChainId) → prefix from networkListIbc → signer
auto-lock:  15 min idle ∥ tab hidden ∥ unmount → mnemonicRef = null → __cyb_wallet_locked event
deletion:   removeEncryptedMnemonic(bech32) → localStorage cleanup
```

## findings

### CRITICAL (3 total, 3 fixed)

| audit | surface | finding | fix |
|-------|---------|---------|-----|
| 1 | DM | plaintext mnemonic in localStorage | AES-256-GCM encryption via [[Web Crypto API]] |
| 1 | DM | mnemonic persisted in React state after modal close | useEffect cleanup on unmount, clearState() in parent |
| 13 | DM | Rune VM `compile()` receives secrets (API keys) — any user script exfiltrates credentials | destructure secrets before compile: `{ secrets: _s, ...safeContext }` |

### HIGH (14 total, 14 fixed)

| audit | surface | finding | fix |
|-------|---------|---------|-----|
| 1 | DM | signingClient stale after setSigner | useEffect on [signer] auto-rebuilds client |
| 1 | DM | unlock flow absent — decryptMnemonic imported by zero components | unlockWallet() + UnlockWalletBar component |
| 3 | DM | CosmJS validation details leaked in error messages | generic "Invalid seed phrase" |
| 3 | DM | pendingMnemonic persisted on unexpected unmount | useEffect cleanup calls clearState() |
| 4 | DM | Keplr fallback triggered for wallet accounts after auto-lock | `!isWalletAccount` guard in getSignerForChain |
| 4 | DM | TypeError on missing network prefix in networkListIbc | null-safe `?.prefix` + bostrom fallback |
| 8 | DM | getSignClientByChainId bypassed Keplr isolation | mnemonicRef path for wallet, Keplr only for keplr-type |
| 10 | DM | pendingMnemonic stays in state on encryption error | clear mnemonic + passwords in catch block |
| 12 | DM | portal components crash — `signer.keplr.getKey()` undefined for wallet | getSignerKeyInfo helper abstracts Keplr vs mnemonic |
| 12 | DM | signArbitrary re-derived amino wallet every call | cached via getAminoWallet() lazy init |
| 17 | DM | mnemonicRef never set during initial import — IBC signing fails | activateWalletSigner() sets mnemonicRef + signer atomically |
| 22 | D | Ledger signing fails silently — stale WebUSB transport | ReconnectingLedgerSigner + 30s health monitoring |
| 26 | D | APDU collision — health check corrupts active signing | _signingInProgress mutex flag |
| 29 | DM | IBC bridge withdraw crash — MsgTransfer built as plain object | MsgTransfer.fromPartial() for amino-safe defaults |

### MEDIUM (31 total, 19 fixed, 12 noted)

| audit | surface | finding | fix |
|-------|---------|---------|-----|
| 1 | DM | modal missing Escape/backdrop/ARIA | keydown Escape, backdrop onClick, role="dialog", aria-modal |
| 5 | DM | useSetupIbcClient crash on unknown network | `if (!networkConfig) return` guard |
| 7 | DM | auto-lock on tab hidden absent | visibilitychange listener clears mnemonicRef immediately |
| 10 | DM | double-submit on "Encrypt & Save" | saving state + disabled={saving} |
| 10 | M | spellCheck sends mnemonic words to Google | spellCheck={false} on MnemonicInput |
| 11 | DM | post-decrypt address mismatch silent | account.address !== address check after decrypt |
| 11 | DM | clipboard retains mnemonic after paste | navigator.clipboard.writeText('') after paste |
| 13 | DM | PBKDF2 600k iterations below 2026 best practice | upgraded to 1,000,000 iterations + version byte |
| 15 | DM | blanket eslint-disable | replaced with targeted @typescript-eslint/no-explicit-any |
| 15 | DM | `as any` casts for signArbitrary and signer detection | hasSignArbitrary() type guard, typed Record narrowing |
| 15 | D | IE clipboardData fallback (dead code) | removed |
| 15 | DM | global `cyb:mnemonic` localStorage key stale with multiple wallets | removed — per-address key only |
| 16 | DM | encrypted mnemonic persists after account deletion | removeEncryptedMnemonic(bech32) on wallet account delete |
| 23 | DM | raw blockchain errors shown to users (20 files) | friendlyErrorMessage() centralized parser in errorMessages.ts |
| 25 | DM | ContainerGradient.TxsStatus renders raw rawLog | wrapped in friendlyErrorMessage |
| 27 | D | Ledger "Data is invalid" — old library omits HRP in APDU | replaced ledger-cosmos-js@2 with @zondax/ledger-cosmos-js@4 |
| 28 | D | sign doc overflow with 41 validators (~10 KB) | batch claiming: 5 validators per tx, poll confirmation between |
| 29 | DM | regex global flag — validation fails every other call | removed `g` flag from all 12 patterns |
| 30 | D | health check CLA 0xe0 causes unnecessary transport recreation | changed to CLA 0x55 (Cosmos getVersion) |

noted (12): setSigner public context (DM), `as any` in CosmJS constructor (DM), weak password below 12 chars (DM), signer retains seed — CosmJS requirement (DM), dual signer memory — JS immutable strings (DM), unguarded JSON.parse in 5 Redux slices (DM), checkAddressNetwork unbounded recursion — fixed in #24 (DM), Ledger reconnect error leaks addresses (D), amino-only Ledger in relayer (D), secrets unencrypted in localStorage — pre-existing (DM), seed word inputs type="text" — by design (M), no beforeunload cleanup — fixed in #24 (D)

### LOW (47 total, 18 fixed, 26 noted)

fixed: modal tabIndex (DM), error message internals (DM), version detection false positive — try-fallback (DM), persist-before-signer ordering (DM), Array.from instead of spread — stack safety (DM), DOMException-only catch (DM), autoComplete/autoCorrect/autoCapitalize on mnemonic inputs (M), autoComplete="new-password" on password inputs (M), __cyb_wallet_locked event name — internal (DM), `wallet` label for non-Keplr signers (DM), typed Dropdown callback (DM), getDebug() secrets stripped (DM), blob versioning for migration (DM), removeEncryptedMnemonic for deletion (DM), neuron errors wrapped in friendlyErrorMessage (DM), wasm action bar errors wrapped (DM), console.log sign doc removed (D), regex patterns fixed (DM)

noted (26): focus trap absent (DM), mnemonic inputs visible — by design (M), eslint-disable scope (DM), gasPrice hardcoded — standard (DM), JS memory mnemonic immutable (DM), unlockWallet concurrency — UI guard sufficient (DM), CustomEvent spoofable — cosmetic only (DM), stack overflow safe for mnemonic sizes (DM), packed length — AES-GCM validates (DM), chainId ignored in signArbitrary — ADR-036 by design (DM), HD path locked to index 0 (D), idle timer race (D), no initial Ledger address verification (D), raw Ledger errors (D), localStorage quota silent fail (DM), recursive setTimeout without cleanup (DM), window.open without noopener (D), forceQuitter legacy key (DM), Tendermint query interpolation (DM), password in useState — DevTools (D), mnemonic words in useState — React limitation (D), secrets in Redux DevTools (D), transaction response logged (D), no negative amount guard (DM), address validation lacks bech32 checksum (DM), error oracle — wrong password vs no mnemonic (DM)

## cumulative status

| severity | total | fixed | accepted | roadmap |
|----------|-------|-------|----------|---------|
| CRITICAL | 3 | 3 | 0 | 0 |
| HIGH | 14 | 14 | 0 | 0 |
| MEDIUM | 31 | 19 | 12 | 0 |
| LOW | 47 | 18 | 26 | 0 |

by surface:

| severity | desktop only | mobile only | both |
|----------|-------------|-------------|------|
| CRITICAL | 0 | 0 | 3 |
| HIGH | 2 | 0 | 12 |
| MEDIUM | 5 | 1 | 13 |
| LOW | 10 | 2 | 32 |

## Keplr isolation

| function | wallet account | Keplr account |
|----------|---------------|---------------|
| getSignerForChain | mnemonicRef → BIP44 signer | Keplr fallback |
| getSignClientByChainId | mnemonicRef → signer | Keplr |
| initSigner | skipped (keystorechange guard) | Keplr |
| keplr_keystorechange | skipped (isWalletAccount guard) | active |
| getSignerKeyInfo | signer.getAccounts() | signer.keplr.getKey() |
| signArbitrary | ADR-036 via CybOfflineSigner | Keplr native |

zero functional [[Keplr]] references remain outside migration logic.

## Ledger integration (desktop only)

three account types after [[Keplr]] removal:

| type | key storage | signing | security boundary | surface |
|------|------------|---------|-------------------|---------|
| wallet | encrypted mnemonic in localStorage | in browser (JS) | password + AES-256-GCM | DM |
| ledger | on [[Ledger]] device (never leaves) | on device | physical device | D |
| read-only | address only | none | N/A | DM |

ReconnectingLedgerSigner: fresh transport per sign, 30s health ping (skipped during signing), address verification on reconnect, adaptive HRP for firmware v2.34+, batch claiming (5 validators per tx).

## cross-verifications

| # | agent | result |
|---|-------|--------|
| 1 | Zed | REJECTED — 5/5 findings hallucinated (wrong line numbers, factually incorrect) |
| 2 | Grok | ACCEPTED — visibilitychange auto-lock implemented, PBKDF2 roadmapped → later fixed |
| 3 | independent | ACCEPTED — getSignClientByChainId Keplr bypass found and fixed |
| 4 | independent | ACCEPTED — pendingMnemonic catch, double-submit, spellCheck found and fixed |
| 5 | 3 parallel agents | ACCEPTED — post-decrypt address verification, clipboard clear found and fixed |

## files changed (18 core + 20 error message)

core: `mnemonicCrypto.ts`, `offlineSigner.ts`, `ledgerSigner.ts`, `errorMessages.ts`, `signerClient.tsx`, `actionBarConnect.tsx`, `ConnectWalletModal.tsx`, `MnemonicInput.tsx`, `Modal.tsx`, `actionBar/index.tsx`, `stageActionBar.tsx`, `pocket.ts`, `utils.ts`, `engine.ts`, `portal/utils.ts`, `ActionBarPortalGift.tsx`, `citizenship/index.tsx`, `ActionBarRelease.tsx`

error messages: 20 action bar + container files migrated to friendlyErrorMessage()

see [[security audit 29 ledger signing]] and [[security audit 30 fix verification]] for Ledger-specific audit details.
