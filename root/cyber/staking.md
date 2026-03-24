---
tags: cyber, core
alias: staking, staking on particles, staking on cyberlinks, stake
crystal-type: process
crystal-domain: cyber
diffusion: 0.0002792576336596908
springs: 0.001761626534480527
heat: 0.0012881037512580303
focus: 0.0009257375274256366
gravity: 11
density: 5.4
---
directing economic weight toward [[particles]] and [[axons]] in the [[cybergraph]]

two mechanisms, different levels of commitment:

## [[will]] — broad staking

lock [[$CYB]] for duration → create [[will]]. [[will]] auto-distributes across all [[cyberlinks]] a [[neuron]] creates. every link receives a share, producing [[attention]] at the target. longer lock → more [[will]] → more [[attention]] per link

this is the default: stake once, [[attention]] flows to everything you link. no per-target management required

## conviction — per-link staking

the $(\tau, a)$ fields in a [[cyberlink]] are a [[UTXO]]. creating a link locks [[tokens]] of denomination $\tau$ with amount $a$ directly into that edge. this is conviction — economic weight bound to a specific assertion

conviction is stronger than [[will]]: it prices a single claim, not the [[neuron]]'s entire portfolio. high conviction on one link signals "I bet specifically on this connection"

conviction can be:
- maintained — the UTXO stays, the link carries weight
- withdrawn — spend the UTXO back to wallet, the link loses economic weight but the structural record remains
- transferred — spend the UTXO to a new owner, the assertion stays but beneficial ownership moves

## fine-tuning

a [[neuron]] can adjust the [[attention]] distribution beyond the defaults:
- redirect [[will]] weight toward specific [[particles]] or [[axons]]
- add conviction to high-confidence links
- withdraw conviction from links the [[neuron]] no longer believes in

the combination of [[will]] (broad) and conviction (specific) gives each [[neuron]] a portfolio of epistemic positions — from passive participation to active betting on specific [[knowledge]]

## eternal staking

locking [[will]] with unlimited duration — maximum commitment, permanent [[attention]] weight. the [[particle]] or [[axon]] receives a permanent floor of [[focus]] that cannot be withdrawn. this is the graph's highest-conviction assertion: "this matters forever"

eternal staking is not burning — the [[tokens]] remain staked, generating [[will]] indefinitely. the [[neuron]] cannot withdraw but the stake continues to earn [[karma]] proportional to the [[focus]] it attracts

## effect on [[focus]]

the [[tri-kernel]] sees the weighted graph:

$$A^{\text{eff}}_{pq} = \sum_\ell a(\ell) \cdot \kappa(\nu(\ell)) \cdot f(m(\ell))$$

where $a(\ell)$ is conviction + [[will]]-derived [[attention]], $\kappa$ is [[karma]], and $f(m)$ is the [[coupling|ICBS]] market weight. staking determines the $a$ term — the economic input to [[focus]] computation

see [[will]] for the lock mechanics. see [[cyber/link]] for the conviction UTXO model. see [[attention]] for how [[will]] produces per-target weight

discover all [[concepts]]
