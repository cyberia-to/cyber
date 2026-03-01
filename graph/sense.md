---
tags: page, prism, cyb
crystal-type: entity
crystal-domain: cyber
---

messaging and notification [[aip]] in [[cyb]]

widget molecule and full application in [[prism]]

the communication channel between a [[neuron]] and the network

## interface

- inputs
	- message stream: incoming [[cyberlinks]], mentions, events
	- notification priority: computed from [[cyberank]] of source [[neuron]]
	- filter: by type (message, event, system), by source, by [[emotion]]
- outputs
	- read/dismiss action
	- reply action → creates a [[cyberlink]]
	- navigate action → opens source [[particle]]

## as widget (molecule)

- compact notification indicator in the [[prism/hud]]
- shows unread count as [[counter]] with [[emotion]] color
- expands to a message list on click
- mobile variant: stacks vertically with swipe actions

## as aip

- full-screen messaging experience
- features
	- [[log]]: chronological message stream
	- [[swarm]]: group communication channels
- every message is a [[particle]]. every reply is a [[cyberlink]]