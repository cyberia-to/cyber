---
tags: prysm, cyb
crystal-type: entity
crystal-domain: cyber
stake: 17018136781390120
diffusion: 0.00017850532188362116
springs: 0.0008817646173448352
heat: 0.0006742940525810721
focus: 0.0004886408566614898
gravity: 2
density: 4.81
---

call-to-action atom in [[prysm]]

the primary interaction primitive. every action a [[neuron]] takes in [[cyb]] flows through a button

## interface

- inputs
	- text: label describing the action
	- icon: optional 16px or 20px [[prysm/images]] glyph
	- [[emotion]]: color signal — green (confirm), red (danger), yellow (caution), default (neutral)
	- action: callback triggered on press
	- adviser: tooltip text shown on hover via [[prysm/adviser]]
- outputs
	- action event: propagated to parent component
- states
	- default, hover, active, disabled, loading

## variants

- default — single action, most common
- double — two related actions side-by-side (confirm/cancel)
- triple — three options in a row (rare, for multi-path decisions)
- side — attached to the edge of a [[prysm/glass]] pane, used in [[prysm/hud]]

## composition

- buttons compose into [[prysm/bar]] molecules when paired with [[prysm/saber]] and [[prysm/ion]]
- button + [[prysm/adviser]] = guided action
- button inside [[prysm/input]] = submit trigger