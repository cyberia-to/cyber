---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8369203325064875
diffusion: 0.00011421459940808424
springs: 0.00005023202437509192
heat: 0.0000818201654881291
focus: 0.00008854094011419438
gravity: 1
density: 0
---
- key: `0x00 | []byte("desirableBandwidth") -> sdk.Uint64ToBigEndian(value)`
- represents amount of cyberlinks that network would like to process
  
  ```
  sdk.Uint64ToBigEndian(value) // where value is total current supply of mvolt (uint64)
  ```