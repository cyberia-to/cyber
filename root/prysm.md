---
icon: 💎
tags: cyb, prysm
alias: design system, prism, prysm design system
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../prysm
stake: 43936669831471920
diffusion: 0.0014254717304230974
springs: 0.0006068291356678248
heat: 0.0008994430440378063
focus: 0.0010746732147194889
gravity: 32
density: 3.2
---

the design system of [[cyb]] — a visual language for interfacing with [[Superintelligence]]

every screen in [[cyb]] is a composition of prysm components. the system defines how humans perceive, navigate, and interact with the [[cybergraph]]

## composition model

four levels, each built from the previous: atoms → molecules → cells → aips

## atoms

indivisible visual primitives

[[prysm/glass]] · [[prysm/text]] · [[prysm/button]] · [[prysm/toggle]] · [[prysm/slider]] · [[prysm/indicator]] · [[prysm/counter]] · [[prysm/address]] · [[prysm/ion]] · [[prysm/saber]] · [[prysm/images]]

## molecules

functional components assembled from atoms

[[prysm/hud]] · [[prysm/mind]] · [[prysm/tabs]] · [[prysm/content]] · [[prysm/display]] · [[prysm/neuron-card]] · [[prysm/aip]] · [[prysm/avatar]] · [[prysm/adviser]] · [[prysm/tooltip]] · [[prysm/input]] · [[prysm/filter]] · [[prysm/table]] · [[prysm/bar]] · [[prysm/pill]] · [[prysm/time-widget]]

## cells

full page regions composed from molecules

[[prysm/portal-cell]] · [[prysm/oracle-cell]] · [[prysm/cyberver-cell]]

## aips

complete autonomous applications built from cells

[[cyb/oracle]] · [[cyb/brain]] · [[cyb/portal]] · [[cyberver]] · [[cyb/sense]] · [[cyb/sigma]] · [[teleport]] · [[sphere]] · [[warp]] · [[aos/hfr]]

## core rules

- [[emotion]] palette: 7 acid colors from [[color-emotion spectrum]] (ROYGBIV)
- typography: monospace, hierarchy through size only (h1 32, h2 24, h3 20, body 16, caption 14, micro 12)
- spacing: 8px grid
- time: UTC 0, Unix epoch = year 0 (e.g. 2026 = year 56)
- [[prysm/responsive]]: desktop >768px, mobile <=768px. atoms identical, molecules rearrange. commander always at bottom

## the prysm and the [[cybergraph]]

prysm renders the cybergraph for human perception. every component maps to a protocol concept: [[particle]] → content renderer, [[neuron]] → identity card, [[cyberlink]] → navigation action, [[cyberank]] → ordering
