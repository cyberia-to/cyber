---
tags: cybics
crystal-type: pattern
crystal-domain: cybics
stake: 2934725452132150
diffusion: 0.00022640025621714676
springs: 0.0015703726850187182
heat: 0.0011419197306379296
focus: 0.0008126958797417986
gravity: 5
density: 5.86
---
extends [[modal logic]] with time: operators for "always" ($\square$), "eventually" ($\diamond$), "until", "next"

linear temporal logic (LTL) reasons over sequences. computation tree logic (CTL) reasons over branching futures. both are decidable and used for model checking — verifying that systems satisfy specifications.

in the [[cybergraph]]: every [[cyberlink]] carries an epoch timestamp. temporal queries traverse the link history: "this [[particle]] was eventually linked to that one", "this relation held until epoch $n$". the append-only [[BBG]] provides a total ordering of state transitions, making temporal reasoning native.