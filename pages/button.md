---
tags: prism, cyb
crystal-type: entity
crystal-domain: cyber
---
# Button

call-to-action atom in [[prism]]

the primary interaction primitive. every action a [[neuron]] takes in [[cyb]] flows through a button

## interface

- inputs
	- text: label describing the action
	- icon: optional 16px or 20px [[images]] glyph
	- [[emotion]]: color signal — green (confirm), red (danger), yellow (caution), default (neutral)
	- action: callback triggered on press
	- adviser: tooltip text shown on hover via [[prism/adviser]]
- outputs
	- action event: propagated to parent component
- states
	- default, hover, active, disabled, loading

## variants

- default — single action, most common
- double — two related actions side-by-side (confirm/cancel)
- triple — three options in a row (rare, for multi-path decisions)
- side — attached to the edge of a [[glass]] pane, used in [[prism/hud]]

## composition

- buttons compose into [[prism/bar]] molecules when paired with [[prism/saber]] and [[prism/ion]]
- button + [[prism/adviser]] = guided action
- button inside [[prism/input]] = submit trigger