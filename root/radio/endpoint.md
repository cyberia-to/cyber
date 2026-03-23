---
alias: iroh endpoint, radio endpoint
tags: cyber
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00032449821149063435
springs: 0.0006260306734616023
heat: 0.0005479749028022616
focus: 0.0004596532883442637
gravity: 7
density: 4.67
---

# endpoint

the main interface to a [[radio]] node

an endpoint wraps a QUIC socket, an Ed25519 keypair identity, a [[radio/relay]] connection, and [[radio/discovery]] services into a single handle. it is the entry point for all networking in [[cyber]]

## identity

each endpoint derives an EndpointId from its Ed25519 PublicKey. the EndpointAddr bundles the id with relay URLs and direct socket addresses. connections are made by cryptographic PublicKey, not by IP address — dial keys, not addresses

## transport

endpoints support three transport modes over QUIC:
- bidirectional streams (BiStream) for request-response patterns
- unidirectional streams (UniStream) for one-way data flows
- datagrams for low-latency fire-and-forget messages

QUIC provides authenticated encryption, multiplexed streams, and no head-of-line blocking. all traffic is encrypted end-to-end using keys derived through [[Hemera]] (Poseidon2) hash functions

## connection to cyber

every [[neuron]] runs an endpoint. the PublicKey is the neuron's network identity on the physical network. [[radio]] connects neurons to each other and to [[radio/relay]] servers. [[radio/discovery]] resolves a bare PublicKey into a routable address, and [[radio/hole-punching]] establishes direct paths whenever possible