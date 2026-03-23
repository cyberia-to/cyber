---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8369203325064875
diffusion: 0.00011973951558470549
springs: 0.00005678599184880357
heat: 0.00009077158663080771
focus: 0.00009505987267315814
gravity: 1
density: 0
---
- key: `0x00 | []byte("desirableBandwidth") -> sdk.Uint64ToBigEndian(value)`
- represents amount of cyberlinks that network would like to process
  
  ```
  sdk.Uint64ToBigEndian(value) // where value is total current supply of mvolt (uint64)
  ```