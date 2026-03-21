---
alias: relay, relay server, home relay, iroh-relay
tags: cyber
crystal-type: entity
crystal-domain: cyber
focus: 0.00028682422943542316
gravity: 6
---

# relay

encrypted relay server for when direct connections between [[radio/endpoint]] nodes fail

## home relay

each endpoint registers with a home relay — the closest relay that becomes its primary contact point. the home relay is selected by latency measured via STUN-over-QUIC probes

## encrypted forwarding

relays forward encrypted traffic without decoding it. they see destination keys but not message content. privacy is preserved even when traffic passes through third-party infrastructure

## protocol

relays speak HTTP/HTTPS upgraded to a custom TCP protocol with [[Hemera]]-based handshakes replacing the original Blake3 KDF. the iroh-relay crate implements the server and client sides

## role in connectivity

when [[radio/hole-punching]] fails, the relay provides guaranteed connectivity as a fallback. relays also assist with peer address resolution, working alongside [[radio/discovery]] to help endpoints locate each other

## incentive in cyber

relays earn [[focus]] for proven delivery via [[stark]] proof chains. this creates a permissionless relay network where operators are compensated for bandwidth. see [[cyber/communication]] for the broader messaging architecture