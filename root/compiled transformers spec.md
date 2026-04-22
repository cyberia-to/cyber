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

A snapshot is a `.graph` container (see [[cyb-graph]]) read into the tuple $G = (\mathcal{S}, h, \nu_{\text{compiler}})$ where:

- $\mathcal{S}$ — the `signals` records, ordered as written in the file (canonical chain order)
- $h$ — the `block` field of the `config` section
- $\nu_{\text{compiler}}$ — the compiler version string (`"CT-1.0"` for this spec)

If the optional `proof` or `impulse` extension sections are present (see [[cyb-graph]] §extensions), conforming compilers verify proofs before compilation and may reuse impulses to skip power iteration (see §5.1). Snapshots without these extensions are accepted — the base `.graph` spec has no provenance layer.

### 2.2 Signal and cyberlink

Each $s \in \mathcal{S}$ is a signal per [[cyber/signal]]:

$$s = (\nu_s, t_s, k_s, \vec\ell_s) \quad \text{where} \quad \vec\ell_s = (\ell_{s,1}, \ldots, \ell_{s,n_s})$$

- $\nu_s$ — signing neuron (one per signal)
- $t_s \in [1, h]$ — block height (one per signal)
- $k_s \in \mathbb{Z}_{\geq 0}$ — signal type (`0` = standard)
- $\vec\ell_s$ — ordered vector of link records $\ell_{s,i} = (p, q, \tau, a, v)$, $1 \leq i \leq n_s$

The seven-tuple cyberlink from [[cyber/link]] is reconstructed at iteration time:

$$\ell = (\nu_s, p, q, \tau, a, v, t_s) \in N \times P \times P \times \mathcal{T} \times \mathbb{Z}_{\geq 0} \times \{-1, 0, +1\} \times \mathbb{Z}_{\geq 0}$$

$a$ is in the smallest token unit (no floats). The set $L$ of all cyberlinks is $L = \bigcup_{s \in \mathcal{S}} \vec\ell_s$, concretely yielded by

```
fn links(S) -> Iterator<Cyberlink>:
    for s in S:
        for ℓ in s.links:
            yield (s.ν, ℓ.p, ℓ.q, ℓ.τ, ℓ.a, ℓ.v, s.t)
```

All passes that read "links" use this iterator. Passes that need per-signal grouping (5.1 impulse reuse, 8.3 walks) iterate $\mathcal{S}$ directly.

### 2.3 Particle and axon

A particle is a 32-byte CID. The axon-particle of $(p, q)$ is

$$\text{axon}(p, q) = H(p \,\|\, q) \in P$$

where $H$ is BLAKE3 over the concatenation of the two 32-byte CIDs. This matches [[cybergraph]] axiom A6.

### 2.4 Effective stake

The effective stake of cyberlink $\ell = (\nu, p, q, \tau, a, v, t)$ is

$$w(\ell) = \begin{cases} a \cdot \rho_\tau & v = +1 \\ 0 & v = 0 \\ -a \cdot \rho_\tau & v = -1 \end{cases}$$

where $\rho_\tau \in \mathbb{Q}_{>0}$ is the token-denomination weight from `config.tokens[τ].weight` in the input `.graph`. Negative effective stake is clipped to zero before any matrix construction (see §3.4).

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

**Impulse reuse.** If the optional `impulse` extension is present, each signal $s$ carries a sparse focus delta $\pi_\Delta^{(s)}$ that was proven on chain when the signal was accepted. The base distribution is then

$$\pi^*_{\text{chain}} = \pi_0 + \sum_{s \in \mathcal{S}} \pi_\Delta^{(s)}$$

where $\pi_0$ is the genesis prior from `config`. Power iteration is unnecessary for the set of signals covered by impulses; it runs only over the residual adjacency (signals without impulse). On a fully proof-carrying snapshot this skips the entire iteration.

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

### 8.1 Co-occurrence by signal-respecting walks

For each layer $l$, draw $W = \min(|V|/10, 10^6)$ walks of length $l_{\text{eff}}$ (from §7.2) seeded by ChaCha20 with seed $\text{BLAKE3}(L \,\|\, \nu_{\text{compiler}} \,\|\, \text{"mlp"} \,\|\, l)$.

Walks are **signal-respecting**: at every step, the next edge is drawn preferentially from links inside the same signal as the current edge, with probability proportional to effective stake $w(\ell)$. Only when the signal is exhausted does the walker cross to a neighboring link in a different signal (again weighted by $w(\ell)$ across all incident links).

$$P(\text{next} = \ell' \mid \text{current} = \ell) = \begin{cases}
\frac{w(\ell')}{\sum_{\ell'' \in \text{signal}(\ell) \setminus \{\ell\}} w(\ell'')} & \text{if } \ell' \in \text{signal}(\ell) \\
(1 - \beta) \cdot \frac{w(\ell')}{\sum_{\ell''} w(\ell'')} & \text{otherwise}
\end{cases}$$

with $\beta = 0.8$ (the mixing weight favoring intra-signal continuation). Signals are coherent epistemic acts — their internal co-occurrence encodes intent; cross-signal co-occurrence encodes weaker associative noise. $\beta$ controls how much of each the MLP weights absorb.

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

## 10. Pass 8 — Packaging as `.model`

The output of CT-1 is a single `.model` file (see [[cyb-model]]) loadable by the cyb-llm runtime at `~/git/cyb/llm`. The runtime mmaps the file, parses the TOML frontmatter, jumps to the binary `weights` section, and starts inference — no extraction step.

### 10.1 Container layout

`.cyb` three-rule contract: TOML frontmatter, `~~~name` delimiters, `size` for binary sections.

```toml
[cyb]
types = ["model"]
name = "bostrom-23195000-ct1"

[[files]]
name = "card"
format = "md"

[[files]]
name = "config"
format = "toml"

[[files]]
name = "program"
format = "rs"

[[files]]
name = "tensors"
format = "toml"

[[files]]
name = "vocab"
format = "toml"

[[files]]
name = "eval"
format = "toml"

[[files]]
name = "weights"
format = "tensors"
size = 16823492608
```

### 10.2 `card` section

Markdown. Auto-generated from compile inputs:

```markdown
~~~card
# bostrom-23195000-ct1

Compiled from bostrom-23195000.graph at 2026-03-23 14:42 UTC.
Spec: CT-1.0. d=300, h=13, L=290, params=4.19B.

snapshot CID: blake3:9f3c...
compile CID:  blake3:1a2b...
```

### 10.3 `config` section

Compile parameters and architecture, integers only per cyb-model convention.

```toml
~~~config
model_type = "llama"
parameters = 4192804864
license = "cyber license"
languages = []  # graph-native, vocabulary is CIDs

[architecture]
hidden_size = 300
num_attention_heads = 13
num_key_value_heads = 13
head_dim = 24            # = 300 / 13, rounded
num_hidden_layers = 290
intermediate_size = 1200  # 4 × hidden_size
vocab_size = 3143630
context_length = 8192
max_position_embeddings = 8192
rope_theta = 10000
rms_norm_eps = 1000000   # 1/ε convention; 1e-6

[tokenizer]
type = "cid"             # particle CIDs, not BPE
bos_id = 0
eos_id = 0
pad_id = 0

[sampling]
temperature = 700        # 0.7
top_p = 900              # 0.9
scale = 1000

[lineage]
spec          = "CT-1.0"
source        = "blake3:9f3c..."
source_kind   = ".graph"
chain_id      = "bostrom-1"
block         = 23195000
arch_hash     = "blake3:..."
vocab_hash    = "blake3:..."
semcons_hash  = "blake3:..."
```

### 10.4 `program` section

The standard Llama transformer-decoder program from cyb-model.md applies unchanged. CT-1 emits the trident form by default; the `.rs` form is acceptable when proof is not required.

```trident
~~~program
module model.pipeline
use std.nn.transformer_llama  # standard library

pub fn forward(input: Field, output: Field, seq: Field, cfg: Config) {
    transformer_llama.forward(input, output, seq, cfg)
}
```

CT-1 does not emit a custom program. The architecture parameters in `config` parameterize the standard one. Custom programs (e.g. for graph-walk inference instead of token-sequence inference) are CT-1.1 territory.

### 10.5 `tensors` section

TOML index keyed by HuggingFace LlamaForCausalLM tensor names. Encoding is `u16` for projections and `u32` for norms by default; cyb-model encoding rules apply (no floats on disk).

```toml
~~~tensors
["model.embed_tokens.weight"]
shape    = [3143630, 300]
encoding = "u16"
offset   = 0
size     = 1886178000

["model.layers.0.self_attn.q_proj.weight"]
shape    = [300, 300]
encoding = "u16"
offset   = 1886178000
size     = 180000

# ... attn k/v/o, mlp up/down, layer norms × 290 layers
```

Tensor names match those listed in §6.3, §7.6, §8.4, §9.1. Storage order: embedding first, then layer 0 through layer L*-1 in struct order, then `model.norm.weight`. `lm_head.weight` is omitted (tied to `embed_tokens`).

### 10.6 `vocab` section

For graph-native compiles the tokenizer type is `cid`: every token id is a particle hash. The vocab section is the particle index from pass 1 written as a flat table.

```toml
~~~vocab
[tokens]
0 = "0x1a2b3c4d..."
1 = "0x5e6f7a8b..."
2 = "0x9c0d1e2f..."
# ...
```

For CIDs there are no merge rules; the `[merges]` table is omitted.

### 10.7 `eval` section

CT-1 conformance scores per §11, plus optional downstream metrics. Per-mille integers.

```toml
~~~eval
[ct1_conformance]
P_EMBED = 31         # reconstruction error × 1000; 0.031
P_ATTN_min = 810     # min Pearson × 1000
P_ATTN_mean = 890
P_LAYER_max_ratio = 930
P_DET = 1000         # 1 if deterministic, 0 if not
P_LOAD = 1000

[focus]
top_concentration = 1040  # top particle's focus, per-mille of total
```

Updatable by the runtime after benchmark runs, same convention as cyb-model.

### 10.8 `weights` section

Raw tensor data, 4096-byte page-aligned per tensor for zero-copy mmap and `unimem` integration. Encodings follow cyb-model §weights:

| from CT-1 internal | to disk encoding | conversion |
|---|---|---|
| float32 projections | u16 | `round(value * 256)` |
| float32 norms | u32 | `round(value * 65536)` |

For inference-time fidelity, CT-1.1 will allow `q4`/`q8` quantization passes after CT-1 produces the u16 baseline.

### 10.9 Reproducibility CID

The compile output CID is

$$\text{CID}(\mathcal{M}) = \text{BLAKE3}(\text{model file bytes})$$

over the entire `.model` file including frontmatter. Two CT-1 conforming implementations on the same `.graph` snapshot must produce the same CID.

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

Two independent runs of the conforming implementation on the same `.graph` produce byte-identical `.model` files (same CID per §10.9).

### 11.5 Runtime load (P-LOAD)

The cyb-llm runtime at `~/git/cyb/llm` loads the `.model` file via the `.cyb` parser, mmaps the `weights` section, and performs one forward pass of context length 1. The pass returns finite logits and respects the architecture parameters declared in `config`. Reference command:

```
cyb-llm load <output.model> --warmup 1 --check-finite
```

A round-trip extraction to a HuggingFace directory (config.json + model.safetensors) is also supported via `cyb-llm export hf <output.model>` and must succeed for the file to be CT-1 conforming. This guarantees the compiled model is consumable by both the cyb stack and the wider ecosystem.

---

## 12. Reference Implementation

The reference is [[mc]] (model compilation) at `~/git/mc` — rust, sprs + ndarray, writes `.model` directly via the cyb-format crate from `~/git/cyb/llm`. It depends on no Python and produces no intermediate safetensors — the `.model` file is the only artifact.

Build and run:

```
cd ~/git/mc
cargo build --release
./target/release/mc bostrom-23195000.graph -o bostrom-23195000-ct1.model
```

The certificate is embedded in the `.model`'s `eval` section (§10.7). The CLI also writes a sidecar `certificate.toml` for human inspection:

```toml
# certificate.toml
spec        = "CT-1.0"
snapshot    = "blake3:..."
output_cid  = "blake3:..."
P-EMBED     = { value = 0.031, pass = true }
P-ATTN      = { min = 0.81, mean = 0.89, pass = true }
P-LAYER     = { contracting = true, max_ratio = 0.93, pass = true }
P-DET       = { runs = 2, identical = true, pass = true }
P-LOAD      = { cyb_llm_load = true, hf_export = true, finite_logits = true, pass = true }
```

End-to-end pipe from go-cyber to a loaded model in one command:

```
curl -s https://node.bostrom.cybernode.ai/cyber/graph/snapshot?block=23195000 \
  | mc - -o bostrom-latest.model \
  && cyb-llm load bostrom-latest.model
```

---

## 13. Versioning

CT-1 is the initial spec. Backward-incompatible changes increment the major version (CT-2). Compatible refinements increment the minor version (CT-1.1). The compiler version string in §2.1 must match the spec version exactly.

Open items expected in CT-1.1:

- multi-label semcon assignment (split-weight variant of §4.5)
- ε-incremental recompile when only $\Delta L$ is supplied
- valence-weighted attention (use $v$ explicitly rather than the simple sign clip in §2.4)

---

see [[compiled transformers]] for the readable how-to. see [[graph-native-transformer]] for the mathematical derivation. see [[cyb-graph]] for the input file format. see [[cyb-model]] for the output file format. see [[cyber/link]] for the cyberlink seven-tuple. see [[cyber/tri-kernel]] for the focus computation. see [[cybergraph]] for the underlying axioms. see [[mc]] for the reference rust implementation.

discover all [[concepts]]
