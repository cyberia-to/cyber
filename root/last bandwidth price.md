---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8385313437336415
diffusion: 0.0001242121030987156
springs: 0.00005936068003642514
heat: 0.00009316374154740768
focus: 0.00009854700386976737
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