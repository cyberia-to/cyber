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

## first principles

- ### the interface is a lens
	- [[cyb]] refracts the [[cybergraph]] into something a human can perceive and act on
	- prysm decomposes this refraction into composable layers: surface → element → region → application
	- each layer adds meaning without hiding the underlying structure
- ### emotion as signal
	- components carry [[emotion]] — a color-coded signal layer
	- emotion reflects the state of the [[cyb/robot]]: confidence, uncertainty, danger, opportunity
	- emotion is computed from [[cyberank]], [[karma]], and context — it is data, rendered as feeling
- ### everything is a [[particle]]
	- every piece of content in [[cyb]] is a [[particle]] — text, image, video, audio, pdf, 3d model
	- prysm defines how each particle type is rendered, selected, linked, and composed
	- the renderer adapts to the particle format. the interface stays consistent
- ### the [[neuron]] is the user
	- identity in prysm is a [[neuron]] with an [[cyb/avatar]]
	- every action traces to a neuron. every view is from a neuron's perspective
	- prysm renders identity as cards, addresses, reputation indicators, and activity streams
- ### glass as medium
	- [[prysm/glass]] is the foundational surface — translucent panes that layer and compose
	- glass carries depth: foreground, midground, background
	- all components sit on glass. glass defines the spatial hierarchy

## composition model

- four levels, each built from the previous
- ### atoms
	- indivisible visual primitives. cannot be decomposed further
	- [[prysm/glass]] — surface panes (plane, side-button)
	- [[prysm/text]] — typography (left, center, right, paragraph)
	- [[prysm/button]] — call-to-action (default, double, triple, side)
	- [[prysm/toggle]] — binary state (on, off, star)
	- [[prysm/slider]] — continuous value (range selector, progress bar)
	- [[prysm/indicator]] — progress display (partial, full)
	- [[prysm/counter]] — numeric display with [[emotion]] color
	- [[prysm/address]] — [[neuron]] address (big, small)
	- [[prysm/ion]] — icon-label pair in six layouts (centric, horizontal, input, star, trapezoid)
	- [[prysm/saber]] — accent line and divider (1px, 2px, horizontal)
	- [[prysm/images]] — icon library (16, 20, 32, 48, 96 px)
- ### molecules
	- functional components assembled from atoms. each molecule has a clear interface: inputs, outputs, states
	- #### navigation
		- [[prysm/hud]] — heads-up display shell, the persistent navigation frame
		- [[mind]] — navigation awareness indicator
		- [[prysm/tabs]] — section navigation (3, 4, 5 items × desktop, mobile)
	- #### content
		- [[prysm/content]] — [[particle]] renderers by format: heading, text, number, link, picture, video, pdf, audio, avatar
		- [[prysm/display]] — content container (empty, highlight, sized text)
		- [[prysm/neuron-card]] — [[neuron]] identity card (big, small × default, hover, clicked)
		- [[prysm/aip]] — entity card for [[particle]], [[neuron]], [[cyb/avatar]], [[aip]] (2-line, 3-line, +menu)
		- [[prysm/avatar]] — identity strip for [[neuron]]/[[cyb/avatar]] (2-line, chooser)
		- [[prysm/adviser]] — contextual hint (closed, positive, negative, neutral, particle-attached)
	- #### input
		- [[prysm/input]] — data entry (text L/R/LR, neuron, token, select)
		- [[prysm/filter]] — result filtering (3-items, wide)
	- #### data
		- [[prysm/table]] — data grid (line, row-L, row-R, sort, sort/dropdown)
		- [[prysm/bar]] — [[prysm/saber]]+[[prysm/ion]] composite (1-sided, bi-sided, horizontal × button, input, display)
	- #### widgets
		- [[cyb/brain]] — graph file manager widget (+memory variant)
		- [[cyb/sense]] — messaging and notification widget
		- [[cyb/sigma]] — wallet and balance widget
		- [[prysm/time-widget]] — personal history widget
- ### cells
	- full page regions composed from molecules. a cell owns a section of the screen
	- [[prysm/portal-cell]] — onboarding region: citizenship, gift, hud, cyb-map
	- [[prysm/cyberver-cell]] — learning region: hud, mentors, learner, stats, faculties
	- [[prysm/oracle-cell]] — search region: aip selector, mind, particle display, content feed
- ### aips
	- complete autonomous applications. each [[aip]] is a full-screen experience built from cells
	- [[cyb/oracle]] — search and discovery
	- [[cyb/brain]] — graph file manager
	- [[cyb/portal]] — onboarding and citizenship
	- [[cyberver]] — learning incentives and staking
	- [[cyb/sense]] — messaging and notifications
	- [[cyb/sigma]] — wallet and token management
	- [[teleport]] — cross-chain transfers
	- [[sphere]] — 3d graph visualization
	- [[warp]] — IBC bridge
	- [[aos/hfr]] — hydrogen fuel rod management

## interfaces

- every prysm component exposes a consistent interface
- ### inputs
	- data: what the component renders (particle, neuron, number, text)
	- [[emotion]]: color signal computed from protocol state
	- context: parent component, screen position, device type
- ### outputs
	- action: what happens on interaction (navigate, submit, link, select)
	- state change: local mutation (toggle, expand, collapse, hover)
	- [[cyberlink]]: when interaction creates a link in the [[cybergraph]]
- ### states
	- every component has at minimum: default, hover, active, disabled
	- stateful components add: loading, error, empty, expanded
	- emotion overlays any state with a color signal

## properties

- ### color
	- base: dark background, light foreground
	- [[emotion]] palette: green (confidence), red (danger), yellow (attention), blue (information), purple (rare)
	- glass tints: surface depth encoded as opacity gradients
- ### typography
	- monospace foundation: all text renders in a single font family
	- hierarchy through size and weight, never through [[bold]] or decoration
	- sizes: h1 (32), h2 (24), h3 (20), body (16), caption (14), micro (12)
- ### spacing
	- 8px grid: all spacing snaps to multiples of 8
	- component padding: 8, 16, 24
	- section gaps: 24, 32, 48
- ### motion
	- transitions: 150ms ease for state changes
	- glass depth shifts: 200ms ease-out
	- no decorative animation. motion serves state communication
- ### responsive
	- two breakpoints: desktop (>768) and mobile (≤768)
	- molecules adapt: tabs reduce items, widgets stack vertically, cards simplify
	- atoms stay identical across breakpoints

## the prysm and the [[cybergraph]]

- prysm renders the cybergraph for human perception
- every component maps to a protocol concept: [[particle]] → content renderer, [[neuron]] → identity card, [[cyberlink]] → navigation action, [[cyberank]] → ordering
- the design system and the protocol co-evolve: new protocol features require new prysm components, new prysm patterns reveal protocol gaps
- prysm is the visual layer of the [[relevance machine]]