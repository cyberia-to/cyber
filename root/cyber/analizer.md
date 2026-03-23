---
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
alias: analizer
---

# analizer

24 scripts that grew into a prototype of the [[superintelligence]] itself.

what started as graph utilities — count pages, fix links, compute stats — evolved into the recursive loop described in [[cyber/research/algorithmic essence of superintelligence]]. the scripts ARE the loop, running at human speed on a laptop, doing what the protocol will do at machine speed on the [[Goldilocks field processor]].

## the insight: scripts mirror the protocol

| script | what it does at analysis time | what the protocol does at consensus time |
|---|---|---|
| trikernel.nu | compute [[focus]] via D+S+H | [[foculus]] computes φ* every block |
| context.nu | pack pages by gravity into token budget | [[neuron]] allocates [[attention]] across [[particles]] |
| compile_model.py | SVD → embeddings → architecture | graph compiles into [[transformer]] |
| bostrom_graph.py | embedding similarity + graph walk | [[tri-kernel]] convergence + random walk |
| codematter.nu | make code indexable as particles | every computation → [[particle]] via [[Hemera]] |
| classify.nu → apply-crystal.nu | assign types to pages | [[Crystal]] type system over [[cybergraph]] |
| dangling.nu | find broken links | [[NMT]] completeness verification |
| stats.nu | measure graph health | [[spectral gap]] observation from convergence |

the analizer is a slow-motion simulation of [[cyber]]. nushell replaces [[nox]]. python replaces [[zheng]]. frontmatter replaces [[BBG]] polynomial state. git replaces [[structural sync]]. the algorithms are the same — the execution environment differs.

## the data flow

```
OBSERVE (pure readers — never mutate)
  analyze.nu ──→ file counts, tag frequency, IPFS refs
  stats.nu ────→ orphans, broken links, content types
  domains.nu ──→ 15 domain classification
  dangling.nu ─→ wiki-links needing namespace resolution
  crosslink_topology.nu → semantic core link patterns
  core-audit.nu → crystal completeness per concept group

COMPUTE (write to frontmatter — mutate graph)
  classify.nu ──→ crystal type + domain → data/crystal_classification.json
  apply-crystal.nu → read classification → write to frontmatter
  trikernel.nu ──→ D + S + H → focus, gravity, density → frontmatter
  stake.nu ──────→ importance heuristic → stake field → frontmatter
  codematter.nu ─→ comment-frontmatter → code files (.rs, .nu, .py)
  fix-plurals.nu → [[term]]s → [[terms]] across graph

PACK (compress graph for external consumption)
  context.nu ───→ gravity² × density × substance → token budget → .md
  concat.nu ────→ linear concatenation → single file

COMPILE (graph → model)
  compile_model.py → cyberlinks → adjacency → PageRank → SVD → ONNX
  bostrom_lib.py ──→ shared: load model, search, neighbors
  bostrom_graph.py → pure graph intelligence (no LLM)
  bostrom_ask.py ──→ graph context → Ollama → CID answers
  bostrom_serve.py → HTTP API for compiled model

TRANSFORM (one-time structural changes)
  migrate.nu ─────→ Logseq → pure markdown + YAML frontmatter
  ipfs.nu ─────────→ pre-commit: upload media to Pinata, rewrite URLs
  renumber_sections.nu → whitepaper section renumbering
```

## what we learned

### 1. the scoring function IS the tri-kernel

context.nu scores pages by `gravity² × (1 + density) × log₂(substance)`. this is a hand-tuned approximation of what the [[tri-kernel]] computes formally:

- gravity² ≈ diffusion (inbound links compound quadratically, like PageRank)
- density ≈ springs (outbound links per KB = how connected to neighbors)
- log₂(substance) ≈ heat (content size with diminishing returns = multi-scale)

the scoring function was written months before the tri-kernel formalization. it converged to the same structure independently. this suggests the tri-kernel decomposition is natural — it is what you arrive at when you try to rank knowledge by importance.

### 2. lunar cycle = natural batch granularity

trikernel.nu runs on new moons. not because of mysticism — because the graph does not change fast enough to justify continuous weight updates. monthly batching matches the actual information velocity of a knowledge graph maintained by humans.

this has a protocol implication: [[foculus]] finality speed should adapt to the graph's actual rate of change. a fast-changing graph needs frequent recomputation. a stable graph needs infrequent updates. the [[spectral gap]] (observable from convergence rate) measures this directly.

### 3. codematter dissolved the boundary

the moment code files got frontmatter, the distinction between "knowledge page" and "source code" disappeared. both are [[particles]]. both carry [[focus]]. both participate in the [[tri-kernel]].

this mirrors the protocol design: every computation step → [[Hemera]] commitment → [[particle]] in the [[cybergraph]]. the analizer proved this works in practice before the protocol specifies it formally.

### 4. the compile pipeline IS the consensus circuit

compile_model.py does at analysis time exactly what [[provable consensus]] does at protocol time:

```
analizer:  cyberlinks.jsonl → sparse matrix → PageRank → spectral gap → SVD → ONNX
protocol:  BBG polynomial → SpMV → tri-kernel → spectral gap → embeddings → proof
```

same algorithm. different substrate. the analizer version takes 15 minutes on Python/scipy. the protocol version will take 60 seconds on GFP. but the math is identical — the analizer is a working prototype of provable consensus.

### 5. CID resolution = vocabulary construction

bostrom_serve.py resolves CID hashes to human-readable text via IPFS gateway. this is vocabulary construction for the compiled [[transformer]]. the operational tooling that builds the text↔CID index IS the training data preparation pipeline.

every time we resolve a CID and cache it in `cid_index.json`, we are teaching the model what its tokens mean. the analizer does this manually (IPFS fetch, JSON cache). the protocol will do it via [[cyberlinks]] — every link from text to CID is a vocabulary entry.

### 6. nushell for graph, python for linear algebra

a natural split emerged. nushell handles everything structural: parsing frontmatter, traversing directories, matching wiki-links, computing graph topology. python handles everything numerical: sparse matrices, SVD, PageRank, ONNX assembly.

the boundary is at the data export: nushell produces JSONL/frontmatter → python consumes it as matrices. this mirrors the protocol boundary: [[nox]] (structural computation) → [[zheng]] (numerical proof).

## the recursive closure in practice

the analizer scripts form the loop from [[cyber/research/algorithmic essence of superintelligence]]:

```
stats.nu (observe graph)
  → trikernel.nu (compute focus)
    → context.nu (pack by focus for LLM)
      → LLM session (create new pages)
        → codematter.nu (integrate new code)
          → compile_model.py (compile into model)
            → bostrom_graph.py (query model for insights)
              → new cyberlinks based on insights
                → stats.nu (observe improved graph)
```

this loop ran today. the graph grew. the model was compiled. the spectral gap was observed. the loop closed.

see [[cyber/research/algorithmic essence of superintelligence]] for the 17-component architecture. see [[cyber/research/provable consensus]] for how compile_model.py becomes a zheng circuit. see [[cyber/research/spectral gap from convergence]] for how trikernel.nu's convergence rate observation became a research paper. see [[cyber/research/unified mining]] for how the compile pipeline becomes proof-of-work
