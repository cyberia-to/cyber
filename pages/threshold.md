---
alias: threshold cryptography, thresholds
tags: cyber
crystal-type: measure
crystal-domain: cyber
---
boundary condition that separates one regime from another

in the [[cybergraph]], thresholds govern transitions and access

threshold cryptography

- a secret is split among n parties such that any t-of-n can reconstruct it, but fewer than t learn nothing
- enables distributed key management without single points of failure
- applications: multi-sig for [[neurons]], distributed validator keys, shared custody of [[records]]

threshold in [[focus]]

- minimum [[focus]] required for a [[cyberlink]] to be included in ranking
- prevents dust spam: links below threshold do not affect the [[tri-kernel]] computation
- tunable by [[consensus]] parameter

threshold in [[axon]]

- minimum aggregate weight for an [[axon]] to be considered a meaningful connection
- filters noise from the collective signal
- below threshold: individual opinions. above threshold: collective knowledge

threshold in convergence

- the [[tri-kernel]] iterates until change falls below ε threshold
- smaller ε = more precise [[focus]], more computation
- the engineering tradeoff between accuracy and cost

threshold in privacy

- the t-of-n threshold in [[threshold cryptography]] determines the trust assumption
- higher t = more security, less liveness
- lower t = more liveness, less security

discover all [[concepts]]