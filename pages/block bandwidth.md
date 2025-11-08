tags:: state
- key: `0x02 | sdk.Uint64ToBigEndian(blockNumber) -> sdk.Uint64ToBigEndian(value)`
- storing used bandwidth for each block
- used for calculation of load using sum of used bandwidth in blocks
- at recovery period window
- used for reverting transactions with cyberlinks if rise more than [[max block bandwidth]]
- ```
  sdk.Uint64ToBigEndian(value) // where value is amount of bandwidth used by all neurons in given block
  ```