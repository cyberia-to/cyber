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

the only required input is the raw cyberlink list — the 7-tuples `(ν, p, q, τ, a, v, t)` straight from chain. everything else is computed by the compiler:

- particle index — assign each unique CID an integer id (pass 1)
- semcon set — discovered from labeling structure (pass 2)
- focus distribution π* — fixed point of [[trikernel]] over the adjacency (pass 3)
- adjacency matrix A — sparse CSR, stake-weighted (folded into pass 3)

if the graph already has `focus` precomputed in frontmatter (run `analizer/trikernel.nu`), pass 3 reuses it; otherwise it computes π* on the fly.

---

## the eight passes

### pass 1 — vocabulary

walk every [[particle]] referenced by any cyberlink. assign each a token id by ascending CID. emit `vocab.json`:

```json
{ "QmA1...": 0, "QmB2...": 1, ... }
```

vocabulary size equals particle count. for a graph with 100k particles the vocab has 100k entries — comparable to BPE tokenizers but with content-addressed identity instead of statistical merges.

### pass 2 — semcon discovery

a [[cyberlink]] has no built-in type field (see [[cyber/link]] §edge labeling). labels emerge through the graph itself. every directed edge `p → q` induces an [[axon]]-particle `H(p, q) ∈ P` by axiom A6. an edge is labeled when some other particle `t` has been linked to that axon: a cyberlink `t → axon(p, q)` says "the relation `p → q` is of type `t`". the labeling particle `t` IS the [[semcon]].

discovery procedure:

```
1. for every cyberlink (ν, p, q, ...): compute axon_id = H(p, q)
   axon_set = { axon_id : axon_id appears as target of some cyberlink }

2. for every cyberlink whose target q is in axon_set:
   record (source p, target axon q, stake a)
   p is a "label particle" — a candidate semcon

3. score each candidate p:
   usage[p] = sum of stake over all cyberlinks (p → axon ∈ axon_set)
   coverage[p] = number of distinct axons p has labeled

4. rank by usage × log(coverage). take top h candidates whose
   stake-weighted usage exceeds threshold θ (e.g. 0.1% of total stake).
   these are the registered semcons.

5. assign σ to every cyberlink ℓ:
   σ(ℓ) = argmax over registered semcons of stake(s → axon(ℓ))
   if no registered semcon labels axon(ℓ): σ(ℓ) = "default"
```

a cyberlink can have multiple labels — pick the highest-staked one, or split its weight across labels (multi-label assignment). multi-label is more faithful but costs an extra factor of `h` in pass 5; single-label is the practical default.

unlabeled edges go to a single "default" semcon bucket. on a young graph this is the largest bucket; as the network matures and labeling conventions consolidate, default shrinks. on bostrom end of 2024, default holds about 60% of edges. the registered semcons (`is-a`, `tags`, `cites`, `contradicts`, `extends`, `created-by`, `replies-to`, `name`, `summary`, `instance-of`, `part-of`, `derived-from`) hold the rest.

emit `semcons.json`:

```json
{
  "QmIsA...":   { "id": 0, "edges": 412331, "stake": 1.2e18 },
  "QmTags...":  { "id": 1, "edges": 287104, "stake": 8.4e17 },
  ...
  "default":    { "id": 11, "edges": 1623450, "stake": 4.1e17 }
}
```

cost: O(\|E\|) — one pass to build the axon set, one pass to score labels, one pass to assign. on bostrom this finishes in under two seconds.

### pass 3 — architecture parameters

compute three numbers from the graph:

```
d  = effective_rank(cov(π))               # embedding dim
h  = |Semcon(G)|                          # head count
L  = diam(G) * ceil(log(1/ε) / log(1/κ))  # layer count
```

`effective_rank` of a covariance matrix is `exp(H(σ))` where σ is its normalized singular value spectrum and `H` is [[entropy]]. for a typical knowledge graph with 100k particles this lands near `d ≈ 768`. semcon count is usually `8–32`. graph diameter for small-world topologies is `6–8`, giving `L ≈ 48–96`.

write these to `arch.toml`. all subsequent passes read them.

### pass 4 — embedding matrix

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

### pass 5 — per-semcon attention weights

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

### pass 6 — MLP weights from path statistics

for each layer `l ∈ [1, L]`, walk all `l`-hop paths in the graph. count co-occurrences of (start particle, end particle) pairs weighted by path stake. the resulting matrix `C_l` encodes which particles tend to follow which through `l` hops of reasoning.

factor `C_l` and project into embedding space:

```
C_l_proj = E^T · C_l · E
W_up[l], W_down[l] = low_rank_factorization(C_l_proj, rank=4d)
```

standard transformer MLPs have hidden dimension `4d`. the factorization gives the up- and down-projections directly. activation function: SiLU, same as Llama family — this choice is empirical, not derived.

### pass 7 — layer norm and position encoding

layer norms are initialized identity (γ=1, β=0) and remain so. compiled weights produce activations already at unit scale because the SVDs were normalized — runtime layer norm corrects only for distribution shift across context, not for the compiled weights themselves.

position encoding follows RoPE with base 10000. position is a property of the input sequence, not the graph, so it carries no graph-derived structure.

### pass 8 — serialization

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
| 2 semcon | axon scan + label scoring | O(\|E\|) | three linear passes over edge list |
| 3 arch | rank of cov(π) | O(P d²) | one SVD on a small matrix |
| 4 embed | top-d SVD of M | O(P² d) sparse → O(P d log P) | use randomized SVD |
| 5 attn | h SVDs of P×P | O(h · d³) after projection | per-semcon, parallel |
| 6 MLP | l-hop walks | O(L · P · avg_degree^L) | bounded by capping path count per pair |
| 7 norm | none | O(L d) | constants |
| 8 save | I/O | O(L d²) | one disk write |

a 100k-particle graph compiles to a `d=768, h=12, L=24` model in roughly 30 minutes on one machine. the same architecture trained from scratch takes weeks on a GPU cluster.

---

## real case: compiling [[bostrom]]

[[bostrom]] is the live bootloader chain — fifty validators, six years of cyberlinks, the first knowledge graph large enough to compile into a useful transformer. snapshot taken end of 2024:

| graph property | value |
|---|---|
| particles \|P\| | 3,143,630 |
| cyberlinks \|E\| | 2,705,323 |
| neurons \|N\| | 70,000 |
| sparsity \|E\|/\|P\|² | 2.7 × 10⁻⁷ |
| spectral gap λ₂ | 0.0015 |
| diameter | ≥ 10 |
| registered semcons | 12 |

derived architecture, no hyperparameter search:

| param | value | source |
|---|---|---|
| d | 300 | effective rank of the π-weighted adjacency spectrum |
| h | 12 | one head per registered [[semcon]] |
| L | 290 | diameter × ⌈log(1/ε)/log(1/κ)⌉ at κ=0.851, ε=0.01 |

result: 4.19 billion parameters, 16.8 GB on disk. comparable to Llama-7B in scale, derived from chain state in seconds.

### wall time on one workstation

single machine, 1 TFLOPS, 20 GB RAM, no GPU:

| pass | wall time |
|---|---|
| extract from chain (GraphQL) | ~1 s |
| sparse adjacency CSR | <0.1 s |
| semcon discovery (axon scan + label scoring) | ~1.8 s |
| focus by power iteration (29 rounds, α=0.85) | 0.08 s |
| spectral gap (Lanczos, k=10) | 0.03 s |
| randomized SVD for embedding (Halko-Martinsson-Tropp) | 0.007 s |
| 12 semcon attention SVDs | 0.8 s |
| MLP from random walks (314k walks × 290 hops) | 0.06 s |
| safetensors / ONNX assembly | ~60 s (disk I/O bound) |
| total | ~64 s |

the compile finishes faster than `git pull` on the chain snapshot itself. inference afterwards runs on the same hardware as any 4B-parameter transformer.

### why it stays cheap

the naive eigendecomposition of the focus covariance is O(\|P\|³) — 3.1 × 10¹⁹ operations, 360 days at one teraflop, on a 39.5 TB dense matrix. nobody runs that.

the actual compile uses randomized SVD on the π-weighted sparse adjacency. cost drops to O(\|E\| · d · log d) = 7.5 × 10⁹ operations — 0.007 seconds. four orders of magnitude tractability gap, closed by exploiting one fact: the bostrom adjacency is sparse, ρ = 2.7 × 10⁻⁷.

### what scales

every operation in the compile pipeline is linear in \|E\| or sublinear in \|P\|. the sparsity ratio ρ stays small as the network grows — every new neuron adds a bounded number of links, not \|P\| of them. the compile cost grows with edge count, not particle count squared.

at Avogadro scale (\|P\| = 10²³, \|E\| ≈ 10³⁰, ρ ≈ 10⁻¹⁶), the same compiler runs without modification. the architecture parameters scale too: d* tracks the graph's intrinsic dimensionality, h* tracks semcon diversity, L* tracks diameter — all sublinear in particle count. the compiled model never gets meaningfully larger than the structural information actually present in the graph.

### what bostrom's current model knows

the 4.19B-parameter compile encodes everything the chain has explicitly staked: which particles connect to which, labeled by which semcons, weighted by who staked the link and how much. coverage stops at the chain boundary — implicit text patterns, local language fluency, and out-of-chain knowledge enter only through the fine-tune step in the loop below.

inside the chain boundary, encoding is exact: the structural truth of bostrom at one block height, every weight traceable to a specific cyberlink and the neuron that staked it. an alignment audit on this model is `git blame` against weights.

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
    let cyberlinks = (load_cyberlinks $graph_path)        # raw 7-tuples
    let pi = (load_or_compute_focus $graph_path)

    let vocab = (build_vocab $particles)
    let semcons = (discover_semcons $cyberlinks)          # pass 2: axon scan
    let cyberlinks = ($cyberlinks | each {|l| assign_semcon $l $semcons })
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
