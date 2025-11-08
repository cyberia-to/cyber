# H-Based Economy Whitepaper
- ## Overview
  We propose a dual-token economic system with two distinct roles:
- **CYB**: the value anchor, designed to be scarce, stable, and accrue long-term value.
- **H**: the liquidity engine, designed for high velocity, stability in price, and functional utility.
  
  The system balances scarcity (CYB) and liquidity (H) through adaptive monetary policy, guided by a premium/discount mechanism.
  
  ---
- ## Core Principles
  1. **CYB benefits from stability**, H benefits from activity.
  2. Scarcity and liquidity are opposing forces — we assign one to each token to avoid conflict.
  3. No zero-sum wins — policy ensures cooperation between CYB and H holders.
  4. Adaptive responses to market conditions based on a single macro signal: premium = (H price / structural D/E ratio).
  
  ---
- ## Key Metrics
- **Price (p)**: market rate of H in CYB.
- **Structural D/E (d)**: H supply / CYB staked (nominal units).
- **Premium (prem)**: p / d.
- **Velocity (vel)**: proportion of H used for gas per period.
- **Spread (spr)**: average market bid-ask spread for H pairs.
  
  ---
- ## Monetary Policy Mechanisms
- ### 1. Adaptive Gas-H Allocation
  Gas fees paid in H are split into three buckets:
- **CYB Buyback/Burn**: funds scarcity of CYB.
- **H-holder Rewards**: tenure yield, spend rebates, LP incentives.
- **Liquidity Safety**: PD/MM subsidies, circuit breakers.
  
  **Adaptive Split:**
- **Premium > 1 (H scarce)**: More to H rewards, less to CYB buybacks.
- **Premium ~ 1 (Balanced)**: Even split.
- **Premium < 1 (H cheap)**: More to CYB buybacks, less to H rewards.
- ### 2. Optional Minting of H
  Stakers may choose whether to mint H upon staking CYB. This self-selection prevents oversupply and allows the market to adjust naturally.
- Mint rebates when H scarce.
- Mint fees when H abundant.
- ### 3. Continuous Tenure Reward
  Reward long-term H holding in CYB at all times.
- Base reward always active.
- Countercyclical multiplier increases rewards when H is abundant.
- Rewards require H to be staked or escrowed.
- ### 4. Spend Incentives
  Encourage gas spending to control liquidity:
- Rebates higher when H is cheap to soak up excess liquidity.
- Rebates lower or zero when H is scarce.
- ### 5. Soft Costs Without Burning
  No H burn for scarcity. Instead:
- Micro-fees/spreads on mint/redeem recycled to rewards and liquidity programs.
- Priority rules favor long-tenured H.
- ### 6. Liquidity Infrastructure
- Protocol market maker (PHMM) funded by liquidity bucket.
- Primary dealer (PD) program with obligations and rewards.
- Circuit breakers and redemption smoothing to avoid liquidity spirals.
  
  ---
- ## Premium/Discount Feedback Loop
  1. Measure p, d, prem.
  2. Adjust gas-H split, mint fees/rebates, and spend incentives.
  3. Balance CYB scarcity and H liquidity adaptively.
  
  ---
- ## Advantages of the Model
- Keeps CYB scarce without starving H.
- Maintains H liquidity and price stability.
- Rewards both long-term holding and high turnover in a countercyclical way.
- Allows self-regulating debt issuance without central governance micro-management.
  
  ---
- ## Governance Considerations
- Parameters (bands, fees, splits) change slowly with rate limits.
- Hard-coded circuit breakers for stability.
- Policy transparent and rule-based to avoid manipulation.
  
  ---
- ## Conclusion
  This dual-token system creates a cooperative game between value and liquidity. By separating scarcity (CYB) and utility (H) and using premium/discount signals to adapt rewards and buybacks, the protocol maintains long-term value growth while keeping the economy liquid and functional.