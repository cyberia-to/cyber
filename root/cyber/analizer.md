---
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
---

# analizer

the nushell + python toolkit for the [[cybergraph]]. 24 scripts across three functional layers: analysis, transformation, and model compilation.

## graph analysis

| script | what it does | usage |
|---|---|---|
| analyze.nu | general dashboard: files, tags, links, IPFS refs, largest pages | `nu analizer/analyze.nu ~/git/cyber` |
| stats.nu | comprehensive stats: orphans, broken links, content types | `nu analizer/stats.nu ~/git/cyber` |
| crosslink_topology.nu | semantic core wiki-link patterns, hub/island detection | `nu analizer/crosslink_topology.nu ~/git/cyber` |
| core-audit.nu | audit 9 concept groups for completeness (frontmatter, icons, aliases) | `nu analizer/core-audit.nu ~/git/cyber` |
| domains.nu | classify pages into 15 knowledge domains | `nu analizer/domains.nu ~/git/cyber` |
| dangling.nu | find [[wiki-links]] that resolve to namespaced matches | `nu analizer/dangling.nu ~/git/cyber` |

## graph transformation

| script | what it does | usage |
|---|---|---|
| trikernel.nu | compute [[tri-kernel]] ([[diffusion]], [[springs]], [[heat]]) → write [[focus]] + [[gravity]] + density to frontmatter. lunar cycle: runs on new moon only | `nu analizer/trikernel.nu ~/git/cyber -s` |
| classify.nu | classify pages by [[crystal]] type (E/P/Q/R/M/S) and domain | `nu analizer/classify.nu ~/git/cyber` |
| apply-crystal.nu | apply crystal classification from classify.nu output to frontmatter | `nu analizer/apply-crystal.nu ~/git/cyber` |
| codematter.nu | add comment-frontmatter to code files (.rs, .nu, .toml, .py, .sh) | `nu analizer/codematter.nu ~/git/cyber -s` |
| fix-plurals.nu | fix [[wiki-link]] floating plural suffixes | `nu analizer/fix-plurals.nu ~/git/cyber` |
| stake.nu | assign stake values to pages based on importance heuristics | `nu analizer/stake.nu ~/git/cyber` |
| migrate.nu | convert Logseq graph to pure markdown with YAML frontmatter | `nu analizer/migrate.nu ~/git/cyber` |
| ipfs.nu | pre-commit hook: upload media to Pinata IPFS, rewrite URLs | `nu analizer/ipfs.nu ~/git/cyber` |

## context generation

| script | what it does | usage |
|---|---|---|
| context.nu | smart context packer: gravity² × density × substance scoring, greedy knapsack into token budget | `nu analizer/context.nu ~/git/cyber -s --budget 500` |
| concat.nu | simple concatenation of all pages into one file | `nu analizer/concat.nu ~/git/cyber -s` |

## model compilation ([[bostrom]] → [[transformer]])

| script | what it does | usage |
|---|---|---|
| compile_model.py | 6-step pipeline: cyberlinks → sparse adjacency → [[PageRank]] + [[spectral gap]] → randomized SVD → architecture params → ONNX | `python3 analizer/compile_model.py data/cyberlinks.jsonl --stakes data/neuron_stakes.json --onnx` |
| bostrom_lib.py | shared module: load_model, search, embedding neighbors | `from analizer.bostrom_lib import load_model` |
| bostrom_graph.py | pure graph intelligence: embedding retrieval + graph walk + spectral role analysis. no LLM | `python3 analizer/bostrom_graph.py "dog"` |
| bostrom_ask.py | [[Ollama]] hybrid: graph context injected into LLM prompt | `python3 analizer/bostrom_ask.py "wiki"` |
| bostrom_serve.py | HTTP server: OpenAI/Ollama-compatible API for compiled model | `python3 analizer/bostrom_serve.py --build-index` |

## dependency chain

```
classify.nu → apply-crystal.nu → core-audit.nu
trikernel.nu → context.nu → LLM context
compile_model.py → bostrom_lib.py → bostrom_{graph,ask,serve}.py
```

## conventions

all scripts accept `graph_path` as first argument. nushell scripts use `def main [graph_path: string]`. python scripts use `sys.argv[1]`.

graph-agnostic: run against any graph, not just [[cyber]]:
```
nu analizer/stats.nu ~/git/cloud-forest
nu analizer/trikernel.nu ~/git/zheng --dry-run
```

see [[CLAUDE.md]] for the full script list with descriptions
