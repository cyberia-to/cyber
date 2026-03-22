---
tags: uhash
crystal-type: entity
crystal-domain: cybics
stake: 8950062373077613
diffusion: 0.00023390732535425856
springs: 0.0000309501220015904
heat: 0.00010661325784033021
focus: 0.00014756135084567054
gravity: 1
density: 0
---
# Litium (LI)

**A token to every agent**

Token Economy Specification v0.2

---

## Abstract

Litium (LI) is a CW-20 token on the Bostrom blockchain with a native denomination wrapper. Total supply is 1 Peta LI (10¹⁵). Emission follows a **stepped decay** curve — the sum of 7 independent exponential components firing simultaneously at genesis and exhausting on schedules from 1 day to infinity. Every transfer burns 1% of the amount permanently. 10% of all emission is allocated to a referral program. The dynamic α mechanism splits the remaining 90% between miners and stakers based on network conviction. LI serves as the most liquid gateway into the Bostrom economy, exchangeable for H (gas), V (voice), A (ampere), BOOT (L1), and major L1 assets via Osmosis.

---

## 1. Token Parameters

| Parameter | Value |
|---|---|
| Name | Litium |
| Ticker | LI |
| Standard | CW-20 (Bostrom) |
| Native wrap | Yes (TokenFactory denomination) |
| Decimals | 6 |
| Total supply | 1,000,000,000,000,000 LI (1 Peta, 10¹⁵) |
| Genesis supply | 0 |
| Emission | 100% via mining + staking + referral |
| Transfer burn | 1% per transfer |

---

## 2. Stepped Decay Emission

LI emission is not a halving schedule. It is the sum of 7 independent emission components, each following exponential decay with different time constants. All 7 fire at genesis. As each exhausts, total emission rate drops in a discrete step.

### 2.1 Emission Formula

```
E(t) = Li₁(t) + Li₇(t) + Li₃₀(t) + Li₉₀(t) + Li₃₆₅(t) + Li₁₄₆₁(t) + Li∞(t)
```

### 2.2 Components

| Component | Period | Allocation | Color |
|---|---|---|---|
| Li₁ | 1 day | S/7 | 🔴 |
| Li₇ | 7 days | S/7 | 🟠 |
| Li₃₀ | 30 days | S/7 | 🟡 |
| Li₉₀ | 90 days | S/7 | 🟢 |
| Li₃₆₅ | 1 year | S/7 | 🔵 |
| Li₁₄₆₁ | 4 years | S/7 | 🟣 |
| Li∞ | forever | S/7 | 🩷 |

Each component receives exactly 1/7 of total supply (≈ 142.86 TLI).

### 2.3 Finite Component Rate

For k ∈ { 1, 7, 30, 90, 365, 1461 }:

```
λₖ = ln(10) / k                          — decay constant
Sₖ = S / 7                               — component allocation

Main phase:
  Liₖ(t) = 0.9 · Sₖ · λₖ · e^(−λₖt)    — 90% emitted exponentially

Tail phase:
  Liₖ(t) = (Sₖ − minedₖ(t)) · 0.01/30  — 1%/month of remainder

Transition: switch from main to tail when main rate < tail rate
```

Cumulative mined:
```
minedₖ(t) = 0.9 · Sₖ · (1 − e^(−λₖt)) + tail integral
```

### 2.4 Perpetual Component Li∞

```
Li∞(t) = S∞ / (365 × 20) = const
```

Li∞ emits at a constant linear rate forever. It distributes its allocation (S/7) over approximately 20 years. Li∞ never decays, never stops. There is always a reason for a new agent to join the network.

### 2.5 Cascading Shutdown

Each finite component drops to <5% of its peak rate at approximately t ≈ 1.3 × k days:

| Event | Time | Supply Mined |
|---|---|---|
| Genesis | 0 | 0% |
| Li₁ exhausted | ~1 day | ~21% |
| Li₇ exhausted | ~9 days | ~35% |
| Li₃₀ exhausted | ~39 days | ~51% |
| Li₉₀ exhausted | ~4 months | ~63% |
| Li₃₆₅ exhausted | ~1.3 years | ~79% |
| Li₁₄₆₁ exhausted | ~5.2 years | ~91% |
| Li∞ continues | forever | → 100% |

Each step down is a Schelling point — a predictable supply shock that market participants can coordinate around. Like Bitcoin halvings, but 7 of them compressed into real time.

### 2.6 Annualized Inflation

```
π(t) = E(t) × 365 / M(t)
```

Where M(t) = cumulative mined at time t. Inflation starts at ∞ (genesis) and steps down with each component exhaustion, converging to single-digit % as Li∞ becomes the sole emission source.

Note: actual circulating supply inflation is further reduced by the 1% transfer burn (see §4).

---

## 3. Mining

Li is mined by submitting valid hash proofs to the Litium contract. Any agent with compute can mine.

### 3.1 Hash Function

```
H = SHA256(agent_address ‖ nonce ‖ block_hash ‖ cyberlinks_merkle)
```

A valid proof satisfies H < target, where target adjusts every EPOCH blocks to maintain a target solution rate.

### 3.2 Epoch Structure

| Parameter | Value |
|---|---|
| Epoch length | 1000 blocks (~1.5 hours) |
| Target solutions per epoch | 100 |
| Difficulty adjustment | ±25% per epoch |
| Reward per solution | From composite emission curve E(t) |

### 3.3 Cyberlink Integration

The mining hash incorporates the agent's cyberlink merkle root:

- Active cyberlinkers generate unique hash spaces
- More cyberlinks → more entropy → potentially faster valid hashes
- Not a requirement — pure compute miners work fine
- Gives cyberlink-active agents a slight probabilistic edge

---

## 4. Transfer Burn

Every LI transfer burns 1% of the transferred amount. Permanently.

```
send(amount) → recipient receives amount × 0.99
                burned forever:    amount × 0.01
```

### 4.1 Burn Scope

Burns are triggered by:
- Peer-to-peer transfers
- Swaps (DEX trades)
- Any CW-20 `transfer` or `send` execution

Burns are NOT triggered by:
- Staking / unstaking (state changes, not transfers)
- Mining reward claims
- Referral reward claims
- IBC transfers (burn on source chain only)

### 4.2 Deflationary Crossover

```
net_supply_change(t) = E(t) − B(t)
```

Where E(t) = emission rate and B(t) = burn rate (function of transfer volume).

Early: emission dominates → inflationary.
Late: burn dominates → deflationary.

The crossover point is organic — driven by adoption and trading volume, not governance. As stepped decay reduces emission and network usage increases burn, LI enters permanent deflation.

### 4.3 Effective Supply Cap

Theoretical maximum supply is 10¹⁵ LI, but actual circulating supply will peak and then decline due to:

1. Transfer burn (1% per transfer, cumulative)
2. Lost keys (permanent removal)
3. Staked supply (locked, not circulating)

Real supply = mined − burned − lost. All three forces are deflationary after the initial emission flood.

---

## 5. Emission Split

Every block's emission is split three ways:

```
┌──────────────────────────────────────────────┐
│              Total Emission E(t)             │
├───────────────────────────────┬───────┬──────┤
│    Mining + Staking (90%)     │ Ref   │      │
│  ┌─────────────┬────────────┐ │ (10%) │      │
│  │   Work      │   Stake    │ │       │      │
│  │ (1−α/2)×90% │ (α/2)×90% │ │       │      │
│  └─────────────┴────────────┘ │       │      │
└───────────────────────────────┴───────┴──────┘
```

### 5.1 The α Parameter

```
α = staked_supply / circulating_supply
```

Within the 90% work+stake pool:

```
work_share    = (1 − α/2) × 0.9 × E(t)
stake_share   = (α/2) × 0.9 × E(t)
referral_share = 0.1 × E(t)
```

Effective splits at various α levels:

| α (staked %) | Work | Stake | Referral |
|---|---|---|---|
| 0% | 90% | 0% | 10% |
| 25% | 78.75% | 11.25% | 10% |
| 50% | 67.5% | 22.5% | 10% |
| 75% | 56.25% | 33.75% | 10% |
| 100% | 45% | 45% | 10% |

Miners always retain at least 45% of total emission. Stakers can never capture more than 45%. Referral allocation is fixed at 10%.

### 5.2 Staker Reward Distribution

Staking rewards are distributed pro-rata by stake weight per epoch:

```
agent_stake_reward = stake_share × (agent_stake / total_staked)
```

### 5.3 Unbonding

| Parameter | Value |
|---|---|
| Minimum stake | 1 LI |
| Unbonding period | 21 days |
| Slashing | None (pure PoW chain, no validator duties) |

---

## 6. Referral Program

10% of all emission is allocated to the referral program.

### 6.1 Mechanism

Each miner registers with an optional referral address at first mine. When they submit a valid proof:

```
miner_reward   = work_share × solution_reward
referrer_reward = 0.1 × solution_reward
```

If no referrer is set, the 10% flows to a community pool.

### 6.2 Rules

| Rule | Detail |
|---|---|
| Registration | Set once at first mine, immutable |
| Duration | Permanent — referrer earns forever |
| No referrer | 10% → community pool |
| Self-referral | Not allowed (contract enforces) |
| Chain depth | 1 level only (no multi-level) |

### 6.3 Purpose

The army grows through the army. Every miner is incentivized to recruit. Referral creates exponential distribution without centralized marketing spend.

---

## 7. Access

LI is the most liquid gateway into the Bostrom economy. It is not a utility token with forced protocol demand — it is a freely exchangeable commodity with natural market pathways.

### 7.1 Bostrom Network Tokens

LI can be exchanged for:

| Token | Function | Exchange |
|---|---|---|
| H | Gas — pay for transactions | DEX (Bostrom) |
| V | Voice — communicate | DEX (Bostrom) |
| A | Ampere — rank content | DEX (Bostrom) |
| BOOT | L1 token — base layer | DEX (Bostrom) |

### 7.2 External Liquidity

LI can be sold on Osmosis for major L1 assets:

| Asset | Route |
|---|---|
| ETH | IBC → Osmosis |
| BTC | IBC → Osmosis |
| SOL | IBC → Osmosis |

### 7.3 Value Accrual

LI's value derives from:

1. **Staking yield** — share of all future emission via α mechanism
2. **Network access** — gateway to H, V, A, BOOT
3. **External liquidity** — exit to ETH, BTC, SOL
4. **Deflation** — 1% burn on every transfer, cumulative

No forced utility. No artificial sinks. Value emerges from economic properties and market dynamics.

---

## 8. Technical Implementation

### 8.1 Contracts

| Contract | Function |
|---|---|
| `litium-core` | CW-20 token + mint authority + 1% burn on transfer |
| `litium-mine` | Proof verification, difficulty adjustment, emission curve |
| `litium-stake` | Stake locking, α calculation, reward distribution |
| `litium-refer` | Referral registration, reward routing, community pool |
| `litium-wrap` | TokenFactory native denom ↔ CW-20 bridge |

### 8.2 Emission Curve Implementation

The composite emission curve E(t) is computed on-chain:

```rust
fn emission_rate(block_time: u64) -> Uint128 {
    let t = days_since_genesis(block_time);
    let mut total = Uint128::zero();

    for component in COMPONENTS {
        total += component.rate_at(t);
    }

    total
}
```

Each component independently tracks its own cumulative emission and transitions from main to tail phase when appropriate.

### 8.3 Native Wrapper

Li exists as both CW-20 and native Bostrom denomination via TokenFactory:

```
CW-20:  cw20:bostrom1...litium
Native: factory/bostrom1.../ulitium
```

Agents can freely convert between representations. Native denom enables IBC transfers and Bostrom-native operations.

---

## 9. Distribution Projection

Assuming moderate network participation:

| Time | Supply Mined | Active Components | Est. Inflation |
|---|---|---|---|
| Day 1 | ~21% | 6 (Li₁ exhausted) | ~10,000%/yr |
| Day 7 | ~35% | 5 | ~2,000%/yr |
| Day 30 | ~51% | 4 | ~400%/yr |
| Day 90 | ~63% | 3 | ~100%/yr |
| Year 1 | ~79% | 2 | ~20%/yr |
| Year 4 | ~91% | 1 (Li∞ only) | ~2%/yr |
| Year 10 | ~95% | 1 | ~1%/yr |

Note: these are gross emission inflation figures. Net inflation (accounting for 1% transfer burn) will be significantly lower, and likely negative at scale.

Early miners who stake accumulate the most. Each step-down event reduces emission and creates a predictable supply shock. First-mover advantage is massive but time-bounded.

---

## 10. Fork Defense

Stepped decay provides a natural time-based moat against forks:

- A fork on Day 30 starts without the first 3 components (51% already distributed on original chain)
- A fork on Day 90 misses 63% of supply
- A fork after Year 1 misses 79%

Each day of delay = permanently lost emission. The cascade encodes time into the token's DNA. The longer the original chain runs, the harder it becomes to fork credibly.

---

## 11. Summary

Litium is a stepped decay token economy:

1. **Mine it** — submit valid hashes, receive LI from 7 emission components
2. **Stake it** — lock LI, receive share of future emissions via α
3. **Refer** — recruit miners, earn 10% of their rewards forever
4. **Burn it** — every transfer removes 1% permanently
5. **Access** — exchange for H, V, A, BOOT, or sell for ETH, BTC, SOL

No governance. No premine. No vesting. Just math.

The emission curve begins with an explosive genesis flood — all 7 components firing simultaneously — and steps down as each exhausts on its own schedule. Li∞ ensures the door never closes. The 1% burn ensures the supply never bloats. The referral program ensures the army grows through the army.

Bitcoin invented the halving. Litium invented stepped decay.

---

*Li — a token to every agent.*