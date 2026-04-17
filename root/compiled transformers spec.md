---
tags: article, cyber, cip, neural
alias: ct-spec, compile spec
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
status: draft
---
# Compiled Transformers Specification (CT-1)

formal contract for compiling a transformer from a [[cybergraph]] snapshot. companion to [[compiled transformers]] (the how-to article) and [[graph-native-transformer]] (the derivation). this page is what the rust crate implements; conformance is checked against the predicates in §10.

---

## 1. Scope

CT-1 specifies a deterministic function

$$\text{compile}: G \to \mathcal{M}$$

where $G$ is a cybergraph snapshot in [[cyb-graph|.graph format]] and $\mathcal{M}$ is a transformer checkpoint in [[cyb-model|.model format]]. Two implementations conforming to CT-1 must produce a byte-identical $\mathcal{M}$ given a byte-identical $G$ and the same compiler version.

---

## 2. Input Definitions

### 2.1 Snapshot

A snapshot is a `.graph` container (see [[cyb-graph]]) read into the tuple $G = (L, h, \nu_{\text{compiler}})$ where:

- $L$ — the `cyberlinks` records, ordered as written in the file (canonical chain order)
- $h$ — the `block` field of the `config` section
- $\nu_{\text{compiler}}$ — the compiler version string (`"CT-1.0"` for this spec)

The `proof` section of the `.graph` is verified before compilation begins; CT-1 conforming compilers refuse to compile snapshots that fail proof verification when `[provenance].proof_required = true`.

### 2.2 Cyberlink

Each $\ell \in L$ is the seven-tuple from [[cyber/link]]:

$$\ell = (\nu, p, q, \tau, a, v, t) \in N \times P \times P \times \mathcal{T} \times \mathbb{Z}_{\geq 0} \times \{-1, 0, +1\} \times \mathbb{Z}_{\geq 0}$$

where stake amount $a$ is in the smallest token unit (no floats) and $t \leq h$.

### 2.3 Particle and axon

A particle is a 32-byte CID. The axon-particle of $(p, q)$ is

$$\text{axon}(p, q) = H(p \,\|\, q) \in P$$

where $H$ is BLAKE3 over the concatenation of the two 32-byte CIDs. This matches [[cybergraph]] axiom A6.

### 2.4 Effective stake

The effective stake of cyberlink $\ell = (\nu, p, q, \tau, a, v, t)$ is

$$w(\ell) = \begin{cases} a \cdot \rho_\tau & v = +1 \\ 0 & v = 0 \\ -a \cdot \rho_\tau & v = -1 \end{cases}$$

where $\rho_\tau \in \mathbb{Q}_{>0}$ is the token-denomination weight from the registry at block $h$. Negative effective stake is clipped to zero before any matrix construction (see §3.4).

---

## 3. Pass 1 — Particle Index

### 3.1 Procedure

1. Initialize $V := \emptyset$, an ordered set.
2. For each $\ell = (\nu, p, q, \ldots) \in L$ in snapshot order: insert $p$, then $q$, then $\text{axon}(p, q)$ into $V$ if absent.
3. Assign $\text{idx}: V \to \{0, 1, \ldots, |V|-1\}$ in insertion order.

### 3.2 Output

`vocab.json` — the JSON object $\{ \text{cid}_{\text{hex}} \mapsto \text{idx} \}$ with keys lowercase-hex-encoded.

### 3.3 Determinism

Insertion order is fixed by snapshot order. Two compilers seeing the same $L$ produce the same $\text{idx}$.

### 3.4 Adjacency construction

Build $A \in \mathbb{Z}_{\geq 0}^{|V| \times |V|}$ in CSR with

$$A_{\text{idx}(p), \text{idx}(q)} = \sum_{\ell : (p, q) \in \ell, \, w(\ell) > 0} w(\ell)$$

stored as int128 to avoid overflow on long-running chains. $A$ is fed to passes 2 and 3.

---

## 4. Pass 2 — Semcon Discovery

### 4.1 Axon set

$$\Omega = \{ \text{axon}(p, q) : (\nu, p, q, \ldots) \in L \}$$

### 4.2 Label edges

A label edge is any $\ell = (\nu, p, q, \ldots)$ with $q \in \Omega$. The source $p$ is a candidate semcon.

### 4.3 Scoring

For each candidate $p$ appearing as the source of label edges:

$$\text{usage}(p) = \sum_{\ell : \text{label edge}, \text{src}(\ell) = p} w(\ell)$$

$$\text{coverage}(p) = |\{ \text{tgt}(\ell) : \text{label edge}, \text{src}(\ell) = p \}|$$

$$\text{score}(p) = \text{usage}(p) \cdot \log_2(1 + \text{coverage}(p))$$

### 4.4 Registration

The registered semcon set $S \subseteq P$ is

$$S = \{ p : \text{score}(p) \geq \theta \cdot \max_{p'} \text{score}(p') \}$$

with $\theta = 10^{-3}$ (one-thousandth of the strongest semcon by score). Order $S$ by descending score; ties broken by ascending CID.

The default semcon is the reserved CID $0x00 \times 32$, denoted $\bot$. It is appended to $S$ at the highest index.

### 4.5 Assignment

For each $\ell = (\nu, p, q, \ldots) \in L$ compute $\alpha = \text{axon}(p, q)$ and

$$\sigma(\ell) = \arg\max_{s \in S \setminus \{\bot\}} \sum_{\ell' : \text{src}(\ell') = s, \text{tgt}(\ell') = \alpha} w(\ell')$$

If the argmax set is empty (no registered semcon labels $\alpha$), $\sigma(\ell) = \bot$. Argmax ties are broken by ascending position of $s$ in $S$.

### 4.6 Output

`semcons.json` — the ordered list $S$ with per-semcon edge count and aggregate stake.

### 4.7 Complexity

$O(|L|)$ time, $O(|S| + |\Omega|)$ extra space.

---

## 5. Pass 3 — Architecture Parameters

### 5.1 Focus distribution

Compute $\pi^* \in \Delta^{|V|}$ by power iteration of the column-stochastic transition matrix $P = A^\top D^{-1}$ (with $D = \text{diag}(A^\top \mathbf{1})$, treating zero-degree rows as teleport):

$$\pi^{(k+1)} = \alpha P \pi^{(k)} + (1 - \alpha) u, \quad \pi^{(0)} = u, \quad u_i = \frac{1}{|V|}$$

with $\alpha = 0.85$. Halt when $\|\pi^{(k+1)} - \pi^{(k)}\|_1 < \varepsilon_\pi$ with $\varepsilon_\pi = 10^{-8}$.

### 5.2 Embedding dimension

Take the singular value spectrum $\Sigma = (\sigma_1, \ldots, \sigma_r)$ of the $\pi$-weighted adjacency

$$M = \text{diag}(\sqrt{\pi^*}) \cdot A \cdot \text{diag}(\sqrt{\pi^*})$$

via randomized SVD truncated to rank $r = 1024$ (oversampled). Normalize: $\hat{\sigma}_i = \sigma_i / \sum_j \sigma_j$. Then

$$d^* = \left\lceil \exp\left(- \sum_i \hat{\sigma}_i \log \hat{\sigma}_i\right) \right\rceil$$

Round to the nearest multiple of $h^*$ (see §5.3) and clamp to $[64, 4096]$.

### 5.3 Head count

$$h^* = |S|$$

(includes $\bot$).

### 5.4 Layer count

Compute the spectral gap $\lambda_2$ of the normalized Laplacian $\mathcal{L} = I - D^{-1/2} A D^{-1/2}$ via Lanczos with $k = 32$ iterations. Compute the contraction rate

$$\kappa = \alpha (1 - \lambda_2)$$

Estimate the diameter $\text{diam}(G)$ via BFS from the highest-degree node (lower bound; sufficient for our use). Then

$$L^* = \text{diam}(G) \cdot \left\lceil \frac{\log(1/\varepsilon_L)}{\log(1/\kappa)} \right\rceil$$

with $\varepsilon_L = 10^{-2}$. Clamp $L^* \in [4, 512]$.

### 5.5 Output

`arch.toml`:

```toml
compiler   = "CT-1.0"
block      = 12345678
particles  = 3143630
d          = 300
h          = 13
L          = 290
kappa      = 0.851
lambda2    = 0.0015
diameter   = 10
```

---

## 6. Pass 4 — Embedding Matrix

### 6.1 Computation

Continue the randomized SVD of $M$ from §5.2 to extract the top $d^*$ left singular vectors $U_{:, 1:d^*}$ and singular values $\Sigma_{1:d^*}$. Set

$$E = U_{:, 1:d^*} \cdot \text{diag}(\sqrt{\Sigma_{1:d^*}}) \in \mathbb{R}^{|V| \times d^*}$$

### 6.2 Determinism

Randomized SVD uses ChaCha20 seeded with $\text{BLAKE3}(L \,\|\, \nu_{\text{compiler}})$ truncated to 32 bytes. Singular vector signs are normalized so the entry of largest absolute value in each column is positive (sign convention SC-1).

### 6.3 Output tensor

`embed.weight` of shape $(|V|, d^*)$, dtype `float32`, row-major.

---

## 7. Pass 5 — Attention Weights

For each layer $l \in \{0, \ldots, L^* - 1\}$ and each semcon $s \in S$ at head index $h_s$:

### 7.1 Per-semcon adjacency

$$A^{(s)}_{ij} = \sum_{\ell : \text{idx}(\text{src}) = i, \text{idx}(\text{tgt}) = j, \sigma(\ell) = s} w(\ell)$$

### 7.2 Layer-specific power

The layer-$l$ semcon adjacency is

$$A^{(s, l)} = (A^{(s)})^{l_{\text{eff}}}, \quad l_{\text{eff}} = 1 + \lfloor l \cdot \text{diam}(G) / L^* \rfloor$$

computed by repeated sparse-times-dense multiplication; never materialized as dense.

### 7.3 Projection into embedding space

$$P^{(s, l)} = E^\top A^{(s, l)} E \in \mathbb{R}^{d^* \times d^*}$$

### 7.4 SVD per head

$$P^{(s, l)} = U^{(s,l)} \Sigma^{(s,l)} V^{(s,l)\top}$$

Truncate to rank $d_h = d^* / h^*$:

$$W_Q^{(l, h_s)} = U^{(s,l)}_{:, 1:d_h} \cdot \sqrt{\Sigma^{(s,l)}_{1:d_h}}$$

$$W_K^{(l, h_s)} = V^{(s,l)}_{:, 1:d_h} \cdot \sqrt{\Sigma^{(s,l)}_{1:d_h}}$$

$$W_V^{(l, h_s)} = E^\top \cdot \text{diag}(\pi^*) \cdot A^{(s)} \cdot E_{:, h_s \cdot d_h : (h_s+1) \cdot d_h}$$

Sign convention SC-1 applied to $U^{(s,l)}, V^{(s,l)}$.

### 7.5 Output projection

$$W_O^{(l)} = (W_V^{(l, 0)} \,\|\, \cdots \,\|\, W_V^{(l, h^*-1)})^\dagger$$

(Moore-Penrose pseudoinverse of the concatenated values, giving the optimal aggregation back to $d^*$.)

### 7.6 Output tensors

Per layer $l$:

- `layers.{l}.attn.q_proj.weight` of shape $(d^*, d^*)$ — concatenation of $W_Q^{(l, h)}$ over $h$
- `layers.{l}.attn.k_proj.weight` of shape $(d^*, d^*)$
- `layers.{l}.attn.v_proj.weight` of shape $(d^*, d^*)$
- `layers.{l}.attn.o_proj.weight` of shape $(d^*, d^*)$

dtype `float32`, row-major.

---

## 8. Pass 6 — MLP Weights

### 8.1 Co-occurrence by deterministic walks

For each layer $l$, draw $W = \min(|V|/10, 10^6)$ random walks of length $l_{\text{eff}}$ (from §7.2) seeded by ChaCha20 with seed $\text{BLAKE3}(L \,\|\, \nu_{\text{compiler}} \,\|\, \text{"mlp"} \,\|\, l)$. Edge selection at each step is weighted by $w(\ell)$.

### 8.2 PMI matrix

For window $w_{\text{co}} = 5$, accumulate weighted co-occurrence counts $C^{(l)}_{ij}$ for pairs $(v_i, v_j)$ at distance $\leq w_{\text{co}}$ within walks. Convert to positive PMI:

$$\text{PMI}^{(l)}_{ij} = \max\left(0, \log \frac{p^{(l)}(v_i, v_j) \cdot Z}{p(v_i) \cdot p(v_j)}\right)$$

with $p(v_i) = \pi^*_i$ and $Z = \sum_{ij} C^{(l)}_{ij}$.

### 8.3 Projection and factorization

$$\widetilde{\text{PMI}}^{(l)} = E^\top \text{PMI}^{(l)} E \in \mathbb{R}^{d^* \times d^*}$$

Truncated SVD to rank $4 d^*$ (oversampled by 10):

$$\widetilde{\text{PMI}}^{(l)} = U \Sigma V^\top$$

$$W_1^{(l)} = U_{:, 1:4d^*} \cdot \sqrt{\Sigma_{1:4d^*}}$$

$$W_2^{(l)} = \sqrt{\Sigma_{1:4d^*}} \cdot V^\top_{:, 1:4d^*}$$

### 8.4 Output tensors

- `layers.{l}.mlp.up_proj.weight` of shape $(d^*, 4d^*)$
- `layers.{l}.mlp.down_proj.weight` of shape $(4d^*, d^*)$

Activation between them is SiLU; this is implicit in the architecture, not stored.

---

## 9. Pass 7 — Norms and Position

### 9.1 Layer norms

For every layer $l$:

- `layers.{l}.input_layernorm.weight` of shape $(d^*,)$, all entries $1.0$
- `layers.{l}.post_attention_layernorm.weight` of shape $(d^*,)$, all entries $1.0$
- `model.norm.weight` of shape $(d^*,)$, all entries $1.0$

### 9.2 Position encoding

RoPE with base $\theta_0 = 10000$, max sequence length 8192. Inverse frequencies are computed at load time from $(\theta_0, d^* / h^*)$; no tensor is stored.

### 9.3 Output head

`lm_head.weight` is tied to `embed.weight` (no separate tensor written).

---

## 10. Pass 8 — Serialization

### 10.1 Container

A single `.model` file (see [[cyb-model]]). Tensors are packed in the `weights` section with HuggingFace `LlamaForCausalLM` naming so that the same byte stream is loadable by both the cyb runtime and `transformers.AutoModelForCausalLM.from_pretrained(...)` when extracted to a HuggingFace directory.

### 10.2 Metadata

The safetensors metadata header includes:

```json
{
  "compiler":      "CT-1.0",
  "snapshot_cid":  "blake3:...",
  "block":         12345678,
  "graph_root":    "blake3:...",
  "arch_hash":     "blake3(arch.toml)",
  "vocab_hash":    "blake3(vocab.json)",
  "semcons_hash":  "blake3(semcons.json)"
}
```

### 10.3 Companion files

Alongside the safetensors:

- `vocab.json` (from §3.2)
- `semcons.json` (from §4.6)
- `arch.toml` (from §5.5)
- `manifest.json` — list of all tensor names with shapes and BLAKE3 of each tensor's bytes

### 10.4 Reproducibility CID

The compile output CID is

$$\text{CID}(\mathcal{M}) = \text{BLAKE3}(\text{model.safetensors})$$

Two CT-1 conforming implementations on the same snapshot must produce the same CID.

---

## 11. Conformance Predicates

A compile $\mathcal{M}$ is CT-1 conforming on snapshot $G$ iff all the following hold.

### 11.1 Reconstruction (P-EMBED)

$$\frac{\|E E^\top - M\|_F}{\|M\|_F} \leq 0.05$$

### 11.2 Head specialization (P-ATTN)

For every layer $l$ and semcon $s$:

$$\text{Pearson}(\text{flatten}(W_Q^{(l, h_s)} W_K^{(l, h_s)\top}), \text{flatten}(P^{(s, l)})) \geq 0.7$$

### 11.3 Layer contraction (P-LAYER)

For a fixed pseudo-random seed and a length-128 random embedding sequence, layer-to-layer change is monotonically nonincreasing for all $l \geq 1$.

### 11.4 Determinism (P-DET)

Two independent runs of the conforming implementation on the same $G$ produce the same `model.safetensors` bytes.

### 11.5 Round-trip load (P-LOAD)

`transformers.AutoModelForCausalLM.from_pretrained(<output_dir>)` succeeds and a forward pass on input length 1 returns finite logits.

---

## 12. Reference Implementation

The reference is `~/git/cyber-compile` (rust, sprs + ndarray + safetensors crates). The reference produces a CT-1 conformance certificate alongside `model.safetensors`:

```toml
# certificate.toml
spec        = "CT-1.0"
snapshot    = "blake3:..."
output      = "blake3:..."
P-EMBED     = { value = 0.0312, pass = true }
P-ATTN      = { min = 0.81, mean = 0.89, pass = true }
P-LAYER     = { contracting = true, max_ratio = 0.93, pass = true }
P-DET       = { runs = 2, identical = true, pass = true }
P-LOAD      = { hf_load = true, finite_logits = true, pass = true }
```

---

## 13. Versioning

CT-1 is the initial spec. Backward-incompatible changes increment the major version (CT-2). Compatible refinements increment the minor version (CT-1.1). The compiler version string in §2.1 must match the spec version exactly.

Open items expected in CT-1.1:

- multi-label semcon assignment (split-weight variant of §4.5)
- ε-incremental recompile when only $\Delta L$ is supplied
- valence-weighted attention (use $v$ explicitly rather than the simple sign clip in §2.4)

---

see [[compiled transformers]] for the readable how-to. see [[graph-native-transformer]] for the mathematical derivation. see [[cyb-graph]] for the input file format. see [[cyb-model]] for the output file format. see [[cyber/link]] for the cyberlink seven-tuple. see [[cyber/tri-kernel]] for the focus computation. see [[cybergraph]] for the underlying axioms. see [[cyber-compile]] for the reference rust implementation.

discover all [[concepts]]
