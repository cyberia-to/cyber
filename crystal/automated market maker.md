---
alias: amm
tags: cyber
crystal-type: entity
crystal-domain: economics
---
An automated market maker is an algorithmic trading mechanism that replaces traditional [[order book]] matching with [[liquidity pool]] contracts governed by a mathematical [[bonding curve]].

Liquidity providers deposit pairs of [[tokens]] into a pool, and the AMM formula determines the exchange rate based on the ratio of reserves. The constant product formula (x * y = k) is the most common variant, ensuring that trades shift the price smoothly along the curve.

In [[bostrom]] and the [[cyber]] ecosystem, the AMM operates through the DEX module, enabling permissionless swaps between network tokens. Any [[neuron]] can provide liquidity or execute trades without relying on a centralized exchange.

Price discovery emerges from arbitrage: when the AMM price diverges from external markets, traders profit by rebalancing the pool, pushing the price back toward equilibrium. This self-correcting mechanism operates continuously and autonomously.

Liquidity providers earn fees proportional to their share of the pool, creating an incentive to supply capital. The trade-off is [[impermanent loss]], where price divergence between deposited assets reduces the value of the position relative to simply holding.

AMMs form the foundational [[DeFi]] primitive for token economies, enabling on-chain price discovery and liquidity for any asset pair in the [[cybergraph]] ecosystem.

discover all [[concepts]]