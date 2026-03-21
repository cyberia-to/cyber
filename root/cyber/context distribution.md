---
tags: cyber, core
icon: "\U0001F9E0"
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../context
exclude: ".claude/**, .git/**"
---
the winning default [[context]] for language models — the [[cybergraph]] ranked by [[tri-kernel]], packed to fit any token window, addressed as one [[personality]]

every context file starts with SOUL.md — the [[personality]] preamble that tells the model who it is. then pages sorted by [[focus]], highest first. each page carries six [[tri-kernel]] fields in frontmatter

seven sizes from 8K to 1.4M tokens

| tokens | pages | target |
|---|---|---|
| 8K | 12 | local 7B |
| 32K | 28 | GPT-4, local 13-32B |
| 128K | 51 | [[Claude]] Haiku, Gemini |
| 200K | 96 | [[Claude]] Sonnet |
| 500K | 336 | large [[context]] |
| 900K | 771 | [[Claude]] Opus 1M |
| 1.4M | 1801 | 2M window, full graph + [[subgraphs]] |

see [[context]] for the concept. see [[cyber/context packing]] for the ranking algorithm. see [[cyber/personality]] for the soul

discover all [[concepts]]
