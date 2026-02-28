---
icon: 💎
tags: cyb, prism
alias: design system
crystal-type: entity
crystal-domain: cyber
---

the design system of [[cyb]] — a visual language for interfacing with [[Superintelligence]]

every screen in [[cyb]] is a composition of prism components. the system defines how humans perceive, navigate, and interact with the [[cybergraph]]

## first principles

- ### the interface is a lens
	- [[cyb]] refracts the [[cybergraph]] into something a human can perceive and act on
	- prism decomposes this refraction into composable layers: surface → element → region → application
	- each layer adds meaning without hiding the underlying structure
- ### emotion as signal
	- components carry [[emotion]] — a color-coded signal layer
	- emotion reflects the state of the [[robot]]: confidence, uncertainty, danger, opportunity
	- emotion is computed from [[cyberank]], [[karma]], and context — it is data, rendered as feeling
- ### everything is a [[particle]]
	- every piece of content in [[cyb]] is a [[particle]] — text, image, video, audio, pdf, 3d model
	- prism defines how each particle type is rendered, selected, linked, and composed
	- the renderer adapts to the particle format. the interface stays consistent
- ### the [[neuron]] is the user
	- identity in prism is a [[neuron]] with an [[avatar]]
	- every action traces to a neuron. every view is from a neuron's perspective
	- prism renders identity as cards, addresses, reputation indicators, and activity streams
- ### glass as medium
	- [[glass]] is the foundational surface — translucent panes that layer and compose
	- glass carries depth: foreground, midground, background
	- all components sit on glass. glass defines the spatial hierarchy

## composition model

- four levels, each built from the previous
- ### atoms
	- indivisible visual primitives. cannot be decomposed further
	- [[glass]] — surface panes (plane, side-button)
	- [[prism/text]] — typography (left, center, right, paragraph)
	- [[button]] — call-to-action (default, double, triple, side)
	- [[prism/toggle]] — binary state (on, off, star)
	- [[prism/slider]] — continuous value (range selector, progress bar)
	- [[prism/indicator]] — progress display (partial, full)
	- [[counter]] — numeric display with [[emotion]] color
	- [[prism/address]] — [[neuron]] address (big, small)
	- [[prism/ion]] — icon-label pair in six layouts (centric, horizontal, input, star, trapezoid)
	- [[prism/saber]] — accent line and divider (1px, 2px, horizontal)
	- [[images]] — icon library (16, 20, 32, 48, 96 px)
- ### molecules
	- functional components assembled from atoms. each molecule has a clear interface: inputs, outputs, states
	- #### navigation
		- [[prism/hud]] — heads-up display shell, the persistent navigation frame
		- [[mind]] — navigation awareness indicator
		- [[prism/tabs]] — section navigation (3, 4, 5 items × desktop, mobile)
	- #### content
		- [[prism/content]] — [[particle]] renderers by format: heading, text, number, link, picture, video, pdf, audio, avatar
		- [[prism/display]] — content container (empty, highlight, sized text)
		- [[prism/neuron-card]] — [[neuron]] identity card (big, small × default, hover, clicked)
		- [[prism/object]] — entity card for [[particle]], [[neuron]], [[avatar]], [[aip]] (2-line, 3-line, +menu)
		- [[prism/subject]] — identity strip for [[neuron]]/[[avatar]] (2-line, chooser)
		- [[prism/adviser]] — contextual hint (closed, positive, negative, neutral, particle-attached)
	- #### input
		- [[prism/input]] — data entry (text L/R/LR, neuron, token, select)
		- [[prism/filter]] — result filtering (3-items, wide)
	- #### data
		- [[prism/table]] — data grid (line, row-L, row-R, sort, sort/dropdown)
		- [[prism/bar]] — [[prism/saber]]+[[prism/ion]] composite (1-sided, bi-sided, horizontal × button, input, display)
	- #### widgets
		- [[brain]] — graph file manager widget (+memory variant)
		- [[sense]] — messaging and notification widget
		- [[sigma]] — wallet and balance widget
		- [[prism/time-widget]] — personal history widget
- ### cells
	- full page regions composed from molecules. a cell owns a section of the screen
	- [[prism/portal-cell]] — onboarding region: citizenship, gift, hud, cyb-map
	- [[prism/cyberver-cell]] — learning region: hud, mentors, learner, stats, faculties
	- [[prism/oracle-cell]] — search region: aip selector, mind, particle display, content feed
- ### aips
	- complete autonomous applications. each [[aip]] is a full-screen experience built from cells
	- [[oracle]] — search and discovery
	- [[brain]] — graph file manager
	- [[portal]] — onboarding and citizenship
	- [[cyberver]] — learning incentives and staking
	- [[sense]] — messaging and notifications
	- [[sigma]] — wallet and token management
	- [[teleport]] — cross-chain transfers
	- [[sphere]] — 3d graph visualization
	- [[warp]] — IBC bridge
	- [[hfr]] — hydrogen fuel rod management

## interfaces

- every prism component exposes a consistent interface
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

## the prism and the [[cybergraph]]

- prism renders the cybergraph for human perception
- every component maps to a protocol concept: [[particle]] → content renderer, [[neuron]] → identity card, [[cyberlink]] → navigation action, [[cyberank]] → ordering
- the design system and the protocol co-evolve: new protocol features require new prism components, new prism patterns reveal protocol gaps
- prism is the visual layer of the [[relevance machine]]