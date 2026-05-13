---
tags: cyber, neural, project
alias: mc, model compilation, cyber-mc
crystal-type: entity
crystal-domain: cyber
crystal-size: bridge
repo: ~/git/mc
---

# mc — model compilation

reference rust implementation of [[compiled transformers spec|CT-0]]. reads a [[cyb-graph|.graph]] snapshot, writes a [[cyb-model|.model]] file, no python in the path.

## what it does

```
.graph (cybergraph snapshot) ──► mc ──► .model (transformer checkpoint)
```

mc executes the eight passes from the spec — vocabulary, semcon discovery, architecture parameters, embedding matrix, per-semcon attention, MLP from random walks, norms, and `.model` packaging. the output is loadable by `~/git/cyb/llm` runtime directly via mmap.

## crate layout

```
~/git/mc
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI: mc <input.graph> -o <output.model>
│   ├── lib.rs               # public API: compile(graph) -> model
│   ├── graph/               # .graph reader (mmap, validation, proof check)
│   ├── pass/
│   │   ├── vocab.rs         # pass 1
│   │   ├── semcons.rs       # pass 2
│   │   ├── arch.rs          # pass 3
│   │   ├── embed.rs         # pass 4 — randomized SVD on φ*-weighted adjacency
│   │   ├── attn.rs          # pass 5 — per-semcon SVDs
│   │   ├── mlp.rs           # pass 6 — walk-based PMI factorization
│   │   ├── norm.rs          # pass 7
│   │   └── pack.rs          # pass 8 — .model writer
│   ├── linalg/              # randomized SVD, Lanczos, sparse mul
│   ├── conformance/         # P-EMBED, P-ATTN, P-LAYER, P-DET, P-LOAD checkers
│   └── cli.rs
└── tests/
    └── conformance/         # CT-0 acceptance suite
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

- [ ] phase 0 — crate skeleton, `.graph` mmap reader, `.model` writer scaffolding
- [ ] phase 1 — passes 1-3 (vocab, semcon discovery, architecture)
- [ ] phase 2 — pass 4 (randomized SVD), pass 5 (per-semcon attention)
- [ ] phase 3 — pass 6 (MLP from walks), pass 7 (norms), pass 8 (packaging)
- [ ] phase 4 — conformance suite, certificate emission
- [ ] phase 5 — recompile bostrom-23195000.graph; check P-LOAD against `cyb-llm`

reference for numerical equivalence: the python prototype at `~/git/cyber/analizer/compile_model.py` (passes 1-6, emits `.npz`).

## why a separate crate

mc is a one-job binary: compile a graph into a model. it does not run inference (that is `~/git/cyb/llm`), it does not host the chain (that is `go-cyber`), and it does not maintain the graph (that is the chain). keeping it focused makes the conformance contract small and the binary auditable.

mc is the bridge between [[cyb-graph]] and [[cyb-model]]. every other concern stays out.

---

see [[compiled transformers]] for the readable how-to. see [[compiled transformers spec]] for the formal contract this crate implements. see [[cyb-graph]] for the input format. see [[cyb-model]] for the output format.

discover all [[concepts]]
