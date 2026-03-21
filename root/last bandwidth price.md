---
tags: state
crystal-type: measure
crystal-domain: cyber
stake: 8385313437336415
focus: 0.0001242121030987156
gravity: 1
---
- value is used to store up-to-date price of bandwidth
- ```
  type Price struct {
    price          sdk.Dec   // current multiplier for bandwidth billing
  }
  ```
- key: `0x00 | []byte("lastBandwidthPrice") -> ProtocolBuffer(Price)`