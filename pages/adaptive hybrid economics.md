tags:: cyber, uhash
- # Adaptive Hybrid Economics
- ## Minimal Implementation Spec
- all values in tokens. no price oracle needed.
- ---
- ## 1. Parameters
- | Symbol | Domain | Description |
  |--------|--------|-------------|
  | T | R+ | total token supply |
  | S | [0, 1] | staking ratio (staked / T) |
  | H | R+ | hashrate (normalized) |
  | F | R+ | fees collected per epoch (tokens) |
  | alpha | [0.3, 0.7] | allocation curve exponent |
  | phi | [phi_min, 0.05] | issuance rate (fraction of T per epoch) |
  | beta | [0, 0.9] | fee burn rate |
- ## 2. Allocation Curve
- rewards split between stakers and miners:
  ```
  R_PoS = G * S^alpha
  R_PoW = G * (1 - S^alpha)
  ```
- alpha controls the shape:
	- alpha = 0.5: neutral prior (square root). equal marginal treatment
	- alpha < 0.5: favors stakers at low participation
	- alpha > 0.5: favors miners, penalizes excessive staking
- ## 3. Gross vs Net Emission
- gross rewards (total tokens emitted + redistributed per epoch):
  ```
  G = phi * T + F * (1 - beta)
  ```
- net new supply per epoch:
  ```
  net_emission = phi * T - F * beta
  ```
- when F * beta > phi * T: net deflation. emission funded by fees, supply shrinks
- ## 4. Staking Equilibrium
- per-token staking yield:
  ```
  yield = G * S^(alpha-1) / T
  ```
- stakers stake until yield equals opportunity cost r:
  ```
  S* = min(1, (G / (r * T))^(1 / (1 - alpha)))
  ```
- ## 5. PID Update Rules
- error signals:
  ```
  e_efficiency = eta_PoW - eta_PoS
  e_fee_coverage = F / (phi * T) - 1
  ```
  where eta_PoW = H / R_PoW, eta_PoS = (S * T) / R_PoS
- alpha update (balance PoW vs PoS efficiency):
  ```
  alpha += Kp_a * e_efficiency + Kd_a * d(e_efficiency)/dt
  ```
- beta update (balance burn vs emission):
  ```
  beta += Kp_b * e_fee_coverage + Kd_b * d(e_fee_coverage)/dt
  ```
- phi update (adjust floor when fees cover it):
  ```
  phi -= Kp_f * e_fee_coverage    (only when system is healthy)
  ```
- ## 6. Gains
- | Mode | Kp_a | Kd_a | Kp_b | Kd_b | Kp_f |
  |------|------|------|------|------|------|
  | conservative (P-only) | 0.005 | 0 | 0.02 | 0 | 0.005 |
  | moderate (PD) | 0.004 | 0.008 | 0.015 | 0.03 | 0.004 |
  | aggressive (PID) | 0.003 | 0.006 | 0.012 | 0.025 | 0.003 |
- derivatives estimated via EMA:
  ```
  d_est(t) = lambda * (e(t) - e(t-1)) + (1 - lambda) * d_est(t-1)
  ```
  typical lambda: 0.2-0.4
- ## 7. Epoch Update
- ```
  function epoch_update(state, params, history):
      T = total_supply()
      S = staked() / T
      H = hashrate()
      F = fees()
  
      G = params.phi * T + F * (1 - params.beta)
      R_pow = G * (1 - S^params.alpha)
      R_pos = G * S^params.alpha
  
      eta_pow = H / R_pow
      eta_pos = (S * T) / R_pos
      e_eff = eta_pow - eta_pos
      e_cov = F / (params.phi * T) - 1
  
      de_eff = ema(e_eff - history.e_eff_prev, history.de_eff)
      de_cov = ema(e_cov - history.e_cov_prev, history.de_cov)
  
      params.alpha = clamp(params.alpha + Kp_a * e_eff + Kd_a * de_eff, 0.3, 0.7)
      params.beta  = clamp(params.beta  + Kp_b * e_cov + Kd_b * de_cov, 0.0, 0.9)
      params.phi   = clamp(params.phi   - Kp_f * e_cov, PHI_MIN, 0.05)
  
      history.e_eff_prev = e_eff
      history.e_cov_prev = e_cov
      history.de_eff = de_eff
      history.de_cov = de_cov
  
      return params, history
  ```
- ## 8. Genesis
- | Parameter | Initial | Rationale |
  |-----------|---------|-----------|
  | alpha | 0.5 | neutral prior |
  | beta | 0.0 | no burn until stable |
  | phi | 0.03 | conservative floor |
- warmup: first ~52 epochs use P-only, wider bounds, no integral term
