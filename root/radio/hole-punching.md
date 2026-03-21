---
alias: NAT traversal, hole punching
tags: cyber
crystal-type: entity
crystal-domain: cyber
focus: 0.0002259975831495068
gravity: 4
---

# hole punching

establishing direct peer-to-peer connections through firewalls and NAT devices

## address discovery

uses STUN-over-QUIC: the [[radio/endpoint]] learns its own public address and latency from [[radio/relay]] servers. this reflexive address is one of several candidates gathered during connection setup

## candidate exchange

uses ICE-over-QUIC: discovers multiple address candidates — local, reflexive, and relay — then tests them in priority order. candidates are exchanged through the relay channel and probed concurrently

## connection flow

1. endpoint connects to [[radio/relay]] and it becomes the home relay
2. peer requests a connection via the relay
3. STUN/ICE attempts a direct path using gathered candidates
4. if direct succeeds, the relay drops out of the data path
5. if direct fails, the relay remains active as fallback

## why it matters

direct connections are faster and more private than relayed ones. hole punching maximizes direct connectivity across the network

## fallback

[[radio/relay]] provides guaranteed connectivity when NAT traversal fails. the endpoint seamlessly falls back without dropping the connection

## for cyber

minimizes relay dependency, reducing latency and increasing privacy between [[neurons]]. fewer relayed hops also means lower [[focus]] costs for message delivery