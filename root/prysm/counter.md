---
tags: prysm, cyb
crystal-type: entity
crystal-domain: cyber
stake: 16344441177307554
diffusion: 0.00024723626327222657
springs: 0.0005151009444735916
heat: 0.00046292422692658145
focus: 0.0003707332603635023
gravity: 4
density: 6.3
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