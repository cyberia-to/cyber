---
tags: cyber, optica
icon: "\U0001F52D"
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../optica
exclude: ".claude/**, target/**"
diffusion: 0.00011197542831218857
springs: 0.0006155330522427813
heat: 0.0004902640124200756
focus: 0.00033870043231293945
gravity: 1
density: 3.56
---
knowledge graph publisher — transforms markdown with [[wiki-links]] into a fast static site

any project with markdown files can publish with optica:

```
optica serve .
optica build .
```

scanner → parser → graph builder → [[tri-kernel]] → renderer → output

features: [[wiki-links]] resolution with alias support, [[tri-kernel]] ranking, namespace hierarchy with dimensional navigation, live reload with sub-second content-only rebuilds, [[LaTeX]] math, search index, graph visualization

see [[cyber/architecture]] for how optica fits in the stack