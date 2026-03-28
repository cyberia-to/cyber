---
tags: cyber, research, nox
crystal-type: research
crystal-domain: comp
alias: cyber control codes, dead codes, reclaimed bytes, 256 symbols
---

# 256 symbols

one byte. 256 values. every value has a meaning.

the byte table splits into seven groups. three are alive (text). three are dead (reclaimed). one is the null boundary.

## the full table

```
GROUP           RANGE         COUNT   STATUS     PURPOSE
─────           ─────         ─────   ──────     ───────
null            0x00          1       boundary   NUL — zero delimiter
nox ISA         0x01–0x1E     30      reclaimed  18 instructions + 12 reserved
spare           0x1F          1       dead       future
printable       0x20–0x7E     95      alive      text, numbers, punctuation
delete          0x7F          1       dead       spare
graph ops       0x80–0x9F     32      reclaimed  cybergraph protocol
extended        0xA0–0xFF     96      alive      Latin-1 supplement / UTF-8 lead
```

## group 1: nox ISA (0x01–0x1E)

18 frozen instructions. 12 reserved slots for future (jets?).

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
1D   GS         hint            16   non-deterministic
1E   RS         look            17   state access
```

reserved (within 0x01–0x1E, not yet assigned):

```
03   ETX        reserved              (legacy: Ctrl+C)
04   EOT        reserved              (legacy: Ctrl+D)
07   BEL        reserved              (legacy: bell)
08   BS         reserved              (legacy: backspace)
09   HT         reserved              (legacy: tab)
0A   LF         reserved              (legacy: newline)
0B   VT         reserved              (legacy: vertical tab)
0C   FF         reserved              (legacy: form feed)
0D   CR         reserved              (legacy: carriage return)
13   DC3        reserved              (legacy: XOFF)
1A   SUB        reserved              (legacy: Ctrl+Z)
1B   ESC        reserved              (legacy: escape)
```

12 reserved slots. conflict-free codes used first (18 instructions). conflicted legacy codes available for expansion — context disambiguates (nox binary stream vs text).

## group 2: printable ASCII (0x20–0x7E)

95 symbols. untouched. text lives here.

```
0x20       space
0x21–0x2F  punctuation:  ! " # $ % & ' ( ) * + , - . /
0x30–0x39  digits:       0 1 2 3 4 5 6 7 8 9
0x3A–0x40  punctuation:  : ; < = > ? @
0x41–0x5A  uppercase:    A B C ... Z
0x5B–0x60  punctuation:  [ \ ] ^ _ `
0x61–0x7A  lowercase:    a b c ... z
0x7B–0x7E  punctuation:  { | } ~
```

nox does not touch printable ASCII. text is sacred.

## group 3: graph operations (0x80–0x9F)

32 dead C1 control codes. completely dead in UTF-8 — these bytes cannot appear standalone in valid UTF-8. self-identifying: if you see 0x85 as independent byte, it is definitionally not text.

```
HEX  OLD    GRAPH OP         WHAT
───  ───    ────────         ────
80   PAD    PARTICLE         create content-addressed node
81   HOP    CYBERLINK        create weighted edge
82   BPH    NEURON           register agent identity
83   NBH    STAKE            lock tokens on particle
84   IND    UNSTAKE          unlock tokens
85   NEL    TRANSFER         move tokens between neurons
86   SSA    FOCUS_QUERY      query current π
87   ESA    EDGE_QUERY       query edges by particle/neuron
88   HTS    PROOF            submit zheng proof
89   HTJ    VERIFY           verify proof
8A   VTS    COMMIT           polynomial commitment
8B   PLD    NULLIFY          spend record (prevent double-spend)
8C   PLU    REVEAL           make sealed value public
8D   RI     SEAL             hide value with commitment
8E   SS2    NAMESPACE        declare namespace
8F   SS3    COMPLETE         prove namespace completeness
90   DCS    EPOCH            mark epoch boundary
91   PU1    CHECKPOINT       state checkpoint
92   PU2    MIGRATE          storage proof for rehash
93   STS    DELEGATE         delegate focus to another neuron
94   CCH    REVOKE           revoke delegation
95   MW     CHALLENGE        challenge a claim
96   SPA    RESPOND          respond to challenge
97   EPA    FINALIZE         finality threshold reached
98   SOS    SPONGE_INIT      init incremental hash
99   SGCI   ABSORB           absorb into sponge state
9A   SCI    SQUEEZE          squeeze from sponge state
9B   CSI    MERKLE_STEP      one level of Merkle proof
9C   ST     MERKLE_ROOT      root of authenticated structure
9D   OSC    SYNC             request state synchronization
9E   PM     SUBSCRIBE        subscribe to focus changes
9F   APC    RESERVED         future use
```

## group 4: extended (0xA0–0xFF)

96 values. in Latin-1: printable characters (¡ ¢ £ ... ÿ). in UTF-8: lead bytes for multi-byte sequences. alive — not reclaimed.

```
0xA0       non-breaking space
0xA1–0xBF  Latin-1 symbols and letters
0xC0–0xDF  UTF-8 two-byte lead (0xC0–0xC1 overlong, technically dead)
0xE0–0xEF  UTF-8 three-byte lead
0xF0–0xF7  UTF-8 four-byte lead
0xF8–0xFF  invalid UTF-8 (dead — 8 more reclaimable codes?)
```

## group 5: boundaries

```
0x00  NUL   null delimiter. universal zero. not reclaimed.
0x1F  US    spare. one free slot in control range.
0x7F  DEL   spare. between printable and extended.
```

## summary

```
nox ISA:         18 used + 12 reserved  = 30 slots  (0x01–0x1E)
printable:       95 untouched                        (0x20–0x7E)
graph ops:       31 used + 1 reserved   = 32 slots  (0x80–0x9F)
extended:        96 alive (Latin-1/UTF-8)            (0xA0–0xFF)
boundaries:      3 (NUL + 2 spare)                   (0x00, 0x1F, 0x7F)
                ───
                256
```

49 reclaimed codes carry living meaning. 12 reserved for growth. 95 printable characters untouched. 96 extended characters untouched. every byte accounted for.

## UTF-8 safety

by construction:
- 0x01–0x1E: valid single-byte UTF-8 but never in well-formed text. binary stream context disambiguates
- 0x80–0x9F: standalone bytes in this range are invalid UTF-8. self-identifying — no framing needed
- 0x20–0x7E: printable. untouched
- 0xA0–0xFF: valid UTF-8 lead/continuation. untouched

the teletype is dead. long live the [[cybergraph]].
