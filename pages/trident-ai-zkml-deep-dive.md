# Trident and the Future of Verifiable AI

## Why the Next Generation of zkML Won't Start from ONNX — It Will Start from Prime Fields

---

### The Problem Everyone Is Solving Wrong

The zero-knowledge machine learning industry has a dirty secret: it's building on sand.

Every major zkML framework — EZKL, DeepProve, JOLT Atlas, zkPyTorch — follows the same pipeline. Take a neural network trained in PyTorch or TensorFlow. Export it to ONNX. Convert floating-point weights and activations into fixed-point integers. Translate those integers into arithmetic constraints over a finite field. Generate a zero-knowledge proof. Verify on-chain.

This pipeline has a fundamental flaw: it starts in the wrong representation and spends enormous effort converting to the right one.

Neural networks live in float32. Zero-knowledge proofs live in $\mathbb{F}_p$. The conversion between them — quantization — is where everything breaks. Accuracy degrades across layers. Operators that are trivial in floating-point (softmax, LayerNorm, GELU) become nightmares in field arithmetic. ONNX has over 120 operators; most zkML frameworks support fewer than 50. The overhead is staggering: EZKL reports 100,000× to 1,000,000× slowdown for general computations. Even specialized frameworks like DeepProve, which achieves 50-150× faster proving than EZKL, still operate orders of magnitude above native execution.

And there's a deeper problem that almost nobody talks about: most of these frameworks use SNARKs — proof systems built on elliptic curve pairings that will be broken by quantum computers. EZKL uses Halo2. Modulus Labs uses custom SNARKs. The proofs being generated today have an expiration date.

Trident offers a different path. Not a better converter from floats to fields. A language where computation is born in the field.

---

### What Trident Actually Is

Trident is a smart contract language for the Neptune blockchain. Its native data type is `Field` — an element of the Goldilocks prime field $\mathbb{F}_p$ where $p = 2^{64} - 2^{32} + 1$. Every variable, every operation, every function call compiles to arithmetic over this field. The compilation target is Triton VM, which generates STARK proofs — hash-based, post-quantum secure, no trusted setup.

This was designed for provable financial computation: private transactions, verifiable lock scripts, recursive proof composition. But the architecture has a property that its designers may not have fully appreciated.

Neural networks are arithmetic circuits. Trident programs are arithmetic circuits. They are the same thing.

A matrix multiplication — the core operation of every neural network — is a sequence of multiply-accumulate operations over field elements. In Trident, this is native. No conversion. No quantization. No overhead beyond the computation itself.

```
fn matmul(a: &[Field], b: &[Field], rows: u32, inner: u32, cols: u32) -> Vec<Field> {
    let mut result = vec![Field::zero(); rows * cols];
    for i in 0..rows {
        for j in 0..cols {
            for k in 0..inner {
                result[i * cols + j] += a[i * inner + k] * b[k * cols + j];
            }
        }
    }
    result
}
```

This isn't pseudocode. This is a function that compiles to Triton VM, executes, and produces a STARK proof of correct execution. The proof is generated automatically. The verification can happen on any blockchain.

---

### The Quantization Problem — Eliminated

Quantization is the single largest source of pain in zkML. Here is what happens when EZKL processes a neural network:

**Step 1**: A PyTorch model uses float32 weights. A weight might be 0.00374291.

**Step 2**: EZKL must represent this in a finite field. It multiplies by a scaling factor (say $2^{16}$) and rounds: $0.00374291 \times 65536 \approx 245$. The weight becomes the field element 245.

**Step 3**: Every multiplication now produces results scaled by $2^{32}$ (the product of two $2^{16}$-scaled values). Rescaling is needed after every multiply — more field operations, more constraints, more proof overhead.

**Step 4**: Nonlinear activations are catastrophic. ReLU requires a comparison (is x > 0?), which in a field has no natural meaning — field elements aren't ordered. Frameworks use range checks or bit decomposition, each adding hundreds of constraints per activation.

**Step 5**: Softmax requires exponentiation and division over all elements. In a field, `exp(x)` must be approximated by polynomial, and division requires computing multiplicative inverses. A single softmax layer over 512 elements can generate tens of thousands of constraints.

**Step 6**: The accumulated quantization error means the ZK circuit's output may differ from the original model's output. EZKL's documentation acknowledges this explicitly.

Now consider Trident's approach.

**There is no Step 1.** The model is trained directly in $\mathbb{F}_p$. Weights are field elements from the start. There is nothing to quantize because there were never any floats.

**There is no Step 3.** Field multiplication in $\mathbb{F}_p$ produces a field element. No rescaling. The result is already in the correct representation. $a \times b \mod p$ is a single field operation — one gate in the arithmetic circuit.

**Step 4 becomes elegant.** Nonlinear activations are implemented via lookup tables — the same mechanism Triton VM uses for its Tip5 hash function's S-box. The lookup argument in the STARK proof authenticates that the activation function was applied correctly, with zero additional constraints beyond the lookup itself. ReLU, GELU, SiLU — all become single lookup operations.

**Step 5 becomes tractable.** In $\mathbb{F}_p$ (prime field), every nonzero element has a multiplicative inverse. Division is a native operation: $a / b = a \times b^{p-2} \mod p$ (Fermat's little theorem). Softmax becomes: exponentiate each element (via lookup or polynomial), sum, divide each by the sum. All native field operations.

**Step 6 doesn't exist.** There is no quantization error because there is no quantization.

The difference is not incremental. It is categorical. EZKL adds constraints to handle the gap between floats and fields. Trident has no gap.

---

### The Lookup Argument: Where Hash Functions Meet Neural Networks

This is the deepest technical insight, and it deserves elaboration.

Triton VM's STARK prover uses a cryptographic hash function called Tip5. Tip5's internal structure includes a nonlinear S-box — a function that maps field elements to field elements in a way that resists algebraic attacks. The S-box is implemented as a lookup table: a precomputed mapping from every possible input to its output, authenticated by a lookup argument in the STARK proof.

The lookup argument says: "the prover claims this input mapped to this output. The verifier checks that this (input, output) pair exists in the lookup table." The proof cost is essentially constant regardless of the function's complexity — it's one lookup, authenticated once.

Neural network activation functions are also nonlinear maps from field elements to field elements. ReLU: $f(x) = \max(0, x)$. GELU: $f(x) = x \cdot \Phi(x)$. SiLU: $f(x) = x \cdot \sigma(x)$. Each can be precomputed as a lookup table over $\mathbb{F}_p$ (or a relevant subset).

The STARK proof mechanism that authenticates Tip5's S-box is identical to the mechanism that would authenticate a ReLU activation. The prover says "I applied ReLU to input $x$ and got output $y$." The verifier checks the lookup table. One lookup. Done.

This is not a coincidence. Cryptographic S-boxes and neural network activations serve the same mathematical purpose: they inject nonlinearity into an otherwise linear system. Cryptographers need nonlinearity to resist algebraic attacks. Neural network designers need nonlinearity to learn non-trivial functions. The mechanism for proving nonlinearity in a STARK is the same in both cases.

Trident inherits this for free. Any function expressible as a lookup table over $\mathbb{F}_p$ becomes a zero-overhead provable activation function. The hash function's security guarantees (the S-box is a permutation with maximal algebraic degree) translate to desirable neural network properties (high expressiveness, no dead zones, information-preserving).

This means Trident's activation functions are not approximations of the "real" activations (as in EZKL, where ReLU must be simulated with range checks). They are exact, native, zero-overhead field operations whose correctness is guaranteed by the same STARK machinery that secures the blockchain.

---

### What a Trident Neural Network Standard Library Looks Like

The `std.nn` library would provide:

**Linear layers** — the foundation of every neural network. Matrix multiply-accumulate over $\mathbb{F}_p$. Native operations, zero overhead.

**Convolutional layers** — sliding window dot products. Same field arithmetic, different access pattern. Bounded loops over spatial dimensions compile to fixed-depth circuits.

**Attention mechanisms** — the core of transformers. Query-key dot product, softmax over attention scores, value aggregation. All field arithmetic: dot products are native, softmax uses field inversion, aggregation is multiply-accumulate.

**Lookup-table activations** — ReLU, GELU, SiLU, Swish, and any custom activation. Implemented via Triton VM's lookup argument. Proof cost is independent of the activation function's mathematical complexity.

**Normalization** — LayerNorm and BatchNorm require computing means and variances. In $\mathbb{F}_p$: sum elements (field addition), divide by count (field inversion), compute variance (sum of squared differences). All native.

**Embedding layers** — lookup from token ID to weight vector. This is literally a lookup table — the same mechanism as activations, reused for a different purpose.

**Loss functions** — cross-entropy, MSE, and others. Computable in field arithmetic. The entire training loop, not just inference, can be proven.

Each function is pure Trident code — no external dependencies, no C++ kernels, no CUDA. The entire neural network compiles to a single arithmetic circuit over $\mathbb{F}_p$, which Triton VM proves as a unit.

---

### The ONNX Bridge: Importing the World's Models

Trident doesn't need to replace PyTorch. It needs to consume its output.

The import path works as follows:

**PyTorch → ONNX export.** Standard practice, well-supported, no changes needed.

**ONNX → Trident transpiler.** Each ONNX operator maps to a `std.nn` function call. The computational graph becomes a Trident program. Float32 weights are quantized to $\mathbb{F}_p$ elements — but this quantization happens once, at import time, not at proof time.

The critical difference from EZKL's approach: EZKL converts ONNX directly to Halo2 constraints, producing a monolithic circuit. Trident produces readable source code — a `.tri` file that a developer can inspect, modify, optimize, and extend. The neural network becomes a program, not a black box.

**Trident → TASM → STARK proof.** The transpiled program compiles and runs like any other Trident program. The proof is automatic.

The export path enables interoperability in the other direction:

**Trident → ONNX export.** Extract the `std.nn` computational graph, convert $\mathbb{F}_p$ weights back to float32, generate an ONNX file. This allows Trident-native models to be inferenced in PyTorch, TensorFlow, or any ONNX-compatible runtime — useful for development, testing, and environments where ZK proof isn't needed.

---

### Training in the Field: A New Paradigm

Current zkML focuses almost entirely on proving inference — running a pre-trained model and generating a proof that the output is correct. Training remains in float32 land. This creates a trust gap: you can prove inference was correct, but you cannot prove training was correct.

Trident enables provable training. The entire training loop — forward pass, loss computation, backpropagation, weight update — is field arithmetic. The STARK proof covers the complete training process.

**Gradient computation in $\mathbb{F}_p$:** Backpropagation is chain-rule multiplication of Jacobians — matrix operations over the same field. The gradient of a linear layer is a transpose-multiply. The gradient of a lookup-table activation is another lookup (the derivative table, precomputed alongside the activation table).

**Optimizer in $\mathbb{F}_p$:** SGD is $w \leftarrow w - \eta \cdot g$ — field subtraction and multiplication. Adam requires running averages (field arithmetic) and square root (field exponentiation with appropriate exponent). All native.

**Provable training claims:**

- "This model was trained on this dataset for this many epochs with this optimizer" — proven by STARK
- "This model achieves accuracy above threshold T on test set D" — proven by STARK
- "This model's weights have not been modified since training" — proven by committing to the weight hash

This enables a model marketplace where sellers prove their training claims without revealing weights (zero-knowledge), buyers verify proofs before purchasing, and the entire transaction settles on-chain.

---

### Post-Quantum Security: The Elephant in the Room

Here is a fact that the zkML industry has not yet confronted: almost every deployed zkML system uses proof systems that will be broken by quantum computers.

- **EZKL** uses Halo2 (polynomial commitments based on discrete log assumption)
- **DeepProve** uses sumcheck + lookup arguments (security depends on collision-resistant hashing — survivable, but the polynomial commitment layer may not be)
- **Modulus Labs / Remainder** uses custom SNARKs (elliptic curve dependent)
- **ZK-DeepSeek** uses recursively composed SNARKs (elliptic curve dependent)
- **Circom / Noir** use Groth16 / Plonk over BN254 (directly broken by Shor's algorithm)

Giza, using StarkWare's STWO prover, is the exception — STARKs are hash-based and post-quantum.

Trident is STARK-native. Every proof generated by Triton VM relies exclusively on hash functions (Tip5) and polynomial commitments over $\mathbb{F}_p$ — no elliptic curves, no pairings, no discrete log. The security reduces to the collision resistance of the hash function, which quantum computers can attack only with Grover's square-root speedup — manageable by doubling the hash output size.

This means every Trident neural network proof is automatically post-quantum secure. Not as an option, not as a configuration flag, not as an expensive upgrade — as a mathematical consequence of the proof system's design.

The implications for verifiable AI are profound. An AI agent's decision proof generated today will remain verifiable in 2040, after quantum computers have matured. A model marketplace built on Trident proofs will not need to migrate proof systems when quantum computing arrives. Regulatory compliance proofs will remain valid across technological epochs.

Every zkML system built on SNARKs today will need to be rebuilt. Trident won't.

---

### Comparison with the Field

**vs. EZKL (Halo2-based, ONNX → zkSNARK)**

EZKL is the most mature general-purpose zkML framework. It accepts any ONNX model and produces a Halo2 proof. This generality is its strength and its weakness. Proof sizes are 15× larger than alternatives. Verification keys can reach megabytes. Proving time for even medium models runs to minutes or hours. The SNARK-based proof system is not post-quantum.

Trident's advantage: native field arithmetic eliminates quantization overhead, STARK proofs are post-quantum, and the compiled circuit is optimized for Triton VM rather than generic Halo2.

**vs. DeepProve / Lagrange (GKR-based)**

DeepProve is the fastest zkML prover, achieving 50-150× speedup over EZKL through GKR (Goldwasser-Kalai-Rothblum) interactive proofs. It's optimized for the matrix multiplications that dominate neural network inference. But it still starts from ONNX, still quantizes, and its proof system security is still under analysis for post-quantum resistance.

Trident's advantage: no ONNX conversion step, field-native computation, and proven post-quantum security via STARKs. DeepProve's GKR approach could potentially be integrated as an alternative backend for Trident's IR — the arithmetic circuit representation is compatible.

**vs. Giza / Cairo (STARK-based, Starknet)**

Giza is closest to Trident's approach: STARK-based proofs, field-native arithmetic, on-chain deployment via Starknet. Its LuminAIR framework using STWO prover shows the viability of STARK-verified AI agents for DeFi.

Trident's advantages: Goldilocks field ($2^{64} - 2^{32} + 1$) is computationally faster than Stark252 ($2^{251} + 17 \cdot 2^{192} + 1$) for 64-bit-native hardware. Trident's `divine()` primitive provides structured witness injection that maps cleanly to optimization algorithms (and to quantum oracles — a connection Cairo lacks entirely). Trident's three-level architecture (Execute Anywhere / Prove Anywhere / Platform Superpowers) enables cross-chain deployment that Cairo cannot offer.

**vs. Ritual (EVM++ AI infrastructure)**

Ritual is not a proof system — it's an orchestration layer. Its EVM++ with ONNX sidecars enables any AI model to be called from smart contracts, with computational integrity verified through ZK proofs, TEE attestations, or optimistic verification. Ritual delegates the actual proving to external systems.

Trident's advantage: Trident is the proving system. It doesn't delegate proof generation — it generates proofs as a first-class capability. A Trident neural network could run as a Ritual sidecar, providing STARK-verified inference to Ritual's network — making Trident the proving backend for Ritual's orchestration frontend.

**vs. Inference Labs (ZK-Verified Inference Network)**

Inference Labs builds the verification layer for AI agents, using zero-knowledge proofs to verify inference honesty while preserving IP privacy. They've produced over 160 million ZK proofs via their Bittensor subnet. Their approach combines cryptographic verification with economic incentives (slashing for dishonest execution via EigenLayer).

Trident's advantage: Inference Labs is framework-agnostic and integrates multiple proving backends (EZKL, DeepProve, Circom, JOLT). Trident could serve as one of these backends — specifically, the post-quantum-secure one. Integration with Inference Labs' network would give Trident-proven models access to decentralized proof generation and economic security.

---

### The divine() Primitive: AI Meets Zero-Knowledge

Trident's `divine()` function tells the prover: "inject a value here that satisfies the subsequent constraints." In ZK computation, this is witness injection — the mechanism by which private information enters the proof without being revealed.

For neural networks, `divine()` serves multiple purposes:

**Private weights.** The model owner provides weights via `divine()`. The STARK proof verifies inference was correct without revealing the weights. This is privacy-preserving inference — the core use case of zkML — achieved as a natural consequence of Trident's proof architecture rather than as an additional cryptographic layer.

```
fn private_inference(input: [Field; N]) -> [Field; M] {
    let weights = divine()        // private: model owner provides
    let bias = divine()           // private: model owner provides
    let output = linear(input, weights, bias)
    let activated = relu_lookup(output)
    activated                     // public: inference result
}
```

**Optimization search.** For AI agents that must find optimal actions in a large space, `divine()` lets the prover inject the solution. The constraints verify optimality. The proof guarantees the solution is valid without revealing the search process.

```
fn optimal_action(state: [Field; S], constraints: &RiskParams) -> Field {
    let action = divine()         // prover searches for optimal action
    let value = evaluate(state, action)
    assert(satisfies_constraints(action, constraints))
    assert(is_local_optimum(state, action, value))
    action
}
```

**Knowledge distillation.** A large model provides outputs via `divine()`. A small model is trained to reproduce them. The STARK proof verifies that the distillation target was faithfully reproduced — provable knowledge transfer.

**Adversarial robustness testing.** An adversary tries to find inputs that fool the model. `divine()` injects adversarial examples. The constraints check whether the model misclassifies. The proof either demonstrates a vulnerability (adversarial example found) or certifies robustness (no adversarial example satisfies constraints within the bounded search).

In the quantum compilation target, every `divine()` becomes a Grover oracle query — providing quadratic speedup on witness search. The AI applications of `divine()` are simultaneously quantum-accelerable without code changes.

---

### Concrete Use Cases

**Verifiable Credit Scoring.** A bank's credit model runs in Trident. Applicant data is private input. Model weights are private witness. Credit decision is public output. STARK proof verifies the decision follows from the model and data — without revealing either. Regulators can verify the model was applied consistently across all applicants by checking proofs. The model's fairness properties (no discriminatory features used) can be encoded as constraints.

**Autonomous DeFi Agents.** A trading agent's neural network policy runs in Trident on Neptune. Each trade decision produces a STARK proof: "this specific model, with these frozen weights, evaluated this market data and produced this trade." The proof is on-chain. Investors can verify the agent follows its stated strategy. The model weights remain private (proprietary trading strategy). MEV attacks are detectable: any deviation from the proven policy is visible.

**Decentralized Model Marketplace.** Model developers train in $\mathbb{F}_p$, prove training correctness and accuracy claims, list models on-chain. Buyers verify proofs before purchasing inference access. Inference runs in Trident, produces proofs, settles payments via smart contracts. The entire pipeline — training proof, accuracy proof, inference proof, payment — runs through one language on one field.

**Provable Content Authenticity.** An AI-generated image has a Trident inference proof attached: "this specific model produced this specific output from this specific prompt." Deepfake detection becomes unnecessary — authentic AI content is provable, and non-proven content is suspect. The model identity (hash of weights) is committed on-chain without revealing the weights themselves.

**Medical AI with Regulatory Compliance.** A diagnostic model runs in Trident. Every diagnosis produces a STARK proof. The proof verifies: the FDA-approved model version was used, the input data was properly preprocessed, the output followed from correct execution. Regulators audit proofs rather than re-running computations. Patient data never leaves the proof system (zero-knowledge). Proof validity is permanent and post-quantum secure.

---

### The Path Forward

Building Trident into an AI platform requires concrete engineering steps:

**Immediate (0-6 months):** Implement `std.nn` core — linear layers, lookup-table activations, basic loss functions. Build ONNX import for simple architectures (MLPs, small CNNs). Demonstrate: import a PyTorch MNIST classifier, prove inference on Triton VM, verify on Neptune.

**Near-term (6-18 months):** Extend `std.nn` to attention mechanisms, embedding layers, normalization. Build ONNX import for transformers. Implement provable training loop. Publish benchmarks against EZKL and DeepProve showing proof time reduction from eliminated quantization overhead.

**Medium-term (18-36 months):** Develop Trident AI agent framework with on-chain deployment. Build model marketplace contracts. Integrate with Ritual as STARK-verified sidecar. Integrate with Inference Labs as post-quantum proving backend. Develop quantum compilation backend for std.nn operations.

**Long-term (36+ months):** Field-native training frameworks that replace PyTorch for applications where provability matters more than raw speed. Quantum-accelerated proving for large models. Standardize Trident as the language of verifiable AI.

---

### Why This Matters

The convergence of AI and cryptography is inevitable. AI agents will manage billions in assets. They will make medical decisions, legal assessments, financial predictions. They will interact with each other autonomously across trust boundaries. Every one of these applications needs verifiability.

The current approach — train in float, convert to field, prove with SNARKs — is a temporary bridge. It will not survive the quantum transition. It will not scale to the models that matter. It will not provide the developer experience that adoption requires.

Trident offers something different: a world where computation starts in the field, lives in the field, and is proven in the field. Where the neural network IS the arithmetic circuit. Where the proof IS the execution trace. Where post-quantum security IS the default.

The architecture already exists. The field is already prime. The only question is who builds the standard library first.

---

*The Trident language specification and Triton VM implementation are open source. The ideas in this paper are meant to catalyze development, not gatekeep it. If you're building zkML tools, consider targeting Goldilocks-field STARK systems. The math is waiting.*
