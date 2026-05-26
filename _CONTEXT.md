# context

the winning default context for language models

## the idea

every model starts empty. context is the only lever between raw capability and useful intelligence. most contexts are ad-hoc — a system prompt, some examples, maybe a document dump. this repo builds something different: a knowledge graph about intelligence itself, ranked by its own relevance algorithm, packed to fit any context window

the source is [cyber](https://github.com/cybercongress/cyber) — a knowledge graph for planetary superintelligence. ~2300 pages covering: the cybergraph (a directed authenticated multigraph over content-addressed nodes), the tri-kernel (diffusion + springs + heat operators that compute collective focus), consensus, cryptography, epistemology, game theory, information geometry, and the full protocol specification

every page carries its own tri-kernel score in the frontmatter:

```yaml
diffusion: 0.030264  # PageRank — where probability flows
springs: 0.000951    # neighbor equilibrium — structural constraints
heat: 0.010356       # multi-scale smoothing — context at resolution τ
focus: 0.017489      # composite: 0.5D + 0.3S + 0.2H
gravity: 342          # inbound links
density: 10.07        # outbound links per KB
```

the packer uses these scores to select the most valuable pages for each context budget. highest focus first, greedy knapsack until full

## sizes

| file | tokens | target models |
|---|---|---|
| `8k.md` | 8K | local 7B, GPT-3.5 |
| `32k.md` | 32K | GPT-4, local 13-32B |
| `128k.md` | 128K | Claude Haiku, Gemini, GPT-4 Turbo |
| `200k.md` | 200K | Claude Sonnet |
| `500k.md` | 500K | Claude with room for dialogue |
| `900k.md` | 900K | Claude Opus 1M |
| `1400k.md` | 1.4M | 2M context windows, full graph + subgraphs |

## usage

drop any size file into your model's system prompt or context window:

```
# claude code
cat 200k.md | claude --system-prompt -

# api
{"system": "<contents of 128k.md>", "messages": [...]}

# local llm
llama-cli -m model.gguf --system-prompt-file 32k.md
```

## build

requires [cyber](https://github.com/cybercongress/cyber) repo with `analizer/context.nu`

```nu
nu build.nu                                # all sizes
nu build.nu --sizes [128 500]              # specific sizes
nu build.nu --cyber-path ~/my/cyber        # custom path
```

## why this context wins

1. self-describing: the knowledge graph contains its own theory of knowledge, attention, and relevance. a model reading it understands what it is reading and why

2. mathematically ranked: pages are selected by the same tri-kernel algorithm the protocol uses for consensus. not curated by hand — ranked by graph structure

3. compositional: every page uses wiki-links. the model sees the full link topology and can reason about relationships between concepts

4. dense: the graph covers mathematics, cryptography, game theory, information theory, biology, economics — all unified under one protocol. maximum knowledge per token

5. recursive: the context describes the process that generated the context. the model can verify and improve the ranking

## license

Don't trust. Don't fear. Don't beg.
