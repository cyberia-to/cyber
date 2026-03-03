---
alias: iroh-gossip, gossip protocol, radio gossip
tags: cyber
crystal-type: entity
crystal-domain: cyber
---

# gossip

topic-based publish/subscribe over epidemic broadcast trees

built on two papers: HyParView for hybrid partial view peer sampling, and PlumTree for efficient broadcast tree construction on top of the random peer graph

## topics

a topic is a 32-byte identifier defining a gossip swarm. subscribe to a topic to receive all messages broadcast to it. topics partition the network into overlapping interest groups

## interface

subscribing returns a (Sender, Receiver) pair. Sender broadcasts messages to the topic. Receiver streams incoming messages as events: Received (message + sender identity), Peer (new peer discovered), Subscribed (joined topic confirmation)

## propagation

builds a broadcast tree overlay on top of the random peer graph. eager push along tree edges, lazy push on the remaining links — combines the reliability of flooding with the efficiency of tree routing. repairs itself when peers leave or join

## transport

uses the ALPN identifier "gossip" over QUIC bidirectional streams routed through [[radio/router]]. connections established via [[radio/endpoint]]

## role in cyber

gossip is how [[neurons]] learn about new [[cyberlinks]] in real time. when a neuron creates a link, it broadcasts to the relevant topic. subscribers update their local view of the [[cybergraph]] without polling. also serves as the notification layer for [[radio/docs]] synchronization — when a replica entry changes, gossip carries the signal

crate: iroh-gossip
