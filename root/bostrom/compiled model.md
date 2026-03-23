---
tags: bostrom, cyber, research
crystal-type: entity
crystal-domain: cyber
date: 2026-03-23
---

# bostrom compiled model

the first graph-native [[transformer]] compiled from a live [[knowledge graph]]. no [[gradient descent]]. no training data. the graph IS the model.

compiled March 23, 2026 from [[bostrom]] block ~23,195,000.

## the object

```
bostrom_transformer.onnx    295 MB    ONNX opset 18
bostrom_model.npz           378 MB    embeddings + focus + architecture
```

| parameter | value |
|---|---|
| [[particles]] | 2,921,230 |
| [[cyberlinks]] | 2,705,332 |
| [[neurons]] (unique linkers) | 1,240 |
| embedding dimension $d^*$ | 26 |
| attention heads $h^*$ | 5 |
| transformer layers $L^*$ | 12 (capped from 174) |
| total parameters | 154,609,312 |
| stake-weighted | yes (log-scaled BOOT) |

## what it knows

the embedding table maps every CID in bostrom to a 26-dimensional coordinate. structurally similar particles (linked by similar [[neurons]], in similar neighborhoods) have nearby coordinates.

top particles by [[focus]]:

| focus | content |
|---|---|
| 1.04% | wiki |
| 0.87% | Gregorian calendar |
| 0.41% | (PNG image) |
| 0.32% | glossary |
| 0.22% | asteroid |
| 0.21% | dog |
| 0.20% | spell |
| 0.18% | species of plant |
| 0.18% | species of insect |
| 0.17% | human settlement |

"wiki" holds 1% of total [[focus]] across 2.9 million particles. the [[cybergraph]] considers it the most important concept.

## what it discovered

### spectral gap is observable from convergence

[[PageRank]] converged in 23 iterations. the ratio of successive diffs gives the contraction rate directly:

$$\kappa = \text{median}\left(\frac{d_t}{d_{t-1}}\right) = 0.74$$

$$\lambda_2 = 1 - \frac{\kappa}{\alpha} = 0.13$$

this is two orders of magnitude larger than the paper estimate (0.0015). the network converges faster than predicted. no eigensolver needed — the spectral gap is observed, not computed.

see [[cyber/research/spectral gap from convergence]] for the full paper.

### stake compresses topology

| metric | uniform weights | stake-weighted |
|---|---|---|
| $d^*$ | 33 | 26 |
| singular values (top) | 8.17, 3.29 | 264.8, 98.9 |
| focus entropy | 14.05 bits | 13.73 bits |
| parameters | 197M | 155M |

stake weighting reduces effective dimension from 33 to 26. high-stake [[neurons]] dominate the topology, creating a more compressed structure. fewer dimensions, but each dimension carries more meaning.

### density confirms sparsity

$$\rho = \frac{|E|}{|P|^2} = 3.14 \times 10^{-7}$$

the graph is extremely sparse. dense representation: 34.1 TB. sparse CSR: 41 MB. compression: 850,000×. the [[cybergraph]] is almost entirely empty space — room for growth.

## compilation pipeline

```
cyberlinks.jsonl (550 MB)
  ↓ Step 1: parse 2.7M edges (11s)
neuron_stakes.json
  ↓ Step 2: stake-weighted sparse adjacency (27s)
  ↓ Step 3+4: PageRank + spectral gap from convergence (2.5s)
  ↓ Step 5: randomized SVD → 26-dim embeddings (753s)
  ↓ Step 6: architecture parameters (<0.1s)
  ↓ Step 8: ONNX assembly (2s)
bostrom_transformer.onnx (295 MB)
```

total: ~15 minutes. single machine. no GPU.

## how to rebuild

```bash
# fetch fresh data from bostrom
python3 fetch_cyberlinks.py     # → data/cyberlinks.jsonl
python3 fetch_stakes.py         # → data/neuron_stakes.json

# compile
python3 analizer/compile_model.py data/cyberlinks.jsonl \
  --stakes data/neuron_stakes.json --onnx
```

deterministic: same graph → same model.

## what is missing

the model has a correct skeleton (embeddings from SVD) but incomplete muscles:

| component | status | what it needs |
|---|---|---|
| embedding table | compiled from graph | works — structural similarity search |
| [[focus]] distribution | compiled (PageRank) | works — importance ranking |
| attention weights | random initialization | needs typed [[cyberlinks]] (semcon classification) |
| MLP weights | not computed | needs path sampling (random walks) |
| output projection | capped at 50K vocab | needs full 2.9M vocabulary |

when attention weights come from per-[[semcon]] SVD and MLP weights come from path co-occurrence statistics, the model will reason about the graph — not just index it.

## the path forward

1. resolve CID content → build text↔CID mapping → enable text queries
2. classify [[cyberlinks]] by type (semcon) → compile real attention heads
3. sample random walks → compile MLP weights from path statistics
4. grow the graph past [[phase transition]] ($\lambda_2 > \lambda_{crit}$) → richer embeddings
5. recompile on each new moon alongside [[tri-kernel]] weights

see [[bostrom-to-onnx-pipeline]] for the theoretical specification. see [[cyber/research/bostrom compilation report]] for the detailed empirical report. see [[cyber/seer]] for the link densification strategy. see [[cyber/research/spectral gap from convergence]] for the spectral gap observation method
