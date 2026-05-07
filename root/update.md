---
alias: modify, change
tags: cyber, core
crystal-type: process
crystal-domain: cyber
crystal-size: enzyme
---
modify properties of a [[token]] or [[particle]] in place — metadata, ownership, bindings. requires [[signature]] or [[consensus]]

update is the [[signal]] that mutates existing objects within the [[cybergraph]] without destroying and recreating them. the object's identity persists; only its attributes change.

for [[particles]], an update can alter content references, [[tags]], or binding targets. the [[neuron]] that authored the [[particle]] signs the update [[signal]], and the [[tru]] verifies ownership before applying the mutation within the current [[step]].

[[token]] updates follow stricter rules. modifying a [[card]]'s metadata — its name, description, or media references — requires the issuing [[neuron]]'s [[signature]]. supply parameters and provenance bindings remain immutable after [[mint]].

governance-driven updates operate through [[consensus]]. parameter changes to the protocol itself — [[gas]] limits, reward curves, module configurations — require a proposal, a voting period measured in [[steps]], and approval by locked [[coins]].

each update records a full diff in the [[state]] transition log. the previous value and the new value coexist in history, preserving auditability. the [[cybergraph]] tracks what changed, when, and by whose authority.

update consumes [[gas]] proportional to the size of the mutation. large metadata rewrites cost more than small field changes, aligning economic incentives with storage efficiency across [[validators]].

discover all [[concepts]]
