---
tags: prism, cyb
crystal-type: entity
crystal-domain: cyber
---
# Counter

numeric display atom in [[prism]]

renders a single number with optional [[emotion]] color. used wherever [[cyb]] shows a quantity: [[karma]], token balance, [[cyberank]] score, link count

## interface

- inputs
	- number: the value to display
	- [[emotion]]: color signal (green for growth, red for decline, neutral for static)
	- adviser: hover text explaining the number via [[prism/adviser]]
	- format: integer, decimal, abbreviated (1.2k, 3.4M)
- outputs
	- display only — no interaction

## composition

- counter inside [[prism/object]] = entity metric
- counter inside [[sigma]] = token balance
- counter inside [[prism/neuron-card]] = [[karma]] or rank display
- counter + [[prism/indicator]] = progress toward a goal