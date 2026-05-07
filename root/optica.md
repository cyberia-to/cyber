---
tags: cyber, optica
icon: "\U0001F52D"
crystal-type: entity
crystal-domain: cyber
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
