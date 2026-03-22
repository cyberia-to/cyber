---
tags: cyb, operation, article
crystal-type: process
crystal-domain: cyber
date: 2026-03-20
scope: mnemonic import, wallet encryption, Keplr removal, Ledger integration, error messages
auditor: Claude Opus 4.6
audits: 30
cross-verifications: 5
diffusion: 0.00010722364868599256
springs: 0.0005582311275045726
heat: 0.00044023867098679523
focus: 0.00030912889679172317
gravity: 0
density: 0.09
---
# security audit: mnemonic import

30 audits + 5 cross-verifications of [[cyb]] wallet security. scope: [[mnemonic]] import/unlock, [[Keplr]] removal, [[Ledger]] hardware integration, error message sanitization.

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

| audit | finding | fix |
|-------|---------|-----|
| 1 | plaintext mnemonic in localStorage | AES-256-GCM encryption via [[Web Crypto API]] |
| 1 | mnemonic persisted in React state after modal close | useEffect cleanup on unmount, clearState() in parent |
| 13 | Rune VM `compile()` receives secrets (API keys) — any user script exfiltrates credentials | destructure secrets before compile: `{ secrets: _s, ...safeContext }` |

### HIGH (14 total, 14 fixed)

| audit | finding | fix |
|-------|---------|-----|
| 1 | signingClient stale after setSigner | useEffect on [signer] auto-rebuilds client |
| 1 | unlock flow absent — decryptMnemonic imported by zero components | unlockWallet() + UnlockWalletBar component |
| 3 | CosmJS validation details leaked in error messages | generic "Invalid seed phrase" |
| 3 | pendingMnemonic persisted on unexpected unmount | useEffect cleanup calls clearState() |
| 4 | Keplr fallback triggered for wallet accounts after auto-lock | `!isWalletAccount` guard in getSignerForChain |
| 4 | TypeError on missing network prefix in networkListIbc | null-safe `?.prefix` + bostrom fallback |
| 8 | getSignClientByChainId bypassed Keplr isolation | mnemonicRef path for wallet, Keplr only for keplr-type |
| 10 | pendingMnemonic stays in state on encryption error | clear mnemonic + passwords in catch block |
| 12 | portal components crash — `signer.keplr.getKey()` undefined for wallet | getSignerKeyInfo helper abstracts Keplr vs mnemonic |
| 12 | signArbitrary re-derived amino wallet every call | cached via getAminoWallet() lazy init |
| 17 | mnemonicRef never set during initial import — IBC signing fails | activateWalletSigner() sets mnemonicRef + signer atomically |
| 22 | Ledger signing fails silently — stale WebUSB transport | ReconnectingLedgerSigner + 30s health monitoring |
| 26 | APDU collision — health check corrupts active signing | _signingInProgress mutex flag |
| 29 | IBC bridge withdraw crash — MsgTransfer built as plain object | MsgTransfer.fromPartial() for amino-safe defaults |

### MEDIUM (31 total, 19 fixed, 12 noted)

| audit | finding | fix |
|-------|---------|-----|
| 1 | modal missing Escape/backdrop/ARIA | keydown Escape, backdrop onClick, role="dialog", aria-modal |
| 5 | useSetupIbcClient crash on unknown network | `if (!networkConfig) return` guard |
| 7 | auto-lock on tab hidden absent | visibilitychange listener clears mnemonicRef immediately |
| 10 | double-submit on "Encrypt & Save" | saving state + disabled={saving} |
| 10 | spellCheck sends mnemonic words to Google | spellCheck={false} on MnemonicInput |
| 11 | post-decrypt address mismatch silent | account.address !== address check after decrypt |
| 11 | clipboard retains mnemonic after paste | navigator.clipboard.writeText('') after paste |
| 13 | PBKDF2 600k iterations below 2026 best practice | upgraded to 1,000,000 iterations + version byte |
| 15 | blanket eslint-disable | replaced with targeted @typescript-eslint/no-explicit-any |
| 15 | `as any` casts for signArbitrary and signer detection | hasSignArbitrary() type guard, typed Record narrowing |
| 15 | IE clipboardData fallback (dead code) | removed |
| 15 | global `cyb:mnemonic` localStorage key stale with multiple wallets | removed — per-address key only |
| 16 | encrypted mnemonic persists after account deletion | removeEncryptedMnemonic(bech32) on wallet account delete |
| 23 | raw blockchain errors shown to users (20 files) | friendlyErrorMessage() centralized parser in errorMessages.ts |
| 25 | ContainerGradient.TxsStatus renders raw rawLog | wrapped in friendlyErrorMessage |
| 27 | Ledger "Data is invalid" — old library omits HRP in APDU | replaced ledger-cosmos-js@2 with @zondax/ledger-cosmos-js@4 |
| 28 | sign doc overflow with 41 validators (~10 KB) | batch claiming: 5 validators per tx, poll confirmation between |
| 29 | regex global flag — validation fails every other call | removed `g` flag from all 12 patterns |
| 30 | health check CLA 0xe0 causes unnecessary transport recreation | changed to CLA 0x55 (Cosmos getVersion) |

noted (12): setSigner public context, `as any` in CosmJS constructor, weak password below 12 chars, signer retains seed (CosmJS requirement), dual signer memory (JS immutable strings), unguarded JSON.parse in 5 Redux slices, checkAddressNetwork unbounded recursion (fixed in #24), Ledger reconnect error leaks addresses, amino-only Ledger in relayer, secrets unencrypted in localStorage (pre-existing), seed word inputs type="text" (by design), no beforeunload cleanup (fixed in #24)

### LOW (47 total, 18 fixed, 26 noted)

fixed: modal tabIndex, error message internals, version detection false positive (try-fallback), persist-before-signer ordering, Array.from instead of spread (stack safety), DOMException-only catch, autoComplete/autoCorrect/autoCapitalize on mnemonic inputs, autoComplete="new-password" on password inputs, __cyb_wallet_locked event name (internal), `wallet` label for non-Keplr signers, typed Dropdown callback, getDebug() secrets stripped, blob versioning for migration, removeEncryptedMnemonic for deletion, neuron errors wrapped in friendlyErrorMessage, wasm action bar errors wrapped, console.log sign doc removed, regex patterns fixed

noted (26): focus trap absent, mnemonic inputs visible (by design), eslint-disable scope, gasPrice hardcoded (standard), JS memory mnemonic immutable, unlockWallet concurrency (UI guard sufficient), CustomEvent spoofable (cosmetic only), stack overflow safe for mnemonic sizes, packed length (AES-GCM validates), chainId ignored in signArbitrary (ADR-036 by design), HD path locked to index 0, idle timer race, no initial Ledger address verification, raw Ledger errors, localStorage quota silent fail, recursive setTimeout without cleanup, window.open without noopener, forceQuitter legacy key, Tendermint query interpolation, password in useState (DevTools), mnemonic words in useState (React limitation), secrets in Redux DevTools, transaction response logged, no negative amount guard, address validation lacks bech32 checksum, error oracle (wrong password vs no mnemonic)

## cumulative status

| severity | total | fixed | accepted | roadmap |
|----------|-------|-------|----------|---------|
| CRITICAL | 3 | 3 | 0 | 0 |
| HIGH | 14 | 14 | 0 | 0 |
| MEDIUM | 31 | 19 | 12 | 0 |
| LOW | 47 | 18 | 26 | 0 |

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

## Ledger integration

three account types after [[Keplr]] removal:

| type | key storage | signing | security boundary |
|------|------------|---------|-------------------|
| wallet | encrypted mnemonic in localStorage | in browser (JS) | password + AES-256-GCM |
| ledger | on [[Ledger]] device (never leaves) | on device | physical device |
| read-only | address only | none | N/A |

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
