tags:: state
alias:: personal bandwidth, account bandwidth
- used for tracking bandwidth of [[neurons]] in the network
- the [[$V]] stake of the given [[neuron]] are easy to understand as the size of his battery
- the creation of [[cyberlinks]] will consume battery charge
- and the battery will be fully recharged during [[recovery period]]
- if a neuron consumes half of its bandwidth
- its battery will be fully charged in the [[recovery period]] divided by 2
- if a [[neuron]] act when network bandwidth consumption is low
- then she will consume less [[neuron bandwidth]]
- [[account bandwidth]] type has the following structure:
- key: `0x01 | []byte(address) -> ProtocolBuffer(AccountBandwidth)`
- ```
  type AccountBandwidth struct {
      address          string     // address of neuron
  	remainedValue    uint64     // current bandwidth value 
  	lastUpdatedBlock uint64     // last block when last time updated
  	maxValue         uint64     // max current bandwidth value of neuron
  }
  ```