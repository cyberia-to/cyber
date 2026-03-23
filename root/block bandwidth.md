---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8586689840730663
diffusion: 0.00011973951558470549
springs: 0.000054408120423651206
heat: 0.000086883069100966
focus: 0.00009356880773964405
gravity: 1
density: 1.56
---
key: `0x02 | sdk.Uint64ToBigEndian(blockNumber) -> sdk.Uint64ToBigEndian(value)`

storing used bandwidth for each block

used for calculation of load using sum of used bandwidth in blocks

at recovery period window

used for reverting transactions with cyberlinks if rise more than [[max block bandwidth]]
```
  sdk.Uint64ToBigEndian(value) // where value is amount of bandwidth used by all neurons in given block
  ```