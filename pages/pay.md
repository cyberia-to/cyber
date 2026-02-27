---
alias: transfer, send
tags: cyber, core
crystal-type: process
crystal-domain: cyber
crystal-size: enzyme
---
transfer of [[token]] balance between two [[neurons]]

a [[neuron]] sends [[coins]] or [[cards]] to another [[neuron]]. the [[vimputer]] debits one balance and credits the other in the same [[step]]. the transfer is atomic — both sides update or neither does

pay is the movement primitive. [[mint]] creates [[supply]], [[burn]] destroys it, [[lock]] freezes it — pay moves it. together these four form the [[plumb]] operations on [[tokens]]

every pay requires a valid [[signature]] from the sender and sufficient balance. the [[consensus]] layer enforces conservation: total [[supply]] before equals total [[supply]] after

discover all [[concepts]]
