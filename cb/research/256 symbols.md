---
tags: cyber, research, nox
crystal-type: research
crystal-domain: comp
alias: cyber control codes, dead codes, reclaimed bytes, 256 symbols
---

# 256 symbols

one byte. 256 values. every value has a meaning.

## the full table

```
GROUP           RANGE         COUNT   STATUS     PURPOSE
─────           ─────         ─────   ──────     ───────
null            0x00          1       boundary   NUL — zero delimiter
nox ISA         0x01–0x1E     30      reclaimed  18 instructions + 12 reserved
spare           0x1F          1       dead       future
printable       0x20–0x7E     95      alive      text, numbers, punctuation
delete          0x7F          1       dead       spare
genesis jets    0x80–0xA3     36      reclaimed  consensus-critical accelerators
jet reserved    0xA4–0xBF     28      reserved   future jets
extended        0xC0–0xFF     64      alive      UTF-8 multi-byte lead
```

## group 1: nox ISA (0x01–0x1E)

18 frozen instructions. 12 reserved slots.

```
HEX  OLD        nox              #   GROUP
───  ───        ───              ─   ─────
01   SOH        axis             0   structural
02   STX        quote            1   structural
05   ENQ        compose          2   structural
06   ACK        cons             3   structural
0E   SO         branch           4   structural
0F   SI         add              5   field
10   DLE        sub              6   field
11   DC1        mul              7   field
12   DC2        inv              8   field
14   DC4        eq               9   field
15   NAK        lt              10   field
16   SYN        xor             11   bitwise
17   ETB        and             12   bitwise
18   CAN        not             13   bitwise
19   EM         shl             14   bitwise
1C   FS         hash            15   hash
1D   GS         call            16   non-deterministic
1E   RS         look            17   state access
```

reserved (12 legacy-conflicted codes, available with context framing):

```
03 ETX  04 EOT  07 BEL  08 BS   09 HT   0A LF
0B VT   0C FF   0D CR   13 DC3  1A SUB  1B ESC
```

## group 2: printable ASCII (0x20–0x7E)

95 symbols. untouched. text is sacred.

```
0x20       space
0x21–0x2F  ! " # $ % & ' ( ) * + , - . /
0x30–0x39  0 1 2 3 4 5 6 7 8 9
0x3A–0x40  : ; < = > ? @
0x41–0x5A  A B C ... Z
0x5B–0x60  [ \ ] ^ _ `
0x61–0x7A  a b c ... z
0x7B–0x7E  { | } ~
```

## group 3: genesis jets (0x80–0xA3)

36 consensus-critical jets. frozen in genesis crystal. every [[cybernode]] must support them. formula = [[nox]] noun (semantic definition). implementation = native code (acceleration). contract: jet(x) == nox_reduce(formula, x) always.

dead C1 codes (0x80–0x9F) cannot appear standalone in valid UTF-8 — self-identifying. 0xA0–0xA3 reclaimed from Latin-1 supplement (non-breaking space + 3 symbols rarely used standalone).

### [[hemera]] — hash (1 jet)

```
80   hash                 Poseidon2 permutation (~5× every hemera call)
```

### [[nebu]] — recursion (4 jets)

```
81   poly_eval            Horner polynomial evaluation
82   merkle_verify        Merkle path verification
83   fri_fold             FRI folding round
84   ntt                  Number Theoretic Transform
```

### [[kuro]] — binary tower (8 jets)

```
85   popcount             bit count on packed binary vectors
86   packed_inner_product SIMD binary inner product
87   binary_matvec        binary matrix-vector multiply (1,400× over F_p)
88   quantize             F_p → F₂ boundary
89   dequantize           F₂ → F_p boundary
8A   activation_lut       lookup table activation functions
8B   gadget_decompose     FHE bootstrapping decomposition (jali → kuro)
8C   barrel_shift         cyclic bit shift
```

### [[jali]] — polynomial ring (5 jets)

```
8D   ntt_batch            batched polynomial NTT (ring-aware)
8E   key_switch           FHE key switching via automorphisms
8F   gadget_decomp        FHE gadget decomposition (jali → kuro)
90   noise_track          FHE noise budget tracking
91   blind_rotate         full blind rotation step
```

### [[genies]] — isogeny curves (5 jets)

```
92   group_action          commutative group action on supersingular curves
93   isogeny_walk          path-based isogeny computation
94   vrf_eval              verifiable random function evaluation
95   vdf_step              verifiable delay function step (sequential)
96   secret_hash           genies → nebu boundary (F_q secret → F_p hemera digest)
```

### [[trop]] — tropical semiring (6 jets)

```
97   trop_matmul           tropical matrix multiply
98   trop_shortest         single-source shortest path
99   trop_hungarian        optimal assignment (Hungarian algorithm)
9A   trop_viterbi          HMM sequence decoding
9B   trop_transport        optimal transport plan
9C   witness_commit        trop → nebu boundary
```

### state — polynomial transitions (6 jets)

```
9D   cyberlink_validate    cyberlink transaction circuit (~3,200 vs ~25,000 constraints)
9E   transfer_template     state transfer (3 vs ~8,000 constraints)
9F   insert_template       table insertion (5 vs ~6,000 constraints)
A0   update_template       record update (5 vs ~4,000 constraints)
A1   aggregate_template    accumulation (2 vs ~4,000 constraints)
A2   conserve_template     balance conservation (n vs ~4,000 constraints)
```

### decider (1 jet)

```
A3   decider               all-history verifier (89 constraints)
```

### counts

| group | algebra | count | speedup target |
|-------|---------|-------|---------------|
| hash | [[hemera]] | 1 | ~5× every hemera call |
| recursion | [[nebu]] (F_p) | 4 | recursive proof composition |
| binary-tower | [[kuro]] (F₂) | 8 | 32-90× quantized inference |
| polynomial-ring | [[jali]] (R_q) | 5 | ~log(n)-n× FHE bootstrapping |
| isogeny-curves | [[genies]] (F_q) | 5 | native F_q privacy |
| tropical-semiring | [[trop]] (min,+) | 6 | O(n²)-O(n³) optimization |
| state | F_p circuits | 6 | 500× state transition proving |
| decider | F_p accumulator | 1 | 89 constraints — all-history verification |

## group 4: jet reserved (0xA4–0xBF)

28 slots for future jets. added by governance, frozen per checkpoint. same contract: formula noun + native impl, jet(x) == nox_reduce(formula, x).

## group 5: extended / UTF-8 lead (0xC0–0xFF)

64 values. UTF-8 multi-byte sequence lead bytes. alive — not reclaimed.

```
0xC0–0xDF  two-byte lead   (0xC0–0xC1 overlong, technically dead)
0xE0–0xEF  three-byte lead
0xF0–0xF7  four-byte lead
0xF8–0xFF  invalid UTF-8   (8 reclaimable if needed)
```

## group 6: boundaries

```
0x00  NUL   zero delimiter. not reclaimed.
0x1F  US    spare.
0x7F  DEL   spare.
```

## summary

```
nox ISA:         18 used + 12 reserved    (0x01–0x1E)
printable:       95 untouched             (0x20–0x7E)
genesis jets:    36 used                  (0x80–0xA3)
jet reserved:    28 for future            (0xA4–0xBF)
UTF-8 lead:      64 alive                 (0xC0–0xFF)
boundaries:       3 (NUL + 2 spare)       (0x00, 0x1F, 0x7F)
                 ───
                 256
```

54 reclaimed (18 ISA + 36 jets). 40 reserved (12 ISA + 28 jets). 95 printable. 64 UTF-8. 3 boundaries. every byte accounted for.

## UTF-8 safety

- 0x01–0x1E: C0 controls. never in well-formed text. context disambiguates
- 0x80–0x9F: invalid standalone UTF-8. self-identifying
- 0xA0–0xA3: Latin-1 supplement. rare standalone. context disambiguates
- 0x20–0x7E: printable. untouched
- 0xC0–0xFF: UTF-8 lead bytes. untouched

the teletype is dead. the jets are alive. long live the [[cybergraph]].
