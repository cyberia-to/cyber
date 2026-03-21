---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8385313437336415
diffusion: 0.00011421459940808424
springs: 0.00005023202437509192
heat: 0.0000818201654881291
focus: 0.00008854094011419438
gravity: 1
density: 0
---
- value is used to store up-to-date price of bandwidth
- ```
  type Price struct {
    price          sdk.Dec   // current multiplier for bandwidth billing
  }
  ```
- key: `0x00 | []byte("lastBandwidthPrice") -> ProtocolBuffer(Price)`