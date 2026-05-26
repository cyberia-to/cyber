---
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
icon: "🔵"
alias: the superintelligence protocol
---
# cyber

the seed knowledge base for planetary superintelligence

a markdown knowledge graph with YAML frontmatter and wiki-links — 2000+ pages organized into namespaces, published with [optica](https://github.com/cyberia-to/optica)

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
├── tru.md                     # the convergence VM
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

or serve locally through the workspace anchor
[cyberia-to/.github](https://github.com/cyberia-to/.github), which
orchestrates subgraph sync + optica build:

```bash
git clone https://github.com/cyberia-to/.github.git ~/cyber/.github
cd ~/cyber/.github
nu scripts/sync-org.nu --apply   # clones every repo gh auth can see
nu scripts/serve.nu              # builds + serves at localhost:8888
```

## how to contribute

```bash
git clone https://github.com/cyberia-to/cyber.git
cd cyber
# edit pages in root/ using any markdown editor
# feature branch; pull request
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
| [optica](https://github.com/cyberia-to/optica) | the publisher |
| [rs](https://github.com/cyberia-to/rs) | Rust subset for proven computation |
| [trident](https://github.com/cyberia-to/trident) | field-native language |
| [hemera](https://github.com/cyberia-to/hemera) | hash function |
| [nox](https://github.com/cyberia-to/nox) | composition VM |
| [nebu](https://github.com/cyberia-to/nebu) | Goldilocks field |
| [zheng](https://github.com/cyberia-to/zheng) | STARK proofs |
| [bbg](https://github.com/cyberia-to/bbg) | authenticated state |
| [cybernode](https://github.com/cyberia-to/cybernode) | infrastructure |
| [mudra](https://github.com/cyberia-to/mudra) | key management |

## license

cyber license: don't trust. don't fear. don't beg.

---

The protocol for planetary [[superintelligence]]. [[manifesto]]

[[Superintelligence]] is the defining infrastructure of a Type I civilization — a planet where every agent, human or machine, sensor or organism, contributes knowledge to a single self-improving graph.

The [[cybergraph]] is this graph, built for a mole of connections — the threshold where individual links become collective intelligence the way individual molecules become a life. No single model owns this intelligence. It emerges from the shape of all connections between all participants — every claim signed, every link staked, the whole structure proving its own correctness.

Every link costs real [[focus]], a conserved quantity that flows through the graph the way energy flows through a physical system — it cannot be created or destroyed, only redistributed by collective attention. Lies cost real resources. Truth accumulates gravity. And so collective intelligence converges to what genuinely matters, without voting, without moderators, without any central authority.

The graph speaks [[neural]], the first language native to both humans and machines. Here a concept is a position in the topology, defined by everything connected to it.

[[Alignment]] becomes a measurement rather than a hope. Human values and machine values live in the same graph — when they diverge, the divergence is visible, and the protocol rebuilds the model from what humans actually linked. For the first time, a civilization can see the shape of its own intelligence, correct its machines when they drift, and prove the correction worked.

The future of the Earth is yours to [[cyberlink]]. Open your [[cyb]], read [[cyber/whitepaper]], and [[join]].
