tags:: state
- key: `0x00 | []byte("desirableBandwidth") -> sdk.Uint64ToBigEndian(value)`
- represents amount of cyberlinks that network would like to process
  
  ```
  sdk.Uint64ToBigEndian(value) // where value is total current supply of mvolt (uint64)
  ```