---
tags: cyber, uhash
crystal-type: entity
crystal-domain: cybics
---

## Minimal Implementation Spec

all values in tokens. no price oracle needed.

---

## 1. Parameters

| Symbol | Domain | Description |
  |--------|--------|-------------|
  | T | R+ | token supply |
  | S | [0, 1] | staking ratio (staked / T) |
  | D_rate | R+ | aggregate difficulty per second (from sliding window) |
  | F | R+ | fees collected (tokens, measured over window) |
  | d | Z+ | proof difficulty (leading zero bits, chosen by client) |
  | N | Z+ | sliding window size (number of proofs) |
  | K | Z+ | PID update interval (every K proofs) |
  | alpha | [0.3, 0.7] | allocation curve exponent |
  | E(t) | R+ | composite emission rate (from stepped decay curve, see [[bostrom/lithium]]) |
  | beta | [0, 0.9] | fee burn rate |

## 2. Per-Proof Instant Payout

no epochs. every valid proof is paid immediately on-chain

client chooses difficulty `d` (number of leading zero bits)

reward per proof:
  ```
  reward(d) = base_rate * d
  ```

`base_rate` adapts via sliding window so total emission tracks target curve

### Sliding Window

  ```
  W = last N proofs: [(d_i, t_i), ...]
  D_rate = sum(d_i for i in W) / (t_last - t_first)   // total difficulty per second
  E_target = E(t)                                        // from stepped decay curve
  base_rate = E_target / D_rate
  ```

when more miners join (D_rate rises), base_rate drops — same total emission

when miners leave (D_rate falls), base_rate rises — incentivizes return

window size N: 1000-10000 proofs (tunable). larger = smoother, slower response

### Client Optimization

  client picks d to maximize net profit:
  ```
  net_profit(d) = base_rate * d - gas_cost
  expected_time(d) = 2^d / hashrate_client
  profit_rate(d) = net_profit(d) / expected_time(d)
  ```
  low d: fast first reward, high gas overhead → good for onboarding
  high d: rare proofs, big rewards, low gas ratio → optimal for steady mining

### Allocation Curve

  rewards split between stakers and miners:
  ```
  R_PoS = G * S^alpha
  R_PoW = G * (1 - S^alpha)
  ```

alpha controls the shape:

- alpha = 0.5: neutral prior (square root). equal marginal treatment
- alpha < 0.5: favors stakers at low participation
- alpha > 0.5: favors miners, penalizes excessive staking

miner reward comes from R_PoW share: `base_rate` is calibrated against R_PoW, not G

## 3. Gross vs Net Emission

gross rewards (emission + redistributed fees):
  ```
  G = E(t) + F * (1 - beta)
  ```

net new supply:
  ```
  net_emission = E(t) - F * beta
  ```

when F * beta > E(t): net deflation. emission funded by fees, supply shrinks

## 4. Staking Equilibrium

per-token staking yield:
  ```
  yield = G * S^(alpha-1) / T
  ```

stakers stake until yield equals opportunity cost r:
  ```
  S* = min(1, (G / (r * T))^(1 / (1 - alpha)))
  ```

## 5. PID Update Rules

error signals:
  ```
  e_efficiency = eta_PoW - eta_PoS
  e_fee_coverage = F / E(t) - 1
  ```
  where eta_PoW = D_rate / R_PoW, eta_PoS = (S * T) / R_PoS

alpha update (balance PoW vs PoS efficiency):
  ```
  alpha += Kp_a * e_efficiency + Kd_a * d(e_efficiency)/dt
  ```

beta update (balance burn vs emission):
  ```
  beta += Kp_b * e_fee_coverage + Kd_b * d(e_fee_coverage)/dt
  ```

E(t) is not PID-controlled — it follows the fixed stepped decay curve from [[bostrom/lithium]]

## 6. Gains

| Mode | Kp_a | Kd_a | Kp_b | Kd_b |
  |------|------|------|------|------|
  | conservative (P-only) | 0.005 | 0 | 0.02 | 0 |
  | moderate (PD) | 0.004 | 0.008 | 0.015 | 0.03 |
  | aggressive (PID) | 0.003 | 0.006 | 0.012 | 0.025 |

derivatives estimated via EMA:
  ```
  d_est(t) = lambda * (e(t) - e(t-1)) + (1 - lambda) * d_est(t-1)
  ```
  typical lambda: 0.2-0.4

## 7. On-Proof Handler

```
  function on_proof(proof, state, params, window):
      // 1. validate proof against claimed difficulty
      assert verify(proof.hash, proof.d)
  
      // 2. compute instant reward
      D_rate = window.total_d / window.time_span
      E_target = emission_rate(now())                    // from stepped decay curve
      R_pow_share = 1 - state.S^params.alpha
      base_rate = (E_target * R_pow_share) / D_rate
      reward = base_rate * proof.d
  
      // 3. pay miner + referrals immediately
      mint(proof.miner, reward * (1 - referral_share))
      mint(proof.referral, reward * referral_share)
  
      // 4. update sliding window
      window.push(proof.d, now())
      window.evict_older_than(N)
  
      // 5. PID update (every K proofs or on timer)
      if window.count % K == 0:
          pid_update(state, params, window)
  
      return reward
  ```
```
  function pid_update(state, params, window):
      T = total_supply()
      S = staked() / T
      F = recent_fees(window.time_span)
  
      E = emission_rate(now())
      G = E + F * (1 - params.beta)
      R_pow = G * (1 - S^params.alpha)
      R_pos = G * S^params.alpha
  
      eta_pow = window.total_d / R_pow
      eta_pos = (S * T) / R_pos
      e_eff = eta_pow - eta_pos
      e_cov = F / E - 1
  
      de_eff = ema(e_eff - params.e_eff_prev, params.de_eff)
      de_cov = ema(e_cov - params.e_cov_prev, params.de_cov)
  
      params.alpha = clamp(params.alpha + Kp_a * e_eff + Kd_a * de_eff, 0.3, 0.7)
      params.beta  = clamp(params.beta  + Kp_b * e_cov + Kd_b * de_cov, 0.0, 0.9)
  
      params.e_eff_prev = e_eff
      params.e_cov_prev = e_cov
      params.de_eff = de_eff
      params.de_cov = de_cov
  ```

## 8. Genesis

| Parameter | Initial | Rationale |
  |-----------|---------|-----------|
  | alpha | 0.5 | neutral prior |
  | beta | 0.0 | no burn until stable |
  | N | 5000 | ~hours of proofs at moderate load |
  | K | 100 | PID updates every 100 proofs |

warmup: first N proofs use fixed base_rate (no sliding average yet). P-only PID, wider bounds