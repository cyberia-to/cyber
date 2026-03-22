---
tags: cybics
crystal-type: pattern
crystal-domain: cybics
stake: 2934725452132150
diffusion: 0.00021145410559878444
springs: 0.001461467039940639
heat: 0.00107517083877691
focus: 0.0007592013325369561
gravity: 5
density: 5.84
---
extends [[modal logic]] with time: operators for "always" ($\square$), "eventually" ($\diamond$), "until", "next"

linear temporal logic (LTL) reasons over sequences. computation tree logic (CTL) reasons over branching futures. both are decidable and used for model checking — verifying that systems satisfy specifications.

in the [[cybergraph]]: every [[cyberlink]] carries an epoch timestamp. temporal queries traverse the link history: "this [[particle]] was eventually linked to that one", "this relation held until epoch $n$". the append-only [[BBG]] provides a total ordering of state transitions, making temporal reasoning native.