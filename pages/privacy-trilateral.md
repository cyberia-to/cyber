# The Privacy Trilateral: ZK + FHE + MPC

*How three cryptographic technologies combine to provide full-spectrum privacy for planetary collective intelligence.*

---

## The Problem

Privacy is not a single problem. It is three problems wearing one name.

1. **Computational integrity**: How do you prove a result is correct without revealing the data that produced it?
2. **Data confidentiality**: How do you compute on data that the computer itself cannot see?
3. **Trust distribution**: How do you prevent any single party from having the power to compromise the system?

No single cryptographic technology solves all three. Each technology in the trilateral — ZK, FHE, MPC — solves exactly one, and has a blind spot that only the other two can fill.

---

## Why One Is Not Enough

Consider a concrete scenario: Alice wants to query the CORE knowledge graph for medical information without revealing her query, and she wants the result to be correct.

**ZK alone** can prove the result is correct, but cannot hide Alice's query from the node that processes it. The prover must see the inputs to generate the proof. Alice's medical query is exposed to whoever runs the computation.

**FHE alone** can encrypt Alice's query so the processing node never sees it. The node traverses the graph and computes the ranking entirely on ciphertexts. But it cannot prove to Alice that the computation was done correctly. She receives an encrypted result and must trust that the node actually ran the tri-kernel algorithm rather than returning garbage or a manipulated answer.

**MPC alone** can split the computation across multiple nodes so no single node sees the full query. But it requires all parties to be online and coordinating synchronously. It doesn't inherently produce a succinct proof of correctness that a third party can verify later. And if enough parties collude, the privacy guarantee breaks.

Each technology has a blind spot. Each blind spot is exactly another technology's strength:

```
                  ┌────────────────────────────────────────────┐
                  │           THE PRIVACY TRIANGLE              │
                  │                                            │
                  │              ZK                            │
                  │           ╱     ╲                          │
                  │     proves       hides                     │
                  │   correctness    witness                   │
                  │        ╱             ╲                     │
                  │      FHE ─────────── MPC                  │
                  │    hides data      distributes             │
                  │    from compute    trust                   │
                  │                                            │
                  │   ZK:  "the answer is correct"             │
                  │   FHE: "I never saw the question"          │
                  │   MPC: "no single party saw anything"      │
                  └────────────────────────────────────────────┘
```

The triangle is not a Venn diagram of overlapping capabilities. It is a structural dependency: each vertex requires the other two to achieve complete privacy.

---

## The Three Technologies

### ZK — Zero-Knowledge Proofs

**One sentence**: Prove a statement is true without revealing why it is true.

**Mechanism**: The prover generates a mathematical proof $\pi$ that a computation was executed correctly. The proof reveals only the public inputs and the result — nothing about the private witness (the secret data used during computation). Verification is fast: $O(\log n)$ work regardless of computation size.

CORE uses STARKs (Scalable Transparent Arguments of Knowledge) — hash-based proofs with no trusted setup and post-quantum security. Every STARK in CORE operates over the Goldilocks field $\mathbb{F}_p$.

**Where ZK appears in CORE:**

**Private transfers.** A transaction proves that energy is conserved (total inputs = total outputs + fee) and that the sender owns the input records, without revealing amounts, sender identity, or receiver identity. The network sees only nullifiers (preventing double-spend) and commitments (encoding new records). The STARK proof guarantees conservation; the commitment scheme guarantees privacy. Circuit cost: ~44,000 constraints.

**Provable computation.** Every state transition in CORE — cyberlink creation, focus update, neural inference, block production — produces a STARK proof. The proof attests that the transition follows protocol rules. Any node can verify any transition without re-executing it. A phone verifies what a datacenter computed. This is how a decentralized network maintains consensus without requiring every node to redo every computation.

**Selective disclosure.** A neuron can prove properties about its state without revealing the state itself. "I have staked more than 10,000 FOCUS" is provable without revealing the exact stake. "My focus contribution to this subgraph exceeds the threshold for voting" is provable without revealing the contribution amount. These are range proofs and threshold proofs — standard ZK primitives composed from the same STARK infrastructure.

**Recursive verification.** A STARK proof can prove the correctness of another STARK verification. This means proofs compose: a proof of 1,000 transactions can be verified in the same time as a proof of 1 transaction. Block proofs aggregate all transaction proofs into a single succinct attestation. Light clients verify entire epochs with one check.

**The ZK blind spot:** The prover must know the witness. Whoever generates the STARK proof sees all the private data. ZK hides data from the *verifier*, not from the *prover*. If the computation must be private even from the entity performing it, ZK alone is insufficient.

### FHE — Fully Homomorphic Encryption

**One sentence**: Compute on encrypted data without ever decrypting it.

**Mechanism**: Data is encrypted under a public key. Arithmetic operations (addition, multiplication) can be performed directly on ciphertexts. The result, when decrypted, equals the result of performing the same operations on the plaintexts. The computer never sees the data — it operates entirely on encrypted values.

CORE uses TFHE (Torus Fully Homomorphic Encryption) instantiated over the Goldilocks field. The ciphertext modulus $q$ equals the STARK field characteristic $p$. This is the critical design choice: the polynomial ring $R_p = \mathbb{F}_p[X]/(X^N + 1)$ used by FHE ciphertexts is a ring of polynomials with Goldilocks coefficients. FHE operations are natively field arithmetic — no cross-domain translation.

**Where FHE appears in CORE:**

**Private queries.** A user encrypts a search query and sends it to a cybergraph node. The node performs graph traversal and ranking computation entirely on ciphertexts. It returns an encrypted result. The node never sees what was queried or what was found. Only the user, holding the decryption key, can read the answer.

This works because the tri-kernel focus computation decomposes into operations that FHE supports: matrix-vector products (homomorphic addition and multiplication), polynomial evaluation (NTT in $R_p$), and function application via Programmable Bootstrapping (PBS). The computation is expensive — orders of magnitude slower than plaintext — but it is mathematically guaranteed that the node learns nothing.

**Private cyberlinks.** A neuron can create edges in the knowledge graph where the source particle, target particle, and weight are all FHE-encrypted. The network cannot see who linked what to what, or with what weight. But the tri-kernel ranking can still compute aggregate focus over encrypted weights, because focus computation uses only addition (homomorphic) and normalization (achievable via bootstrapping). The collective intelligence benefits from the link without knowing its contents.

**Encrypted model inference.** A neural network evaluates on FHE-encrypted inputs. The linear layers (matrix multiplications) use homomorphic addition and multiplication. The nonlinear activations (ReLU, GELU) use Programmable Bootstrapping — the fundamental TFHE operation.

PBS is where the Rosetta Stone identity manifests most clearly. PBS evaluates a lookup table on encrypted data by encoding the function as a test polynomial $v(X) = \sum f(i) \cdot X^i$ and blind-rotating it by the encrypted input. The same lookup table that the STARK uses for proof authentication and the neural network uses for activation is now the FHE bootstrap function. One table, three uses, zero redundancy — because all three systems operate over $\mathbb{F}_p$.

**The FHE blind spot:** Trust is concentrated. A single node holds the encrypted data and performs the computation. If that node is physically compromised (side-channel attacks, memory extraction), the ciphertexts are at risk. More fundamentally, the FHE decryption key is a single point of failure — whoever holds it can decrypt everything. FHE hides data from software but cannot distribute trust across parties.

### MPC — Multi-Party Computation

**One sentence**: Multiple parties jointly compute a function without any party learning any other party's input.

**Mechanism**: Each party holds a share of the secret data. The parties exchange messages according to a protocol. At the end, each party learns the output (or their share of it) and nothing else. The security guarantee is that no coalition smaller than a threshold can reconstruct any party's input.

CORE uses Shamir secret sharing over $\mathbb{F}_p$ for threshold schemes and garbled circuits for general two-party computation. The hash function Poseidon2 was chosen specifically for MPC compatibility — its $x^7$ power-map S-box has multiplicative depth 3, requiring only 3 communication rounds per hash evaluation in secret-shared protocols. Alternative hashes like Tip5 use lookup-based S-boxes that are impossible to evaluate on secret-shared data without exponential overhead.

**Where MPC appears in CORE:**

**Threshold key management.** The FHE decryption key is never held by a single party. Instead, it is split across multiple guardians via Shamir secret sharing. Decryption requires a threshold (e.g., 3-of-5) of guardians to cooperate. No individual guardian can decrypt anything alone. Key generation itself is performed via MPC — the key is born distributed and never exists in complete form on any single machine.

This solves the FHE blind spot directly: the decryption key has no single point of failure because it has no single point of existence.

**Private collective operations.** Multiple neurons want to compute aggregate statistics — average stake in a subgraph, total focus contributed to a topic, consensus ranking across private individual rankings — without revealing their individual values. The neurons engage in an MPC protocol: each contributes their value as secret shares, the protocol computes the aggregate, and each participant learns only the result. Individual contributions remain private.

This is essential for collective intelligence: the network must be able to compute collective properties (aggregate focus, consensus rankings, total energy) from individual contributions (personal stakes, private links, encrypted values) without any party seeing the individual data.

**Distributed randomness.** CORE needs unpredictable, unbiasable random values for PoUW challenge generation, STARK Fiat-Shamir challenges, and protocol parameter selection. An MPC-based distributed randomness beacon ensures no single party can predict or manipulate the output. The protocol uses Poseidon2 as the MPC-friendly commitment function — each participant commits to a random value, then all values are combined via MPC to produce the beacon output.

**The MPC blind spot:** It requires liveness — all participating parties must be online and communicating. It doesn't produce a succinct proof for external verifiers. And the communication cost scales with the number of parties and the complexity of the function. For asynchronous computation (where parties contribute at different times), you need FHE. For proof of correctness that anyone can verify later, you need ZK.

---

## How They Combine

Each pair of technologies fills the other's gap. All three together provide full-spectrum privacy.

### ZK + FHE: Verifiable Encrypted Computation

FHE computes on encrypted data. ZK proves the computation was correct. Together: compute on data you can't see, and prove you did it right.

```
Flow:
  1. Client encrypts input under FHE:     ct = Enc(pk, data)
  2. Server evaluates circuit on ct:       ct' = Eval(circuit, ct)
  3. Server generates STARK proof:         π = Prove(circuit, ct, ct')
  4. Client verifies proof:                Verify(π) → accept/reject
  5. Client decrypts result:               result = Dec(sk, ct')

Properties:
  - Server never sees data                 (FHE)
  - Client knows result is correct         (ZK)
  - Proof is O(log n) to verify            (STARK)
```

This works natively in CORE because FHE operations over $R_p$ are arithmetic operations over $\mathbb{F}_p$ — the same operations that STARK constraints express. The STARK proof covers the FHE evaluation without any cross-domain translation. Proof size: ~200 KB. Verification: <10 ms.

### ZK + MPC: Distributed Proving

MPC distributes trust. ZK produces a proof. Together: multiple parties jointly generate a proof without any party seeing the full witness.

```
Flow:
  1. Each party holds secret share:        [x]_i = share_i(x)
  2. Parties run MPC to evaluate circuit:  [y]_i = MPC_Eval(circuit, [x]_i)
  3. Parties jointly construct STARK proof: π = MPC_Prove([trace]_i)
  4. Anyone verifies proof:                Verify(π) → accept/reject

Properties:
  - No single party sees full input        (MPC)
  - External verifiers trust result        (ZK)
  - Proof is portable and succinct         (STARK)
```

Use case: distributed validation where multiple validators must attest to a state transition without any single validator seeing the complete state.

### FHE + MPC: Threshold Encrypted Computation

FHE encrypts data. MPC manages the keys. Together: compute on encrypted data where no single entity can decrypt, ever.

```
Flow:
  1. Key generation via MPC:               (pk, [sk]_i) = MPC_KeyGen()
  2. Client encrypts under public key:     ct = Enc(pk, data)
  3. Any node evaluates on ciphertext:     ct' = Eval(circuit, ct)
  4. Threshold decryption via MPC:         result = MPC_Dec([sk]_i, ct')

Properties:
  - Data encrypted throughout              (FHE)
  - Key never exists in complete form      (MPC)
  - Any node can compute, none can decrypt (FHE + MPC)
```

This is how CORE handles the FHE key management problem at scale. The network's FHE key is generated by an MPC ceremony at genesis (or periodically refreshed). The public key is known to everyone — anyone can encrypt. The secret key is distributed across guardians. Decryption requires threshold cooperation. No single point of failure. No trusted party.

### ZK + FHE + MPC: The Full Trilateral

All three together. The complete privacy stack.

**Scenario: Private verifiable AI inference on encrypted medical data**

```
  1. MPC key ceremony:     Guardians generate (pk, [sk]_i) — no party sees full key
  2. FHE encryption:       Alice encrypts medical data: ct = Enc(pk, data)
  3. FHE evaluation:       Node runs diagnostic model: ct' = Model(ct)
  4. STARK proof:          Node generates proof π of correct execution
  5. Threshold decryption: Alice requests result from guardians: result = MPC_Dec([sk]_i, ct')
  6. Verification:         Anyone checks Verify(π) → accept

  Properties achieved:
    ✓ Alice's data never exposed                (FHE)
    ✓ Result provably correct                   (ZK)
    ✓ No single point of key compromise         (MPC)
    ✓ Model weights can also be private          (FHE on both sides)
    ✓ Proof is post-quantum secure              (STARK, hash-based)
    ✓ Phone can verify datacenter's work        (O(log n) verification)
```

This is not a theoretical composition — it is a practical protocol where each step uses the same field ($\mathbb{F}_p$), the same hash (Poseidon2), and the same polynomial infrastructure (NTT). The trilateral holds together because the algebraic substrate is shared.

---

## Privacy Tiers

CORE doesn't require full trilateral privacy for every operation. Privacy is opt-in and escalating. Each tier activates more of the trilateral as the privacy requirements increase:

### Tier 0 — Transparent

Everything public. All data visible on-chain.

**Technologies**: ZK only (proof of correctness, not privacy).

**Use case**: Public knowledge graph contributions. A neuron that wants to publicly link two particles and be credited for the link. The STARK proves the link is valid (neuron has sufficient stake, particles exist, weight is within bounds). No secrets involved.

**What's hidden**: Nothing.

### Tier 1 — Private Ownership

Who owns what is hidden. Amounts are hidden. The graph structure (which particles are linked) may be public, but ownership of energy records is private.

**Technologies**: ZK with commitments and nullifiers.

**Mechanism**: Records are Poseidon2 commitments: $\text{commit}(r) = \text{Poseidon2}(\text{particle}, \text{value}, \text{owner}, \text{nonce})$. Spending a record reveals only a nullifier (preventing double-spend) and creates new commitments. The STARK proof guarantees conservation ($\sum \text{inputs} = \sum \text{outputs} + \text{fee}$) without revealing individual values or owners.

**Use case**: Every standard CORE transaction. This is the baseline — the minimum privacy level for all economic activity on the network.

**What's hidden**: Record values, record owners, transaction graph (who paid whom).

### Tier 2 — Private Computation

Inputs and intermediate values are hidden even from the computing node. The computation itself is encrypted.

**Technologies**: ZK + FHE.

**Mechanism**: User encrypts inputs under FHE. A node evaluates the computation on ciphertexts. A STARK proof attests to correct evaluation. The user decrypts the result.

**Use case**: Private knowledge graph queries. Encrypted model inference. Any scenario where the user doesn't trust the processing node with their data.

**What's hidden**: Everything in Tier 1 plus: query content, computation inputs, intermediate states.

### Tier 3 — Distributed Trust

Keys and collective secrets are distributed. No single party can compromise the system even with physical access.

**Technologies**: ZK + FHE + MPC (full trilateral).

**Mechanism**: FHE keys generated via MPC ceremony. Threshold decryption for results. Distributed randomness for challenges. Multi-guardian key recovery.

**Use case**: Institutional-grade privacy. Medical records. Government data. Corporate intelligence. Any scenario where the threat model includes nation-state adversaries or physical compromise of individual nodes.

**What's hidden**: Everything in Tier 2 plus: decryption capability is distributed, no single point of key compromise, protocol parameters are collectively determined.

---

## The Algebraic Foundation

The trilateral is not three independent libraries bolted together. It is three applications of arithmetic over a single field.

| Technology | Algebraic home | Key operation | Field primitive |
|------------|---------------|---------------|-----------------|
| **ZK (STARK)** | $\mathbb{F}_p$ polynomial constraints | FRI commitment (polynomial evaluation + low-degree test) | `ntt` + `p2r` |
| **FHE (TFHE)** | $R_p = \mathbb{F}_p[X]/(X^N+1)$ | Programmable Bootstrapping (blind rotation of test polynomial) | `ntt` + `lut` |
| **MPC (Shamir)** | $\mathbb{F}_p$ secret shares | Threshold reconstruction ($k$ shares → secret via Lagrange interpolation) | `fma` |

All three operate over the Goldilocks field $p = 2^{64} - 2^{32} + 1$. All three use Poseidon2 for commitments and hashing — chosen specifically because its $x^7$ S-box is efficient in all three domains (7 constraints in STARK, multiplicative depth 3 in MPC, moderate depth in FHE). All three benefit from NTT acceleration — the same butterfly network serves FRI folding (ZK), polynomial multiplication (FHE), and, if needed, verifiable secret-share refresh (MPC).

This is why the [GFP](./gfp-spec.md) (Goldilocks Field Processor) accelerates the entire privacy stack with four hardware primitives:

- `fma` (field multiply-accumulate): STARK constraint evaluation, FHE polynomial arithmetic, MPC share recombination
- `ntt` (Number-Theoretic Transform): FRI commitment, PBS polynomial multiply, convolution
- `p2r` (Poseidon2 round): Commitment hashing, nullifier derivation, MPC-friendly randomness
- `lut` (lookup table): STARK lookup argument, FHE test polynomial, neural activation

One chip. Three technologies. Four primitives. One field.

---

## Design Choices and Their Consequences

### Why STARKs, not SNARKs

SNARKs (Groth16, PLONK) produce smaller proofs (~200 bytes vs ~200 KB) but require trusted setup and rely on elliptic curve assumptions that quantum computers break. STARKs are larger but transparent (no setup ceremony), hash-based (post-quantum), and native to the Goldilocks field. For a system meant to outlast current hardware generations, STARK is the only choice.

### Why TFHE, not BGV/CKKS

BGV and CKKS support SIMD-style batching (packing many plaintexts into one ciphertext) which can be faster for matrix operations. But TFHE's Programmable Bootstrapping is uniquely powerful: it evaluates an arbitrary function during noise refresh, eliminating the need for separate bootstrapping and evaluation steps. For CORE, where the primary FHE workload is function evaluation (activations, S-boxes, comparisons), TFHE's PBS is the right primitive. And when instantiated over Goldilocks, PBS uses the same lookup table as the STARK and the neural network — the Rosetta Stone identity.

### Why Poseidon2, not SHA-256 or Tip5

SHA-256 is 50-100× more expensive inside a STARK circuit (bit-oriented operations must be decomposed into field arithmetic). Tip5 is fast in STARKs but uses a lookup-based S-box that is **impossible** for MPC (the lookup must be represented as a degree-$2^{64}$ polynomial on secret-shared data) and **impossible** for FHE (same problem on encrypted data). Poseidon2's $x^7$ power map is the only S-box design that is simultaneously efficient in ZK, viable in MPC (depth 3), and evaluable under FHE. It is not the optimal choice for any single domain — but it is the only choice that works across all three.

This is the defining pattern of the trilateral: every component is chosen for cross-domain compatibility, not single-domain optimality. The system is optimized at the architecture level, not the component level.

### Why Goldilocks, not BN254 or BabyBear

BN254 is the standard SNARK field — optimized for elliptic curve pairings that CORE doesn't use (and that quantum computers break). BabyBear (31-bit) is faster per-operation but too small for meaningful FHE (ciphertext noise requires >32-bit modulus). Goldilocks is the sweet spot: 64-bit (fits in one CPU register), prime (proper field structure), NTT-friendly ($2^{32}$ roots of unity for both STARK and FHE), and large enough for FHE noise management. No other field satisfies all four constraints simultaneously.

---

## Threat Model Summary

| Threat | Technology | Defense mechanism |
|--------|-----------|-------------------|
| Node sees user data | FHE | Computation on encrypted data; node never sees plaintext |
| Node returns wrong result | ZK | STARK proof of correct execution; verifiable by anyone |
| Single key holder is compromised | MPC | Threshold key distribution; no single point of failure |
| Quantum computer breaks crypto | ZK (STARK) | Hash-based proofs; no elliptic curve assumptions |
| Surveillance of transaction graph | ZK | Commitments + nullifiers hide sender, receiver, amounts |
| Collusion of minority of nodes | MPC | Threshold schemes; security holds below threshold |
| Physical access to server | FHE + MPC | Data encrypted (FHE) + key distributed (MPC) |
| Man-in-the-middle | ZK | Proofs are non-interactive and self-authenticating |

Full-spectrum privacy means: no single attack vector compromises the system. Each row in the table requires a different technology. The trilateral covers them all.

---

## Summary

```
ZK proves correctness.
FHE hides data from computation.
MPC distributes trust across parties.

Together:
  Compute on data no one can see.
  Prove the computation was correct.
  Ensure no single party can compromise the system.

All three over one field.
All three through one chip.
All three from genesis.
```

Privacy is not a feature. It is the condition under which collective intelligence is possible. Without privacy, nobody contributes real data. Without real data, the graph is empty. Without the graph, there is no intelligence. The trilateral is not optional — it is load-bearing.

ZK + FHE + MPC. Three technologies. One field. Complete privacy.
