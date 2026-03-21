---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8369203325064875
focus: 0.0001242121030987156
gravity: 1
---
- key: `0x00 | []byte("desirableBandwidth") -> sdk.Uint64ToBigEndian(value)`
- represents amount of cyberlinks that network would like to process
  
  ```
  sdk.Uint64ToBigEndian(value) // where value is total current supply of mvolt (uint64)
  ```