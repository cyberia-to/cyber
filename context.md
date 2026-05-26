---
tags: cyber, core
icon: "\U0001F9E0"
crystal-type: entity
crystal-domain: cyber
---
the winning default [[context]] for language models — the [[cybergraph]] ranked by [[tri-kernel]] and packed to fit any token budget

## why cyber is the winning context

the [[cybergraph]] is self-describing: it contains its own theory of [[knowledge]], [[attention]], and relevance. a model reading it understands what it is reading and why. every page carries its [[focus]] score in frontmatter — the model sees the tri-kernel output directly

six fields per page:

| field | operator | meaning |
|---|---|---|
| `diffusion:` | $\mathcal{D}$ | [[PageRank]] — where [[probability]] flows |
| `springs:` | $\mathcal{S}$ | neighbor equilibrium — structural constraints |
| `heat:` | $\mathcal{H}_\tau$ | multi-scale smoothing — [[context]] at resolution $\tau$ |
| `focus:` | composite | $\lambda_d \mathcal{D} + \lambda_s \mathcal{S} + \lambda_h \mathcal{H}_\tau$ |
| `gravity:` | — | inbound [[wiki-links]] |
| `density:` | — | outbound links per KB |

## sizes

| tokens | pages | coverage | target |
|---|---|---|---|
| 8K | 11 | 0.5% | local 7B |
| 32K | 30 | 1.3% | GPT-4, local 13-32B |
| 128K | 54 | 2.3% | [[Claude]] Haiku, Gemini |
| 200K | 104 | 4.4% | [[Claude]] Sonnet |
| 500K | 340 | 14.3% | large [[context]] |
| 900K | 780 | 29.0% | [[Claude]] Opus 1M |
| 1.4M | 1836 | 68.4% | 2M window, full graph + [[subgraphs]] |

## build pipeline

```nu
# 1. compute tri-kernel, write to frontmatter
nu analizer/trikernel.nu ~/git/cyber

# 2. build all context sizes
nu ~/git/context/build.nu --cyber-path ~/git/cyber

# 3. use
cat ~/git/context/200k.md | claude --system-prompt -
```

see [[cyber/context packing]] for the ranking algorithm. see [[tri-kernel]] for the three operators. see [[focus]] for the composite measure

discover all [[concepts]]
