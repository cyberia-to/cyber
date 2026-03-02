---
tags: prism, cyb
crystal-type: entity
crystal-domain: cyber
stake: 17237820130547482
---
icon library atom in [[prism]]

the complete set of glyphs used across [[cyb]]. every icon has a semantic meaning tied to a protocol concept

## interface

- inputs
	- name: icon identifier (token logo, action verb, navigation target)
	- size: 16, 20, 32, 48, 96 px
- outputs
	- rendered glyph

## categories

- token logos: [[CYB]], [[HYDROGEN]], [[BOOT]], [[VOLT]], [[AMPERE]], [[BTC]], [[ETH]], [[ATOM]]
- action icons: search, learn, link, stake, send, receive, delegate
- navigation glyphs: home, back, forward, menu, close, expand
- status icons: success, error, warning, info, loading
- brand marks: [[cyber]], [[cyb]], [[cyberia]]

## sizing

- 16px: inline with text, inside [[button]] labels and [[prism/ion]] atoms
- 20px: standalone small icon, inside [[prism/tabs]]
- 32px: medium emphasis, inside [[prism/neuron-card]] and [[prism/object]]
- 48px: large emphasis, in [[prism/hud]] and onboarding
- 96px: hero display, in [[portal]] welcome screens

## composition

- [[prism/ion]] = images + text label — the standard icon-text pair
- [[button]] = images + text + action — the standard interactive element
