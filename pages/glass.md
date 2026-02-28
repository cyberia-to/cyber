---
tags: prism, cyb
crystal-type: entity
crystal-domain: cyber
---
the foundational surface atom in [[prism]]

translucent pane that contains other components. all composition in [[cyb]] happens on glass

## interface

- inputs
	- depth: foreground, midground, background — controls opacity and blur
	- tint: [[emotion]] color overlay
	- edge: whether the pane has a visible border
- outputs
	- containment: child components inherit the glass depth context

## variants

- plane — flat surface, standard container
- side-button — surface with an attached action trigger on the edge

## properties

- opacity gradient encodes depth in the z-axis
- blur increases with depth — foreground is sharp, background is diffuse
- tint modulates with [[emotion]] state of the parent [[aip]]

## composition

- glass is the only surface atom. every [[prism/hud]], [[prism/display]], and [[prism/object]] sits on glass
- glass panes nest: a card (glass) inside a section (glass) inside a screen (glass)
- nesting depth should stay ≤ 3 to maintain visual clarity