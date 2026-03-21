---
tags: prysm, cyb
crystal-type: entity
crystal-domain: cyber
stake: 17237820130547482
diffusion: 0.0001921986379728409
springs: 0.00041102574755434463
heat: 0.0003513046080098785
focus: 0.00028966796485470104
gravity: 2
density: 8.86
---
icon library atom in [[prysm]]

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

- 16px: inline with text, inside [[prysm/button]] labels and [[prysm/ion]] atoms
- 20px: standalone small icon, inside [[prysm/tabs]]
- 32px: medium emphasis, inside [[prysm/neuron-card]] and [[prysm/object]]
- 48px: large emphasis, in [[prysm/hud]] and onboarding
- 96px: hero display, in [[cyb/portal]] welcome screens

## composition

- [[prysm/ion]] = images + text label — the standard icon-text pair
- [[prysm/button]] = images + text + action — the standard interactive element