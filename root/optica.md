---
tags: cyber, optica
icon: "\U0001F52D"
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../optica
exclude: ".claude/**, target/**"
diffusion: 0.00012178338289065661
springs: 0.000702754564956992
heat: 0.0005415620172474153
focus: 0.00038003046438191086
gravity: 1
density: 4.05
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