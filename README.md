# 🔵 cyber

the seed knowledge base for planetary superintelligence

a markdown knowledge graph with YAML frontmatter and wiki-links — 2000+ pages organized into namespaces, published with [optica](https://github.com/cybercongress/optica)

**[cyber.page](https://cyber.page/)** — live site

## structure

```
root/                          # all pages
├── cyber/                     # the protocol
│   ├── graph.md               # cybergraph — formal definition, six axioms
│   ├── hierarchy.md           # 4D scaling — cells, zones, domains
│   ├── truth/                 # truth architecture
│   │   ├── serum.md           # honesty equilibrium (BTS)
│   │   ├── coupling.md        # TRUE/FALSE market (ICBS)
│   │   └── valence.md         # ternary epistemic seed
│   ├── tokens.md              # the nouns
│   ├── nomics.md              # the verbs and rules
│   ├── netics.md              # the whole machine as feedback diagram
│   ├── self/                  # what the protocol does autonomously
│   └── research/              # open research areas
├── cyb/                       # the browser/interface
│   ├── fs/                    # filesystem over the cybergraph
│   └── languages.md           # 15 computation languages
├── cyberia/                   # the network state
├── bostrom/                   # the bootloader chain
├── species/                   # Latin binomial species pages
├── focus.md                   # collective attention distribution
├── particle.md                # content-addressed node
├── neuron.md                  # the one who links
├── tru.md                     # the truth machine
├── nox.md                     # composition VM
└── cyberspace.md              # the navigable semantic space
```

## key concepts

| Concept | What it is |
|---------|-----------|
| [particle](https://cyber.page/particle) | content-addressed node — identity = hash of content |
| [cyberlink](https://cyber.page/cyber-link) | signed, staked, timestamped assertion binding two particles |
| [neuron](https://cyber.page/neuron) | agent who links — human, AI, sensor, or program |
| [focus](https://cyber.page/focus) | collective attention distribution over all particles |
| [cyberank](https://cyber.page/cyber-rank) | per-particle probability of observation (tri-kernel fixed point) |
| [will](https://cyber.page/cyber-will) | locked balance × time — budget for attention allocation |
| [karma](https://cyber.page/karma) | earned trust from contribution |
| [cyberspace](https://cyber.page/cyberspace) | the navigable semantic space that emerges from markup + graph |

## how to use

browse at [cyber.page](https://cyber.page/)

or serve locally:

```bash
git clone https://github.com/cybercongress/cyber.git ~/git/cyber
git clone https://github.com/cybercongress/optica.git ~/git/optica
cd ~/git/optica && cargo build --release
~/git/optica/target/release/optica serve ~/git/cyber --open
```

serves on http://localhost:8888

## how to contribute

```bash
git clone https://github.com/cybercongress/cyber.git
cd cyber
# edit pages in root/ using any markdown editor
# make contribution into a feature branch
# pull request
```

pages are pure markdown with YAML frontmatter:

```markdown
---
tags: cyber, core
alias: alternative name
icon: "🔵"
---
content with [[wiki-links]] and $\LaTeX$ math
```

## subgraphs

cyber imports 10 external repos as subgraphs — their pages appear in the published graph:

| Subgraph | What it is |
|----------|-----------|
| [optica](https://github.com/cybercongress/optica) | the publisher |
| [rs](https://github.com/cybercongress/rs) | Rust subset for proven computation |
| [trident](https://github.com/cybercongress/trident) | field-native language |
| [hemera](https://github.com/cybercongress/hemera) | hash function |
| [nox](https://github.com/cybercongress/nox) | composition VM |
| [nebu](https://github.com/cybercongress/nebu) | Goldilocks field |
| [zheng](https://github.com/cybercongress/zheng) | STARK proofs |
| [bbg](https://github.com/cybercongress/bbg) | authenticated state |
| [cybernode](https://github.com/cybercongress/cybernode) | infrastructure |
| [mudra](https://github.com/cybercongress/mudra) | key management |

## license

cyber license: don't trust. don't fear. don't beg.
