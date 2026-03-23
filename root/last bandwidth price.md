---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8385313437336415
diffusion: 0.00011973951558470549
springs: 0.00005678599184880357
heat: 0.00009077158663080771
focus: 0.00009505987267315814
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