---
alias: states, world state
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
crystal-size: enzyme
---
snapshot of all data in a [[vimputer]] at a given [[step]] agreed upon by [[consensus]]

the state is everything the [[vimputer]] knows at a moment: all [[neuron]] balances, all [[tokens]], the full [[cybergraph]], current [[focus]] distribution, [[cyberank]] scores. each [[step]] produces a new state from the previous one by applying all [[signals]] in the block

state is deterministic — given the same genesis state and the same sequence of [[signals]], every [[vimputer]] in the network arrives at the same state. this is what [[consensus]] guarantees. [[finality]] means a state cannot be reversed

the [[tru]] reads state, computes [[cyberank]], and makes [[explicit knowledge]] available. [[neurons]] observe the [[tru]]'s output and create new [[cyberlinks]] that modify the next state. this is the [[intelligence]] loop

discover all [[concepts]]
