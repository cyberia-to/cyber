---
tags: cyber, neural, project
alias: mc, model compilation, cyber-mc
crystal-type: entity
crystal-domain: cyber
crystal-size: bridge
repo: ~/git/mc
---

# mc — model compilation

reference rust implementation of [[compiled transformers spec|CT-1]]. reads a [[cyb-graph|.graph]] snapshot, writes a [[cyb-model|.model]] file, no python in the path.

## what it does

```
.graph (cybergraph snapshot) ──► mc ──► .model (transformer checkpoint)
```

mc executes the eight passes from the spec — vocabulary, semcon discovery, architecture parameters, embedding matrix, per-semcon attention, MLP from random walks, norms, and `.model` packaging. the output is loadable by `~/git/cyb/llm` runtime directly via mmap.

## crate layout

```
cyb/mc/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI: inspect / compile / snapshot / bundle
│   ├── lib.rs
│   ├── graph/               # .graph reader (mmap, signal iter, config parser)
│   │   ├── config.rs        # GraphConfig TOML: chain_id, block, tokens
│   │   ├── frontmatter.rs   # .cyb three-rule frontmatter
│   │   ├── reader.rs        # Graph::open, signals(), config_parsed()
│   │   └── record.rs        # Signal (44B header + 105B×n links), SignalIter
│   ├── pass/
│   │   ├── pass1_index.rs   # particle index + CSR adjacency
│   │   ├── pass2_semcon.rs  # semcon discovery + per-axon assignment
│   │   ├── pass3_arch.rs    # π* power iteration, rSVD d*, Lanczos λ2, BFS diam
│   │   ├── pass4_embed.rs   # embedding matrix E via seeded rSVD
│   │   ├── pass5_attn.rs    # Q/K/V/O per layer (random init fallback)
│   │   ├── pass6_mlp.rs     # signal-respecting walks, PMI, SVD factorization
│   │   ├── pass7_norms.rs   # all-ones layer norms
│   │   └── pass8_pack.rs    # .model packaging
│   ├── snapshot/            # JSONL → .graph converter
│   └── bundle/              # .graph + .model → .gm descriptor
└── tests/
    └── smoke.rs             # round-trip signal encoding test
```

## dependencies

```toml
[dependencies]
sprs = "0.11"             # sparse CSR matrices
ndarray = "0.16"          # dense linalg
ndarray-linalg = "0.17"   # SVD, Lanczos
cyber-hemera = { path = "../../hemera/rs" }  # Poseidon2 over Goldilocks — CIDs and RNG seeds
rand_chacha = "0.3"       # deterministic RNG
toml_edit = "0.22"        # frontmatter parsing
memmap2 = "0.9"           # zero-copy graph load
rayon = "1"               # parallel per-semcon SVDs
clap = "4"                # CLI
cyb-format = { path = "../cyb/llm" }  # .model writer from cyb-llm crate
```

no python. no pytorch. no safetensors crate (`.model` is the format, not safetensors). no GPU dependency for the compile path — only CPU sparse linalg.

## CLI

```
mc <input.graph> [options] -o <output.model>

Options:
  -o, --output <PATH>           output .model file
  --no-proof                    skip .graph proof verification (local snapshots)
  --certificate <PATH>          write conformance certificate to PATH (default: stdout)
  --no-cert                     skip conformance checks (faster, for development)
  --threads <N>                 parallelism (default: num_cpus)
  --seed <HEX>                  override deterministic seed (default: from snapshot CID)
  --max-rank <N>                cap embedding dimension (default: from spec, 4096)
  --max-layers <N>              cap layer count (default: from spec, 512)
  --dry-run                     compute architecture, do not emit weights
```

stream from stdin:

```
curl -s https://node.bostrom.cybernode.ai/cyber/graph/snapshot?block=23195000 \
  | mc - -o bostrom-23195000.model
```

## status

- [x] crate skeleton, `.graph` mmap reader, `.model` writer scaffolding
- [x] pass 1: particle index + CSR adjacency (hemera axon hashing)
- [x] pass 2: semcon discovery (usage × log coverage scoring)
- [x] pass 3: power iteration for π*, randomized SVD for d*, Lanczos for λ2, BFS diameter
- [x] pass 4: embedding matrix E via seeded randomized SVD
- [x] pass 5: per-semcon attention Q/K/V/O (random init fallback when no typed links)
- [x] pass 6: signal-respecting walks, PMI co-occurrence, SVD factorization
- [x] pass 7: all-ones layer norms
- [x] pass 8: `.model` file packaging (card/config/program/tensors/vocab/eval/weights)
- [x] `mc snapshot --from-jsonl` for one-time JSONL migration
- [x] `mc snapshot --rpc <url>` live import from go-cyber Tendermint RPC (tx-search based)
- [x] `mc bundle` for `.gm` descriptor production
- [ ] conformance suite (P-EMBED, P-ATTN, P-LAYER checks)
- [ ] bostrom end-to-end compile + P-LOAD check against cyb/run

subcommands: `mc inspect`, `mc compile`, `mc snapshot`, `mc bundle`.

## why a separate crate

mc is a one-job binary: compile a graph into a model. it does not run inference (that is `~/git/cyb/llm`), it does not host the chain (that is `go-cyber`), and it does not maintain the graph (that is the chain). keeping it focused makes the conformance contract small and the binary auditable.

mc is the bridge between [[cyb-graph]] and [[cyb-model]]. every other concern stays out.

---

see [[compiled transformers]] for the readable how-to. see [[compiled transformers spec]] for the formal contract this crate implements. see [[cyb-graph]] for the input format. see [[cyb-model]] for the output format.

discover all [[concepts]]
