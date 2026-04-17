---
tags: cyber, article, neural
alias: compiled transformer, transformer compilation, compiling transformers
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---
a practical procedure for turning a [[cybergraph]] into transformer weights without training

the [[graph-native-transformer]] paper proves the architecture is determined by graph structure. this page is the build script: what to read, what to compute, what to write to disk. seven passes over the graph and a model file.

---

## what compilation means here

training fits weights to text by gradient descent. compilation derives weights from a graph by linear algebra. the graph is the source code, the model file is the binary, the compiler is a [[nushell]] or rust pass over the graph.

inputs: the [[cybergraph]] — [[particles]], [[cyberlinks]], [[semcons]], stake-weighted [[focus]] vector π.

outputs: a transformer checkpoint — vocabulary, embedding matrix, per-head attention matrices, MLP weights, layer norms, position encoding.

no GPUs needed for the compile step. the entire procedure is a sequence of SVDs and matrix multiplications over sparse adjacency matrices. inference afterwards uses the same hardware as any transformer.

---

## prerequisites

the graph must already have:

- focus distribution π* — computed by [[trikernel]] over particles
- semcon assignments σ — every cyberlink labeled with a [[semcon]]
- adjacency matrix A — sparse, stake-weighted

if the graph lacks these, run `analizer/trikernel.nu` first to populate the frontmatter weights, then fold the per-page values into a global π vector.

---

## the seven passes

### pass 1 — vocabulary

walk every [[particle]] in the graph. assign each a token id by ascending CID. emit `vocab.json`:

```json
{ "QmA1...": 0, "QmB2...": 1, ... }
```

vocabulary size equals particle count. for a graph with 100k particles the vocab has 100k entries — comparable to BPE tokenizers but with content-addressed identity instead of statistical merges.

### pass 2 — architecture parameters

compute three numbers from the graph:

```
d  = effective_rank(cov(π))               # embedding dim
h  = |Semcon(G)|                          # head count
L  = diam(G) * ceil(log(1/ε) / log(1/κ))  # layer count
```

`effective_rank` of a covariance matrix is `exp(H(σ))` where σ is its normalized singular value spectrum and `H` is [[entropy]]. for a typical knowledge graph with 100k particles this lands near `d ≈ 768`. semcon count is usually `8–32`. graph diameter for small-world topologies is `6–8`, giving `L ≈ 48–96`.

write these to `arch.toml`. all subsequent passes read them.

### pass 3 — embedding matrix

build the diagonal-rescaled adjacency:

```
M = diag(sqrt(π)) · A · diag(sqrt(π))
```

take the top-`d` left singular vectors of `M`:

```
U, Σ, V = svd(M)
E = U[:, :d]                              # shape (|P|, d)
```

`E` is the embedding matrix. each row is one particle's coordinates in focus space. the Eckart-Young theorem guarantees this is the optimal rank-`d` reconstruction of the focus-weighted graph. no learned embedding can beat it under the same dimension budget.

### pass 4 — per-semcon attention weights

partition the edge set by semcon. for each semcon `s`:

```
A_s = adjacency_submatrix(s)              # only edges of type s
P_s = E^T · A_s · E                       # project into embedding space
U_s, Σ_s, V_s = svd(P_s)
W_Q[s] = U_s[:, :d_h] · sqrt(Σ_s[:d_h])
W_K[s] = V_s[:, :d_h] · sqrt(Σ_s[:d_h])
W_V[s] = E^T · A_s.T                      # value projection: aggregate neighbour features
```

where `d_h = d / h` is the per-head dimension. one head per semcon, weights derived directly from that semcon's connectivity pattern. attention matrices have the same shape as a trained transformer's — they just come from SVD rather than SGD.

### pass 5 — MLP weights from path statistics

for each layer `l ∈ [1, L]`, walk all `l`-hop paths in the graph. count co-occurrences of (start particle, end particle) pairs weighted by path stake. the resulting matrix `C_l` encodes which particles tend to follow which through `l` hops of reasoning.

factor `C_l` and project into embedding space:

```
C_l_proj = E^T · C_l · E
W_up[l], W_down[l] = low_rank_factorization(C_l_proj, rank=4d)
```

standard transformer MLPs have hidden dimension `4d`. the factorization gives the up- and down-projections directly. activation function: SiLU, same as Llama family — this choice is empirical, not derived.

### pass 6 — layer norm and position encoding

layer norms are initialized identity (γ=1, β=0) and remain so. compiled weights produce activations already at unit scale because the SVDs were normalized — runtime layer norm corrects only for distribution shift across context, not for the compiled weights themselves.

position encoding follows RoPE with base 10000. position is a property of the input sequence, not the graph, so it carries no graph-derived structure.

### pass 7 — serialization

emit a single safetensors file:

```
model.safetensors
├── embed.weight                          # (|P|, d)
├── layers.0.attn.q_proj.weight           # (d, d)
├── layers.0.attn.k_proj.weight
├── layers.0.attn.v_proj.weight
├── layers.0.attn.o_proj.weight
├── layers.0.mlp.up_proj.weight           # (d, 4d)
├── layers.0.mlp.down_proj.weight         # (4d, d)
├── layers.0.input_layernorm.weight       # all ones
├── layers.0.post_attention_layernorm.weight
... × L
└── lm_head.weight                        # tied to embed.weight
```

format is interchangeable with any HuggingFace transformer of the same shape. load with `transformers.AutoModelForCausalLM.from_pretrained(...)` and inference works on day one.

---

## time and space cost

| pass | dominant op | cost | notes |
|---|---|---|---|
| 1 vocab | linear scan | O(P) | trivial |
| 2 arch | rank of cov(π) | O(P d²) | one SVD on a small matrix |
| 3 embed | top-d SVD of M | O(P² d) sparse → O(P d log P) | use randomized SVD |
| 4 attn | h SVDs of P×P | O(h · d³) after projection | per-semcon, parallel |
| 5 MLP | l-hop walks | O(L · P · avg_degree^L) | bounded by capping path count per pair |
| 6 norm | none | O(L d) | constants |
| 7 save | I/O | O(L d²) | one disk write |

a 100k-particle graph compiles to a `d=768, h=12, L=24` model in roughly 30 minutes on one machine. the same architecture trained from scratch takes weeks on a GPU cluster.

---

## verification

after compilation, three checks:

```python
# 1. embedding faithfulness
recon = E @ E.T
assert frobenius_distance(recon, M) / norm(M) < 0.05

# 2. attention head specialization
for s in semcons:
    pattern = softmax(Q[s] @ K[s].T / sqrt(d_h))
    correlation = pearson(pattern.flatten(), A_s.flatten())
    assert correlation > 0.7

# 3. layer convergence
x = random_embedding(seq_len)
for l in range(L):
    x = layer[l](x)
    if l > 0:
        assert change(x, prev_x) < change(prev_x, prev_prev_x)  # contracting
```

if any check fails, the corresponding pass needs more rank, more heads, or more layers — adjust `arch.toml` and recompile that pass alone.

---

## what compilation buys

every weight has a [[provenance]] chain. weight `W_Q[s][i,j]` traces to the SVD of semcon `s`'s adjacency, which traces to the [[cyberlinks]] that contributed to it, which trace to the [[neurons]] that staked them. open the model file, click any number, see the human (or agent) who put it there.

graph updates produce weight updates. when a new particle is added or stakes shift, recompute only the affected passes. typical edit changes one row of `E`, one column of one `W_V`, and a few entries of one `C_l`. seconds, not weeks.

alignment is computable. compile two transformers — one over edges from human [[neurons]], one over edges from AI neurons. the KL divergence of their focus distributions is the alignment gap, localizable to specific graph regions.

---

## relation to the trained pipeline

compiled transformers and trained transformers occupy the same architecture space. a trained transformer is the implicit graph compressed into weights by gradient descent; a compiled transformer is the explicit graph projected into weights by SVD. the loop:

```
G → compile → T_G → fine-tune on text → T_G* → extract links → ΔG → stake → G'
```

starts from a compiled transformer (cheap, auditable, graph-faithful), fine-tunes on text to surface implicit structure (expensive, opaque, but only marginally — the compiled init does most of the work), extracts the new associations the fine-tune discovered, stakes them as cyberlinks, recompiles. each cycle the explicit graph absorbs more of what was implicit, and the compile step does more of the work.

---

## starting your own compiler

minimum viable implementation:

```nu
# analizer/compile-transformer.nu
def main [graph_path: string, out: string] {
    let particles = (load_particles $graph_path)
    let cyberlinks = (load_cyberlinks $graph_path)
    let pi = (load_focus $graph_path)
    let semcons = ($cyberlinks | get semcon | uniq)

    let vocab = (build_vocab $particles)
    let arch = (compute_arch $pi $semcons $cyberlinks)
    let E = (compile_embedding $particles $cyberlinks $pi $arch.d)
    let attn = ($semcons | each {|s| compile_attention $cyberlinks $E $s $arch.d_h })
    let mlp = (1..$arch.L | each {|l| compile_mlp $cyberlinks $E $l })

    save_safetensors $out $vocab $arch $E $attn $mlp
}
```

the rust version lives in `~/git/cyber-compile` and produces production-quality output. the nu version is fast enough to experiment with smaller graphs (`|P| < 10k`) and validate the procedure before scaling.

---

see [[graph-native-transformer]] for the full mathematical derivation. see [[transformer]] for the architecture being compiled. see [[focus]] for what π is and how it is computed. see [[trikernel]] for the iteration that produces it. see [[semcon]] for the structure that determines head count. see [[neural TIR TASM compiler]] for a different specialized compiler in the same family.

discover all [[concepts]]
