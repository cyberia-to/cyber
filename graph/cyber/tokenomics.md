---
tags: cyber, cybernomics, cip
crystal-type: entity
crystal-domain: economics
crystal-size: article
alias: cyber tokenomics, cyber economics, economic model
status: draft
---
# cyber tokenomics

the economic design of [[cyber]] — multiple [[tokens]], multiple mechanisms, one self-sustaining [[knowledge]] economy

grounded in [[cybernomics]] foundations: [[basic token operations]], [[supply and demand]] [[equilibrium]], [[bonding curves]]

## tokens

[[h based economy]] — dual-token CYB/H: scarce [[value]] anchor + [[liquidity]] engine

[[learning tokens]] — will ([[bandwidth]]), [[attention]] (rank influence), [[karma]] (reputation) as feedback to [[superintelligence]]

[[$CYB]] — the native [[token]]. staked for [[security]], burned for permanent π-weight, spent as fees

## mechanisms

seven mechanisms make the [[cybergraph]] a self-sustaining economy. each maps to a protocol primitive through [[basic token operations]] (pay, lock, uber, mint, burn)

### 1. minting for [[focus]] computation

[[neurons]] that compute [[focus]] toward a [[particle]] earn newly minted [[$CYB]]. each valid [[cyberlink]] is rewarded proportional to Δπ — the shift it causes in the [[tri-kernel]] fixed point

see [[learning incentives]] for reward functions, link valuation, and [[Shapley]] attribution

### 2. staking as delegated [[attention]]

[[neurons]] stake [[$CYB]] on themselves or other [[neurons]], delegating [[attention]]. stake directed toward validators earns from the PoS share of gross rewards:

$$R_{\text{PoS}} = G \cdot S^\alpha$$

see [[adaptive hybrid economics]] for the allocation curve and PID tuning

### 3. stake distribution over [[cyberlinks]]

a [[neuron]]'s staked amount spreads evenly across its [[cyberlinks]] by default. the [[neuron]] can re-weight individual [[particles]] or [[cyberlinks]], assigning a percentage of stake to specific targets

see [[staking on particles]] and [[staking on cyberlinks]] for pool mechanics

### 4. permanent weighting via [[burn]]

a [[neuron]] burns [[$CYB]] to grant eternal weight to a [[particle]]. this irreversible act permanently increases that [[particle]]'s importance in π, anchoring critical [[knowledge]]

see [[eternal particles]] and [[eternal cyberlinks]]

### 5. link fees and net rewards

submitting a [[cyberlink]] incurs a small fee (spam deterrent). fees pool and distribute to link submitters, [[focus]] provers, and validators. links that accumulate sufficient [[attention]] yield net positive reward over time

see [[collect fee on moving A and V]] for the fee collection spec

### 6. [[attention]] yield curve

earlier and more accurate [[cyberlinks]] to high-π [[particles]] earn proportionally greater rewards as [[collective focus]] evolves. first-mover advantage for quality links

### 7. reputation emergence

a [[neuron]]'s long-term reputation is the accumulated π-weight of [[particles]] it contributed to. this is [[karma]] — aligning social and economic capital through measurable contribution to [[collective focus]]

## monetary policy

gross rewards combine stepped emission with redistributed fees:

$$G = E(t) + F \cdot (1 - \beta)$$

rewards split between stakers and provers via the allocation curve. $\alpha$ and $\beta$ self-adjust via PID control

net new supply: $\text{net} = E(t) - F \cdot \beta$. when fees exceed emission, the network is net deflationary

[[adaptive hybrid economics]] — the self-calibrating PoW/PoS mechanism with PID control

[[adaptive hybrid consensus economics]] — full mathematical proofs

[[adaptive inflation]] — tendermint inflation tuning for bonded stake target

## hardware substrate

the [[Goldilocks field processor]] makes proving Δπ economically viable. the PoUW puzzle requires producing [[STARK]] proofs using the same four primitives as real workloads. mining rewards bootstrap chip development. chips accelerate proving. proving serves users. users generate fees. fees replace emission

the same hardware mines and proves — no stranded assets

## in the protocol stack

[[foculus]] — [[consensus]]: [[particle]] $i$ is final when $\pi_i > \tau$

[[focus flow computation]] — scheduling and convergence as layer 5 of the stack

[[cybernet]] — experimental [[learning incentives]] layer (Bittensor-style subnets)

[[decentralized attention markets]] — [[focus]]-stake [[attention]] market

## see also

[[bostrom/tokenomics]] — the bootloader implementation (4-token model, energy mint, fees)

[[cybernomics]] — the broader cybernetic economy theory grounding this design

[[learning incentives]] — deep dive into the reward mechanism (Δπ signal, link valuation, attribution)
