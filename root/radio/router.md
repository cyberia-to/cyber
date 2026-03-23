---
alias: ALPN router, protocol router, radio router
tags: cyber
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00016618243783751883
springs: 0.000057158867977061384
heat: 0.00010228447044750125
focus: 0.00012069577340138163
gravity: 2
density: 3.29
---

# router

ALPN-based protocol multiplexer for [[radio/endpoint]]

## ALPN

Application-Layer Protocol Negotiation — a TLS extension that lets peers agree on which protocol to use during the QUIC handshake, before any application data flows

## dispatch

each protocol registers a unique ALPN string. when a connection arrives, the router examines the negotiated ALPN and dispatches to the correct protocol handler. similar to an HTTP router in web frameworks, but operating at the QUIC connection level

## protocol handlers

a handler is any code implementing an accept method that receives a Connection and processes it according to that protocol's rules. [[radio/blob]], [[radio/gossip]], [[radio/docs]], and [[radio/willow]] each register their own ALPN and handler

## extensibility

custom protocols register their ALPN with the router and handle connections independently. radio is a toolkit, not a monolith — the router is the seam that makes composition possible without coupling

## role in cyber

the router is how a single [[radio/endpoint]] serves multiple protocols simultaneously — transferring [[radio/blob]], syncing [[radio/docs]], broadcasting via [[radio/gossip]], all on the same QUIC socket, multiplexed by ALPN. one port, many protocols, zero coordination overhead