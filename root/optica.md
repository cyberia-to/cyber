---
tags: cyber, optica
icon: "\U0001F52D"
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00011995724671972114
springs: 0.0005498271614917363
heat: 0.0004460146902295187
focus: 0.00031412970985329443
gravity: 2
density: 3.55
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
