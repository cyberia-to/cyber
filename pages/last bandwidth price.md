tags:: state
- value is used to store up-to-date price of bandwidth
- ```
  type Price struct {
    price          sdk.Dec   // current multiplier for bandwidth billing
  }
  ```
- key: `0x00 | []byte("lastBandwidthPrice") -> ProtocolBuffer(Price)`