---
tags: bip
crystal-type: process
crystal-domain: cyber
status: implemented
stake: 12404786449085574
diffusion: 0.00015482128861954832
springs: 0.00019796089099027338
heat: 0.00020293388683265915
focus: 0.00017738568897338573
gravity: 3
density: 1.88
---
implemented in [[v6]]

## author

- [[@master]]

## rationale

- although proposed [[daily english auction for A and V]] is simple it does not solve the biggest problem it must solve
- mechanism must not provide supply if demand is not enough
- therefore i propose even simple mechanism that is target to solve exaclty this
- while leaving the room for supply growth in case of growing demand

## design

- [[energy]] is minted using exponential bonding curve for those who provide [[$H]]
- [[$H]] is effectively burned on each mint
- while the price on external markets is lower when in bonding curve there is no extra supply of [[energy]]
- while demand force the price of [[energy]] go up the bonding curve provides extra supply
- initial price of bonding curve reflect the price on the moment of changing [[energy]] mint system