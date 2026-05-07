---
tags: cyb, cyber, core
alias: text particle, prose, markdown
crystal-type: entity
crystal-domain: cyb
---
prose, code, and thought as [[particle]]. the most linked content type in the [[cybergraph]]

source format: [[markdown]]. everything that flows as readable sequence — articles, notes, arguments, instructions, documentation, messages, poetry

---

## rendering

every character is a GPU operation, not a DOM node. the pipeline:

```
markdown source → parse → glyph layout (rustybuzz) → raster (swash) → GPU glyph atlas → fragment shader
```

monospace or proportional. any scale. any surface. the same text particle renders identically on a 4K desktop, a mobile screen, and a paper PDF

## in the cybergraph

text is how humans think in the [[cybergraph]]. a [[neuron]] writes — a text particle enters the graph. the text particle gets a CID. other neurons link to it, affirm it, contradict it, extend it. [[cyberank]] accumulates. the text that matters rises

types of text particles: research papers, blog posts, code files, chat messages, definitions, proofs in prose, wiki pages, transcripts, arguments, manifestos

## properties

- human-readable without tooling — raw markdown is readable as text
- composable — text particles nest inside [[component]] particles
- linkable at any granularity — a sentence, a paragraph, an entire document — the CID is the handle
- diff-able — two text particles can be compared. a chain of text particles is version history in the graph

## relation to other languages

a scientific paper is text + [[formula]] + [[table]] + [[pixels]]. the text holds the argument. the other types hold the evidence. the [[component]] holds the paper

see [[markdown]] for the source format. see [[component]] for composition. see [[particle]] for the type system

discover all [[concepts]]
