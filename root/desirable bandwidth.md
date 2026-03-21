---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8369203325064875
diffusion: 0.0001242121030987156
springs: 0.00005936068003642514
heat: 0.00009316374154740768
focus: 0.00009854700386976737
gravity: 1
density: 0
---
- key: `0x00 | []byte("desirableBandwidth") -> sdk.Uint64ToBigEndian(value)`
- represents amount of cyberlinks that network would like to process
  
  ```
  sdk.Uint64ToBigEndian(value) // where value is total current supply of mvolt (uint64)
  ```