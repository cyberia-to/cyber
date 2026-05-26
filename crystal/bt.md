---
tags: cyber, language
alias: Bt, bitwise, binary language
crystal-type: entity
crystal-domain: cyber
---
the binary language. gate semantics over F₂ — the substrate for Boolean logic and any computation whose natural unit is a bit

| Type | Field | Size | Native ops |
|---|---|---|---|
| `Bit` | F₂ | 1 bit | AND (mul), XOR (add) |
| `Bit2` | F₂² | 2 bits | Extension field ops |
| `Bit8` | F₂⁸ | 1 byte | AES-native |
| `Bit32` | F₂³² | 4 bytes | Hash-native |
| `Bit64` | F₂⁶⁴ | 8 bytes | Double word |
| `Bit128` | F₂¹²⁸ | 16 bytes | Security parameter |

characteristic: 2. [[proof]] system: FRI-Binius. AND is multiplication. XOR is addition. both are free. what Bt cannot do cheaply: integer arithmetic. 3 + 5 = 6 in F₂ (XOR), not 8. to perform actual addition with carry, you must build ripple-carry adders from AND/XOR gates

use cases: Blake3/SHA-256 circuits (proving legacy hashes), Keccak verification (Ethereum bridge), AES circuits, binary Merkle tree verification, binary protocol parsing

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture