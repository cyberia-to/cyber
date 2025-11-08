# Title: Reward Function Design for Incentivizing Convergence in Decentralized Knowledge Graphs
## Abstract

We propose a reward mechanism for decentralized systems built on the Collective Focus Theorem (CFT), where agents submit microblocks to a DAG representing partial computations toward a converging focus vector \( \pi \). The reward mechanism ensures that agents are fairly incentivized for submitting verified, convergence-aligned work. Our design combines multiple convergence indicators into a hybrid reward function, balancing simplicity, robustness, game resistance, and long-term epistemic value.
## 1. Introduction

The Collective Focus Theorem proves that token-weighted random walks in authenticated, directed graphs converge to a unique stationary distribution \( \pi \). To compute this distribution in a distributed fashion, agents ("neurons") submit microblocks containing partial focus updates. However, to encourage meaningful participation and prevent manipulation, we need a verifiable and fair reward system.

This paper defines and compares several candidate reward functions and proposes a hybrid approach optimized for long-term convergence and resistance to manipulation.
## 2. Design Goals

The reward function must:
- Incentivize accurate computation toward \( \pi \)
- Be locally verifiable
- Prevent reward extraction via oscillation or noise
- Reward both short-term and long-term contributions
- Scale with network size
## 3. Microblock Data Model

Each microblock includes:
- Parent block references
- Subgraph slice and cyberlinks
- Token state \( t_j \)
- Input and output focus values: \( \pi^{(t)}, \pi^{(t+1)} \)
- Proof hash and signature
## 4. Candidate Reward Functions
### 4.1 \( \Delta\pi \) Norm-Based Reward

\[ R_1 = \alpha \cdot \sum_j |\pi_j^{(t+1)} - \pi_j^t| \]

**Pros:** Simple, easy to verify  
**Cons:** Gameable by oscillation
### 4.2 Entropy Reduction Reward

\[ R_2 = \beta \cdot (H(\pi^t) - H(\pi^{t+1})) \]

Where entropy is \( H(\pi) = -\sum_j \pi_j \log \pi_j \)

**Pros:** Rewards semantic sharpening  
**Cons:** Computationally heavier
### 4.3 Cosine Similarity to Target

\[ R_3 = \gamma \cdot \text{cos}(\pi^{(t+1)}, \pi^*) \]

**Pros:** Alignment with oracle\( \pi^* \)  
**Cons:** Requires trusted reference; hard for local compute
### 4.4 Spectral Gap Improvement

\[ R_4 = \delta \cdot (\lambda_2^t - \lambda_2^{t+1}) \]

Where \( \lambda_2 \) is the second eigenvalue of the transition matrix

**Pros:** Measures global convergence speedup  
**Cons:** Expensive and non-local
### 4.5 Predictive Alignment Reward

\[ R_5 = \epsilon \cdot \text{align}(\pi_j^{(t+1)}, \pi_j^{T}) \]

**Pros:** Favors early correct contributions  
**Cons:** Requires delayed validation
## 5. Composite Hybrid Model

We propose the following hybrid reward function:

\[
\text{Reward} = \alpha \cdot \Delta\pi + \beta \cdot \Delta H + \gamma \cdot \text{DAGWeight} + \epsilon \cdot \text{AlignmentBonus}
\]

Where:
- \( \Delta\pi \): total L1 delta in focus vector
- \( \Delta H \): entropy drop
- DAGWeight: number of descendant blocks referencing this one
- AlignmentBonus: comparison with future \( \pi^T \) (optional, delayed)
## 6. Implementation Strategy
- Fast local rewards use \( \Delta\pi \) and \( \Delta H \)
- Checkpoints add alignment and spectral verification bonuses
- Validators sample and verify blocks probabilistically
## 7. Conclusion

This hybrid function aligns incentives with convergence, penalizes noise, rewards foresight, and supports scaling. It anchors computation in provable improvements to \( \pi \), the collective intelligence substrate.

Future work includes benchmarking reward trajectories and testing robustness against coordinated spam and adversarial updates.