---
tags: prysm, cyb
crystal-type: entity
crystal-domain: cyber
stake: 16344441177307554
diffusion: 0.00026483981887414135
springs: 0.0005829228263835098
heat: 0.0005078998791768039
focus: 0.00040887673318748644
gravity: 4
density: 7.08
---

numeric display atom in [[prysm]]

renders a single number with optional [[emotion]] color. used wherever [[cyb]] shows a quantity: [[karma]], token balance, [[cyberank]] score, link count

## interface

- inputs
	- number: the value to display
	- [[emotion]]: color signal (green for growth, red for decline, neutral for static)
	- adviser: hover text explaining the number via [[prysm/adviser]]
	- format: integer, decimal, abbreviated (1.2k, 3.4M)
- outputs
	- display only — no interaction

## composition

- counter inside [[prysm/object]] = entity metric
- counter inside [[cyb/sigma]] = token balance
- counter inside [[prysm/neuron-card]] = [[karma]] or rank display
- counter + [[prysm/indicator]] = progress toward a goal