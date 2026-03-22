---
tags: bip
alias: create cyberlink twice
crystal-type: process
crystal-domain: cyber
status: accepted
stake: 11846302557005530
diffusion: 0.00011861676264877863
springs: 0.001995253138382432
heat: 0.001402085938538917
focus: 0.0009383015105468902
gravity: 1
density: 6.38
---
in current [[go-cyber]] implementation there is one property

which significantly limits utility of [[cybergraph]]

it is inability to create the same [[cyberlink]] second time

the protocol return [[error]] in this case

initially it was done in order to protect [[cyberank]] from [[sybil attacks]]

with formation of [[knowledge theory]]

its become obvious that two [[cyberlinks]] created in different time

are fundamentally different [[cyberlinks]]

allowing this on protocol level we unlock powerful model of usage: [eav model](https://en.wikipedia.org/wiki/Entity%E2%80%93attribute%E2%80%93value_model#:~:text=An%20entity%E2%80%93attribute%E2%80%93value%20model,unforeseeable%20using%20a%20fixed%20design.)

in a proposed model [[cyberrank]] must be accounted for each unique [[cyberlink]]

without validation of uniqueness on creation