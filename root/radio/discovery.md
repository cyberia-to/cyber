---
alias: endpoint discovery, Pkarr, radio discovery
tags: cyber
crystal-type: entity
crystal-domain: cyber
---

# discovery

how [[radio/endpoint]] nodes find each other given only a PublicKey

## mechanisms

three discovery systems work together:

### DNS discovery
resolve endpoint addresses via DNS records. suitable for well-known infrastructure nodes and [[radio/relay]] servers

### mDNS
multicast DNS discovers endpoints on the local network without internet or relays. enables zero-configuration local connectivity between nearby [[neurons]]

### Pkarr
Public-Key Addressable Resource Records: publish and resolve endpoint info using elliptic curve keys via DHT. a decentralized naming layer that maps PublicKey to current network addresses

## address resolution

discovery resolves a bare PublicKey into a routable EndpointAddr containing the id, relay URLs, and direct socket addresses. QAD (QUIC Address Discovery) supplements this by learning endpoint locations through the QUIC protocol itself

## for cyber

[[neurons]] publish their endpoint addresses through these mechanisms. other neurons discover them by PublicKey alone. the [[cybergraph]] stores public keys as [[particle]] addresses — discovery bridges the knowledge graph to the physical network