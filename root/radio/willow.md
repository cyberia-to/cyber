---
alias: iroh-willow, Willow protocol, willow
tags: cyber
crystal-type: entity
crystal-domain: cyber
diffusion: 0.0001737672950547578
springs: 0.0020928463598874855
heat: 0.0014414444040997266
focus: 0.0010030264363135749
gravity: 3
density: 1.39
---

# willow

confidential sync protocol for private data sharing between peers

implementation of the Willow protocol (willowprotocol.org) with Meadowcap as the companion capability-based access control system

## key property

encrypted reconciliation — peers can sync data without revealing what they have to unauthorized parties. the protocol leaks no information about entries beyond what the peer is authorized to see

## architecture

engine (core sync loop), form (data structures), interest (subscription model), session (peer sync sessions), store (persistence layer), proto (wire protocol implementation)

## capability certificates

Meadowcap defines who can read and write what. capabilities are delegatable — a holder can create restricted sub-capabilities with narrower scope. certificates chain from the namespace authority down to individual peers

## role in cyber

willow enables private [[cybergraph]] partitions. [[neurons]] that need confidential collaboration — private research, restricted data, personal knowledge — sync via willow without exposing content to the broader network. complements the public [[cybergraph]] with private overlays where access is controlled by capability certificates rather than public broadcast

see willowprotocol.org for the full specification

crate: iroh-willow