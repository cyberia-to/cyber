---
tags: cyber, article, neural
alias: compiled transformer, transformer compilation, compiling transformers
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---
a practical procedure for turning a [[cybergraph]] into transformer weights without training.

[[graph native transformer]] proves that embedding dimension, head count, and layer depth are determined by graph structure alone. the [[CT-0]] spec defines the exact 8-pass compilation from `.graph` to `.model`.

on [[bostrom]] (3.1M particles, 12 semcons): d=300, h=12, L=290 → 4.19B parameters compiled in ~64 seconds on one workstation. every weight traces to specific [[cyberlinks]] and the [[neurons]] that staked them. graph updates produce weight updates — recompile one pass, not the whole model.

the loop: compile on explicit graph → fine-tune on text → extract implicit links → stake → recompile. each cycle the explicit graph absorbs more of what was implicit.

discover all [[concepts]]
