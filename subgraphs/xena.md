---
name: xena
repo: ../xena
subgraph: true
visibility: private
archived: false
---

# xena

Zero-copy XelisHash V3 miner for Apple Silicon. Uses [[honeycrisp]] (acpu, aruminium, unimem) for unified-memory kernel dispatch. Targets both CPU (P-cores only, 10-thread optimum) and GPU paths with SBOX→tgmem and fast-div optimizations; part of the multi-miner family alongside [[erga]], [[mona]], and [[zoya]].
